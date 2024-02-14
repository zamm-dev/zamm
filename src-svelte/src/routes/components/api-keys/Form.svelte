<script lang="ts" context="module">
  export interface FormFields {
    apiKey: string;
    saveKey: boolean;
    saveKeyLocation: string;
  }
</script>

<script lang="ts">
  import { cubicInOut } from "svelte/easing";
  import { getApiKeys, setApiKey, type Service } from "$lib/bindings";
  import { standardDuration } from "$lib/preferences";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";
  import { apiKeys } from "$lib/system-info";
  import TextInput from "$lib/controls/TextInput.svelte";
  import Button from "$lib/controls/Button.svelte";
  import Explanation from "$lib/Explanation.svelte";

  export let service: Service;
  export let apiKeyUrl: string | undefined = undefined;
  export let fields: FormFields;
  export let formClose: () => void = () => undefined;
  const exportExplanation =
    `Exports this API key for use in other programs on your computer.\n` +
    `Don't worry about this option if you're not a programmer.`;

  $: growDuration = 2 * $standardDuration;

  function growY(node: HTMLElement) {
    const rem = 18;
    const totalFinalPadding = 1 * rem;

    const height = node.offsetHeight;
    return {
      duration: growDuration,
      easing: cubicInOut,
      tick: (t: number) => {
        const totalHeight = height * t;
        const totalCurrentPadding = Math.min(totalFinalPadding, totalHeight);
        const contentHeight = totalHeight - totalCurrentPadding;
        node.style.setProperty(
          "--vertical-padding",
          `${totalCurrentPadding / 2}px`,
        );
        node.style.setProperty("--form-height", `${contentHeight}px`);
      },
    };
  }

  function submitApiKey() {
    setApiKey(
      fields.saveKey ? fields.saveKeyLocation : null,
      service,
      fields.apiKey,
    )
      .then(() => {
        formClose();
      })
      .catch((err) => {
        snackbarError(err);
      })
      .finally(() => {
        setTimeout(async () => {
          // delay here instead of in CSS transition so that the text updates
          // simultaneously with the transition
          apiKeys.set(await getApiKeys());
        }, 0.75 * growDuration);
      });
  }
</script>

<div class="container" transition:growY>
  <div class="inset-container">
    <form on:submit|preventDefault={submitApiKey}>
      {#if apiKeyUrl}
        <p>
          Tip: Get your {service} key
          <a href={apiKeyUrl} target="_blank">here</a>.
        </p>
      {/if}

      <div class="form-row">
        <label for="apiKey">API key:</label>
        <TextInput name="apiKey" bind:value={fields.apiKey} />
      </div>

      <div class="form-row">
        <label for="saveKey" class="accessibility-only"
          >Export as environment variable?</label
        >
        <input
          type="checkbox"
          id="saveKey"
          name="saveKey"
          bind:checked={fields.saveKey}
        />
        <div>
          <label for="saveKeyLocation">Export from:</label>
          <Explanation text={exportExplanation} />
        </div>
        <TextInput
          name="saveKeyLocation"
          placeholder="e.g. /home/user/.bashrc"
          bind:value={fields.saveKeyLocation}
        />
      </div>

      <div class="save-button">
        <Button text="Save" />
      </div>
    </form>
  </div>
</div>

<style>
  .container {
    --form-height: 100%;
    --vertical-padding: 0.5rem;
    --horizontal-overshoot: 1rem;
    overflow: hidden;
    margin: 0 calc(-1 * var(--horizontal-overshoot));
    padding: var(--vertical-padding) 0;
  }

  .inset-container {
    height: var(--form-height);
    overflow: hidden;
    box-shadow: inset 0.05em 0.05em 0.3em rgba(0, 0, 0, 0.4);
    background-color: var(--color-background);
  }

  form {
    padding: 0.5rem var(--horizontal-overshoot);
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    flex-wrap: nowrap;
  }

  form p {
    margin: 0 0 0.25rem;
    color: #666666;
    text-align: center;
  }

  label {
    white-space: nowrap;
  }

  .form-row {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .save-button {
    align-self: flex-start;
  }
</style>
