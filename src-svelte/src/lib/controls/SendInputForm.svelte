<script lang="ts">
  import autosize from "autosize";
  import Button from "$lib/controls/Button.svelte";
  import IconSend from "~icons/gravity-ui/arrow-right";
  import { onMount } from "svelte";

  interface Props {
    sendInput: (message: string) => void;
    accessibilityLabel: string;
    isBusy?: boolean;
    currentMessage?: string;
    placeholder?: string;
    onTextInputResize?: () => void;
  }

  let {
    sendInput,
    accessibilityLabel,
    isBusy = false,
    currentMessage = $bindable(""),
    placeholder = "Type your message here...",
    onTextInputResize = () => undefined,
  }: Props = $props();
  let textareaInput: HTMLTextAreaElement | null = null;

  onMount(() => {
    if (!textareaInput) {
      throw new Error("Textarea input not found");
    }

    autosize(textareaInput);
    textareaInput.addEventListener("autosize:resized", onTextInputResize);

    return () => {
      if (!textareaInput) {
        throw new Error("Textarea input not found");
      }

      autosize.destroy(textareaInput);
    };
  });

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Enter" && !event.shiftKey && !event.ctrlKey) {
      submitInput(event);
    }
  }

  function submitInput(e: Event) {
    e.preventDefault();
    const message = currentMessage.trim();
    if (message && !isBusy) {
      sendInput(currentMessage);
      currentMessage = "";
      requestAnimationFrame(() => {
        if (!textareaInput) {
          console.error("Textarea input not found");
          return;
        }
        autosize.update(textareaInput);
      });
    }
  }
</script>

<form
  class="atomic-reveal cut-corners outer"
  autocomplete="off"
  onsubmit={submitInput}
>
  <label for="message" class="accessibility-only">{accessibilityLabel}</label>
  <textarea
    id="message"
    name="message"
    rows="1"
    {placeholder}
    onkeydown={handleKeydown}
    bind:this={textareaInput}
    bind:value={currentMessage}
  ></textarea>
  <Button ariaLabel="Send" disabled={isBusy} unwrapped rightEnd
    ><IconSend /></Button
  >
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
