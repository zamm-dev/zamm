<script lang="ts">
  import TerminalSession from "./TerminalSession.svelte";
  import { commands, type TerminalSessionInfo } from "$lib/bindings";
  import { onMount } from "svelte";
  import { unwrap } from "$lib/tauri";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";
  import Loading from "$lib/Loading.svelte";

  export let id: string;
  let session: TerminalSessionInfo | undefined = undefined;

  onMount(async () => {
    try {
      session = await unwrap(commands.getTerminalSession(id));
    } catch (error) {
      snackbarError(error as string | Error);
    }
  });
</script>

{#if session}
  <TerminalSession {session} />
{:else}
  <Loading />
{/if}
