<script lang="ts">
  import type { Snippet } from "svelte";
  import AnimationControl from "./AnimationControl.svelte";

  interface Props {
    children?: Snippet;
  }

  let { children }: Props = $props();
</script>

<AnimationControl>
  <div class="background-container">
    {@render children?.()}
  </div>
</AnimationControl>

<style>
  .background-container {
    position: fixed;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
  }
</style>
