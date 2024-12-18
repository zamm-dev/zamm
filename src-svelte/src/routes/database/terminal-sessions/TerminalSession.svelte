<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
  import SendInputForm from "$lib/controls/SendInputForm.svelte";
  import { unwrap } from "$lib/tauri";
  import { commands, type TerminalSessionInfo } from "$lib/bindings";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";
  import EmptyPlaceholder from "$lib/EmptyPlaceholder.svelte";
  import Scrollable from "$lib/Scrollable.svelte";
  import { sidebar } from "../../SidebarUI.svelte";
  import { replaceState } from "$app/navigation";
  import { page } from "$app/stores";
  import { pageTransition } from "../../PageTransition.svelte";

  interface Props {
    session?: TerminalSessionInfo | undefined;
  }

  let { session = $bindable(undefined) }: Props = $props();
  let expectingResponse = $state(false);
  let growable: Scrollable | undefined = $state();
  let awaitingSession = $derived(session === undefined);
  let accessibilityLabel = $derived(
    awaitingSession ? "Enter command to run" : "Enter input for command",
  );
  let placeholder = $derived(
    awaitingSession
      ? "Enter command to run (e.g. /bin/bash)"
      : "Enter input for command",
  );

  function resizeTerminalView() {
    growable?.resizeScrollable();
    setTimeout(() => {
      growable?.scrollToBottom();
    }, 100);
  }

  async function sendCommand(newInput: string) {
    try {
      expectingResponse = true;
      if (session === undefined) {
        session = await unwrap(commands.runCommand(newInput));
        const newUrl = `/database/terminal-sessions/${session.id}/`;
        try {
          replaceState(newUrl, $page.state);
        } catch (error) {
          console.error("Failed to update URL, are we on Storybook?", error);
        }
        $sidebar?.updateIndicator(newUrl);
        $pageTransition?.addVisitedRoute(newUrl);
      } else {
        let result = await unwrap(
          commands.sendCommandInput(session.id, newInput),
        );
        session.output += result;
      }
      resizeTerminalView();
    } catch (error) {
      snackbarError(error as string | Error);
    } finally {
      expectingResponse = false;
    }
  }
</script>

<InfoBox title="Terminal Session" fullHeight>
  <div class="terminal-container composite-reveal full-height">
    {#if session?.command}
      <p class="atomic-reveal">
        Command: <span class="command">{session.command}</span>
      </p>
    {:else}
      <EmptyPlaceholder>
        No running process.<br />Get started by entering a command below.
      </EmptyPlaceholder>
    {/if}

    <Scrollable bind:this={growable}>
      <pre>{session?.output ?? ""}</pre>
    </Scrollable>

    {#if session === undefined || session.is_active}
      <SendInputForm
        {accessibilityLabel}
        {placeholder}
        sendInput={sendCommand}
        isBusy={expectingResponse}
        onTextInputResize={resizeTerminalView}
      />
    {:else}
      <EmptyPlaceholder>
        This terminal session is no longer active.
      </EmptyPlaceholder>
    {/if}
  </div>
</InfoBox>

<style>
  .terminal-container {
    height: 100%;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  p {
    margin: 0;
    padding: 0.5rem 1rem;
    background: var(--color-background);
    border-radius: var(--corner-roundness);
  }

  span.command {
    font-weight: bold;
  }
</style>
