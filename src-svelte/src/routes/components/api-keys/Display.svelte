<script lang="ts">
  import { commands } from "$lib/bindings";
  import { unwrap } from "$lib/tauri";
  import { apiKeys as apiKeysStore } from "$lib/system-info";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";
  import InfoBox from "$lib/InfoBox.svelte";
  import Loading from "$lib/Loading.svelte";
  import Service from "./Service.svelte";
  import { onMount } from "svelte";

  let isLoading = true;
  export let editDemo = false;

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

  $: apiKeys = $apiKeysStore;
</script>

<InfoBox title="API Keys" {...$$restProps}>
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
