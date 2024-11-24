<script lang="ts">
  import AnimationControl from "../../routes/AnimationControl.svelte";
  import Snackbar from "$lib/snackbar/Snackbar.svelte";
  import { onMount } from "svelte";
  import { animationsOn } from "$lib/preferences";
  import type { Snippet } from "svelte";

  interface Props {
    fullHeight?: boolean;
    animated?: boolean;
    children: Snippet;
  }

  let { fullHeight = false, animated = false, children }: Props = $props();
  let ready = $state(false);

  onMount(() => {
    animationsOn.set(animated);
    ready = true;
  });
</script>

<div
  id="mock-app-layout"
  class="storybook-wrapper"
  class:full-height={fullHeight}
>
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
</style>
