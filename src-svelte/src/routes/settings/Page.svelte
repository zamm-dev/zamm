<script lang="ts">
  import Settings from "./Settings.svelte";
  import Database from "./Database.svelte";
</script>

<div class="container">
  <Settings />
  <Database />
</div>

<style>
  .container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
</style>
