import { defineConfig } from "vitest/config";
import { sveltekit } from "@sveltejs/kit/vite";
import { svelteTesting } from "@testing-library/svelte/vite";
import Icons from "unplugin-icons/vite";
import * as path from "path";

export default defineConfig({
  plugins: [
    sveltekit(),
    svelteTesting(),
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
    globalSetup: "src/testSetup.ts",
    poolOptions: {
      threads: {
        singleThread: true,
      },
    },
  },
});
