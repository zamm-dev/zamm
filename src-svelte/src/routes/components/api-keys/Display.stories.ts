import ApiKeysDisplay from "./Display.svelte";
import type { StoryObj } from "@storybook/svelte";
import TauriInvokeDecorator from "$lib/__mocks__/invoke";
import SvelteStoresDecorator from "$lib/__mocks__/stores";
import {
  MockAppLayoutDecorator,
  MockPageTransitionsDecorator,
} from "$lib/__mocks__/decorators";

export default {
  component: ApiKeysDisplay,
  title: "Screens/Dashboard/API Keys Display",
  argTypes: {},
  decorators: [SvelteStoresDecorator, TauriInvokeDecorator],
};

const Template = ({ ...args }) => ({
  Component: ApiKeysDisplay,
  props: args,
});

const writeToFile = "/api/sample-calls/set_api_key-existing-no-newline.yaml";
const unknownKeys = "/api/sample-calls/get_api_keys-empty.yaml";
const knownKeys = "/api/sample-calls/get_api_keys-openai.yaml";
const unsetKey = "/api/sample-calls/set_api_key-unset.yaml";

export const Loading: StoryObj = Template.bind({}) as any;
Loading.parameters = {
  shouldWait: true,
  viewport: {
    defaultViewport: "mobile2",
  },
};
Loading.decorators = [MockAppLayoutDecorator];

export const Unknown: StoryObj = Template.bind({}) as any;
Unknown.parameters = {
  sampleCallFiles: [unknownKeys, writeToFile, knownKeys],
  stores: {
    systemInfo: {
      shell_init_file: ".bashrc",
    },
  },
  viewport: {
    defaultViewport: "mobile2",
  },
};
Unknown.decorators = [MockAppLayoutDecorator];

export const Known: StoryObj = Template.bind({}) as any;
Known.parameters = {
  sampleCallFiles: [knownKeys, writeToFile],
  viewport: {
    defaultViewport: "mobile2",
  },
};
Known.decorators = [MockAppLayoutDecorator];

export const Editing: StoryObj = Template.bind({}) as any;
Editing.args = {
  editDemo: true,
};
Editing.parameters = {
  sampleCallFiles: [unknownKeys, writeToFile],
  viewport: {
    defaultViewport: "mobile2",
  },
};
Editing.decorators = [MockAppLayoutDecorator];

export const EditingPreFilled: StoryObj = Template.bind({}) as any;
EditingPreFilled.args = {
  editDemo: true,
};
EditingPreFilled.parameters = {
  sampleCallFiles: [knownKeys, writeToFile],
  stores: {
    systemInfo: {
      shell_init_file: "/root/.profile",
    },
  },
  viewport: {
    defaultViewport: "mobile2",
  },
};
EditingPreFilled.decorators = [MockAppLayoutDecorator];

export const Unset: StoryObj = Template.bind({}) as any;
Unset.parameters = {
  sampleCallFiles: [knownKeys, unsetKey, unknownKeys],
  preferences: {
    animationSpeed: 1,
  },
  viewport: {
    defaultViewport: "mobile2",
  },
};
Unset.decorators = [MockAppLayoutDecorator];

export const Fast: StoryObj = Template.bind({}) as any;
Fast.parameters = {
  sampleCallFiles: [unknownKeys, writeToFile, knownKeys],
  stores: {
    systemInfo: {
      shell_init_file: ".bashrc",
    },
  },
  preferences: {
    animationSpeed: 1,
  },
  viewport: {
    defaultViewport: "mobile2",
  },
};
Fast.decorators = [MockPageTransitionsDecorator];

export const SlowMotion: StoryObj = Template.bind({}) as any;
SlowMotion.parameters = {
  sampleCallFiles: [unknownKeys, writeToFile, knownKeys],
  preferences: {
    animationSpeed: 0.1,
  },
  viewport: {
    defaultViewport: "mobile2",
  },
};
SlowMotion.decorators = [MockPageTransitionsDecorator];
