<script lang="ts">
  import PageTransition from "./PageTransition.svelte";

  export let currentRoute = "/";
  let newRoute = "";

  function navigate() {
    currentRoute = newRoute;
    newRoute = "";
  }
</script>

<input type="text" aria-label="Route" bind:value={newRoute} />
<button on:click={navigate}>Navigate</button>
<PageTransition {currentRoute} />
