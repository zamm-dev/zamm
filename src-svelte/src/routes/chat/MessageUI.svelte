<script lang="ts">
  import { onMount } from "svelte";

  export let role: "System" | "Human" | "AI";
  const classList = `message atomic-reveal ${role.toLowerCase()}`;
  let textElement: HTMLDivElement | null;

  onMount(() => {
    setTimeout(() => {
      if (textElement) {
        try {
          const range = document.createRange();
          range.selectNodeContents(textElement);
          const textRect = range.getBoundingClientRect();
          textElement.style.width = `${textRect.width}px`;
        } catch (err) {
          console.warn("Cannot resize chat message bubble: ", err);
        }
      }
    }, 10);
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
    max-width: 60%;
    padding: 0.75rem;
    box-sizing: border-box;
    background-color: var(--message-color);
    white-space: pre-line;
    text-align: left;
  }

  .text {
    box-sizing: content-box;
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
