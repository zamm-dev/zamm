<script lang="ts">
  import MockFullPageLayout from "./MockFullPageLayout.svelte";
  import PageTransition from "../../routes/PageTransition.svelte";
  import { firstAppLoad, firstPageLoad } from "$lib/firstPageLoad";
  import { animationSpeed, transparencyOn } from "$lib/preferences";
  import Background from "../../routes/Background.svelte";
  import { onMount } from "svelte";
  interface Props {
    children?: import("svelte").Snippet;
  }

  let { children }: Props = $props();
  let showChildren = $state(false);

  onMount(() => {
    firstAppLoad.set(true);
    firstPageLoad.set(true);
    animationSpeed.set(0.1);
    transparencyOn.set(true);

    setTimeout(() => {
      showChildren = true;
    }, 500);
  });

  const children_render = $derived(children);
</script>

<div id="mock-transitions">
  <MockFullPageLayout>
    <div class="bg">
      <Background />
    </div>

    {#if showChildren}
      <PageTransition currentRoute="/storybook-demo">
        {@render children_render?.()}
      </PageTransition>
    {/if}
  </MockFullPageLayout>
</div>

<style>
  #mock-transitions {
    margin: -1rem;
    overflow: hidden;
  }

  .bg {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    z-index: -1;
  }
</style>
