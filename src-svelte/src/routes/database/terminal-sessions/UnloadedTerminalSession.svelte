<script lang="ts">
  import TerminalSession from "./TerminalSession.svelte";
  import { commands, type RecoveredTerminalSession } from "$lib/bindings";
  import { onMount } from "svelte";
  import { unwrap } from "$lib/tauri";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";
  import Loading from "$lib/Loading.svelte";

  export let id: string;
  let terminalSession: RecoveredTerminalSession | undefined = undefined;

  onMount(async () => {
    try {
      terminalSession = await unwrap(commands.getTerminalSession(id));
    } catch (error) {
      snackbarError(error as string | Error);
    }
  });
</script>

{#if terminalSession}
  <TerminalSession
    sessionId={terminalSession.id}
    command={terminalSession.command}
    output={terminalSession.output}
    isActive={terminalSession.is_active}
  />
{:else}
  <Loading />
{/if}
