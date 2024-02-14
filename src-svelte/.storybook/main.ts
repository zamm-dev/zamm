import { join, dirname } from "path";
import { mergeConfig, Plugin, InlineConfig } from "vite";
import path from "path";
import { StorybookConfig } from "@storybook/sveltekit";

/**
 * This function is used to resolve the absolute path of a package.
 * It is needed in projects that use Yarn PnP or are set up within a monorepo.
 */
function getAbsolutePath(value: string) {
  return dirname(require.resolve(join(value, "package.json")));
}

// https://github.com/storybookjs/storybook/issues/20562
const unpluginIconsWorkaround = (config: InlineConfig) => {
  if (!config.plugins) return config;

  const [_, ...userPlugins] = config.plugins as Plugin[];
  const docgenPlugin = userPlugins.find(
    (plugin) => plugin.name === "storybook:svelte-docgen-plugin",
  );
  if (docgenPlugin) {
    const origTransform = docgenPlugin.transform;
    const newTransform: typeof origTransform = (code, id, options) => {
      if (id.startsWith("~icons/")) {
        return;
      }
      // eslint-disable-next-line @typescript-eslint/ban-types
      return (origTransform as Function)?.call(docgenPlugin, code, id, options);
    };
    docgenPlugin.transform = newTransform;
    docgenPlugin.enforce = "post";
  }
  return config;
};

const config: StorybookConfig = {
  stories: ["../src/**/*.stories.@(js|jsx|mjs|ts|tsx)"],
  addons: [
    getAbsolutePath("@storybook/addon-links"),
    getAbsolutePath("@storybook/addon-essentials"),
    getAbsolutePath("@storybook/addon-interactions"),
    getAbsolutePath("@storybook/addon-mdx-gfm"),
    getAbsolutePath("@storybook/addon-viewport"),
  ],
  framework: {
    name: getAbsolutePath("@storybook/sveltekit"),
    options: {},
  },
  core: {
    disableWhatsNewNotifications: true,
  },
  staticDirs: ["../static", "../../src-tauri"],
  docs: {
    autodocs: "tag",
  },
  async viteFinal(config: InlineConfig) {
    const workaroundConfig = unpluginIconsWorkaround(config);
    return mergeConfig(workaroundConfig, {
      resolve: {
        alias: { $lib: path.resolve(__dirname, "../src/lib") },
      },
    });
  },
};
export default config;
