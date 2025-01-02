<script lang="ts">
  import MockAppLayout from "./MockAppLayout.svelte";
  import PageTransition from "../../routes/PageTransition.svelte";
  import { firstAppLoad, firstPageLoad } from "$lib/firstPageLoad";
  import {
    animationSpeed,
    transparencyOn,
    animationsOn,
  } from "$lib/preferences";
  import { onMount, type Snippet } from "svelte";

  interface Props {
    customStoreValues?: boolean;
    children: Snippet;
  }

  let { customStoreValues = false, children }: Props = $props();
  let ready = $state(false);

  onMount(() => {
    firstAppLoad.set(true);
    firstPageLoad.set(true);
    animationsOn.set(true);
    transparencyOn.set(true);
    if (!customStoreValues) {
      animationSpeed.set(0.1);
    }

    setTimeout(() => {
      ready = true;
    }, 500);
  });
</script>

<div id="mock-transitions">
  <MockAppLayout fullHeight showBackground>
    {#if ready}
      <PageTransition currentRoute="/storybook-demo">
        {@render children?.()}
      </PageTransition>
    {/if}
  </MockAppLayout>
</div>

<style>
  #mock-transitions {
    width: 100vw;
    height: 100vh;
    margin: -1rem;
    overflow: hidden;
  }
</style>
