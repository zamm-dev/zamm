<script lang="ts">
  import AnimationControl from "../../routes/AnimationControl.svelte";
  import Snackbar from "$lib/snackbar/Snackbar.svelte";
  import Background from "../../routes/Background.svelte";
</script>

<div id="mock-full-page-layout" class="storybook-wrapper full-height">
  <div class="bg">
    <Background />
  </div>
  <AnimationControl>
    <slot />
    <Snackbar />
  </AnimationControl>
</div>

<style>
  .bg {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    z-index: -1;
  }

  .storybook-wrapper {
    max-width: 50rem;
    position: relative;
    /* these are the only two lines that are different from MockAppLayout.svelte */
    height: calc(100vh - 2rem);
    box-sizing: border-box;
  }
</style>
