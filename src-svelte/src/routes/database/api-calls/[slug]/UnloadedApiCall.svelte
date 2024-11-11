<script lang="ts">
  import ApiCall from "./ApiCall.svelte";
  import { type LlmCall, commands } from "$lib/bindings";
  import { unwrap } from "$lib/tauri";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";
  import Loading from "$lib/Loading.svelte";

  interface Props {
    id: string;
    [key: string]: any;
  }

  let { id, ...rest }: Props = $props();
  let apiCall: LlmCall | undefined = $state(undefined);

  unwrap(commands.getApiCall(id))
    .then((retrievedApiCall) => {
      apiCall = retrievedApiCall;
    })
    .catch((error) => {
      snackbarError(error);
    });
</script>

{#if apiCall}
  <ApiCall {...rest} {apiCall} />
{:else}
  <Loading />
{/if}
