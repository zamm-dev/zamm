import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://kit.svelte.dev/docs/integrations#preprocessors
  // for more information about preprocessors
  preprocess: vitePreprocess(),

  kit: {
    adapter: adapter(),
    prerender: {
      crawl: false,
      entries: [
        "*",
        "/database/api-calls/new/",
        "/database/api-calls/[slug]",
        "/database/terminal-sessions/new/",
        "/database/terminal-sessions/[slug]",
      ],
    },
  },
};

export default config;
