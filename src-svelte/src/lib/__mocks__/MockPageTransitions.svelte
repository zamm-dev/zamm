<script lang="ts">
  import MockFullPageLayout from "./MockFullPageLayout.svelte";
  import PageTransition from "../../routes/PageTransition.svelte";
  import { firstAppLoad, firstPageLoad } from "$lib/firstPageLoad";
  import {
    animationSpeed,
    transparencyOn,
    backgroundAnimation,
  } from "$lib/preferences";
  import Background from "../../routes/Background.svelte";

  firstAppLoad.set(true);
  firstPageLoad.set(true);
  animationSpeed.set(0.1);
  transparencyOn.set(true);
  backgroundAnimation.set(true);
</script>

<div id="mock-transitions">
  <MockFullPageLayout>
    <div class="bg">
      <Background />
    </div>

    <PageTransition currentRoute="/storybook-demo">
      <slot />
    </PageTransition>
  </MockFullPageLayout>
</div>

<style>
  #mock-transitions {
    margin: -1rem;
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
