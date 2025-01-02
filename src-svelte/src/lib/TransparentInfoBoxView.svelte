<script lang="ts">
  import MockAppLayout from "$lib/__mocks__/MockAppLayout.svelte";
  import InfoBoxView from "./InfoBoxView.svelte";

  let props = $props();
</script>

<MockAppLayout showBackground>
  <InfoBoxView {...props} />
</MockAppLayout>
