<script lang="ts">
  import Form, { type FormFields } from "./Form.svelte";
  import type { Service } from "$lib/bindings";
  import { systemInfo } from "$lib/system-info";

  interface Props {
    name: Service;
    apiKeyUrl?: string | undefined;
    apiKey: string | null;
    editing?: boolean;
  }

  let {
    name,
    apiKeyUrl = undefined,
    apiKey,
    editing = $bindable(false),
  }: Props = $props();
  let formFields: FormFields = $state({
    apiKey: "",
    saveKey: true,
    saveKeyLocation: "",
  });

  function toggleEditing() {
    editing = !editing;
    if (editing) {
      initializeFormFields();
    }
  }

  function formClose() {
    editing = false;
  }

  function initializeFormFields() {
    if (formFields.apiKey === "") {
      formFields.apiKey = apiKey ?? "";
    }
    if (formFields.saveKeyLocation === "") {
      formFields.saveKeyLocation = $systemInfo?.shell_init_file ?? "";
    }
  }

  let active = $derived(apiKey !== null);
  let label = $derived(active ? "Active" : "Inactive");
</script>

<div class="container atomic-reveal">
  <div
    class="row"
    onclick={toggleEditing}
    onkeypress={toggleEditing}
    role="row"
    tabindex="0"
  >
    <div class="service" role="cell">{name}</div>
    <div class="api-key" class:active role="cell">{label}</div>
  </div>

  {#if editing}
    <Form {formClose} service={name} {apiKeyUrl} bind:fields={formFields} />
  {/if}
</div>

<style>
  .row {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 1rem;
    cursor: pointer;
  }

  .service {
    text-align: left;
    font-family: var(--font-body);
    flex: 1;
  }

  .api-key {
    --inactive-color: gray;
    text-align: center;
    text-transform: uppercase;
    font-family: var(--font-body);
    background-color: var(--inactive-color);
    color: white;
    flex: 1;
    border-radius: var(--corner-roundness);
    transition-property: background-color, box-shadow;
    transition-duration: var(--standard-duration);
    transition-timing-function: ease-in;
  }

  .api-key.active {
    box-shadow: 0 0 var(--shadow-blur) 0 green;
    background-color: green;
  }

  .container :global(.api-key.active.wait-for-infobox) {
    background-color: var(--inactive-color);
    box-shadow: none;
  }
</style>
