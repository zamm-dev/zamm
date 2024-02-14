<script lang="ts">
  import autosize from "autosize";
  import Button from "$lib/controls/Button.svelte";
  import { onMount } from "svelte";

  export let sendChatMessage: (message: string) => void;
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
    if (message) {
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
  <Button unwrapped rightEnd text="Send" />
</form>

<style>
  form {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
  }

  textarea {
    margin: 0 0.75rem;
    flex: 1;
    background-color: transparent;
    font-size: 1rem;
    font-family: var(--font-body);
    width: 100%;
    min-height: 1.2rem;
    resize: none;
  }
</style>
