<script lang="ts">
  let visible = true;

  function remount() {
    visible = false;
    setTimeout(() => (visible = true), 50);
  }
</script>

<button on:click={remount}>Remount</button>

{#if visible}
  <slot />
{/if}
