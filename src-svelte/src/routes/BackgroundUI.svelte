<script lang="ts">
  import { onMount } from "svelte";

  const CHAR_EM = 20;
  const CHAR_GAP = 5;
  const BLOCK_SIZE = CHAR_EM + CHAR_GAP;
  const ANIMATE_INTERVAL_MS = 50;
  const CHAR_INTERVAL_MS = 100;
  const ANIMATES_PER_CHAR = Math.round(CHAR_INTERVAL_MS / ANIMATE_INTERVAL_MS);
  const DDJ = [
    "道可道非常道",
    "名可名非常名",
    "無名天地之始",
    "有名萬物之母",
    "故常無欲以觀其妙",
    "常有欲以觀其徼",
    "此兩者同出而異名",
    "同謂之玄",
    "玄之又玄",
    "眾妙之門",
  ];
  export let animated = false;
  let animationState: string;
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
    if (animateInterval) {
      console.warn("Animation already running");
      return;
    }

    animateInterval = setInterval(draw, ANIMATE_INTERVAL_MS);
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

    dropsPosition = Array(numColumns)
      .fill(0)
      .map(() => Math.ceil(Math.random() * -numRows));
    dropsAnimateCounter = Array(numColumns).fill(0);

    startAnimating();
  }

  function draw() {
    if (!ctx || !canvas || numColumns === 0) {
      console.warn("Canvas not ready for drawing");
      return;
    }

    ctx.fillStyle = "#FAF9F633";
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    ctx.fillStyle = "#D5CDB4C0";
    ctx.font = CHAR_EM + "px sans-serif";

    for (var column = 0; column < dropsPosition.length; column++) {
      const textLine = DDJ[column % DDJ.length];
      const textCharacter = textLine[dropsPosition[column] % textLine.length];
      ctx.fillText(
        textCharacter,
        CHAR_GAP + column * BLOCK_SIZE,
        dropsPosition[column] * BLOCK_SIZE,
      );

      if (dropsPosition[column] > numRows) {
        dropsPosition[column] = Math.ceil(Math.random() * -numRows);
      }

      if (dropsAnimateCounter[column]++ % ANIMATES_PER_CHAR === 0) {
        dropsPosition[column]++;
      }
    }
  }

  onMount(() => {
    resizeCanvas();
    window.addEventListener("resize", resizeCanvas);

    return () => {
      stopAnimating();
      window.removeEventListener("resize", resizeCanvas);
    };
  });

  $: animationState = animated ? "running" : "paused";
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
  }
</style>
