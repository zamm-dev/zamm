<script lang="ts">
  import PersistentChat from "./PersistentChat.svelte";

  let visible = true;

  function remount() {
    visible = false;
    setTimeout(() => (visible = true), 50);
  }
</script>

<button on:click={remount}>Remount</button>

{#if visible}
  <PersistentChat />
{/if}
