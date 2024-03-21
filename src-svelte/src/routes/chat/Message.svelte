<script lang="ts">
  import type { ChatMessage } from "$lib/bindings";
  import MessageUI from "./MessageUI.svelte";
  import CodeRender from "./CodeRender.svelte";
  import SvelteMarkdown from "svelte-markdown";

  export let message: ChatMessage;
  let resizeBubbleBound: (chatWidthPx: number) => void;

  export function resizeBubble(chatWidthPx: number) {
    resizeBubbleBound(chatWidthPx);
  }
</script>

<MessageUI
  role={message.role}
  bind:resizeBubble={resizeBubbleBound}
  {...$$restProps}
>
  <div class="markdown">
    <SvelteMarkdown source={message.text} renderers={{ code: CodeRender }} />
  </div>
</MessageUI>

<style>
  .markdown {
    width: fit-content;
  }

  .markdown :global(p) {
    margin: var(--internal-spacing) 0;
  }

  .markdown :global(:first-child) {
    margin-top: 0;
  }

  .markdown :global(:last-child) {
    margin-bottom: 0;
  }
</style>
