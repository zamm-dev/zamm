<script lang="ts">
  import { onMount } from "svelte";

  export let maxHeight: string;
  export let initialPosition: "top" | "bottom" = "top";
  export let autoscroll = false;
  export let scrollDelay = 0;
  let scrollContents: HTMLDivElement | undefined = undefined;
  let scrollInterval: NodeJS.Timeout | undefined = undefined;
  let topIndicator: HTMLDivElement;
  let bottomIndicator: HTMLDivElement;
  let topShadow: HTMLDivElement;
  let bottomShadow: HTMLDivElement;

  export function getDimensions() {
    if (scrollContents) {
      return scrollContents.getBoundingClientRect();
    }

    throw new Error("Scrollable component not mounted");
  }

  export function scrollToBottom() {
    if (scrollContents) {
      scrollContents.scrollTop = scrollContents.scrollHeight;
    }
  }

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

  function stopScrolling() {
    if (scrollInterval) {
      clearInterval(scrollInterval);
    }
  }

  onMount(() => {
    let topScrollObserver = new IntersectionObserver(
      intersectionCallback(topShadow),
    );
    topScrollObserver.observe(topIndicator);
    let bottomScrollObserver = new IntersectionObserver(
      intersectionCallback(bottomShadow),
    );
    bottomScrollObserver.observe(bottomIndicator);

    if (initialPosition === "bottom") {
      scrollToBottom();

      // hack for Storybook screenshot test on Webkit
      setTimeout(() => scrollToBottom(), 10);
    }

    if (autoscroll) {
      setTimeout(() => {
        scrollInterval = setInterval(() => {
          if (scrollContents) {
            scrollContents.scrollBy(0, 1);
          }
        }, 10);
      }, scrollDelay);
    }

    return () => {
      topScrollObserver.disconnect();
      bottomScrollObserver.disconnect();
      stopScrolling();
    };
  });

  $: style = `max-height: ${maxHeight}`;
</script>

<div class="scrollable composite-reveal" {style}>
  <div class="shadow top" bind:this={topShadow}></div>
  <div
    class="scroll-contents composite-reveal"
    {style}
    bind:this={scrollContents}
    on:mousedown={stopScrolling}
    on:wheel={stopScrolling}
    role="none"
  >
    <div class="indicator top" bind:this={topIndicator}></div>
    <slot />
    <div class="indicator bottom" bind:this={bottomIndicator}></div>
  </div>
  <div class="shadow bottom" bind:this={bottomShadow}></div>
</div>

<style>
  .scrollable {
    position: relative;
  }

  .scrollable :global(.shadow.visible) {
    display: block;
  }

  .scroll-contents {
    overflow-y: auto;
  }

  :global(.wait-for-infobox) .scroll-contents {
    scrollbar-color: transparent transparent;
  }

  .shadow {
    z-index: 1;
    height: 0.375rem;
    width: 100%;
    position: absolute;
    display: none;
  }

  :global(.wait-for-infobox .shadow.bottom.visible),
  :global(.wait-for-infobox .shadow.top.visible) {
    display: none;
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
