<script lang="ts">
  import getComponentId from "$lib/label-id";

  interface Props {
    name: string;
    value: string;
    label: string;
    children?: import("svelte").Snippet;
  }

  let { name, value = $bindable(), label, children }: Props = $props();
  let id = getComponentId("select");

  const children_render = $derived(children);
</script>

<div class="setting atomic-reveal">
  <label for={id}>{label}</label>
  <div class="select-wrapper">
    <select {name} {id} bind:value>
      {@render children_render?.()}
    </select>
  </div>
</div>

<style>
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

  select :global(option) {
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
</style>
