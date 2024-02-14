<script lang="ts">
  import Slider from "$lib/Slider.svelte";

  export let label: string;
  export let min = 0;
  export let max: number;
  export let step: number | undefined = undefined;
  export let value: number = min;
  export let onUpdate: (newValue: number) => void = () => undefined;
</script>

<div class="settings-slider container">
  <Slider {label} {min} {max} {step} {onUpdate} bind:value />
</div>

<style>
  .container {
    padding: calc(0.5 * var(--side-padding)) var(--side-padding);
    border-radius: var(--corner-roundness);
    transition: background 0.5s;

    --label-height: 1.5em;
    --toggle-height: calc(1.2 * var(--label-height));
    min-height: var(--toggle-height);
    display: flex;
    align-items: center;
  }

  .container:hover {
    background: hsla(60, 100%, 50%, 0.2);
  }
</style>
