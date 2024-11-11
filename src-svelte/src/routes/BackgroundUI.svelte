<script module lang="ts">
  const CHAR_GAP = 2;
  const ANIMATES_PER_CHAR = 2;
  const STATIC_INITIAL_DRAWS = 100;
  export const DDJ = [
    "道可道非常道",
    "名可名非常名",
    "无名天地之始",
    "有名万物之母",
    "故常无欲以观其妙",
    "常有欲以观其徼",
    "此两者同出而异名",
    "同谓之玄",
    "玄之又玄",
    "众妙之门",
  ];

  export function getDdjLineNumber(column: number, numFullColumns: number) {
    return (numFullColumns - 1 - column + DDJ.length) % DDJ.length;
  }
</script>

<script lang="ts">
  import { onMount } from "svelte";
  import { derived } from "svelte/store";
  import { standardDuration, newEmStore } from "$lib/preferences";
  import prand from "pure-rand";

  interface Props {
    animated?: boolean;
  }

  let { animated = false }: Props = $props();
  const rng = prand.xoroshiro128plus(8650539321744612);
  const animateIntervalMs = derived(standardDuration, ($sd) => $sd / 2);
  $effect(() => updateAnimationState(animated));
  $effect(() => updateAnimationSpeed($animateIntervalMs));
  let charEm = newEmStore(26);
  let textFont = derived(
    charEm,
    ($charEm) => $charEm + "px 'Zhi Mang Xing', sans-serif",
  );
  let blockSize = derived(charEm, ($charEm) => $charEm + CHAR_GAP);
  let background: HTMLDivElement | null = null;
  let canvas: HTMLCanvasElement | null = null;
  let ctx: CanvasRenderingContext2D | null = null;
  let animateInterval: NodeJS.Timeout | undefined = undefined;
  let dropsPosition: number[] = [];
  let dropsAnimateCounter: number[] = [];
  let numColumns = 0;
  let numFullColumns = 0;
  let numRows = 0;

  function stopAnimating() {
    clearInterval(animateInterval);
    animateInterval = undefined;
  }

  function startAnimating() {
    if (!animated) {
      // this is possible from the animation speed change trigger
      console.warn("Animation not enabled");
      return;
    }

    if (animateInterval) {
      console.warn("Animation already running");
      return;
    }

    animateInterval = setInterval(draw, $animateIntervalMs);
  }

  function nextColumnPosition() {
    return prand.unsafeUniformIntDistribution(-numRows, 0, rng);
  }

  function resizeCanvas() {
    if (!canvas || !background) {
      return;
    }

    stopAnimating();

    canvas.width = background.clientWidth;
    canvas.height = background.clientHeight;
    // note that BLOCK_SIZE already contains CHAR_GAP
    // this is just adding CHAR_GAP-sized left padding to the animation
    numColumns = Math.ceil((canvas.width - CHAR_GAP) / $blockSize);
    numFullColumns = Math.round((canvas.width - CHAR_GAP) / $blockSize - 0.1);
    numRows = Math.ceil(canvas.height / $blockSize);

    ctx = canvas.getContext("2d");
    if (!ctx) {
      console.warn("Canvas context not available");
      return;
    }
    ctx.fillStyle = "#FAF9F6";
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    dropsPosition = Array(numColumns).fill(0).map(nextColumnPosition);
    dropsAnimateCounter = Array(numColumns).fill(0);

    if (animated) {
      startAnimating();
    } else {
      for (let i = 0; i < STATIC_INITIAL_DRAWS; i++) {
        draw();
      }
    }
  }

  function draw() {
    if (!ctx || !canvas || numColumns === 0) {
      return;
    }

    ctx.fillStyle = "#FAF9F633";
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    ctx.fillStyle = "#D5CDB4C0";
    ctx.font = $textFont;

    for (var column = 0; column < dropsPosition.length; column++) {
      const textLine = DDJ[getDdjLineNumber(column, numFullColumns)];
      const textCharacter = textLine[dropsPosition[column] % textLine.length];
      ctx.fillText(
        textCharacter,
        CHAR_GAP + column * $blockSize,
        dropsPosition[column] * $blockSize,
      );

      if (dropsPosition[column] > numRows) {
        dropsPosition[column] = nextColumnPosition();
      }

      if (dropsAnimateCounter[column]++ % ANIMATES_PER_CHAR === 0) {
        dropsPosition[column]++;
      }
    }
  }

  function updateAnimationState(nowAnimating: boolean) {
    if (nowAnimating) {
      startAnimating();
    } else {
      stopAnimating();
    }
  }

  function updateAnimationSpeed(_newSpeed: number) {
    stopAnimating();
    startAnimating();
  }

  onMount(() => {
    const fontFile = new FontFace(
      "Zhi Mang Xing",
      "url(/public-fonts/zhi-mang-xing.ttf)",
    );
    document.fonts.add(fontFile);
    let charEmUnsubscribe: () => void | undefined;

    fontFile.load().then(
      () => {
        window.addEventListener("resize", resizeCanvas);
        charEmUnsubscribe = charEm.subscribe(() => {
          setTimeout(resizeCanvas, 100);
        });
      },
      (err) => {
        console.error(err);
      },
    );

    return () => {
      stopAnimating();
      window.removeEventListener("resize", resizeCanvas);
      if (charEmUnsubscribe) {
        charEmUnsubscribe();
      }
    };
  });
</script>

<div class="background" bind:this={background}>
  <canvas bind:this={canvas}></canvas>
</div>

<style>
  .background {
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
    z-index: -100;
    overflow: hidden;
    border-radius: var(--main-corners);
  }
</style>
