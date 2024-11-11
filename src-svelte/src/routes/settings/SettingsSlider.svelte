<script lang="ts">
  import Slider from "$lib/Slider.svelte";

  interface Props {
    label: string;
    min?: number;
    max: number;
    step?: number | undefined;
    value?: number;
    onUpdate?: (newValue: number) => void;
  }

  let {
    label,
    min = 0,
    max,
    step = undefined,
    value = $bindable(min),
    onUpdate = () => undefined,
  }: Props = $props();
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
    background: var(--color-hover);
  }
</style>
