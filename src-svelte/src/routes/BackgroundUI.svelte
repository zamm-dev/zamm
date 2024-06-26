<script lang="ts">
  import { onMount } from "svelte";
  import { standardDuration } from "$lib/preferences";
  import prand from "pure-rand";

  const rng = prand.xoroshiro128plus(8650539321744612);
  const CHAR_EM = 26;
  const CHAR_GAP = 2;
  const TEXT_FONT = CHAR_EM + "px 'Zhi Mang Xing', sans-serif";
  const BLOCK_SIZE = CHAR_EM + CHAR_GAP;
  const ANIMATES_PER_CHAR = 2;
  const STATIC_INITIAL_DRAWS = 100;
  const DDJ = [
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
  export let animated = false;
  $: animateIntervalMs = $standardDuration / 2;
  let background: HTMLDivElement | null = null;
  let canvas: HTMLCanvasElement | null = null;
  let ctx: CanvasRenderingContext2D | null = null;
  let animateInterval: NodeJS.Timeout | undefined = undefined;
  let dropsPosition: number[] = [];
  let dropsAnimateCounter: number[] = [];
  let numColumns = 0;
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

    animateInterval = setInterval(draw, animateIntervalMs);
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
    numColumns = Math.ceil((canvas.width - CHAR_GAP) / BLOCK_SIZE);
    numRows = Math.ceil(canvas.height / BLOCK_SIZE);

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
    ctx.font = TEXT_FONT;

    for (var column = 0; column < dropsPosition.length; column++) {
      const textLine = DDJ[column % DDJ.length];
      const textCharacter = textLine[dropsPosition[column] % textLine.length];
      ctx.fillText(
        textCharacter,
        CHAR_GAP + column * BLOCK_SIZE,
        dropsPosition[column] * BLOCK_SIZE,
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

    fontFile.load().then(
      () => {
        resizeCanvas();
        window.addEventListener("resize", resizeCanvas);
      },
      (err) => {
        console.error(err);
      },
    );

    return () => {
      stopAnimating();
      window.removeEventListener("resize", resizeCanvas);
    };
  });

  $: updateAnimationState(animated);
  $: updateAnimationSpeed(animateIntervalMs);
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
