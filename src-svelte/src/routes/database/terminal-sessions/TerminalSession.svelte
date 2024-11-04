<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
  import SendInputForm from "$lib/controls/SendInputForm.svelte";
  import { unwrap } from "$lib/tauri";
  import { commands } from "$lib/bindings";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";
  import EmptyPlaceholder from "$lib/EmptyPlaceholder.svelte";
  import Scrollable from "$lib/Scrollable.svelte";

  export let sessionId: string | undefined = undefined;
  export let command: string | undefined = undefined;
  export let output = "";
  export let isActive = true;
  let expectingResponse = false;
  let growable: Scrollable | undefined;
  $: awaitingSession = sessionId === undefined;
  $: accessibilityLabel = awaitingSession
    ? "Enter command to run"
    : "Enter input for command";
  $: placeholder = awaitingSession
    ? "Enter command to run (e.g. /bin/bash)"
    : "Enter input for command";

  function resizeTerminalView() {
    growable?.resizeScrollable();
    setTimeout(() => {
      growable?.scrollToBottom();
    }, 100);
  }

  async function sendCommand(newInput: string) {
    try {
      expectingResponse = true;
      if (sessionId === undefined) {
        let result = await unwrap(commands.runCommand(newInput));
        command = newInput;
        sessionId = result.id;
        output += result.output;
      } else {
        let result = await unwrap(
          commands.sendCommandInput(sessionId, newInput),
        );
        output += result;
      }
      resizeTerminalView();
    } catch (error) {
      snackbarError(error as string);
    } finally {
      expectingResponse = false;
    }
  }
</script>

<InfoBox title="Terminal Session" fullHeight>
  <div class="terminal-container composite-reveal full-height">
    {#if command}
      <p class="atomic-reveal">
        Command: <span class="command">{command}</span>
      </p>
    {:else}
      <EmptyPlaceholder>
        No running process.<br />Get started by entering a command below.
      </EmptyPlaceholder>
    {/if}

    <Scrollable bind:this={growable}>
      <pre>{output}</pre>
    </Scrollable>

    {#if isActive}
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
