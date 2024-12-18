<script lang="ts">
  import MockPageTransitions from "./MockPageTransitions.svelte";

  let props = $props();
</script>

<MockPageTransitions customStoreValues {...props} />
