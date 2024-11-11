<script lang="ts">
  import ApiCallReference from "$lib/ApiCallReference.svelte";
  import { type LightweightLlmCall } from "$lib/bindings";

  interface Props {
    item: LightweightLlmCall;
  }

  let { item }: Props = $props();
  let reference = $derived({
    id: item.id,
    snippet: item.response_message.text.trim(),
  });
</script>

<ApiCallReference selfContained nolink apiCall={reference} />
