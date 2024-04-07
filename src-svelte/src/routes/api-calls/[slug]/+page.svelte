<script>
  import { page } from "$app/stores";
  import ApiCall from "./ApiCall.svelte";
</script>

<ApiCall id={$page.params.slug} />
