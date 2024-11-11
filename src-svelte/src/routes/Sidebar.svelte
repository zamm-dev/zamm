<script lang="ts">
  import { page } from "$app/stores";
  import SidebarUi from "./SidebarUI.svelte";

  let currentRoute: string = $derived($page?.url?.pathname || "/");
</script>

<SidebarUi {currentRoute} />
