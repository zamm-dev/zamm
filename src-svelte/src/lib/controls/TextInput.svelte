<script lang="ts">
  interface Props {
    name: string;
    placeholder?: string | undefined;
    value: string;
  }

  let { name, placeholder = undefined, value = $bindable() }: Props = $props();
</script>

<div class="fancy-input">
  <input type="text" id={name} {name} {placeholder} bind:value />
  <span class="focus-border"></span>
</div>

<style>
  .fancy-input {
    position: relative;
    flex: 1;
  }

  input[type="text"] {
    min-width: 1rem;
    width: 100%;
    border-bottom: 1px solid var(--color-border);
    background-color: var(--color-background);
    font-family: var(--font-mono);
    font-weight: bold;
    font-size: 1rem;
  }

  input[type="text"] + .focus-border {
    position: absolute;
    bottom: -1px;
    left: 0;
    width: 0;
    height: 2px;
    background-color: blue;
    transition: width calc(0.5 * var(--standard-duration)) ease-out;
  }

  input[type="text"]:focus + .focus-border {
    width: 100%;
  }
</style>
