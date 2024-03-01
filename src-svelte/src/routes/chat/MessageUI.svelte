<script lang="ts">
  import { onMount } from "svelte";
  import type { Writable } from "svelte/store";

  export let role: "System" | "Human" | "AI";
  export let conversationWidthPx: Writable<number> | undefined = undefined;
  const classList = `message atomic-reveal ${role.toLowerCase()}`;
  let textElement: HTMLDivElement | null;

  let initialResizeTimeoutId: ReturnType<typeof setTimeout> | undefined;
  let finalResizeTimeoutId: ReturnType<typeof setTimeout> | undefined;

  // chat window size at which the message bubble should be full width
  const MIN_FULL_WIDTH_PX = 400;
  const MAX_WIDTH_PX = 600;

  function maxMessageWidth(chatWidthPx: number) {
    if (chatWidthPx <= MIN_FULL_WIDTH_PX) {
      return chatWidthPx;
    }

    const fractionalWidth = Math.max(0.8 * chatWidthPx, MIN_FULL_WIDTH_PX);
    return Math.min(fractionalWidth, MAX_WIDTH_PX);
  }

  function resizeBubble(chatWidthPx: number) {
    if (chatWidthPx > 0 && textElement) {
      try {
        textElement.style.width = "";

        const maxWidth = maxMessageWidth(chatWidthPx);
        const currentWidth = textElement.getBoundingClientRect().width;
        const newWidth = Math.min(currentWidth, maxWidth);
        textElement.style.width = `${newWidth}px`;

        if (finalResizeTimeoutId) {
          clearTimeout(finalResizeTimeoutId);
        }
        finalResizeTimeoutId = setTimeout(() => {
          if (textElement) {
            const range = document.createRange();
            range.selectNodeContents(textElement);
            const textRect = range.getBoundingClientRect();
            const actualTextWidth = textRect.width;

            const finalWidth = Math.min(actualTextWidth, newWidth);
            textElement.style.width = `${finalWidth}px`;
          }
        }, 10);
      } catch (err) {
        console.warn("Cannot resize chat message bubble: ", err);
      }
    }
  }

  onMount(() => {
    conversationWidthPx?.subscribe((chatWidthPx) => {
      if (initialResizeTimeoutId) {
        clearTimeout(initialResizeTimeoutId);
      }
      initialResizeTimeoutId = setTimeout(() => resizeBubble(chatWidthPx), 100);
    });
  });
</script>

<div class={classList} role="listitem">
  <div class="arrow"></div>
  <div class="text-container">
    <div class="text" bind:this={textElement}>
      <slot />
    </div>
  </div>
</div>

<style>
  .message {
    --message-color: gray;
    --arrow-size: 0.5rem;
    position: relative;
  }

  .message .text-container {
    margin: 0.5rem var(--arrow-size);
    border-radius: var(--corner-roundness);
    width: fit-content;
    padding: 0.75rem;
    box-sizing: border-box;
    background-color: var(--message-color);
    text-align: left;
  }

  .text {
    box-sizing: content-box;
    max-width: 600px;
  }

  /* this takes sidebar width into account */
  @media (max-width: 635px) {
    .text {
      max-width: 400px;
    }
  }

  @media (min-width: 635px) {
    .message .text-container {
      max-width: calc(80% + 2.1rem);
    }
  }

  .message .arrow {
    position: absolute;
    width: 0;
    height: 0;
    bottom: 0.75rem;
    border: var(--arrow-size) solid transparent;
  }

  .message.human {
    --message-color: #e5ffe5;
  }

  .message.human .text-container {
    margin-left: auto;
  }

  .message.human .arrow {
    right: 1px;
    border-right: none;
    border-left-color: var(--message-color);
  }

  .message.ai {
    --message-color: #e5e5ff;
  }

  .message.ai .text-container {
    margin-right: auto;
  }

  .message.ai .arrow {
    left: 1px;
    border-left: none;
    border-right-color: var(--message-color);
  }
</style>
