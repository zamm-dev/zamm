<script lang="ts">
  import ApiCallDisplay from "./ApiCallDisplay.svelte";
  import { type LlmCall } from "$lib/bindings";
  import Actions from "./Actions.svelte";

  interface Props {
    apiCall: LlmCall;
    showActions?: boolean;
    [key: string]: any;
  }

  let { apiCall, showActions = true, ...rest }: Props = $props();
</script>

<div class="container">
  <ApiCallDisplay {apiCall} {...rest} />
  {#if showActions}
    <Actions {apiCall} />
  {/if}
</div>

<style>
  .container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
</style>
