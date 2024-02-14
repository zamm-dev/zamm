<script lang="ts">
  import MockAppLayout from "$lib/__mocks__/MockAppLayout.svelte";
</script>

<MockAppLayout>
  <div class="background-container">
    <slot />
  </div>
</MockAppLayout>

<style>
  .background-container {
    position: fixed;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
  }
</style>
