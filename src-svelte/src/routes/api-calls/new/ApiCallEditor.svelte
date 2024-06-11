<script lang="ts" context="module">
  import type { Prompt as PromptType, LlmCallReference } from "$lib/bindings";
  import { writable } from "svelte/store";

  export const canonicalRef = writable<LlmCallReference | undefined>(undefined);
  export const prompt = writable<PromptType>({
    type: "Chat",
    messages: [{ role: "System", text: "" }],
  });

  export function getDefaultApiCall(): PromptType {
    return {
      type: "Chat",
      messages: [{ role: "System", text: "" }],
    };
  }

  export function resetNewApiCall() {
    canonicalRef.set(undefined);
    prompt.set(getDefaultApiCall());
  }
</script>

<script lang="ts">
  import ApiCallReference from "$lib/ApiCallReference.svelte";
  import InfoBox from "$lib/InfoBox.svelte";
  import PromptComponent from "../[slug]/Prompt.svelte";
  import Button from "$lib/controls/Button.svelte";
  import { chat } from "$lib/bindings";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";
  import { goto } from "$app/navigation";

  let expectingResponse = false;

  async function submitApiCall() {
    if (expectingResponse) {
      return;
    }

    expectingResponse = true;

    try {
      const createdLlmCall = await chat({
        provider: "OpenAI",
        llm: "gpt-4",
        temperature: null,
        canonical_id: $canonicalRef?.id,
        prompt: $prompt.messages,
      });
      resetNewApiCall();

      goto(`/api-calls/${createdLlmCall.id}`);
    } catch (error) {
      snackbarError(error as string | Error);
    } finally {
      expectingResponse = false;
    }
  }
</script>

<InfoBox title="New API Call">
  {#if $canonicalRef}
    <div class="canonical-display">
      <span class="label">Original API call:</span>
      <ApiCallReference apiCall={$canonicalRef} />
    </div>
  {/if}

  <PromptComponent editable bind:prompt={$prompt} />

  <div class="action">
    <Button on:click={submitApiCall}>Submit</Button>
  </div>
</InfoBox>

<style>
  .canonical-display {
    background-color: var(--color-offwhite);
    padding: 0.75rem;
    border-radius: var(--corner-roundness);
    display: flex;
    flex-direction: row;
    gap: 0.5rem;
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
