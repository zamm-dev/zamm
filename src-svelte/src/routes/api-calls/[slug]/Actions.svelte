<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
  import { type LlmCall } from "$lib/bindings";
  import { lastMessageId, conversation } from "../../chat/Chat.svelte";
  import { canonicalRef, prompt } from "../new/ApiCallEditor.svelte";
  import { goto } from "$app/navigation";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";
  import Button from "$lib/controls/Button.svelte";
  import ButtonGroup from "$lib/controls/ButtonGroup.svelte";

  export let apiCall: LlmCall | undefined = undefined;

  function editApiCall() {
    if (!apiCall) {
      snackbarError("API call not yet loaded");
      return;
    }

    if (apiCall.request.prompt.type === "Unknown") {
      snackbarError("Can't edit unknown prompt type");
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

    if (apiCall.request.prompt.type === "Unknown") {
      snackbarError("Can't restore unknown prompt type");
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
  <ButtonGroup>
    <Button unwrapped leftEnd on:click={editApiCall}>Edit API call</Button>
    <Button unwrapped rightEnd on:click={restoreConversation}
      >Restore conversation</Button
    >
  </ButtonGroup>
</InfoBox>
