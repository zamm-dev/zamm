<script lang="ts">
  import BackgroundUI from "./BackgroundUI.svelte";
  import { animationsOn, backgroundAnimation } from "$lib/preferences";

  let animated: boolean = $state(false);
  $effect(() => {
    animated = $animationsOn && $backgroundAnimation;
  });
</script>

<BackgroundUI {animated} />
