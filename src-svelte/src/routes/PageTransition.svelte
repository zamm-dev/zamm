<script lang="ts" context="module">
  import { cubicIn, backOut } from "svelte/easing";

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

  export function getTransitionTiming(
    totalDurationMs: number,
    overlapFraction: number,
  ): TransitionTiming {
    const spacingFraction = -overlapFraction;
    // let
    //   d = duration of a single transition
    //   s = spacing between transitions as fraction of d
    //   and T = total duration of entire transition,
    // then
    //   d + (d * (1 + spacing)) = T
    //   d * (2 + spacing) = T
    //   d = T / (2 + spacing)
    const transitionDurationMs = totalDurationMs / (2 + spacingFraction);
    const transitionDelayMs = transitionDurationMs * (1 + spacingFraction);
    return {
      duration: Math.round(transitionDurationMs),
      delay: Math.round(transitionDelayMs),
    };
  }

  export function getTransitions(
    totalDurationMs: number,
    overlapFraction: number,
  ): Transitions {
    const { duration, delay } = getTransitionTiming(
      totalDurationMs,
      overlapFraction,
    );
    const out = { x: "-20%", duration, easing: cubicIn };
    return {
      out,
      in: { ...out, delay, easing: backOut },
    };
  }
</script>

<script lang="ts">
  import { fly } from "svelte/transition";
  import { standardDuration } from "$lib/preferences";
  import { firstAppLoad, firstPageLoad } from "$lib/firstPageLoad";
  import { onMount, tick } from "svelte";

  export let currentRoute: string;
  let ready = false;
  const visitedKeys = new Set<string>();

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

  // twice the speed of sidebar UI slider
  $: totalDurationMs = 2 * $standardDuration;
  $: transitions = getTransitions(totalDurationMs, 0);
  $: checkFirstPageLoad(currentRoute);
</script>

{#key currentRoute}
  {#if ready}
    <div
      class="transition-container"
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
    height: 100%;
    width: 100%;
    box-sizing: border-box;
    padding: 1rem;
  }
</style>
