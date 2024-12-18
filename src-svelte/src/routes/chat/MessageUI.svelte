<script lang="ts" module>
  export function styleKhmer(text: string) {
    var newText = "";
    var isKhmer = false;
    for (let i = 0; i < text.length; i++) {
      var ch = text.charAt(i);
      if (ch >= "\u1780" && ch <= "\u17FF") {
        if (!isKhmer) {
          newText += '<span class="khmer">';
        }
        isKhmer = true;
      } else {
        if (isKhmer) {
          newText += "</span>";
        }
        isKhmer = false;
      }
      newText += ch;
    }
    if (isKhmer) {
      newText += "</span>";
    }
    return newText;
  }
</script>

<script lang="ts">
  import { onMount } from "svelte";

  interface Props {
    role: "System" | "Human" | "AI";
    forceHighlight?: boolean;
    children?: import("svelte").Snippet;
  }

  let { role, forceHighlight = false, children }: Props = $props();
  const classList = `message atomic-reveal ${role.toLowerCase()}`;
  let textElement: HTMLDivElement | null = $state(null);

  let finalResizeTimeoutId: ReturnType<typeof setTimeout> | undefined;

  const remPx = 18;
  // arrow size, left padding, and right padding
  const messagePaddingPx = (0.5 + 0.75 + 0.75) * remPx;
  // chat window size at which the message bubble should be full width
  const MIN_FULL_WIDTH_PX = 400;
  const MAX_WIDTH_PX = 600;

  function maxMessageWidth(chatWidthPx: number) {
    const availableWidthPx = chatWidthPx - messagePaddingPx;
    if (availableWidthPx <= MIN_FULL_WIDTH_PX) {
      return Math.ceil(availableWidthPx);
    }

    const fractionalWidth = Math.max(0.8 * availableWidthPx, MIN_FULL_WIDTH_PX);
    return Math.ceil(Math.min(fractionalWidth, MAX_WIDTH_PX));
  }

  function resetChildren(textElement: HTMLDivElement) {
    const pElements = textElement.querySelectorAll("p");
    pElements.forEach((pElement) => {
      pElement.style.width = "";
    });

    const codeElements = textElement.querySelectorAll<HTMLDivElement>(".code");
    codeElements.forEach((codeElement) => {
      codeElement.style.width = "";
    });
  }

  function resizeChildren(textElement: HTMLDivElement, maxWidth: number) {
    const pElements = textElement.querySelectorAll("p");
    pElements.forEach((pElement) => {
      const range = document.createRange();
      range.selectNodeContents(pElement);
      const textRect = range.getBoundingClientRect();
      const actualTextWidth = Math.ceil(textRect.width);

      pElement.style.width = `${actualTextWidth}px`;
    });

    const codeElements = textElement.querySelectorAll<HTMLDivElement>(".code");
    codeElements.forEach((codeElement) => {
      let existingWidth = codeElement.getBoundingClientRect().width;
      if (existingWidth > maxWidth) {
        codeElement.style.width = `${maxWidth}px`;
      }
    });
  }

  export async function resizeBubble(chatWidthPx: number) {
    if (chatWidthPx > 0 && textElement) {
      try {
        const markdownElement =
          textElement.querySelector<HTMLDivElement>(".markdown");
        if (!markdownElement) {
          return;
        }

        resetChildren(markdownElement);

        const maxPotentialWidth = maxMessageWidth(chatWidthPx);
        const currentWidth = markdownElement.getBoundingClientRect().width;
        const maxActualWidth = Math.ceil(
          Math.min(currentWidth, maxPotentialWidth),
        );
        markdownElement.style.width = `${maxActualWidth}px`;

        if (finalResizeTimeoutId) {
          clearTimeout(finalResizeTimeoutId);
        }
        finalResizeTimeoutId = setTimeout(() => {
          resizeChildren(markdownElement, maxActualWidth);
          markdownElement.style.width = "";
        }, 100);
      } catch (err) {
        console.warn("Cannot resize chat message bubble: ", err);
      }
    }
  }

  onMount(() => {
    for (const element of textElement?.querySelectorAll("p") ?? []) {
      element.innerHTML = styleKhmer(element.innerHTML);
    }

    return () => {
      if (finalResizeTimeoutId) {
        clearTimeout(finalResizeTimeoutId);
      }
    };
  });

  const children_render = $derived(children);
</script>

<div class={classList} class:force-highlight={forceHighlight} role="listitem">
  <div class="arrow"></div>
  <div class="text-container">
    <div class="text" bind:this={textElement}>
      {@render children_render?.()}
    </div>
  </div>
</div>

<style>
  .message {
    --message-color: gray;
    --arrow-size: 0.5rem;
    --internal-spacing: 0.75rem;
    position: relative;
  }

  .message :global(.code) {
    transition: box-shadow var(--standard-duration);
  }

  .message.human :global(.code) {
    box-shadow: inset 0.02rem 0.02rem 0.3rem 0.1rem rgba(0, 102, 0, 0.1);
  }

  .message.human :global(.code:hover),
  .message.human.force-highlight :global(.code) {
    box-shadow:
      inset 0.02rem 0.02rem 0.3rem 0.1rem rgba(0, 102, 0, 0.2),
      0.02rem 0.02rem 0.3rem 0.1rem rgba(0, 102, 0, 0.05);
  }

  .message.ai :global(.code) {
    box-shadow: inset 0.02rem 0.02rem 0.3rem 0.1rem rgba(0, 0, 102, 0.1);
  }

  .message.ai :global(.code:hover),
  .message.ai.force-highlight :global(.code) {
    box-shadow:
      inset 0.02rem 0.02rem 0.3rem 0.1rem rgba(0, 0, 102, 0.2),
      0.02rem 0.02rem 0.3rem 0.1rem rgba(0, 0, 102, 0.05);
  }

  .message .text-container {
    margin: 0.5rem var(--arrow-size);
    border-radius: var(--corner-roundness);
    width: fit-content;
    padding: var(--internal-spacing);
    box-sizing: border-box;
    background-color: var(--message-color);
    text-align: left;
  }

  .message .text-container :global(p span.khmer) {
    font-size: 1.2rem;
  }

  .message:first-child .text-container {
    margin-top: 0;
  }

  .message:last-child .text-container {
    margin-bottom: 0;
  }

  .text {
    box-sizing: content-box;
    width: fit-content;
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
    --message-color: var(--color-human);
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
    --message-color: var(--color-ai);
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
