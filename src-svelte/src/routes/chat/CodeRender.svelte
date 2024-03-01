<script lang="ts">
  import Highlight from "svelte-highlight";
  import bash from "svelte-highlight/languages/bash";
  import javascript from "svelte-highlight/languages/javascript";
  import typescript from "svelte-highlight/languages/typescript";
  import rust from "svelte-highlight/languages/rust";
  import python from "svelte-highlight/languages/python";
  import plaintext from "svelte-highlight/languages/plaintext";
  import "svelte-highlight/styles/github.css";

  export let text: string;
  export let lang: string;

  function getLanguageStr() {
    if (lang) {
      return lang.split(" ")[0];
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
      case "typescript":
        return typescript;
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
  .code :global(code) {
    border-radius: var(--corner-roundness);
    background-color: #ffffff88;
  }
</style>
