<script lang="ts">
  import Table from "$lib/Table.svelte";
  import { commands, type LightweightLlmCall } from "$lib/bindings";
  import { unwrap } from "$lib/tauri";
  import ApiCallBlurb from "./ApiCallBlurb.svelte";

  const getApiCalls = (offset: number) => unwrap(commands.getApiCalls(offset));
  const apiCallUrl = (apiCall: LightweightLlmCall) =>
    `/database/api-calls/${apiCall.id}/`;

  export let dateTimeLocale: string | undefined = undefined;
  export let timeZone: string | undefined = undefined;
</script>

<Table
  blurbLabel="Message"
  getItems={getApiCalls}
  itemUrl={apiCallUrl}
  renderItem={ApiCallBlurb}
  {dateTimeLocale}
  {timeZone}
>
  Looks like you haven't made any calls to an LLM yet.<br />Get started via
  <a href="/chat">chat</a>
  or by making one <a href="/database/api-calls/new/">from scratch</a>.
</Table>
