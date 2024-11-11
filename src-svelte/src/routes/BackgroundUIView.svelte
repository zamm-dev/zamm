<script lang="ts">
  import MockAppLayout from "$lib/__mocks__/MockAppLayout.svelte";
  interface Props {
    children?: import("svelte").Snippet;
  }

  let { children }: Props = $props();

  const children_render = $derived(children);
</script>

<MockAppLayout>
  <div class="background-container">
    {@render children_render?.()}
  </div>
</MockAppLayout>

<style>
  .background-container {
    position: fixed;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
  }
</style>
