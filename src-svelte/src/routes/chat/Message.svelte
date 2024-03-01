<script lang="ts">
  import type { ChatMessage } from "$lib/bindings";
  import MessageUI from "./MessageUI.svelte";
  import CodeRender from "./CodeRender.svelte";
  import SvelteMarkdown from "svelte-markdown";

  export let message: ChatMessage;
</script>

<MessageUI role={message.role} {...$$restProps}>
  <div class="markdown">
    <SvelteMarkdown source={message.text} renderers={{ code: CodeRender }} />
  </div>
</MessageUI>

<style>
  .markdown :global(:first-child) {
    margin-top: 0;
  }

  .markdown :global(:last-child) {
    margin-bottom: 0;
  }
</style>
