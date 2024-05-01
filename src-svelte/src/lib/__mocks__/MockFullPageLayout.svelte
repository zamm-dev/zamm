<script lang="ts">
  import AnimationControl from "../../routes/AnimationControl.svelte";
  import Snackbar from "$lib/snackbar/Snackbar.svelte";
</script>

<div id="mock-full-page-layout" class="storybook-wrapper full-height">
  <AnimationControl>
    <slot />
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
