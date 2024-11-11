<script lang="ts" generics="Item extends { id: string, timestamp: string }">
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";
  import Scrollable from "$lib/Scrollable.svelte";
  import { onMount, type Component } from "svelte";
  import EmptyPlaceholder from "$lib/EmptyPlaceholder.svelte";

  const PAGE_SIZE = 50;
  const MIN_BLURB_WIDTH = "5rem";
  const MIN_TIME_WIDTH = "12.5rem";

  interface Props {
    dateTimeLocale?: string | undefined;
    timeZone?: string | undefined;
    blurbLabel: string;
    itemUrl: (item: Item) => string;
    getItems: (offset: number) => Promise<Item[]>;
    renderItem: Component<{ item: Item }>;
    children?: import("svelte").Snippet;
  }

  let {
    dateTimeLocale = undefined,
    timeZone = undefined,
    blurbLabel,
    itemUrl,
    getItems,
    renderItem,
    children,
  }: Props = $props();
  let items: Item[] = $state([]);
  let newItemsPromise: Promise<void> | undefined = undefined;
  let allItemsLoaded = false;
  let blurbWidth = $state(MIN_BLURB_WIDTH);
  let timeWidth = $state(MIN_TIME_WIDTH);
  let headerBlurbWidth = $state("15rem"); // 3 * MIN_BLURB_WIDTH

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

  function resizeBlurbWidth() {
    blurbWidth = MIN_BLURB_WIDTH;
    // time width doesn't need a reset because it never decreases

    setTimeout(() => {
      const textWidths = getWidths(".database-page .text-container");
      const timeWidths = getWidths(".database-page .time");
      const minTextWidth = Math.floor(Math.min(...textWidths));
      blurbWidth = `${minTextWidth}px`;
      const maxTimeWidth = Math.ceil(Math.max(...timeWidths));
      timeWidth = `${maxTimeWidth}px`;

      headerBlurbWidth = blurbWidth;
    }, 10);
  }

  function loadNewItems() {
    if (newItemsPromise) {
      return;
    }

    if (allItemsLoaded) {
      return;
    }

    newItemsPromise = getItems(items.length)
      .then((newItems) => {
        items = [...items, ...newItems];
        allItemsLoaded = newItems.length < PAGE_SIZE;
        newItemsPromise = undefined;

        requestAnimationFrame(resizeBlurbWidth);
      })
      .catch((error) => {
        snackbarError(error);
      });
  }

  onMount(() => {
    resizeBlurbWidth();
    window.addEventListener("resize", resizeBlurbWidth);

    return () => {
      window.removeEventListener("resize", resizeBlurbWidth);
    };
  });

  let minimumWidths = $derived(
    `--blurb-width: ${blurbWidth}; ` +
      `--header-blurb-width: ${headerBlurbWidth}; ` +
      `--time-width: ${timeWidth}`,
  );

  const children_render = $derived(children);
</script>

<div class="container database-page full-height" style={minimumWidths}>
  <div class="blurb header">
    <div class="text-container">
      <div class="text">{blurbLabel}</div>
    </div>
    <div class="time">Time</div>
  </div>
  <div class="scrollable-container full-height">
    <Scrollable on:bottomReached={loadNewItems}>
      {#if items.length > 0}
        {#each items as item (item.id)}
          {@const Blurb = renderItem}
          <a href={itemUrl(item)}>
            <div class="blurb instance">
              <div class="text-container">
                <div class="text">
                  <Blurb {item} />
                </div>
              </div>
              <div class="time">{formatTimestamp(item.timestamp)}</div>
            </div>
          </a>
        {/each}
      {:else}
        <div class="blurb placeholder">
          <div class="text-container">
            <EmptyPlaceholder>
              {@render children_render?.()}
            </EmptyPlaceholder>
          </div>
          <div class="time"></div>
        </div>
      {/if}
    </Scrollable>
  </div>
</div>

<style>
  .container {
    gap: 0.25rem;
  }

  .scrollable-container {
    --side-padding: 0.8rem;
    margin: 0 calc(-1 * var(--side-padding));
    width: calc(100% + 2 * var(--side-padding));
    box-sizing: border-box;
  }

  .blurb {
    display: flex;
    color: black;
  }

  .blurb.placeholder :global(p) {
    margin-top: 0.5rem;
  }

  .blurb.header .text-container,
  .blurb.header .time {
    text-align: center;
    font-weight: bold;
  }

  .blurb .text-container {
    flex: 1;
  }

  .blurb.instance {
    padding: 0.2rem var(--side-padding);
    border-radius: var(--corner-roundness);
    transition: background 0.5s;
    height: 1.62rem;
    box-sizing: border-box;
  }

  .blurb.instance:hover {
    background: var(--color-hover);
  }

  .blurb .text-container .text {
    max-width: var(--blurb-width);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .blurb.header .text-container .text {
    max-width: var(--header-blurb-width);
  }

  .blurb .time {
    min-width: var(--time-width);
    box-sizing: border-box;
    text-align: right;
  }
</style>
