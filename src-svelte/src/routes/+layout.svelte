<script lang="ts">
  import AppLayout from "./AppLayout.svelte";
  import "./styles.css";

  let { data, children } = $props();
</script>

<AppLayout currentRoute={data.url}>
  {@render children?.()}
</AppLayout>
