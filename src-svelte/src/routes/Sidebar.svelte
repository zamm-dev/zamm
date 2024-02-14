<script lang="ts">
  import { page } from "$app/stores";
  import SidebarUi from "./SidebarUI.svelte";

  let currentRoute: string;
  $: currentRoute = $page?.url?.pathname || "/";
</script>

<SidebarUi {currentRoute} />
