<script lang="ts">
  import { onMount } from "svelte";

  export let maxHeight: string;
  export let initialPosition: "top" | "bottom" = "top";
  let scrollContents: HTMLDivElement | undefined = undefined;
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

    return () => {
      topScrollObserver.disconnect();
      bottomScrollObserver.disconnect();
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

  .shadow {
    z-index: 1;
    height: 0.375rem;
    width: 100%;
    position: absolute;
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
