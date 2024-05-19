<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
  import { type LlmCall } from "$lib/bindings";
  import { lastMessageId, conversation } from "../../chat/Chat.svelte";
  import { goto } from "$app/navigation";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";
  import Button from "$lib/controls/Button.svelte";

  export let apiCall: LlmCall | undefined = undefined;

  function restoreConversation() {
    if (!apiCall) {
      snackbarError("API call not yet loaded");
      return;
    }

    lastMessageId.set(apiCall.id);
    const restoredConversation = [
      ...apiCall.request.prompt.messages,
      apiCall.response.completion,
    ];
    conversation.set(restoredConversation);

    goto("/chat");
  }
</script>

<InfoBox title="Actions" childNumber={1}>
  <div class="action-buttons">
    <Button on:click={restoreConversation}>Restore conversation</Button>
  </div>
</InfoBox>

<style>
  .action-buttons {
    width: fit-content;
    margin: 0 auto;
  }
</style>
