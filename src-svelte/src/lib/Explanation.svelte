<script lang="ts">
  export let text: string;
</script>

<sup>
  <span class="link-like" title={text}>[?]</span>
</sup>

<style>
  .link-like {
    cursor: help;
  }
</style>
