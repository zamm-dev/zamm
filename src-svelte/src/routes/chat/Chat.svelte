<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
  import Scrollable, { type ResizedEvent } from "$lib/Scrollable.svelte";
  import Message from "./Message.svelte";
  import TypingIndicator from "./TypingIndicator.svelte";
  import { type ChatMessage, chat } from "$lib/bindings";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";
  import Form from "./Form.svelte";
  import { onMount } from "svelte";

  export let initialMessage = "";
  export let conversation: ChatMessage[] = [
    {
      role: "System",
      text: "You are ZAMM, a chat program. Respond in first person.",
    },
  ];
  export let expectingResponse = false;
  export let showMostRecentMessage = true;
  let initialMount = true;
  let messageComponents: Message[] = [];
  let growable: Scrollable | undefined;
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
    if (initialMount && showMostRecentMessage) {
      growable?.scrollToBottom();
    }
  }

  function appendMessage(message: ChatMessage) {
    conversation = [...conversation, message];
    setTimeout(async () => {
      const latestMessage = messageComponents[messageComponents.length - 1];
      await latestMessage?.resizeBubble(conversationWidthPx);
      growable?.scrollToBottom();
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
      let llmCall = await chat("OpenAI", "gpt-4", null, conversation);
      appendMessage(llmCall.response.completion);
    } catch (err) {
      snackbarError(err as string);
    } finally {
      expectingResponse = false;
    }
  }

  onMount(() => {
    setTimeout(() => {
      // hack: Storybook window resize doesn't cause remount
      initialMount = false;
    }, 1_000);
  });
</script>

<InfoBox title="Chat" fullHeight>
  <div class="chat-container composite-reveal">
    <Scrollable
      initialPosition={showMostRecentMessage ? "bottom" : "top"}
      on:resize={onScrollableResized}
      bind:this={growable}
    >
      <div class="composite-reveal" role="list">
        {#if conversation.length > 1}
          {#each conversation.slice(1) as message, i (i)}
            <Message
              {message}
              {conversationWidthPx}
              bind:this={messageComponents[i]}
            />
          {/each}
          {#if expectingResponse}
            <TypingIndicator />
          {/if}
        {:else}
          <p class="empty-conversation atomic-reveal">
            This conversation is currently empty.<br />Get it started by typing
            a message below.
          </p>
        {/if}
      </div>
    </Scrollable>

    <Form
      {sendChatMessage}
      currentMessage={initialMessage}
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

  .empty-conversation {
    color: var(--color-faded);
    font-size: 0.85rem;
    font-style: italic;
    text-align: center;
  }
</style>
