<script lang="ts">
  import { onMount } from "svelte";
  import getComponentId from "./label-id";
  import {
    draggable,
    type DragOptions,
    type DragEventData,
  } from "@neodrag/svelte";

  const rootFontSize = parseFloat(
    getComputedStyle(document.documentElement).fontSize,
  );
  const sliderId = getComponentId("slider");
  const transitionAnimation =
    `transition: ` + `transform var(--standard-duration) ease-out;`;
  const overshoot = 0.4 * rootFontSize; // how much overshoot to allow per-side

  export let label: string | undefined = undefined;
  export let min = 0;
  export let max: number;
  export let step: number | undefined = undefined;
  export let value: number = min;
  export let onUpdate: (newValue: number) => void = () => undefined;
  const range = max - min;
  let track: HTMLDivElement | null;
  let toggleBound: HTMLDivElement | null;
  let toggleLabel: HTMLDivElement | null;
  let leeway = 0;
  let left = 0;
  let transition = transitionAnimation;
  // needed because unlike Neodrag, we want the class to apply as soon as mousedown
  // happens
  let dragging = false;

  let toggleDragOptions: DragOptions = {
    axis: "x",
    bounds: () => {
      if (!toggleBound) {
        throw new Error("Toggle bound not mounted");
      }
      return toggleBound;
    },
    inverseScale: 1,
    // need to set this for neodrag to know that this is a controlled drag
    position: { x: 0, y: 0 },
    render: (data: DragEventData) => {
      left = data.offsetX;
    },
    onDragStart: () => {
      transition = "";
      dragging = true;
    },
    onDragEnd: (data: DragEventData) => {
      transition = transitionAnimation;
      const newValue = calculateValue(data.offsetX);
      updateValue(newValue);
      // for some reason, unlike in Switch.svelte, onClick runs after onDragEnd
      // so we need to wait a bit to stop the dragging
      setTimeout(() => (dragging = false), 100);
    },
  };

  function updateValue(newValue: number) {
    const minStep = step || 0.01;
    newValue = Math.round(newValue / minStep) * minStep;
    if (newValue < min) {
      newValue = min;
    } else if (newValue > max) {
      newValue = max;
    }

    if (newValue !== value) {
      value = newValue;
      try {
        onUpdate(newValue);
      } catch (e) {
        console.error(`Error in callback: ${e}`);
      }
    }

    // necessary in case we overshoot regular bounds
    toggleDragOptions = calculatePosition(value);
  }

  function toPercentage(value: number) {
    return (value - min) / range;
  }

  function calculateValue(position: number) {
    const percentageValue = position / leeway;
    return min + range * percentageValue;
  }

  function calculatePosition(value: number) {
    if (!track || !toggleLabel) {
      return toggleDragOptions;
    }

    leeway = track.clientWidth - toggleLabel.clientWidth;
    const x = leeway * toPercentage(value);
    return {
      ...toggleDragOptions,
      position: { x, y: 0 },
    };
  }

  function handleResize() {
    // disable transition temporarily to avoid progress bar and thumb going out of sync
    transition = "";
    toggleDragOptions = calculatePosition(value);
    setTimeout(() => (transition = transitionAnimation), 100);
  }

  function onClick(e: MouseEvent) {
    if (dragging) {
      return;
    }

    if (!track || !toggleLabel) {
      console.warn("Click event fired on non-existing track or toggle");
      return;
    }

    // toggle midpoint should go where cursor is, at least as much as possible
    // what calculateValue expects though is the left edge of the toggle
    const toggleTargetLeft =
      e.clientX - toggleLabel.getBoundingClientRect().width / 2;
    const offsetX = toggleTargetLeft - track.getBoundingClientRect().left;
    const newValue = calculateValue(offsetX);
    updateValue(newValue);
  }

  function onKeyPress(e: KeyboardEvent) {
    const finalStepSize = step || range / 10.0;
    if (e.key === "ArrowLeft" || e.key === "ArrowDown") {
      updateValue(value - finalStepSize);
    } else if (e.key === "ArrowRight" || e.key === "ArrowUp") {
      updateValue(value + finalStepSize);
    }
  }

  onMount(() => {
    toggleDragOptions = calculatePosition(value);

    if (track) {
      let observer = new ResizeObserver(handleResize);
      observer.observe(track);

      return () => observer.disconnect();
    }
  });

  $: left = toggleDragOptions.position?.x ?? 0;
