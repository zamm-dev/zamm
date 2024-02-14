<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
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
  let conversationContainer: HTMLDivElement | undefined = undefined;
  let conversationView: HTMLDivElement | undefined = undefined;
  let topIndicator: HTMLDivElement;
  let bottomIndicator: HTMLDivElement;
  let topShadow: HTMLDivElement;
  let bottomShadow: HTMLDivElement;

  onMount(() => {
    resizeConversationView();
    window.addEventListener("resize", resizeConversationView);

    let topScrollObserver = new IntersectionObserver(
      intersectionCallback(topShadow),
    );
    topScrollObserver.observe(topIndicator);
    let bottomScrollObserver = new IntersectionObserver(
      intersectionCallback(bottomShadow),
    );
    bottomScrollObserver.observe(bottomIndicator);

    return () => {
      window.removeEventListener("resize", resizeConversationView);
      topScrollObserver.disconnect();
      bottomScrollObserver.disconnect();
    };
  });

  function intersectionCallback(shadow: HTMLDivElement) {
    return (entries: IntersectionObserverEntry[]) => {
      let indicator = entries[0];
      if (indicator.isIntersecting) {
        shadow.classList.remove("visible");
      } else {
        shadow.classList.add("visible");
      }
    };
  }

  function showChatBottom() {
    if (conversationView) {
      conversationView.scrollTop = conversationView.scrollHeight;
    }
  }

  function resizeConversationView() {
    if (conversationView) {
      conversationView.style.maxHeight = "8rem";
      requestAnimationFrame(() => {
        if (conversationView && conversationContainer) {
          conversationView.style.maxHeight = `${conversationContainer.clientHeight}px`;
          if (showMostRecentMessage) {
            showChatBottom();
          }
        }
      });
    }
  }

  async function sendChatMessage(message: string) {
    if (expectingResponse) {
      return;
    }

    const chatMessage: ChatMessage = {
      role: "Human",
      text: message,
    };
    conversation = [...conversation, chatMessage];
    expectingResponse = true;
    setTimeout(showChatBottom, 50);

    try {
      let llmCall = await chat("OpenAI", "gpt-4", null, conversation);
      conversation = [...conversation, llmCall.response.completion];
      setTimeout(showChatBottom, 50);
    } catch (err) {
      snackbarError(err as string);
    } finally {
      expectingResponse = false;
    }
  }
</script>

<InfoBox title="Chat" fullHeight>
  <div class="chat-container composite-reveal">
    <div
      class="conversation-container composite-reveal"
      bind:this={conversationContainer}
    >
      <div class="shadow top" bind:this={topShadow}></div>
      <div
        class="conversation composite-reveal"
        role="list"
        bind:this={conversationView}
      >
        <div class="indicator top" bind:this={topIndicator}></div>
        {#if conversation.length > 1}
          {#each conversation.slice(1) as message}
            <Message {message} />
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
        <div class="indicator bottom" bind:this={bottomIndicator}></div>
      </div>
      <div class="shadow bottom" bind:this={bottomShadow}></div>
    </div>

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

  .conversation-container {
    flex-grow: 1;
    position: relative;
  }

  .conversation {
    max-height: 8rem;
    overflow-y: auto;
    position: relative;
  }

  .empty-conversation {
    color: var(--color-faded);
    font-size: 0.85rem;
    font-style: italic;
    text-align: center;
  }

  .shadow {
    z-index: 1;
    height: 0.375rem;
    width: 100%;
    position: absolute;
    display: none;
  }

  .conversation-container :global(.shadow.visible) {
    display: block;
  }

  .shadow.top {
    top: 0;
    background-image: radial-gradient(
      farthest-side at 50% 0%,
      rgba(150, 150, 150, 0.4) 0%,
      rgba(0, 0, 0, 0) 100%
    );
  }

  .shadow.bottom {
    bottom: 0;
    background-image: radial-gradient(
      farthest-side at 50% 100%,
      rgba(150, 150, 150, 0.4) 0%,
      rgba(0, 0, 0, 0) 100%
    );
  }

  .indicator {
    height: 1px;
    width: 100%;
  }

  .indicator.top {
    margin-bottom: -1px;
  }

  .indicator.bottom {
    margin-top: -1px;
  }
</style>
