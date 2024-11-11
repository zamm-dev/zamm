<script lang="ts">
  interface Props {
    size?: string;
  }

  let { size = "18px" }: Props = $props();
</script>

<img src="/logos/typodermic.png" alt="Typodermic" width={size} />
