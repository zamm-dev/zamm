<script lang="ts">
  import MockAppLayout from "./MockAppLayout.svelte";
  import PageTransition from "../../routes/PageTransition.svelte";
  import { firstAppLoad, firstPageLoad } from "$lib/firstPageLoad";
  import { animationSpeed, transparencyOn } from "$lib/preferences";
  import Background from "../../routes/Background.svelte";
  import { onMount, type Snippet } from "svelte";

  interface Props {
    children: Snippet;
  }

  let { children }: Props = $props();
  let ready = $state(false);

  onMount(() => {
    firstAppLoad.set(true);
    firstPageLoad.set(true);
    animationSpeed.set(0.1);
    transparencyOn.set(true);

    setTimeout(() => {
      ready = true;
    }, 500);
  });
</script>

<div id="mock-transitions">
  <MockAppLayout animated fullHeight>
    <div class="bg">
      <Background />
    </div>

    {#if ready}
      <PageTransition currentRoute="/storybook-demo">
        {@render children?.()}
      </PageTransition>
    {/if}
  </MockAppLayout>
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
