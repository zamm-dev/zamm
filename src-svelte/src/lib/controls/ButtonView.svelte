<script lang="ts">
  import Button from "./Button.svelte";

  interface Props {
    text: string;
    [key: string]: any;
  }

  let { text, ...rest }: Props = $props();
</script>

<Button {...rest}>{text}</Button>