</script>

<div class="container">
  {#if label}
    <div class="label" id={sliderId}>{label}</div>
  {/if}
  <div
    class="slider atomic-reveal"
    role="slider"
    tabindex="0"
    aria-valuemin={min}
    aria-valuemax={max}
    aria-valuenow={value}
    aria-labelledby={sliderId}
    style="--overshoot: {overshoot}px;"
    on:click={onClick}
    on:keydown={onKeyPress}
  >
    <div class="groove-layer groove" bind:this={track}>
      <div class="groove-layer shadow"></div>
      <div
        class="groove-contents progress"
        style="--leeway: {leeway}px; --left: {left}px; {transition}"
      ></div>
    </div>
    <div class="groove-layer bounds" bind:this={toggleBound}></div>
    <div
      class="toggle-label"
      use:draggable={toggleDragOptions}
      style="--leeway: {leeway}px; --left: {left}px; {transition}"
      bind:this={toggleLabel}
    >
      <div class="toggle" class:grabbing={dragging}></div>
    </div>
  </div>
</div>

<style>
  .container {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 1rem;
    flex-wrap: wrap;
    width: 100%;

    --skew: -20deg;
    --label-width: 3rem;
    --label-height: 1.5rem;
    --toggle-height: calc(1.2 * var(--label-height));
    --toggle-width: calc(1.05 * var(--label-width));
    --track-height: calc(1 * var(--label-height));
  }

  .label {
    white-space: nowrap;
    flex: 1;
  }

  .slider {
    --groove-contents-layer: 1;
    --groove-layer: 2;
    --toggle-layer: 3;
    flex: 1;
    height: var(--track-height);
    min-width: 7rem;
    cursor: pointer;
    transform: skew(var(--skew));
    margin-right: calc(-0.5 * var(--toggle-height) * sin(var(--skew)));
    padding: 0;
  }

  .groove-layer {
    width: 100%;
    height: var(--label-height);
    border-radius: var(--corner-roundness);
    z-index: var(--groove-layer);
    position: relative;
  }

  .groove-layer.groove {
    overflow: hidden;
  }

  .groove-layer.shadow {
    box-shadow: inset 0.05em 0.05em 0.3em rgba(0, 0, 0, 0.4);
  }

  .groove-layer.bounds {
    width: calc(100% + 2 * var(--overshoot));
    margin-left: calc(-1 * var(--overshoot));
    background: transparent;
    position: float;
    top: calc(-1 * var(--track-height));
  }

  .groove-contents.progress {
    --total-width: calc(var(--leeway) + var(--overshoot));
    z-index: var(--groove-contents-layer);
    position: absolute;
    top: 0;
    left: calc(-1 * var(--total-width));
    transform: translateX(var(--left));
    width: var(--total-width);
    height: var(--track-height);
    background: linear-gradient(to left, #00f, #bbbbff);
  }

  .toggle-label {
    width: var(--label-width);
    height: var(--label-height);
    display: flex;
    flex-direction: row;
    align-items: center;
    z-index: var(--toggle-layer);
    position: absolute;
    left: 0;
    top: 0;
    transform: translateX(var(--left));
  }

  .toggle {
    position: absolute;
    width: var(--toggle-width);
    height: var(--toggle-height);
    background-color: #ddd;
    box-shadow:
      0.1em 0.1em 0.15em rgba(0, 0, 0, 0.1),
      inset -0.1em -0.1em 0.15em rgba(0, 0, 0, 0.3),
      inset 0.1em 0.1em 0.15em rgba(255, 255, 255, 0.7);
    border-radius: var(--corner-roundness);
  }

  .toggle:hover {
    cursor: grab;
  }

  :global(.toggle.grabbing),
  .toggle.grabbing:hover {
    cursor: grabbing;
  }
</style>
