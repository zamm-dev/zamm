<script lang="ts">
  import type { ChatMessage } from "$lib/bindings";
  import MessageUI from "./MessageUI.svelte";
  import CodeRender from "./CodeRender.svelte";
  import SvelteMarkdown, { type Renderers } from "svelte-markdown";

  interface Props {
    message: ChatMessage;
    [key: string]: any;
  }

  let { message, ...rest }: Props = $props();
  let messageUI: MessageUI;
  const renderers: Partial<Renderers> = {
    code: CodeRender,
  } as unknown as Partial<Renderers>;

  export function resizeBubble(chatWidthPx: number) {
    return messageUI.resizeBubble(chatWidthPx);
  }
</script>

<MessageUI role={message.role} bind:this={messageUI} {...rest}>
  <div class="markdown">
    <SvelteMarkdown source={message.text} {renderers} />
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
