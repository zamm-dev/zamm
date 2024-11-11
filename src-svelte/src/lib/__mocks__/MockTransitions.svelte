<script lang="ts">
  import { onMount } from "svelte";
  import MockAppLayout from "./MockAppLayout.svelte";
  interface Props {
    children?: import("svelte").Snippet;
  }

  let { children }: Props = $props();

  let visible = $state(false);

  onMount(() => {
    setTimeout(() => {
      visible = true;
    }, 50);
  });
</script>

<MockAppLayout>
  {#if visible}
    {@render children?.()}
  {/if}
</MockAppLayout>
