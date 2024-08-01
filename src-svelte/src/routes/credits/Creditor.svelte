<script lang="ts" context="module">
  const MAX_URL_LENGTH = 15;

  function formatUrlHelper(urlString: string) {
    const url = new URL(urlString);
    if (url.hostname.endsWith("github.com")) {
      if (url.pathname.endsWith("/")) {
        return url.pathname.slice(1, -1);
      }
      return url.pathname.slice(1);
    }

    if (url.hostname.endsWith("typodermicfonts.com")) {
      return url.pathname.slice(1, -1);
    }

    const hostname = url.hostname.startsWith("www.")
      ? url.hostname.slice(4)
      : url.hostname;

    let pathname: string;
    if (url.pathname === "/") {
      pathname = "";
    } else if (url.pathname.endsWith("/")) {
      pathname = url.pathname.slice(0, -1);
    } else {
      pathname = url.pathname;
    }
    return hostname + pathname;
  }

  export function formatUrl(urlString: string) {
    const formattedUrl = formatUrlHelper(urlString);
    if (formattedUrl.length > MAX_URL_LENGTH) {
      return formattedUrl.slice(0, MAX_URL_LENGTH - 3) + "...";
    }
    return formattedUrl;
  }
</script>

<script lang="ts">
  import IconPackage from "~icons/vaadin/package";
  import IconPerson from "~icons/ion/person";
  import GitHubIcon from "./GitHubIcon.svelte";
  import TypodermicIcon from "./TypodermicIcon.svelte";

  export let isPerson = false;
  export let logo: string | undefined = undefined;
  export let name: string;
  export let url: string;
  export let urlDisplay = formatUrl(url);

  const isGitHubLink = url.startsWith("https://github.com");
  const isTypodermicLink = url.startsWith("https://typodermicfonts.com");
  const logoLink = logo ? `/logos/${logo}` : undefined;
</script>

<div class="creditor atomic-reveal">
  {#if logo}
    <img class:person={isPerson} src={logoLink} alt={name} />
  {:else if isPerson}
    <div class="logo-placeholder">
      <IconPerson />
    </div>
  {:else}
    <div class="logo-placeholder">
      <IconPackage />
    </div>
  {/if}
  <div class="details">
    <h4>{name}</h4>
    <div class="external-link">
      {#if isGitHubLink}
        <GitHubIcon />
      {:else if isTypodermicLink}
        <TypodermicIcon />
      {/if}
      <a href={url} target="_blank" rel="noopener noreferrer">
        {urlDisplay}
      </a>
    </div>
  </div>
</div>

<style>
  .creditor {
    padding: 0.5rem 0 0.5rem 1.5rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: 14rem;
  }

  @media (min-width: 46rem),
    only screen and (-webkit-min-device-pixel-ratio: 2) and (min-width: 34.5rem) {
    .creditor {
      padding: 0.75rem;
    }
  }

  img {
    width: 2rem;
    border-radius: var(--corner-roundness);
  }

  img.person {
    width: 2.5rem;
  }

  .logo-placeholder,
  .logo-placeholder :global(svg) {
    width: 2rem;
    height: 2rem;
    color: var(--color-faded);
  }

  h4 {
    font-weight: normal;
    margin: 0;
  }

  .external-link {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
</style>
