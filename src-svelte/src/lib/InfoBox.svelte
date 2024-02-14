<script lang="ts" context="module">
  import {
    type TransitionTimingMs,
    type TransitionTimingFraction,
    PrimitiveTimingMs,
    PrimitiveTimingFraction,
    TimingGroupAsCollection,
    TimingGroupAsIndividual,
    inverseCubicInOut,
  } from "$lib/animation-timing";

  class BorderBoxTimingCollection extends TimingGroupAsCollection {
    growX(): TransitionTimingMs {
      return this.children[0];
    }

    growY(): TransitionTimingMs {
      return this.children[1];
    }

    round(): BorderBoxTimingCollection {
      return new BorderBoxTimingCollection(super.round().children);
    }

    delayByMs(delayMs: number): BorderBoxTimingCollection {
      return new BorderBoxTimingCollection(super.delayByMs(delayMs).children);
    }

    scaleBy(factor: number): BorderBoxTimingCollection {
      return new BorderBoxTimingCollection(super.scaleBy(factor).children);
    }

    asIndividual(): BorderBoxTiming {
      const groupTimingFraction = super.asIndividual();
      return new BorderBoxTiming({
        overall: groupTimingFraction.overall,
        children: groupTimingFraction.children,
      });
    }
  }

  export function newBorderBoxTimingCollection({
    growX,
    growY,
  }: {
    growX: TransitionTimingMs;
    growY: TransitionTimingMs;
  }): BorderBoxTimingCollection {
    return new BorderBoxTimingCollection([growX, growY]);
  }

  export class BorderBoxTiming extends TimingGroupAsIndividual {
    growX(): TransitionTimingFraction {
      return this.children[0];
    }

    growY(): TransitionTimingFraction {
      return this.children[1];
    }

    round(): BorderBoxTiming {
      const rounded = super.round();
      return new BorderBoxTiming({
        overall: rounded.overall,
        children: rounded.children,
      });
    }

    asCollection(): BorderBoxTimingCollection {
      const groupTimingMs = super.asCollection();
      return new BorderBoxTimingCollection(groupTimingMs.children);
    }
  }

  class TitleTimingCollection extends TimingGroupAsCollection {
    typewriter(): TransitionTimingMs {
      return this.children[0];
    }

    cursorFade(): TransitionTimingMs {
      return this.children[1];
    }

    delayByMs(delayMs: number): TitleTimingCollection {
      return new TitleTimingCollection(super.delayByMs(delayMs).children);
    }

    scaleBy(factor: number): TitleTimingCollection {
      return new TitleTimingCollection(super.scaleBy(factor).children);
    }

    asIndividual(): TitleTiming {
      const groupTimingFraction = super.asIndividual();
      return new TitleTiming({
        overall: groupTimingFraction.overall,
        children: groupTimingFraction.children,
      });
    }
  }

  export function newTitleTimingCollection({
    typewriter,
    cursorFade,
  }: {
    typewriter: TransitionTimingMs;
    cursorFade: TransitionTimingMs;
  }): TitleTimingCollection {
    return new TitleTimingCollection([typewriter, cursorFade]);
  }

  export class TitleTiming extends TimingGroupAsIndividual {
    typewriter(): TransitionTimingFraction {
      return this.children[0];
    }

    cursorFade(): TransitionTimingFraction {
      return this.children[1];
    }

    asCollection(): TitleTimingCollection {
      const groupTimingMs = super.asCollection();
      return new TitleTimingCollection(groupTimingMs.children);
    }
  }

  export interface InfoBoxTiming {
    borderBox: BorderBoxTiming;
    title: TitleTiming;
    infoBox: TransitionTimingMs;
    overallFadeIn: TransitionTimingMs;
  }

  class InfoBoxTimingCollection extends TimingGroupAsCollection {
    borderBox(): BorderBoxTimingCollection {
      return this.children[0] as BorderBoxTimingCollection;
    }

    title(): TitleTimingCollection {
      return this.children[1] as TitleTimingCollection;
    }

    infoBox(): TransitionTimingMs {
      return this.children[2];
    }

    overallFadeIn(): TransitionTimingMs {
      return this.children[3];
    }

    delayByMs(delayMs: number): InfoBoxTimingCollection {
      return new InfoBoxTimingCollection(super.delayByMs(delayMs).children);
    }

    scaleBy(factor: number): InfoBoxTimingCollection {
      return new InfoBoxTimingCollection(super.scaleBy(factor).children);
    }

    finalize(): InfoBoxTiming {
      return {
        borderBox: this.borderBox().asIndividual(),
        title: this.title().asIndividual(),
        infoBox: this.infoBox(),
        overallFadeIn: this.overallFadeIn(),
      };
    }
  }

  function newInfoBoxTimingCollection({
    borderBox,
    title,
    infoBox,
    overallFadeIn,
  }: {
    borderBox: BorderBoxTimingCollection;
    title: TitleTimingCollection;
    infoBox: TransitionTimingMs;
    overallFadeIn: TransitionTimingMs;
  }) {
    return new InfoBoxTimingCollection([
      borderBox,
      title,
      infoBox,
      overallFadeIn,
    ]);
  }

  export function getAnimationTiming(
    overallDelayMs: number,
    timingScaleFactor: number,
  ): InfoBoxTiming {
    const growX = new PrimitiveTimingMs({
      startMs: 0,
      durationMs: 200,
    });
    const growY = new PrimitiveTimingMs({
      startMs: growX.endMs() - 20,
      durationMs: 150,
    });
    const borderBox = newBorderBoxTimingCollection({ growX, growY });
    const typewriter = new PrimitiveTimingMs({
      // give X a head start
      startMs: growX.startMs() + 20,
      // but finish at the same time
      endMs: growX.endMs(),
    });
    const cursorFade = new PrimitiveTimingMs({
      // stay for a second after typewriter finishes
      startMs: typewriter.endMs() + 40,
      // finishes simultaneously with Y
      endMs: growY.endMs(),
    });
    const infoBox = new PrimitiveTimingMs({
      // can start fading in before border box finishes growing completely, so long as
      // border box growth is *mostly* done and already contains the entirety of the
      // info box
      delayMs: growY.startMs(),
      durationMs: 260,
    });
    const title = newTitleTimingCollection({ typewriter, cursorFade });

    const effectsGroup = new TimingGroupAsCollection([
      borderBox,
      title,
      infoBox,
    ]).delayByMs(overallDelayMs);
    const [delayedBorder, delayedTitle, delayedInfo] = effectsGroup.children;

    const overallFadeIn = new PrimitiveTimingMs({
      startMs: Math.max(0, effectsGroup.startMs() - 50),
      endMs: effectsGroup.startMs(),
    });

    const infoBoxTimingCollection = newInfoBoxTimingCollection({
      borderBox: delayedBorder as BorderBoxTimingCollection,
      title: delayedTitle as TitleTimingCollection,
      infoBox: delayedInfo,
      overallFadeIn,
    });
    return infoBoxTimingCollection.scaleBy(timingScaleFactor).finalize();
  }
