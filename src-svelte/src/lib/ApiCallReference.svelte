<script lang="ts">
  import ApiCallReferenceLink from "./ApiCallReferenceLink.svelte";
  import type { LlmCallReference } from "./bindings";

  interface Props {
    apiCall: LlmCallReference;
    selfContained?: boolean;
    [key: string]: any;
  }

  let { apiCall, selfContained = false, ...rest }: Props = $props();
</script>

{#if selfContained}
  <div class="ellipsis-container">
    <ApiCallReferenceLink {apiCall} {...rest} />
  </div>
{:else}
  <ApiCallReferenceLink {apiCall} {...rest} />
{/if}

<style>
  .ellipsis-container {
    display: flex;
    width: 100%;
  }
</style>
