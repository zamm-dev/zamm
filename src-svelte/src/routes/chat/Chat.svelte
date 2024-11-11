<script lang="ts" module>
  import { writable } from "svelte/store";
  import { type ChatMessage } from "$lib/bindings";

  export const DEFAULT_SYSTEM_MESSAGE: ChatMessage = {
    role: "System",
    text: "You are ZAMM, a chat program. Respond in first person.",
  };

  export const lastMessageId = writable<string | undefined>(undefined);
  export const conversation = writable<ChatMessage[]>([DEFAULT_SYSTEM_MESSAGE]);
  export const nextChatMessage = writable<string>("");

  export function resetConversation() {
    lastMessageId.set(undefined);
    conversation.set([DEFAULT_SYSTEM_MESSAGE]);
    nextChatMessage.set("");
  }
</script>

<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
  import Scrollable, { type ResizedEvent } from "$lib/Scrollable.svelte";
  import Message from "./Message.svelte";
  import TypingIndicator from "./TypingIndicator.svelte";
  import { commands } from "$lib/bindings";
  import { unwrap } from "$lib/tauri";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";
  import EmptyPlaceholder from "$lib/EmptyPlaceholder.svelte";
  import SendInputForm from "$lib/controls/SendInputForm.svelte";

  interface Props {
    expectingResponse?: boolean;
    showMostRecentMessage?: boolean;
  }

  let {
    expectingResponse = $bindable(false),
    showMostRecentMessage = true,
  }: Props = $props();
  let messageComponents: Message[] = $state([]);
  let growable: Scrollable | undefined = $state();
  let chatContainer: HTMLDivElement | undefined = $state(undefined);
  let conversationWidthPx = 100;

  function resizeConversationView() {
    growable?.resizeScrollable();
  }

  async function onScrollableResized(e: ResizedEvent) {
    conversationWidthPx = e.detail.width;
    const resizePromises = messageComponents.map((message) =>
      message.resizeBubble(e.detail.width),
    );
    await Promise.all(resizePromises);
    if (showMostRecentMessage) {
      growable?.scrollToBottom();
    }
    if (!chatContainer) {
      console.warn("Chat container not initialized");
      return;
    }
    chatContainer.dispatchEvent(
      new CustomEvent("info-box-update", { bubbles: true }),
    );
  }

  function appendMessage(message: ChatMessage) {
    conversation.update((messages) => [...messages, message]);
    setTimeout(async () => {
      const latestMessage = messageComponents[messageComponents.length - 1];
      await latestMessage?.resizeBubble(conversationWidthPx);
      if (growable?.scrollToBottom) {
        growable.scrollToBottom();
      }
    }, 10);
  }

  async function sendChatMessage(message: string) {
    if (expectingResponse) {
      return;
    }

    const chatMessage: ChatMessage = {
      role: "Human",
      text: message,
    };
    appendMessage(chatMessage);
    expectingResponse = true;

    try {
      let llmCall = await unwrap(
        commands.chat({
          provider: "OpenAI",
          llm: "gpt-4",
          temperature: null,
          previous_call_id: $lastMessageId,
          prompt: $conversation,
        }),
      );
      lastMessageId.set(llmCall.id);
      appendMessage(llmCall.response_message);
    } catch (err) {
      snackbarError(err as string);
    } finally {
      expectingResponse = false;
    }
  }
</script>

<InfoBox title="Chat" fullHeight>
  <div
    class="chat-container composite-reveal full-height"
    bind:this={chatContainer}
  >
    <Scrollable
      initialPosition={showMostRecentMessage ? "bottom" : "top"}
      on:resize={onScrollableResized}
      bind:this={growable}
    >
      <div class="composite-reveal" role="list">
        {#if $conversation.length > 1}
          {#each $conversation.slice(1) as message, i (i)}
            <Message {message} bind:this={messageComponents[i]} />
          {/each}
          {#if expectingResponse}
            <TypingIndicator />
          {/if}
        {:else}
          <EmptyPlaceholder>
            This conversation is currently empty.<br />Get it started by typing
            a message below.
          </EmptyPlaceholder>
        {/if}
      </div>
    </Scrollable>

    <SendInputForm
      accessibilityLabel="Chat with the AI:"
      sendInput={sendChatMessage}
      isBusy={expectingResponse}
      bind:currentMessage={$nextChatMessage}
      onTextInputResize={resizeConversationView}
    />
  </div>
</InfoBox>

<style>
  .chat-container {
    height: 100%;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
</style>
