<script lang="ts">
  export let text: string;
  export let unwrapped = false;
  export let rightEnd = false;
</script>

{#if unwrapped}
  <button class="cut-corners inner" class:right-end={rightEnd} type="submit">
    {text}
  </button>
{:else}
  <button class="cut-corners outer" class:right-end={rightEnd} type="submit">
    <div class="cut-corners inner" class:right-end={rightEnd}>{text}</div>
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

  .outer.right-end,
  .inner.right-end {
    --cut-top-left: 0.01rem;
    height: 100%;
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

  .outer {
    --background-color: #eee;
    display: inline-block;
  }
</style>
