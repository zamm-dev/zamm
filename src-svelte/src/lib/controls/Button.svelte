<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let unwrapped = false;
  export let disabled = false;
  export let leftEnd = false;
  export let rightEnd = false;
  export let ariaLabel: string | undefined = undefined;
  const dispatchClickEvent = createEventDispatcher();

  function handleClick() {
    dispatchClickEvent("click");
  }
</script>

{#if unwrapped}
  <button
    class="cut-corners inner"
    class:left-end={leftEnd}
    class:right-end={rightEnd}
    class:disabled
    type="submit"
    aria-label={ariaLabel}
    on:click={handleClick}
  >
    <slot />
  </button>
{:else}
  <button
    class="cut-corners outer"
    class:left-end={leftEnd}
    class:right-end={rightEnd}
    class:disabled
    type="submit"
    aria-label={ariaLabel}
    on:click={handleClick}
  >
    <div
      class="cut-corners inner"
      class:left-end={leftEnd}
      class:right-end={rightEnd}
    >
      <slot />
    </div>
  </button>
{/if}

<style>
  .outer,
  .inner {
    --cut: var(--controls-corner-cut);
    font-size: 0.9rem;
    font-family: var(--font-body);
    text-transform: uppercase;
    transition-property: filter, transform;
    transition: calc(0.5 * var(--standard-duration)) ease-out;
  }

  .outer.left-end,
  .inner.left-end {
    --cut-bottom-right: 0.01rem;
  }

  .outer.right-end,
  .inner.right-end {
    --cut-top-left: 0.01rem;
  }

  .inner {
    padding: 5px 10px;
  }

  .inner:hover {
    filter: brightness(1.05);
  }

  .inner:active {
    transform: translateY(0.08rem) scale(0.98);
  }

  .inner.disabled,
  .disabled .inner {
    filter: grayscale(1);
    color: var(--color-faded);
    pointer-events: none;
  }

  .inner.disabled:hover,
  .disabled .inner:hover {
    filter: grayscale(1);
  }

  .inner.disabled:active,
  .disabled .inner:active {
    transform: none;
  }

  .outer {
    --background-color: #eee;
    display: inline-block;
  }
</style>
