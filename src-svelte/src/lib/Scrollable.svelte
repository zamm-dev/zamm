<script lang="ts" module>
  export type ResizedEvent = CustomEvent<DOMRect>;
</script>

<script lang="ts">
  import { onMount } from "svelte";
  import { createEventDispatcher } from "svelte";
  import FixedScrollable from "./FixedScrollable.svelte";

  interface Props {
    minHeight?: string;
    initialPosition?: "top" | "bottom";
    children?: import("svelte").Snippet;
    [key: string]: any;
  }

  let {
    minHeight = "8rem",
    initialPosition = "top",
    children,
    ...rest
  }: Props = $props();
  let scrollableHeight: string = $state(minHeight);
  let container: HTMLDivElement | null = $state(null);
  let scrollable: FixedScrollable | null = $state(null);
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
      if (scrollable?.getDimensions) {
        // CI environment guard
        dispatchResizeEvent("resize", scrollable.getDimensions());
      } else {
        console.warn("scrollable.getDimensions() is not a function");
      }
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

  const children_render = $derived(children);
</script>

<div class="growable container composite-reveal" bind:this={container}>
  <FixedScrollable
    {initialPosition}
    maxHeight={scrollableHeight}
    bind:this={scrollable}
    on:bottomReached={bottomReached}
    {...rest}
  >
    {@render children_render?.()}
  </FixedScrollable>
</div>

<style>
  .container {
    flex-grow: 1;
  }
</style>
