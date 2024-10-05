<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
  import IconAdd from "~icons/mingcute/add-fill";
  import ApiCallsTable from "./ApiCallsTable.svelte";

  export let dateTimeLocale: string | undefined = undefined;
  export let timeZone: string | undefined = undefined;
</script>

<InfoBox title="LLM API Calls" fullHeight>
  <div class="container full-height">
    <a class="new-api-call" href="/api-calls/new/" title="New API call">
      <IconAdd />
    </a>
    <ApiCallsTable {dateTimeLocale} {timeZone} />
  </div>
</InfoBox>

<style>
  .container {
    gap: 0.25rem;
  }

  a.new-api-call {
    position: absolute;
    top: 1rem;
    right: 1rem;
  }

  a.new-api-call :global(svg) {
    transform: scale(1.2);
    color: var(--color-faded);
  }
</style>
