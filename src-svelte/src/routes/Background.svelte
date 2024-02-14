<script lang="ts">
  import BackgroundUI from "./BackgroundUI.svelte";
  import { backgroundAnimation } from "$lib/preferences";
</script>

<BackgroundUI bind:animated={$backgroundAnimation} />