</script>

<script lang="ts">
  import getComponentId from "./label-id";
  import RoundDef from "./RoundDef.svelte";
  import { cubicInOut, cubicOut, linear } from "svelte/easing";
  import { animationSpeed, animationsOn } from "./preferences";
  import { fade, type TransitionConfig } from "svelte/transition";
  import { firstAppLoad, firstPageLoad } from "./firstPageLoad";
  import { SubAnimation, PropertyAnimation } from "$lib/animation-timing";

  export let title = "";
  export let childNumber = 0;
  export let preDelay = $firstAppLoad ? 0 : 100;
  export let maxWidth: string | undefined = undefined;
  export let fullHeight = false;
  let maxWidthStyle = maxWidth === undefined ? "" : `max-width: ${maxWidth};`;
  const infoboxId = getComponentId("infobox");
  let titleElement: HTMLElement | undefined;
  const perChildStagger = 100;
  const totalDelay = preDelay + childNumber * perChildStagger;

  function revealOutline(
    node: Element,
    timing: BorderBoxTiming,
  ): TransitionConfig {
    const parentNode = node.parentNode as Element;
    const actualWidth = parentNode.clientWidth;
    const actualHeight = parentNode.clientHeight;
    const heightPerTitleLinePx = 26;
    const titleHeight = (titleElement as HTMLElement).clientHeight;
    // multiply by 1.3 to account for small pixel differences between browsers
    const titleIsMultiline = titleHeight > heightPerTitleLinePx * 1.3;
    const minHeight = titleHeight + heightPerTitleLinePx; // add a little for padding
    const minWidth = 3.5 * heightPerTitleLinePx;

    const growWidth = new PropertyAnimation({
      timing: timing.growX(),
      property: "width",
      min: minWidth,
      max: actualWidth,
      unit: "px",
      easingFunction: titleIsMultiline ? cubicOut : linear,
    });

    const growHeight = new PropertyAnimation({
      timing: timing.growY(),
      property: "height",
      min: minHeight,
      max: actualHeight,
      unit: "px",
      easingFunction: cubicInOut,
    });

    return {
      delay: timing.overall.delayMs(),
      duration: timing.overall.durationMs(),
      tick: (tGlobalFraction: number) => {
        growWidth.max = parentNode.clientWidth;
        growHeight.max = parentNode.clientHeight;
        const width = growWidth.tickForGlobalTime(tGlobalFraction);
        const height = growHeight.tickForGlobalTime(tGlobalFraction);
        node.setAttribute("style", width + height);

        if (tGlobalFraction === 1) {
          node.removeAttribute("style");
        }
      },
    };
  }

  class TypewriterEffect extends SubAnimation<void> {
    constructor(anim: { node: Element; timing: TransitionTimingFraction }) {
      const text = anim.node.textContent ?? "";
      const length = text.length + 1;
      super({
        timing: anim.timing,
        tick: (tLocalFraction: number) => {
          const i = Math.trunc(length * tLocalFraction);
          anim.node.textContent = i === 0 ? "" : text.slice(0, i - 1);
        },
      });
    }
  }

  class FadeCursorEffect extends SubAnimation<void> {
    constructor(anim: { node: Element; timing: TransitionTimingFraction }) {
      const easingFunction = cubicOut;
      super({
        timing: anim.timing,
        tick: (tLocalFraction: number) => {
          const opacity = 1 - easingFunction(tLocalFraction);
          anim.node.setAttribute("style", `--cursor-opacity: ${opacity};`);
        },
      });
    }
  }

  function revealTitle(node: Element, timing: TitleTiming): TransitionConfig {
    const typewriter = new TypewriterEffect({
      node,
      timing: timing.typewriter(),
    });
    const cursorFade = new FadeCursorEffect({
      node,
      timing: timing.cursorFade(),
    });

    return {
      delay: timing.overall.delayMs(),
      duration: timing.overall.durationMs(),
      tick: (tGlobalFraction: number) => {
        if (timing.durationMs() === 0) {
          return;
        }
        typewriter.tickForGlobalTime(tGlobalFraction);
        cursorFade.tickForGlobalTime(tGlobalFraction);
      },
    };
  }

  class RevealContent extends SubAnimation<void> {
    constructor(anim: { node: HTMLElement; timing: TransitionTimingFraction }) {
      const easingFunction = linear;
      super({
        timing: anim.timing,
        tick: (tLocalFraction: number) => {
          const opacity = easingFunction(tLocalFraction);
          anim.node.style.opacity = opacity;

          if (tLocalFraction === 0) {
            anim.node.classList.add("wait-for-infobox");
          } else if (tLocalFraction >= 0.9) {
            anim.node.classList.remove("wait-for-infobox");
          }
        },
      });
    }
  }

  function revealInfoBox(node: Element, timing: InfoBoxTiming) {
    // the items near the bottom can be revealed early instead of waiting for the
    // border box to completely finish growing. This is because the cubic in-out growth
    // feels very slow towards the end, and to wait for this to finish before starting
    // the fade-in makes the fade-in of the last item in particular feel
    // disproportionately slow. Therefore, we cap the "effective" bottom of the node
    // at 70% of the parent's actual height.
    const earlyRevealFraction = 0.3;
    const revealCutoffFraction = 1 - earlyRevealFraction;
    // how much time we have in total to kick off animations:
    // 1. This should actually take the same amount of time as Y takes to grow,
    //    except that it's slightly delayed to give Y growth a headstart
    // 2. This should leave enough time for the last element to transition
    const totalKickoffMs = timing.borderBox.asCollection().growY().durationMs();
    const theoreticalTotalKickoffFraction =
      totalKickoffMs / timing.infoBox.durationMs();
    if (theoreticalTotalKickoffFraction > 1) {
      throw new Error("Info box animation is too short to reveal all elements");
    }
    const actualTotalKickoffFraction =
      theoreticalTotalKickoffFraction * revealCutoffFraction;
    const perElementRevealFraction = 1 - actualTotalKickoffFraction;

    const getChildKickoffFraction = (child: Element, border: DOMRect) => {
      const childRect = child.getBoundingClientRect();
      const childBottomYRelativeToInfoBox =
        childRect.top + childRect.height - border.top;
      const equivalentYProgress = inverseCubicInOut(
        childBottomYRelativeToInfoBox / border.height,
      );
      const adjustedYProgress = Math.min(
        revealCutoffFraction,
        equivalentYProgress,
      );
      const delayFraction = adjustedYProgress * theoreticalTotalKickoffFraction;
      return new PrimitiveTimingFraction({
        delayFraction,
        durationFraction: perElementRevealFraction,
      });
    };

    const getNodeAnimations = (
      currentNode: Element,
      root?: DOMRect,
    ): RevealContent[] => {
      if (root === undefined) {
        root = currentNode.getBoundingClientRect();
      }

      // if there are text-only elements that are not part of any node, we fade-in the
      // whole parent at once to avoid the text appearing before anything else -- e.g.
      // if there's something like "some text in <em>some tag</em>", the "some text in"
      // will appear immediately while "some tag" takes a moment to fade in
      const isAtomicNode =
        currentNode.classList.contains("atomic-reveal") ||
        (!currentNode.classList.contains("composite-reveal") &&
          (currentNode.children.length === 0 ||
            currentNode.children.length === currentNode.childNodes.length));
      if (isAtomicNode) {
        return [
          new RevealContent({
            node: currentNode as HTMLElement,
            timing: getChildKickoffFraction(currentNode, root),
          }),
        ];
      } else {
        const revealAnimations: RevealContent[] = [];
        for (const child of currentNode.children) {
          revealAnimations.push(...getNodeAnimations(child, root));
        }
        return revealAnimations;
      }
    };

    let revealAnimations = getNodeAnimations(node);

    const config = { childList: true, subtree: true };
    const mutationCallback: MutationCallback = () => {
      revealAnimations = getNodeAnimations(node);
      // hide all new nodes immediately
      revealAnimations.forEach((anim) => {
        anim.tickForGlobalTime(0);
      });
    };
    const observer = new MutationObserver(mutationCallback);
    observer.observe(node, config);

    return {
      delay: timing.infoBox.delayMs(),
      duration: timing.infoBox.durationMs(),
      tick: (tGlobalFraction: number) => {
        if (timing.infoBox.durationMs() === 0) {
          return;
        }

        revealAnimations.forEach((anim) => {
          anim.tickForGlobalTime(tGlobalFraction);
        });

        if (tGlobalFraction === 1) {
          observer.disconnect();
        }
      },
    };
  }

  $: shouldAnimate = $animationsOn && $firstPageLoad;
  $: timingScaleFactor = shouldAnimate ? 1 / $animationSpeed : 0;
  $: timing = getAnimationTiming(totalDelay, timingScaleFactor);
  $: overallFadeInArgs = {
    delay: timing.overallFadeIn.delayMs(),
    duration: timing.overallFadeIn.durationMs(),
  };
