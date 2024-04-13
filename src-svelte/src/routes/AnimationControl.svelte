<script lang="ts">
  import {
    animationSpeed,
    animationsOn,
    standardDuration,
  } from "$lib/preferences";

  $: standardDurationMs = $standardDuration.toFixed(2) + "ms";
  $: style =
    `--base-animation-speed: ${$animationSpeed}; ` +
    `--standard-duration: ${standardDurationMs};`;
</script>

<div
  id="animation-control"
  class="full-height"
  class:animations-disabled={!$animationsOn}
  {style}
>
  <slot />
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
