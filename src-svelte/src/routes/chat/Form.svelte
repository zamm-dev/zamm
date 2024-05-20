<script lang="ts">
  import autosize from "autosize";
  import Button from "$lib/controls/Button.svelte";
  import IconSend from "~icons/gravity-ui/arrow-right";
  import { onMount } from "svelte";

  export let sendChatMessage: (message: string) => void;
  export let chatBusy = false;
  export let currentMessage = "";
  export let onTextInputResize: () => void = () => undefined;
  let textareaInput: HTMLTextAreaElement;

  onMount(() => {
    autosize(textareaInput);
    textareaInput.addEventListener("autosize:resized", onTextInputResize);

    return () => {
      autosize.destroy(textareaInput);
    };
  });

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Enter" && !event.shiftKey && !event.ctrlKey) {
      event.preventDefault();
      submitChat();
    }
  }

  function submitChat() {
    const message = currentMessage.trim();
    if (message && !chatBusy) {
      sendChatMessage(currentMessage);
      currentMessage = "";
      requestAnimationFrame(() => {
        autosize.update(textareaInput);
      });
    }
  }
</script>

<form
  class="atomic-reveal cut-corners outer"
  autocomplete="off"
  on:submit|preventDefault={submitChat}
>
  <label for="message" class="accessibility-only">Chat with the AI:</label>
  <textarea
    id="message"
    name="message"
    placeholder="Type your message here..."
    rows="1"
    on:keydown={handleKeydown}
    bind:this={textareaInput}
    bind:value={currentMessage}
  />
  <Button ariaLabel="Send" unwrapped rightEnd><IconSend /></Button>
</form>

<style>
  form {
    display: flex;
    flex-direction: row;
    align-items: stretch;
    justify-content: space-between;
  }

  textarea {
    margin: auto 0.75rem;
    flex: 1;
    background-color: transparent;
    font-size: 1rem;
    font-family: var(--font-body);
    width: 100%;
    min-height: 1.2rem;
    max-height: 9.8rem;
    resize: none;
  }

  form :global(button) {
    width: 4rem;
    min-height: 2rem;
    display: flex;
    justify-content: center;
    align-items: center;
  }
</style>
