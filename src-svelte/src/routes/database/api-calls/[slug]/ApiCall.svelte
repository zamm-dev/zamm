<script lang="ts">
  import ApiCallDisplay from "./ApiCallDisplay.svelte";
  import { type LlmCall } from "$lib/bindings";
  import Actions from "./Actions.svelte";

  export let apiCall: LlmCall;
  export let showActions = true;
</script>

<div class="container">
  <ApiCallDisplay {...$$restProps} bind:apiCall />
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
