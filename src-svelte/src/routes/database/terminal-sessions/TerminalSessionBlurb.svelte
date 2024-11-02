<script lang="ts">
  import { type TerminalSessionReference } from "$lib/bindings";

  export let item: TerminalSessionReference;
</script>

<div class="ellipsis-container">
  <span class="blurb">
    {item.command}
    {#if item.last_io}
      <span class="last-io">{item.last_io}</span>
    {/if}
  </span>
</div>

<style>
  .ellipsis-container {
    display: flex;
    width: 100%;
  }

  .blurb {
    min-width: 0;
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    line-height: 1.22rem;
  }

  .last-io {
    margin-left: 0.5rem;
    color: var(--color-faded);
    font-style: italic;
  }
</style>
