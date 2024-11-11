<script lang="ts">
  import Highlight from "svelte-highlight";
  import bash from "svelte-highlight/languages/bash";
  import html from "svelte-highlight/languages/xml";
  import css from "svelte-highlight/languages/css";
  import javascript from "svelte-highlight/languages/javascript";
  import typescript from "svelte-highlight/languages/typescript";
  import rust from "svelte-highlight/languages/rust";
  import python from "svelte-highlight/languages/python";
  import plaintext from "svelte-highlight/languages/plaintext";
  import "svelte-highlight/styles/github.css";

  interface Props {
    text: string;
    lang: string;
  }

  let { text, lang }: Props = $props();

  function getLanguageStr() {
    if (lang) {
      return lang.split(" ")[0].toLowerCase();
    }
    return "plaintext";
  }

  function getLanguage() {
    let languageStr = getLanguageStr();
    switch (languageStr) {
      case "sh":
      case "bash":
        return bash;
      case "js":
      case "javascript":
        return javascript;
      case "ts":
      case "typescript":
        return typescript;
      case "html":
        return html;
      case "css":
        return css;
      case "rust":
        return rust;
      case "py":
      case "python":
        return python;
      default:
        return plaintext;
    }
  }

  let language = getLanguage();
</script>

<div class="code">
  <Highlight {language} code={text} />
</div>

<style>
  .code {
    overflow-x: auto;
    box-sizing: border-box;
    border-radius: var(--corner-roundness);
    background-color: #ffffff88;
  }

  .code :global(code) {
    padding: var(--internal-spacing) 1rem;
    background-color: transparent;
    font-family: var(--font-mono);
    font-size: 0.8rem;
  }

  .code,
  .code :global(pre),
  .code :global(code) {
    width: fit-content;
  }
</style>
