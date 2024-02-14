<script lang="ts">
  import MessageUI from "./MessageUI.svelte";
  import { animationsOn } from "$lib/preferences";

  $: animationState = $animationsOn ? "running" : "paused";
</script>

<MessageUI role="AI">
  <div class="typing-indicator" style="--animation-state: {animationState};">
    <div class="dot"></div>
    <div class="dot"></div>
    <div class="dot"></div>
  </div>
</MessageUI>

<style>
  .typing-indicator {
    --animation-state: running;
    --dot-speed: 0.5s;
    align-items: center;
    display: flex;
    justify-content: center;
    gap: 0.25rem;
    min-height: 22px;
  }

  .dot {
    --initial-animation-progress: 0;
    border-radius: 9999px;
    height: 0.5rem;
    width: 0.5rem;

    background: rgba(148 163 184 / 1);
    opacity: 1;
    animation: blink var(--dot-speed) infinite alternate;
    animation-delay: calc(
      (-2 + var(--initial-animation-progress)) * var(--dot-speed)
    );
    animation-play-state: var(--animation-state);
  }

  .dot:nth-child(1) {
    --initial-animation-progress: 0;
  }

  .dot:nth-child(2) {
    --initial-animation-progress: 0.25;
  }

  .dot:nth-child(3) {
    --initial-animation-progress: 0.5;
  }

  @keyframes blink {
    100% {
      opacity: 0;
    }
  }
</style>
