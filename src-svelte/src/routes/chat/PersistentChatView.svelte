<script lang="ts">
  import Chat from "./Chat.svelte";

  let visible = true;

  function remount() {
    visible = false;
    setTimeout(() => (visible = true), 50);
  }
</script>

<button on:click={remount}>Remount</button>

{#if visible}
  <Chat />
{/if}
