import UnloadedTerminalSession from "./UnloadedTerminalSession.svelte";
import type { StoryFn, StoryObj } from "@storybook/svelte";
import TauriInvokeDecorator from "$lib/__mocks__/invoke";
import SvelteStoresDecorator from "$lib/__mocks__/stores";
import MockFullPageLayout from "$lib/__mocks__/MockFullPageLayout.svelte";

export default {
  component: UnloadedTerminalSession,
  title: "Screens/Database/Terminal Session",
  argTypes: {},
  decorators: [
    SvelteStoresDecorator,
    TauriInvokeDecorator,
    (story: StoryFn) => {
      return {
        Component: MockFullPageLayout,
        slot: story,
      };
    },
  ],
};

const Template = ({ ...args }) => ({
  Component: UnloadedTerminalSession,
  props: args,
});

export const InProgress: StoryObj = Template.bind({}) as any;
InProgress.args = {
  id: "3717ed48-ab52-4654-9f33-de5797af5118",
};
InProgress.parameters = {
  sampleCallFiles: [
    "/api/sample-calls/get_terminal_session-bash.yaml",
    "/api/sample-calls/send_command_input-bash-interleaved.yaml",
  ],
};

export const Finished: StoryObj = Template.bind({}) as any;
Finished.args = {
  id: "3717ed48-ab52-4654-9f33-de5797af5118",
};
Finished.parameters = {
  sampleCallFiles: [
    "/api/sample-calls/get_terminal_session-bash-interleaved.yaml",
  ],
};
