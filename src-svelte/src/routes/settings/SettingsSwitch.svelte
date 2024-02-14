<script lang="ts">
  import Switch from "$lib/Switch.svelte";

  export let label: string;
  export let toggledOn = false;
  export let onToggle: (toggledOn: boolean) => void = () => undefined;
  let switchChild: Switch;
</script>

<div
  class="settings-switch container"
  on:click|preventDefault={switchChild.toggle}
  role="none"
>
  <Switch
    {label}
    bind:this={switchChild}
    bind:toggledOn
    {onToggle}
    letParentToggle={true}
  />
</div>

<style>
  .container {
    padding: calc(0.5 * var(--side-padding)) var(--side-padding);
    border-radius: var(--corner-roundness);
    transition: background 0.5s;
    cursor: pointer;
  }

  .container:hover {
    background: hsla(60, 100%, 50%, 0.2);
  }
</style>
