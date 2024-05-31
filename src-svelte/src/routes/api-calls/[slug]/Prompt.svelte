<script lang="ts">
  import autosize from "autosize";
  import SubInfoBox from "$lib/SubInfoBox.svelte";
  import { type Prompt } from "$lib/bindings";
  import { onMount } from "svelte";

  export let prompt: Prompt;
  export let editable = false;
  let textareas: HTMLTextAreaElement[] = [];

  function toggleRole(i: number) {
    if (!editable) {
      return;
    }

    switch (prompt.messages[i].role) {
      case "System":
        prompt.messages[i].role = "Human";
        break;
      case "Human":
        prompt.messages[i].role = "AI";
        break;
      case "AI":
        prompt.messages[i].role = "System";
        break;
    }
  }

  function editText(
    i: number,
    e: Event & { currentTarget: EventTarget & HTMLTextAreaElement },
  ) {
    prompt.messages[i].text = e.currentTarget.value;
  }

  function addMessage() {
    prompt.messages = [...prompt.messages, { role: "System", text: "" }];
  }

  onMount(() => {
    textareas.forEach((textarea) => autosize(textarea));
    return () => {
      textareas.forEach((textarea) => autosize.destroy(textarea));
    };
  });
</script>

<SubInfoBox subheading="Prompt">
  <div class="prompt composite-reveal">
    {#each prompt.messages ?? [] as message, i}
      <div class={"message atomic-reveal " + message.role.toLowerCase()}>
        <span
          class="role"
          class:editable
          role="button"
          aria-label="Toggle message type"
          tabindex="0"
          on:click={() => toggleRole(i)}
          on:keypress={() => toggleRole(i)}>{message.role}</span
        >
        {#if editable}
          <textarea
            rows="1"
            placeholder="Set text for new prompt message..."
            value={message.text}
            on:input={(e) => editText(i, e)}
            bind:this={textareas[i]}
          />
        {:else}
          <pre>{message.text}</pre>
        {/if}
      </div>
    {/each}
    {#if editable}
      <button title="Add a new message to the chat" on:click={addMessage}
        >+</button
      >
    {/if}
  </div>
</SubInfoBox>

<style>
  .prompt {
    margin-bottom: 1rem;
  }

  .role.editable {
    cursor: pointer;
  }

  .message {
    transition-property: background-color;
    transition: var(--standard-duration) ease-out;
    margin: 0.5rem 0;
    padding: 0.5rem;
    border-radius: var(--corner-roundness);
    display: flex;
    gap: 1rem;
    align-items: center;
  }

  .message.system {
    background-color: var(--color-system);
  }

  .message.human {
    background-color: var(--color-human);
  }

  .message.ai {
    background-color: var(--color-ai);
  }

  .message .role {
    color: var(--color-faded);
    width: 4rem;
    min-width: 4rem;
    text-align: center;
  }

  textarea {
    font-family: var(--font-mono);
    font-size: 1rem;
    background-color: transparent;
    width: 100%;
    resize: none;
  }

  button {
    width: 100%;
    padding: 0.2rem;
    box-sizing: border-box;
    background: transparent;
    border: 2px dashed var(--color-border);
    border-radius: var(--corner-roundness);
    font-size: 1rem;
    color: var(--color-faded);
    cursor: pointer;
    transition: calc(0.5 * var(--standard-duration)) ease-out;
  }

  button:hover {
    background: var(--color-border);
    color: #fff;
  }
</style>
