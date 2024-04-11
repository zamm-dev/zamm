<script lang="ts">
  import ApiCallDisplay from "./ApiCallDisplay.svelte";
  import { type LlmCall, getApiCall } from "$lib/bindings";
  import Actions from "./Actions.svelte";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";

  export let id: string;
  let apiCall: LlmCall | undefined = undefined;

  getApiCall(id)
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
