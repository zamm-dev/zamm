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
  import GitHubIcon from "./GitHubIcon.svelte";

  export let logo: string | undefined = undefined;
  export let name: string;
  export let url: string;
  export let urlDisplay = formatUrl(url);

  const isGitHubLink = url.startsWith("https://github.com");
  const logoLink = logo ? `/logos/${logo}` : undefined;
</script>

<div class="creditor atomic-reveal">
  {#if logo}
    <img src={logoLink} alt={name} />
  {/if}
  <div class="details">
    <h4>{name}</h4>
    <div class="external-link">
      {#if isGitHubLink}
        <GitHubIcon />
      {/if}
      <a href={url} target="_blank" rel="noopener noreferrer">
        {urlDisplay}
      </a>
    </div>
  </div>
</div>

<style>
  .creditor {
    padding: 0.5rem 0;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  @media (min-width: 40rem) {
    .creditor {
      padding: 1rem;
    }
  }

  img {
    width: 2rem;
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
