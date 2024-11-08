<script lang="ts">
  import UnloadedTerminalSession from "../UnloadedTerminalSession.svelte";
  import { page } from "$app/stores";
</script>

<UnloadedTerminalSession id={$page.params.slug} />
