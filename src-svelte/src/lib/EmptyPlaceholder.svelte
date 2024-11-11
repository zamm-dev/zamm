<script lang="ts">
  interface Props {
    children?: import("svelte").Snippet;
  }

  let { children }: Props = $props();
  const children_render = $derived(children);
</script>

<p class="empty atomic-reveal">
  {@render children_render?.()}
</p>

<style>
  .empty {
    color: var(--color-faded);
    font-size: 0.85rem;
    font-style: italic;
    text-align: center;
  }

  .empty :global(a) {
    color: var(--color-faded);
    text-decoration: underline;
  }
</style>
