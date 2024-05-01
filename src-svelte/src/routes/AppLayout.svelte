<script lang="ts">
  import Snackbar from "$lib/snackbar/Snackbar.svelte";
  import Sidebar from "./Sidebar.svelte";
  import Background from "./Background.svelte";
  import "./styles.css";
  import { onMount } from "svelte";
  import PageTransition from "./PageTransition.svelte";
  import AnimationControl from "./AnimationControl.svelte";
  import { getPreferences } from "$lib/bindings";
  import {
    soundOn,
    backgroundAnimation,
    animationSpeed,
    transparencyOn,
    volume,
    animationsOn,
  } from "$lib/preferences";

  export let currentRoute: string;
  let ready = false;

  onMount(async () => {
    const prefs = await getPreferences();
    if (prefs.sound_on != null) {
      soundOn.set(prefs.sound_on);
    }

    if (prefs.volume != null) {
      volume.set(prefs.volume);
    }

    if (prefs.animations_on != null) {
      animationsOn.set(prefs.animations_on);
    }

    if (prefs.background_animation == null) {
      backgroundAnimation.set(true);
    } else {
      backgroundAnimation.set(prefs.background_animation);
    }

    if (prefs.animation_speed != null) {
      animationSpeed.set(prefs.animation_speed);
    }

    if (prefs.transparency_on != null) {
      transparencyOn.set(prefs.transparency_on);
    }

    ready = true;
  });
</script>

<div id="app">
  <AnimationControl>
    <Sidebar />

    <div id="main-container">
      <div class="background-layout">
        <Background />
      </div>
      <Snackbar />

      <div class="content-container">
        <main>
          {#if ready}
            <PageTransition {currentRoute}>
              <slot />
            </PageTransition>
          {/if}
        </main>
      </div>
    </div>
  </AnimationControl>
</div>

<style>
  #app {
    box-sizing: border-box;
    height: 100vh;
    width: 100vw;
    position: absolute;
    top: 0;
    left: 0;
    background-color: var(--color-background);
    --main-corners: var(--corner-roundness) 0 0 var(--corner-roundness);
  }

  #main-container {
    --sidebar-space: calc(var(--sidebar-width) - 0.5px);
    height: 100vh;
    box-sizing: border-box;
    margin-left: var(--sidebar-space);
    border-radius: var(--main-corners);
    background-color: var(--color-offwhite);
    box-shadow: calc(-1 * var(--shadow-offset)) 0 var(--shadow-blur) 0 #ccc;
  }

  .background-layout {
    z-index: 0;
    border-radius: var(--main-corners);
    position: absolute;
    top: 0;
    bottom: 0;
    left: var(--sidebar-space);
    right: 0;
  }

  .content-container {
    width: 100%;
    height: 100%;
    position: relative;
    overflow-y: auto;
    overflow-x: hidden;
  }

  main {
    position: relative;
    max-width: 70rem;
    margin: 0 auto;
  }
</style>
