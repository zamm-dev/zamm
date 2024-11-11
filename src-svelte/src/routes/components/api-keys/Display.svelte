<script lang="ts">
  import { commands } from "$lib/bindings";
  import { unwrap } from "$lib/tauri";
  import { apiKeys as apiKeysStore } from "$lib/system-info";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";
  import InfoBox from "$lib/InfoBox.svelte";
  import Loading from "$lib/Loading.svelte";
  import Service from "./Service.svelte";
  import { onMount } from "svelte";

  let isLoading = $state(true);
  interface Props {
    editDemo?: boolean;
    [key: string]: any;
  }

  let { editDemo = false, ...rest }: Props = $props();

  onMount(() => {
    unwrap(commands.getApiKeys())
      .then((keys) => {
        apiKeysStore.set(keys);
      })
      .catch((error) => {
        snackbarError(error);
      })
      .finally(() => {
        isLoading = false;
      });
  });

  let apiKeys = $derived($apiKeysStore);
</script>

<InfoBox title="API Keys" {...rest}>
  {#if isLoading}
    <Loading />
  {:else}
    <div class="api-keys" role="table">
      <Service
        name="OpenAI"
        apiKeyUrl="https://platform.openai.com/api-keys"
        apiKey={apiKeys.openai}
        editing={editDemo}
      />
    </div>
  {/if}
</InfoBox>
