<script>
  import { page } from "$app/stores";
  import ApiCall from "./UnloadedApiCall.svelte";
</script>

<ApiCall id={$page.params.slug} />
