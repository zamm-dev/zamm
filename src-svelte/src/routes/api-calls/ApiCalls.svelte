<script lang="ts">
  import { getApiCalls, type LightweightLlmCall } from "$lib/bindings";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";
  import InfoBox from "$lib/InfoBox.svelte";
  import Scrollable from "$lib/Scrollable.svelte";
  import { onMount } from "svelte";
  import EmptyPlaceholder from "$lib/EmptyPlaceholder.svelte";
  import IconAdd from "~icons/mingcute/add-fill";

  const PAGE_SIZE = 50;
  const MIN_MESSAGE_WIDTH = "5rem";
  const MIN_TIME_WIDTH = "12.5rem";

  export let dateTimeLocale: string | undefined = undefined;
  export let timeZone: string | undefined = undefined;
  let llmCalls: LightweightLlmCall[] = [];
  let llmCallsPromise: Promise<void> | undefined = undefined;
  let allCallsLoaded = false;
  let messageWidth = MIN_MESSAGE_WIDTH;
  let timeWidth = MIN_TIME_WIDTH;
  let headerMessageWidth = MIN_MESSAGE_WIDTH;

  const formatter = new Intl.DateTimeFormat(dateTimeLocale, {
    year: "numeric",
    month: "numeric",
    day: "numeric",
    hour: "numeric",
    minute: "numeric",
    hour12: true,
    timeZone,
  });

  export function formatTimestamp(timestamp: string): string {
    const timestampUTC = timestamp + "Z";
    const date = new Date(timestampUTC);
    return formatter.format(date);
  }

  function getWidths(selector: string) {
    const elements = document.querySelectorAll(selector);
    const results = Array.from(elements)
      .map((el) => el.getBoundingClientRect().width)
      .filter((width) => width > 0);
    return results;
  }

  function resizeMessageWidth() {
    messageWidth = MIN_MESSAGE_WIDTH;
    // time width doesn't need a reset because it never decreases

    setTimeout(() => {
      const textWidths = getWidths(".api-calls-page .text-container");
      const timeWidths = getWidths(".api-calls-page .time");
      const minTextWidth = Math.floor(Math.min(...textWidths));
      messageWidth = `${minTextWidth}px`;
      const maxTimeWidth = Math.ceil(Math.max(...timeWidths));
      timeWidth = `${maxTimeWidth}px`;

      headerMessageWidth = messageWidth;
    }, 10);
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
        llmCallsPromise = undefined;

        requestAnimationFrame(resizeMessageWidth);
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

  $: minimumWidths =
    `--message-width: ${messageWidth}; ` +
    `--header-message-width: ${headerMessageWidth}; ` +
    `--time-width: ${timeWidth}`;
</script>

<InfoBox title="LLM API Calls" fullHeight>
  <div class="container api-calls-page full-height" style={minimumWidths}>
    <a class="new-api-call" href="/api-calls/new/">
      <IconAdd />
    </a>
    <div class="message header">
      <div class="text-container">
        <div class="text">Message</div>
      </div>
      <div class="time">Time</div>
    </div>
    <div class="scrollable-container full-height">
      <Scrollable on:bottomReached={loadApiCalls}>
        {#if llmCalls.length > 0}
          {#each llmCalls as call (call.id)}
            <a href={`/api-calls/${call.id}`}>
              <div class="message instance">
                <div class="text-container">
                  <div class="text">{call.response_message.text}</div>
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
                started via <a href="/chat">chat</a> or by making one
                <a href="/api-calls/new/">from scratch</a>.
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

  .scrollable-container {
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
    height: 1.62rem;
    box-sizing: border-box;
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

  .message.header .text-container .text {
    max-width: var(--header-message-width);
  }

  .message .time {
    min-width: var(--time-width);
    box-sizing: border-box;
    text-align: right;
  }
</style>
