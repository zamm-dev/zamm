<script lang="ts">
  import InfoBox from "./InfoBox.svelte";
  import { firstPageLoad } from "$lib/firstPageLoad";
  import { animationSpeed } from "$lib/preferences";

  firstPageLoad.set(true);
</script>

<div
  class="screenshot-container"
  style="--base-animation-speed: {$animationSpeed};"
>
  <InfoBox {...$$restProps}>
    <p class="atomic-reveal">
      How do we know that even the realest of realities wouldn't be subjective,
      in the final analysis? Nobody can prove his existence, can he? &mdash; <em
        >Simulacron 3</em
      >
    </p>
  </InfoBox>
</div>

<style>
  .screenshot-container {
    padding: 1rem;
    margin: -1rem;
  }
</style>
