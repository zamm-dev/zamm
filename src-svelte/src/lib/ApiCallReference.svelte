<script lang="ts">
  import ApiCallReferenceLink from "./ApiCallReferenceLink.svelte";
  import type { LlmCallReference } from "./bindings";

  export let apiCall: LlmCallReference;
  export let selfContained = false;
</script>

{#if selfContained}
  <div class="ellipsis-container">
    <ApiCallReferenceLink {apiCall} {...$$restProps} />
  </div>
{:else}
  <ApiCallReferenceLink {apiCall} {...$$restProps} />
{/if}

<style>
  .ellipsis-container {
    display: flex;
    width: 100%;
  }
</style>
