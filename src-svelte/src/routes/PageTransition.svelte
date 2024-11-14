<script lang="ts" module>
  import { writable } from "svelte/store";
  import { cubicIn, backOut } from "svelte/easing";

  export interface PageTransitionContext {
    addVisitedRoute: (newRoute: string) => void;
  }
  export const pageTransition = writable<PageTransitionContext | null>(null);

  export enum TransitionType {
    // user just opened the app
    Init,
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
  import { onMount } from "svelte";

  interface Props {
    currentRoute: string;
    children?: import("svelte").Snippet;
  }

  let { currentRoute, children }: Props = $props();
  let oldRoute = currentRoute;
  let ready = $state(false);
  const visitedKeys = new Set<string>();
  let transitions: Transitions = $state(getTransitions(TransitionType.Init));

  onMount(async () => {
    ready = true;
    setTimeout(() => {
      firstAppLoad.set(false);
    }, $standardDuration);

    pageTransition.set({
      addVisitedRoute: (newRoute: string) => {
        visitedKeys.add(newRoute);
      },
    });
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
      case TransitionType.Init:
        return {
          out: {
            // doesn't matter for init
            x: "-20%",
            duration: 0,
            easing: cubicIn,
            delay: 0,
          },
          in: {
            x: "-20%",
            duration: $standardDuration,
            easing: backOut,
            delay: 0,
          },
        };
    }
  }

  function updateFlyDirection(route: string) {
    if (!ready) {
      return;
    }

    if (oldRoute === route) {
      return;
    }

    const direction = getTransitionType(oldRoute, route);
    transitions = getTransitions(direction);
    oldRoute = route;
  }

  $effect.pre(() => {
    checkFirstPageLoad(currentRoute);
    updateFlyDirection(currentRoute);
  });

  const children_render = $derived(children);
</script>

{#key currentRoute}
  {#if ready}
    <div
      class="transition-container full-height"
      in:fly|global={transitions.in}
      out:fly|global={transitions.out}
    >
      {@render children_render?.()}
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
