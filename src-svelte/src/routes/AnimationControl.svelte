<script lang="ts">
  import {
    animationSpeed,
    animationsOn,
    standardDuration,
  } from "$lib/preferences";
  interface Props {
    children?: import("svelte").Snippet;
  }

  let { children }: Props = $props();

  let standardDurationMs = $derived($standardDuration.toFixed(2) + "ms");
  let style = $derived(
    `--base-animation-speed: ${$animationSpeed}; ` +
      `--standard-duration: ${standardDurationMs};`,
  );

  const children_render = $derived(children);
</script>

<div
  id="animation-control"
  class="full-height"
  class:animations-disabled={!$animationsOn}
  {style}
>
  {@render children_render?.()}
</div>

<style>
  #animation-control {
    height: 100%;
    position: relative;
  }

  #animation-control.animations-disabled :global(*) {
    animation-play-state: paused !important;
    transition: none !important;
  }
</style>
