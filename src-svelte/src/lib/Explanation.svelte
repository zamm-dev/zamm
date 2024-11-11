<script lang="ts">
  interface Props {
    text: string;
  }

  let { text }: Props = $props();
</script>

<sup>
  <span class="link-like" title={text}>[?]</span>
</sup>

<style>
  .link-like {
    cursor: help;
  }
</style>
