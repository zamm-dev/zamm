<script lang="ts">
  import AnimationControl from "../../routes/AnimationControl.svelte";
  import Snackbar from "$lib/snackbar/Snackbar.svelte";
  interface Props {
    children?: import("svelte").Snippet;
  }

  let { children }: Props = $props();

  const children_render = $derived(children);
</script>

<div id="mock-full-page-layout" class="storybook-wrapper full-height">
  <AnimationControl>
    {@render children_render?.()}
    <Snackbar />
  </AnimationControl>
</div>

<style>
  .storybook-wrapper {
    max-width: 50rem;
    position: relative;
    /* these are the only two lines that are different from MockAppLayout.svelte */
    height: calc(100vh - 2rem);
    box-sizing: border-box;
  }
</style>
