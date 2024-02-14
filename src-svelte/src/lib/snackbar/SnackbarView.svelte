<script lang="ts">
  import Snackbar, { snackbarError } from "./Snackbar.svelte";

  let count = 0;

  function showError() {
    count++;
    const noun = count === 1 ? "thing" : "things";
    snackbarError(`${count} ${noun} went wrong!`);
  }
</script>

<Snackbar />

<button on:click={showError}>Show Error</button>
