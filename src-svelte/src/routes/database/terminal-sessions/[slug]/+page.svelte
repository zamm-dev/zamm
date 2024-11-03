<script lang="ts">
  import TerminalSession from "../TerminalSession.svelte";
  import { commands, type RecoveredTerminalSession } from "$lib/bindings";
  import { onMount } from "svelte";
  import { unwrap } from "$lib/tauri";
  import { page } from "$app/stores";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";

  let terminalSession: RecoveredTerminalSession | undefined = undefined;

  onMount(async () => {
    try {
      terminalSession = await unwrap(
        commands.getTerminalSession($page.params.slug),
      );
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
  />
{:else}
  <p>Loading...</p>
{/if}
