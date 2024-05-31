<script lang="ts" context="module">
  import type { Prompt as PromptType } from "$lib/bindings";
  import { writable } from "svelte/store";

  export const prompt = writable<PromptType>({
    type: "Chat",
    messages: [{ role: "System", text: "" }],
  });
</script>

<script lang="ts">
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
        prompt: $prompt.messages,
      });

      goto(`/api-calls/${createdLlmCall.id}`);
    } catch (error) {
      snackbarError(error as string | Error);
    } finally {
      expectingResponse = false;
    }
  }
</script>

<InfoBox title="New API Call">
  <PromptComponent editable bind:prompt={$prompt} />

  <div class="action">
    <Button on:click={submitApiCall}>Submit</Button>
  </div>
</InfoBox>

<style>
  .action :global(button) {
    width: 100%;
  }
</style>
