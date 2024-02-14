import { defineConfig } from "vitest/config";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import Icons from "unplugin-icons/vite";
import * as path from "path";

export default defineConfig({
  plugins: [
    svelte({ hot: !process.env.VITEST }),
    Icons({
      compiler: "svelte",
    }),
  ],
  resolve: {
    alias: {
      $lib: path.resolve("src/lib"),
      $app: path.resolve("src/vitest-mocks"),
    },
  },
  test: {
    include: ["src/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}"],
    globals: true,
    environment: "jsdom",
    alias: [{ find: /^svelte$/, replacement: "svelte/internal" }],
  },
});