</script>

<section
  class="container"
  class:full-height={fullHeight}
  aria-labelledby={infoboxId}
  style={maxWidthStyle}
  in:fade|global={overallFadeInArgs}
>
  <RoundDef />

  <div class="border-container">
    <div class="border-box" in:revealOutline|global={timing.borderBox}></div>
    <div class="info-box">
      <h2
        in:revealTitle|global={timing.title}
        id={infoboxId}
        bind:this={titleElement}
      >
        {title}
      </h2>
      <div
        class="info-content composite-reveal"
        in:revealInfoBox|global={timing}
      >
        <slot />
      </div>
    </div>
  </div>
</section>

<style>
  .container {
    --cut: 1rem;
    position: relative;
    flex: 1;
    padding: 0;
  }

  .container.full-height,
  .container.full-height .border-container,
  .container.full-height .info-box {
    height: 100%;
    box-sizing: border-box;
  }

  .container.full-height .info-box {
    display: flex;
    flex-direction: column;
  }

  .container.full-height .info-content {
    flex: 1;
  }

  .border-box {
    width: 100%;
    height: 100%;
    position: absolute;
    filter: url(#round) drop-shadow(0px 1px 4px rgba(26, 58, 58, 0.4));
    z-index: 1;
  }

  .border-box::before {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: var(--color-foreground);
    -webkit-mask:
      linear-gradient(-45deg, transparent 0 var(--cut), #fff 0) bottom right,
      linear-gradient(135deg, transparent 0 var(--cut), #fff 0) top left;
    -webkit-mask-size: 51% 100%;
    -webkit-mask-repeat: no-repeat;
    mask:
      linear-gradient(-45deg, transparent 0 var(--cut), #fff 0) bottom right,
      linear-gradient(135deg, transparent 0 var(--cut), #fff 0) top left;
    mask-size: 51% 100%;
    mask-repeat: no-repeat;
  }

  .info-box {
    position: relative;
    z-index: 2;
    padding: 1rem;
    text-align: justify;
  }

  .info-box h2 {
    --cursor-opacity: 0;
    margin: -0.25rem 0 0.5rem var(--cut);
    text-align: left;
  }

  .info-box :global(p:last-child) {
    margin-bottom: 0;
  }

  .info-box h2::after {
    content: "█";
    opacity: var(--cursor-opacity);
  }
</style>
