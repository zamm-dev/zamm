<script lang="ts">
  import ApiCall from "./ApiCall.svelte";
  import { type LlmCall, commands } from "$lib/bindings";
  import { unwrap } from "$lib/tauri";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";
  import Loading from "$lib/Loading.svelte";

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

{#if apiCall}
  <ApiCall {...$$restProps} {apiCall} />
{:else}
  <Loading />
{/if}
