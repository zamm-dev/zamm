<script lang="ts">
  import MockFullPageLayout from "./MockFullPageLayout.svelte";
  import PageTransition from "../../routes/PageTransition.svelte";
  import { firstAppLoad, firstPageLoad } from "$lib/firstPageLoad";
  import { animationSpeed } from "$lib/preferences";

  firstAppLoad.set(true);
  firstPageLoad.set(true);
  animationSpeed.set(0.1);
</script>

<MockFullPageLayout>
  <PageTransition currentRoute="/storybook-demo">
    <slot />
  </PageTransition>
</MockFullPageLayout>
