<script lang="ts" context="module">
  import type { ChatMessage } from "$lib/bindings";

  let persistentConversation: ChatMessage[] = [
    {
      role: "System",
      text: "You are ZAMM, a chat program. Respond in first person.",
    },
  ];
</script>

<script lang="ts">
  import Chat from "./Chat.svelte";
</script>

<Chat bind:conversation={persistentConversation} />
