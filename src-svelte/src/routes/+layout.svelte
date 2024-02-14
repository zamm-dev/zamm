<script>
  import AppLayout from "./AppLayout.svelte";
  import "./styles.css";

  export let data;
</script>

<AppLayout currentRoute={data.url}>
  <slot />
</AppLayout>
