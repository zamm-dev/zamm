<script lang="ts">
  export let animated = false;
  let animationState: string;

  $: animationState = animated ? "running" : "paused";
</script>

<div class="background" style="--animation-state: {animationState};">
  <div class="bg"></div>
  <div class="bg bg2"></div>
  <div class="bg bg3"></div>
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
    --position: 55%;
    --base-duration: calc(150 * var(--standard-duration));
    --max-left: -15%;
    --max-right: +10%;
    --animation-state: paused;
  }

  .bg {
    --duration: var(--base-duration);
    --color-overlay: #49d8d7;
    animation: slide var(--duration) ease-in-out infinite alternate;
    animation-play-state: var(--animation-state);
    background-image: linear-gradient(
      120deg,
      transparent var(--position),
      var(--color-overlay) var(--position),
      transparent calc(var(--position) + 10%)
    );
    bottom: 0;
    left: -100%;
    opacity: 0.1;
    position: absolute;
    right: -100%;
    top: 0%;
    z-index: -1;
  }

  .bg2 {
    --duration: calc(1.33 * var(--base-duration));
    --color-overlay: #4949d8;
    animation-direction: alternate-reverse;
    animation-delay: calc(-0.5 * var(--base-duration));
  }

  .bg3 {
    --duration: calc(1.66 * var(--base-duration));
    --color-overlay: #49d849;
    animation-delay: calc(-0.7 * var(--base-duration));
  }

  @keyframes slide {
    0% {
      transform: translateX(var(--max-left));
    }
    100% {
      transform: translateX(var(--max-right));
    }
  }
</style>
