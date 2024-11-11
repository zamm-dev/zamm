<script lang="ts">
  import PageTransition from "./PageTransition.svelte";

  interface Props {
    currentRoute?: string;
  }

  let { currentRoute = $bindable("/") }: Props = $props();
  let newRoute = $state("");

  function navigate() {
    currentRoute = newRoute;
    newRoute = "";
  }
</script>

<input type="text" aria-label="Route" bind:value={newRoute} />
<button onclick={navigate}>Navigate</button>
<PageTransition {currentRoute} />
