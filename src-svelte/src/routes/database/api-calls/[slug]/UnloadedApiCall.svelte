<script lang="ts">
  import ApiCallDisplay from "./ApiCallDisplay.svelte";
  import { type LlmCall, commands } from "$lib/bindings";
  import { unwrap } from "$lib/tauri";
  import Actions from "./Actions.svelte";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";

  export let id: string;
  let apiCall: LlmCall | undefined = undefined;

  unwrap(commands.getApiCall(id))
    .then((retrievedApiCall) => {
      apiCall = retrievedApiCall;
    })
    .catch((error) => {
      snackbarError(error);
    });
</script>

<div class="container">
  <ApiCallDisplay {...$$restProps} bind:apiCall />
  <Actions {apiCall} />
</div>

<style>
  .container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
</style>
