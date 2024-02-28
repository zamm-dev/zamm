<script lang="ts">
  import type { ChatMessage } from "$lib/bindings";
  import MessageUI from "./MessageUI.svelte";
  export let message: ChatMessage;
</script>

<MessageUI role={message.role} {...$$restProps}>
  {message.text}
</MessageUI>
