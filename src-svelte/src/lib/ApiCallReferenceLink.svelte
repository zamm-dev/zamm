<script lang="ts">
  import type { LlmCallReference } from "$lib/bindings";

  interface Props {
    apiCall: LlmCallReference;
    nolink?: boolean;
  }

  let { apiCall, nolink = false }: Props = $props();
</script>

{#if nolink}
  <span class="nolink">{apiCall.snippet}</span>
{:else}
  <a href="/database/api-calls/{apiCall.id}">{apiCall.snippet}</a>
{/if}

<style>
  span,
  a {
    min-width: 0;
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    line-height: 1.22rem;
  }
</style>
