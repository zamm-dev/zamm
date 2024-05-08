<script lang="ts" context="module">
  import { cubicIn, backOut } from "svelte/easing";

  export enum TransitionType {
    // user is going leftward -- meaning both incoming and outgoing are moving right
    Left,
    // user is going rightward -- meaning both incoming and outgoing are moving left
    Right,
    // incoming goes left, outgoing goes right
    Swap,
  }

  interface TransitionTiming {
    duration: number;
    delay?: number;
  }

  interface Transition extends TransitionTiming {
    x: string;
    easing: (t: number) => number;
  }

  interface Transitions {
    out: Transition;
    in: Transition;
  }

  export function getTransitionType(
    oldRoute: string,
    newRoute: string,
  ): TransitionType {
    if (oldRoute === "/" || newRoute === "/") {
      return TransitionType.Swap;
    }

    if (newRoute.startsWith(oldRoute)) {
      // e.g. we're drilling down from /path/ to /path/subpath/
      return TransitionType.Right;
    }

    if (oldRoute.startsWith(newRoute)) {
      // e.g. we're moving back up from /path/subpath/ to /path/
      return TransitionType.Left;
    }

    return TransitionType.Swap;
  }
</script>

<script lang="ts">
  import { fly } from "svelte/transition";
  import { standardDuration } from "$lib/preferences";
  import { firstAppLoad, firstPageLoad } from "$lib/firstPageLoad";
  import { onMount, tick } from "svelte";

  export let currentRoute: string;
  let oldRoute = currentRoute;
  let ready = false;
  const visitedKeys = new Set<string>();
  let transitions: Transitions = getTransitions(TransitionType.Swap);

  onMount(async () => {
    const regularDelay = transitions.in.delay;
    transitions.in.delay = 0;
    ready = true;
    await tick();
    transitions.in.delay = regularDelay;
    firstAppLoad.set(false);
    checkFirstPageLoad(currentRoute);
  });

  function checkFirstPageLoad(route: string) {
    if (!ready) {
      return;
    }

    firstPageLoad.set(!visitedKeys.has(route));
    visitedKeys.add(route);
  }

  function getTransitions(transitionType: TransitionType) {
    const baseSlideTransition = {
      duration: $standardDuration,
      easing: cubicIn,
      delay: 0,
    };

    switch (transitionType) {
      case TransitionType.Right:
        return {
          out: {
            ...baseSlideTransition,
            x: "-100%",
          },
          in: {
            ...baseSlideTransition,
            x: "100%",
          },
        };
      case TransitionType.Left:
        return {
          out: {
            ...baseSlideTransition,
            x: "100%",
          },
          in: {
            ...baseSlideTransition,
            x: "-100%",
          },
        };
      case TransitionType.Swap:
        return {
          out: {
            x: "-20%",
            duration: $standardDuration,
            easing: cubicIn,
            delay: 0,
          },
          in: {
            x: "-20%",
            duration: $standardDuration,
            easing: backOut,
            delay: $standardDuration,
          },
        };
    }
  }

  function updateFlyDirection(route: string) {
    if (!ready) {
      return;
    }

    const direction = getTransitionType(oldRoute, route);
    transitions = getTransitions(direction);
    oldRoute = route;
  }

  $: checkFirstPageLoad(currentRoute);
  $: updateFlyDirection(currentRoute);
</script>

{#key currentRoute}
  {#if ready}
    <div
      class="transition-container full-height"
      in:fly|global={transitions.in}
      out:fly|global={transitions.out}
    >
      <slot />
    </div>
  {/if}
{/key}

<style>
  .transition-container {
    position: absolute;
    min-height: 100vh;
    width: 100%;
    box-sizing: border-box;
    padding: 1rem;
  }
</style>
