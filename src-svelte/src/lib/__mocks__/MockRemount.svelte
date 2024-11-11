<script lang="ts">
  interface Props {
    children?: import("svelte").Snippet;
  }

  let { children }: Props = $props();
  let visible = $state(true);

  function remount() {
    visible = false;
    setTimeout(() => (visible = true), 50);
  }
</script>

<button onclick={remount}>Remount</button>

{#if visible}
  {@render children?.()}
{/if}
