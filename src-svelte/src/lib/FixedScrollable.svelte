<script lang="ts">
  import { onMount } from "svelte";
  import { createEventDispatcher } from "svelte";

  interface Props {
    maxHeight: string;
    initialPosition?: "top" | "bottom";
    autoscroll?: boolean;
    scrollDelay?: number;
    children?: import("svelte").Snippet;
  }

  let {
    maxHeight,
    initialPosition = "top",
    autoscroll = false,
    scrollDelay = 0,
    children,
  }: Props = $props();
  let scrollContents: HTMLDivElement | undefined = $state(undefined);
  let scrollInterval: NodeJS.Timeout | undefined = undefined;
  let topIndicator: HTMLDivElement | undefined = $state();
  let bottomIndicator: HTMLDivElement | undefined = $state();
  let topShadow: HTMLDivElement | undefined = $state();
  let bottomShadow: HTMLDivElement | undefined = $state();
  const dispatchBottomReachedEvent = createEventDispatcher();

  export function getDimensions() {
    if (scrollContents) {
      // we need dimensions after taking scrollbar into account
      return {
        width: scrollContents.clientWidth,
        height: scrollContents.clientHeight,
      };
    }

    throw new Error("Scrollable component not mounted");
  }

  export function scrollToBottom() {
    if (scrollContents) {
      scrollContents.scrollTop = scrollContents.scrollHeight;
    }
  }

  function intersectionCallback(shadow: HTMLDivElement | undefined) {
    return (entries: IntersectionObserverEntry[]) => {
      if (!shadow) {
        console.warn("Shadow not mounted");
        return;
      }

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
    if (!topIndicator || !bottomIndicator) {
      throw new Error("Scrollable component not mounted");
    }

    let topScrollObserver = new IntersectionObserver(
      intersectionCallback(topShadow),
    );
    topScrollObserver.observe(topIndicator);
    let bottomScrollObserver = new IntersectionObserver(
      (entries: IntersectionObserverEntry[]) => {
        intersectionCallback(bottomShadow)(entries);

        let indicator = entries[0];
        if (indicator.isIntersecting) {
          dispatchBottomReachedEvent("bottomReached");
        }
      },
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

  let style = $derived(`max-height: ${maxHeight}`);

  const children_render = $derived(children);
</script>

<div class="scrollable composite-reveal" {style}>
  <div class="shadow top" bind:this={topShadow}></div>
  <div
    class="scroll-contents composite-reveal"
    {style}
    bind:this={scrollContents}
    onmousedown={stopScrolling}
    onwheel={stopScrolling}
    role="none"
  >
    <div class="indicator top" bind:this={topIndicator}></div>
    {@render children_render?.()}
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
