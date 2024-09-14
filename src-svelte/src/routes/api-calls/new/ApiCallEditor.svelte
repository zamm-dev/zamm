<script lang="ts" context="module">
  import type { LlmCallReference } from "$lib/bindings";
  import type { ChatPromptVariant } from "$lib/additionalTypes";
  import { writable } from "svelte/store";

  interface Model {
    apiName: string;
    humanName: string;
  }

  const OPENAI_MODELS: Model[] = [
    { apiName: "gpt-4", humanName: "GPT-4" },
    { apiName: "gpt-4o", humanName: "GPT-4o" },
    { apiName: "gpt-4o-mini", humanName: "GPT-4o mini" },
  ];

  const OLLAMA_MODELS: Model[] = [
    { apiName: "llama3:8b", humanName: "Llama 3" },
    { apiName: "gemma2:9b", humanName: "Gemma 2" },
  ];

  export const canonicalRef = writable<LlmCallReference | undefined>(undefined);
  export const prompt = writable<ChatPromptVariant>({
    type: "Chat",
    messages: [{ role: "System", text: "" }],
  });
  export const provider = writable<"OpenAI" | "Ollama">("OpenAI");
  export const llm = writable<string>("gpt-4");

  export function getDefaultApiCall(): ChatPromptVariant {
    return {
      type: "Chat",
      messages: [{ role: "System", text: "" }],
    };
  }

  function resetApiCallConversation() {
    canonicalRef.set(undefined);
    prompt.set(getDefaultApiCall());
  }

  export function resetNewApiCall() {
    resetApiCallConversation();
    provider.set("OpenAI");
    llm.set("gpt-4");
  }
</script>

<script lang="ts">
  import ApiCallReference from "$lib/ApiCallReference.svelte";
  import InfoBox from "$lib/InfoBox.svelte";
  import PromptComponent from "../[slug]/Prompt.svelte";
  import Button from "$lib/controls/Button.svelte";
  import EmptyPlaceholder from "$lib/EmptyPlaceholder.svelte";
  import { chat } from "$lib/bindings";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";

  export let expectingResponse = false;
  let selectModels = OPENAI_MODELS;

  async function submitApiCall() {
    if (expectingResponse) {
      return;
    }

    expectingResponse = true;

    try {
      const createdLlmCall = await chat({
        provider: $provider,
        llm: $llm,
        temperature: null,
        canonical_id: $canonicalRef?.id,
        prompt: $prompt.messages,
      });
      resetApiCallConversation();

      goto(`/api-calls/${createdLlmCall.id}`);
    } catch (error) {
      snackbarError(error as string | Error);
    } finally {
      expectingResponse = false;
    }
  }

  onMount(() => {
    let initial = true;

    const unsubscribeProvider = provider.subscribe((newProvider: string) => {
      if (initial) {
        initial = false;
        selectModels = newProvider === "OpenAI" ? OPENAI_MODELS : OLLAMA_MODELS;
        return;
      }

      selectModels = newProvider === "OpenAI" ? OPENAI_MODELS : OLLAMA_MODELS;
      llm.set(selectModels[0].apiName);
    });

    return unsubscribeProvider;
  });
</script>

<InfoBox title="New API Call">
  <EmptyPlaceholder>
    Manually build an API call below, or <a href="/api-calls/new/import/"
      >import one</a
    > from another source.
  </EmptyPlaceholder>

  <div class="model-settings">
    <div class="setting">
      <label for="provider">Provider: </label>
      <div class="select-wrapper">
        <select name="provider" id="provider" bind:value={$provider}>
          <option value="OpenAI">OpenAI</option>
          <option value="Ollama">Ollama</option>
        </select>
      </div>
    </div>

    <div class="setting">
      <label for="model">Model: </label>
      <div class="select-wrapper">
        <select name="model" id="model" bind:value={$llm}>
          {#each selectModels as model}
            <option value={model.apiName}>{model.humanName}</option>
          {/each}
        </select>
      </div>
    </div>

    {#if $canonicalRef}
      <div class="setting canonical-display">
        <span class="label">Original API call:</span>
        <ApiCallReference apiCall={$canonicalRef} />
      </div>
    {/if}
  </div>

  <PromptComponent editable bind:prompt={$prompt} />

  <div class="action">
    <Button disabled={expectingResponse} on:click={submitApiCall}>Submit</Button
    >
  </div>
</InfoBox>

<style>
  .model-settings {
    display: grid;
    grid-template-columns: 1fr 1fr;
    column-gap: 1rem;
    row-gap: 0.5rem;
    margin-bottom: 1rem;
  }

  .setting {
    display: flex;
    flex-direction: row;
    gap: 0.5rem;
  }

  select {
    -webkit-appearance: none;
    -ms-appearance: none;
    -moz-appearance: none;
    appearance: none;
    border: none;
    padding-right: 1rem;
    box-sizing: border-box;
    direction: rtl;
    width: 100%;
  }

  select option {
    direction: ltr;
  }

  select,
  .select-wrapper {
    background-color: transparent;
    font-family: var(--font-body);
    font-size: 1rem;
  }

  .select-wrapper {
    flex: 1;
    border-bottom: 1px dotted var(--color-faded);
    position: relative;
    display: inline-block;
  }

  .select-wrapper::after {
    content: "▼";
    display: inline-block;
    position: absolute;
    right: 0.25rem;
    top: 0.35rem;
    color: var(--color-faded);
    font-size: 0.5rem;
    pointer-events: none;
  }

  .setting.canonical-display {
    display: flex;
    flex-direction: row;
    gap: 0.5rem;
    grid-column: span 2;
    align-items: center;
  }

  .canonical-display .label {
    white-space: nowrap;
  }

  .action {
    width: 100%;
    display: flex;
    justify-content: center;
  }

  .action :global(button) {
    width: 100%;
    max-width: 25rem;
  }
</style>
