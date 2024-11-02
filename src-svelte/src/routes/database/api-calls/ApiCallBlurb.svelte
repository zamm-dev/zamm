<script lang="ts">
  import ApiCallReference from "$lib/ApiCallReference.svelte";
  import { type LightweightLlmCall } from "$lib/bindings";

  export let item: LightweightLlmCall;
  $: reference = {
    id: item.id,
    snippet: item.response_message.text.trim(),
  };
</script>

<ApiCallReference selfContained nolink apiCall={reference} />
