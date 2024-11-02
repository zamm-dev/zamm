<script lang="ts">
  import Table from "$lib/Table.svelte";
  import { commands, type TerminalSessionReference } from "$lib/bindings";
  import { unwrap } from "$lib/tauri";
  import TerminalSessionBlurb from "./TerminalSessionBlurb.svelte";

  const getTerminalSessions = (offset: number) =>
    unwrap(commands.getTerminalSessions(offset));
  const terminalSessionUrl = (apiCall: TerminalSessionReference) =>
    `/database/terminal-sessions/${apiCall.id}/`;

  export let dateTimeLocale: string | undefined = undefined;
  export let timeZone: string | undefined = undefined;
</script>

<Table
  blurbLabel="Command"
  getItems={getTerminalSessions}
  itemUrl={terminalSessionUrl}
  renderItem={TerminalSessionBlurb}
  {dateTimeLocale}
  {timeZone}
>
  Looks like you haven't <a href="/database/terminal-sessions/new/">started</a> any
  terminal sessions yet.
</Table>
