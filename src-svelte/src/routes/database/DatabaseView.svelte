<script lang="ts" context="module">
  import { writable } from "svelte/store";

  type DataTypeEnum = "llm-calls" | "terminal";
  export const dataType = writable<DataTypeEnum>("llm-calls");

  export function resetDataType() {
    dataType.set("llm-calls");
  }
</script>

<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
  import IconAdd from "~icons/mingcute/add-fill";
  import ApiCallsTable from "./api-calls/ApiCallsTable.svelte";
  import Select from "$lib/controls/Select.svelte";
  import EmptyPlaceholder from "$lib/EmptyPlaceholder.svelte";

  export let dateTimeLocale: string | undefined = undefined;
  export let timeZone: string | undefined = undefined;

  $: infoBoxTitle =
    $dataType === "llm-calls" ? "LLM API Calls" : "Terminal Sessions";
  $: newHref =
    $dataType === "llm-calls"
      ? "/database/api-calls/new/"
      : "/database/terminal-sessions/new/";
  $: newTitle =
    $dataType === "llm-calls" ? "New API call" : "New Terminal Session";
</script>

<InfoBox title={infoBoxTitle} fullHeight>
  <div class="container full-height">
    <a class="new-button" href={newHref} title={newTitle}>
      <IconAdd />
    </a>
    <Select name="data-type" label="Showing " bind:value={$dataType}>
      <option value="llm-calls">LLM Calls</option>
      <option value="terminal">Terminal Sessions</option>
    </Select>
    {#if $dataType === "llm-calls"}
      <ApiCallsTable {dateTimeLocale} {timeZone} />
    {:else}
      <EmptyPlaceholder>
        Terminal sessions cannot be viewed yet.<br />You may
        <a href="/database/terminal-sessions/new/">start</a> a new one.
      </EmptyPlaceholder>
    {/if}
  </div>
</InfoBox>

<style>
  .container {
    gap: 0.25rem;
  }

  .container :global(.select-wrapper) {
    flex: 0;
    margin-bottom: 0.25rem;
  }

  .container :global(select) {
    width: fit-content;
  }

  a.new-button {
    position: absolute;
    top: 1rem;
    right: 1rem;
  }

  a.new-button :global(svg) {
    transform: scale(1.2);
    color: var(--color-faded);
  }
</style>
