<script lang="ts">
  interface Props {
    children?: import("svelte").Snippet;
  }

  let { children }: Props = $props();
  const children_render = $derived(children);
</script>

<div class="outer-container">
  <div class="button-container cut-corners outer">
    {@render children_render?.()}
  </div>
</div>

<style>
  .outer-container {
    width: fit-content;
    margin: 0 auto;
  }

  .button-container {
    display: flex;
    flex-direction: column;
  }

  .button-container :global(.left-end) {
    --cut-top-left: 8px;
  }

  .button-container :global(.right-end) {
    --cut-bottom-right: 8px;
  }

  @media (min-width: 35rem) {
    .button-container {
      flex-direction: row;
    }
  }

  @media (min-width: 29rem) {
    :global(.high-dpi-adjust) .button-container {
      flex-direction: row;
    }
  }
</style>
