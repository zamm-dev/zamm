<script>
  import { page } from "$app/stores";
  import UnloadedApiCall from "./UnloadedApiCall.svelte";
</script>

<UnloadedApiCall id={$page.params.slug} />
