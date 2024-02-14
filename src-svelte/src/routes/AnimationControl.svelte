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

<div class="container" class:animations-disabled={!$animationsOn} {style}>
  <slot />
</div>

<style>
  .container {
    height: 100%;
    position: relative;
  }

  .container.animations-disabled :global(*) {
    animation-play-state: paused !important;
    transition: none !important;
  }
</style>
