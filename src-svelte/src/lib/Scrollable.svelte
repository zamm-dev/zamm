<script lang="ts" context="module">
  export type ResizedEvent = CustomEvent<DOMRect>;
</script>

<script lang="ts">
  import { onMount } from "svelte";
  import { createEventDispatcher } from "svelte";
  import FixedScrollable from "./FixedScrollable.svelte";

  export let minHeight = "8rem";
  export let initialPosition: "top" | "bottom" = "top";
  let scrollableHeight: string = minHeight;
  let container: HTMLDivElement | null = null;
  let scrollable: FixedScrollable | null = null;
  const dispatchResizeEvent = createEventDispatcher();
  const dispatchBottomReachedEvent = createEventDispatcher();

  export function resizeScrollable() {
    if (!scrollable) {
      return;
    }

    scrollableHeight = minHeight;
    requestAnimationFrame(() => {
      if (!container || !scrollable) {
        console.warn("Container or scrollable not mounted");
        return;
      }

      const newHeight = Math.floor(container.getBoundingClientRect().height);
      scrollableHeight = `${newHeight}px`;
      dispatchResizeEvent("resize", scrollable.getDimensions());
    });
  }

  export function scrollToBottom() {
    scrollable?.scrollToBottom();
  }

  function bottomReached() {
    dispatchBottomReachedEvent("bottomReached");
  }

  onMount(() => {
    resizeScrollable();
    const windowResizeCallback = () => resizeScrollable();
    window.addEventListener("resize", windowResizeCallback);

    return () => {
      window.removeEventListener("resize", windowResizeCallback);
    };
  });
</script>

<div class="growable container composite-reveal" bind:this={container}>
  <FixedScrollable
    {initialPosition}
    maxHeight={scrollableHeight}
    bind:this={scrollable}
    on:bottomReached={bottomReached}
    {...$$restProps}
  >
    <slot />
  </FixedScrollable>
</div>

<style>
  .container {
    flex-grow: 1;
  }
</style>
