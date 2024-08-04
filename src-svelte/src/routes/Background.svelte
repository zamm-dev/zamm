<script lang="ts">
  import BackgroundUI from "./BackgroundUI.svelte";
  import { animationsOn, backgroundAnimation } from "$lib/preferences";

  $: animated = $animationsOn && $backgroundAnimation;
</script>

<BackgroundUI bind:animated />
