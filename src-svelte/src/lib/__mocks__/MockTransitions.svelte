<script lang="ts">
  import { onMount } from "svelte";
  import MockAppLayout from "./MockAppLayout.svelte";

  let visible = false;

  onMount(() => {
    setTimeout(() => {
      visible = true;
    }, 50);
  });
</script>

<MockAppLayout>
  {#if visible}
    <slot />
  {/if}
</MockAppLayout>
