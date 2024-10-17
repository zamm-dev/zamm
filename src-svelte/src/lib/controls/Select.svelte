<script lang="ts">
  import getComponentId from "$lib/label-id";

  export let name: string;
  export let value: string;
  export let label: string;
  let id = getComponentId("select");
</script>

<div class="setting">
  <label for={id}>{label}</label>
  <div class="select-wrapper">
    <select {name} {id} bind:value>
      <slot />
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
