<script lang="ts">
  import { getApiCalls, type LlmCall } from "$lib/bindings";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";
  import InfoBox from "$lib/InfoBox.svelte";
  import Scrollable from "$lib/Scrollable.svelte";
  import { onMount } from "svelte";
  import EmptyPlaceholder from "$lib/EmptyPlaceholder.svelte";

  const PAGE_SIZE = 10;
  const MIN_MESSAGE_WIDTH = "5rem";

  let llmCalls: LlmCall[] = [];
  let llmCallsPromise: Promise<void> | undefined = undefined;
  let allCallsLoaded = false;
  let messageWidth = MIN_MESSAGE_WIDTH;

  const formatter = new Intl.DateTimeFormat(undefined, {
    year: "numeric",
    month: "numeric",
    day: "numeric",
    hour: "numeric",
    minute: "numeric",
    hour12: true,
  });

  export function formatTimestamp(timestamp: string): string {
    const timestampUTC = timestamp + "Z";
    const date = new Date(timestampUTC);
    return formatter.format(date);
  }

  function resizeMessageWidth() {
    const textContainer = document.querySelector(".text-container");
    if (!textContainer) {
      console.warn("Could not find text container for resize");
      return;
    }

    messageWidth = MIN_MESSAGE_WIDTH;

    requestAnimationFrame(() => {
      messageWidth = `${textContainer.clientWidth}px`;
    });
  }

  function loadApiCalls() {
    if (llmCallsPromise) {
      return;
    }

    if (allCallsLoaded) {
      return;
    }

    llmCallsPromise = getApiCalls(llmCalls.length)
      .then((newCalls) => {
        llmCalls = [...llmCalls, ...newCalls];
        allCallsLoaded = newCalls.length < PAGE_SIZE;
      })
      .catch((error) => {
        snackbarError(error);
      });
  }

  onMount(() => {
    resizeMessageWidth();
    window.addEventListener("resize", resizeMessageWidth);

    return () => {
      window.removeEventListener("resize", resizeMessageWidth);
    };
  });
</script>

<InfoBox title="LLM API Calls" fullHeight>
  <div class="container" style={`--message-width: ${messageWidth}`}>
    <div class="message header">
      <div class="text-container">Message</div>
      <div class="time">Time</div>
    </div>
    <div class="scrollable-container">
      <Scrollable on:bottomReached={loadApiCalls}>
        {#if llmCalls.length > 0}
          {#each llmCalls as call (call.id)}
            <a href={`/api-calls/${call.id}`}>
              <div class="message instance">
                <div class="text-container">
                  <div class="text">{call.response.completion.text}</div>
                </div>
                <div class="time">{formatTimestamp(call.timestamp)}</div>
              </div>
            </a>
          {/each}
        {:else}
          <div class="message placeholder">
            <div class="text-container">
              <EmptyPlaceholder>
                Looks like you haven't made any calls to an LLM yet.<br />Get
                started by using the chat functionality.
              </EmptyPlaceholder>
            </div>
            <div class="time"></div>
          </div>
        {/if}
      </Scrollable>
    </div>
  </div>
</InfoBox>

<style>
  .container {
    display: flex;
    flex-direction: column;
    gap: var(--internal-spacing);
    height: 100%;
  }

  .scrollable-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    --side-padding: 0.8rem;
    margin: 0 calc(-1 * var(--side-padding));
    width: calc(100% + 2 * var(--side-padding));
    box-sizing: border-box;
  }

  .message {
    display: flex;
    color: black;
  }

  .message.placeholder :global(p) {
    margin-top: 0.5rem;
  }

  .message.header {
    margin-bottom: 0.5rem;
  }

  .message.header .text-container,
  .message.header .time {
    text-align: center;
    font-weight: bold;
  }

  .message .text-container {
    flex: 1;
  }

  .message.instance {
    padding: 0.2rem var(--side-padding);
    border-radius: var(--corner-roundness);
    transition: background 0.5s;
  }

  .message.instance:hover {
    background: var(--color-hover);
  }

  .message .text-container .text {
    max-width: var(--message-width);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .message .time {
    width: 12rem;
    text-align: right;
  }
</style>
