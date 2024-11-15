<script lang="ts">
  import Switch from "$lib/Switch.svelte";

  interface Props {
    label: string;
    toggledOn?: boolean;
    onToggle?: (toggledOn: boolean) => void;
  }

  let {
    label,
    toggledOn = $bindable(false),
    onToggle = () => undefined,
  }: Props = $props();
  let switchChild: Switch | undefined;

  function toggleSwitchChild(e: Event) {
    e.preventDefault();
    switchChild?.toggle();
  }
</script>

<div
  class="settings-switch container atomic-reveal"
  onclick={toggleSwitchChild}
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
    background: var(--color-hover);
  }
</style>
