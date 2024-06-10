<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
  import { type LlmCall } from "$lib/bindings";
  import { lastMessageId, conversation } from "../../chat/Chat.svelte";
  import { canonicalRef, prompt } from "../new/ApiCallEditor.svelte";
  import { goto } from "$app/navigation";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";
  import Button from "$lib/controls/Button.svelte";

  export let apiCall: LlmCall | undefined = undefined;

  function editApiCall() {
    if (!apiCall) {
      snackbarError("API call not yet loaded");
      return;
    }

    canonicalRef.set({
      id: apiCall.id,
      snippet: apiCall.response.completion.text,
    });
    prompt.set(apiCall.request.prompt);

    goto("/api-calls/new/");
  }

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
    <div class="button-container cut-corners outer">
      <Button unwrapped leftEnd on:click={editApiCall}>Edit API call</Button>
      <Button unwrapped rightEnd on:click={restoreConversation}
        >Restore conversation</Button
      >
    </div>
  </div>
</InfoBox>

<style>
  .action-buttons {
    width: fit-content;
    margin: 0 auto;
  }

  .button-container {
    display: flex;
  }

  .button-container :global(.left-end) {
    --cut-top-left: 8px;
  }

  .button-container :global(.right-end) {
    --cut-bottom-right: 8px;
  }

  @media (max-width: 35rem) {
    .button-container {
      flex-direction: column;
    }
  }
</style>
