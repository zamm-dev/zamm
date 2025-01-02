<script lang="ts">
  import AnimationControl from "../../routes/AnimationControl.svelte";
  import Snackbar from "$lib/snackbar/Snackbar.svelte";
  import { onMount } from "svelte";
  import type { Snippet } from "svelte";
  import Background from "../../routes/Background.svelte";

  interface Props {
    fullHeight?: boolean;
    showBackground?: boolean;
    children: Snippet;
  }

  let {
    fullHeight = false,
    showBackground = false,
    children,
  }: Props = $props();
  let ready = $state(false);

  onMount(() => {
    ready = true;
  });
</script>

<div
  id="mock-app-layout"
  class="storybook-wrapper"
  class:full-height={fullHeight}
>
  {#if showBackground}
    <div class="bg">
      <Background />
    </div>
  {/if}

  <AnimationControl>
    {#if ready}
      {@render children?.()}
    {/if}
    <Snackbar />
  </AnimationControl>
</div>

<style>
  .storybook-wrapper {
    max-width: 50rem;
    position: relative;
  }

  .storybook-wrapper.full-height {
    height: calc(100vh - 2rem);
    box-sizing: border-box;
  }

  .bg {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    z-index: -1;
  }
</style>
