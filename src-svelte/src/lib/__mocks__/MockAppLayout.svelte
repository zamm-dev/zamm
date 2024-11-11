<script lang="ts">
  import AnimationControl from "../../routes/AnimationControl.svelte";
  import Snackbar from "$lib/snackbar/Snackbar.svelte";
  interface Props {
    children?: import("svelte").Snippet;
  }

  let { children }: Props = $props();

  const children_render = $derived(children);
</script>

<div id="mock-app-layout" class="storybook-wrapper">
  <AnimationControl>
    {@render children_render?.()}
    <Snackbar />
  </AnimationControl>
</div>

<style>
  .storybook-wrapper {
    max-width: 50rem;
    position: relative;
  }
</style>
