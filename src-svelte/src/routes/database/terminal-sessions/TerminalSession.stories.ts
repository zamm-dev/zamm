import TerminalSession from "./TerminalSession.svelte";
import type { StoryFn, StoryObj } from "@storybook/svelte";
import TauriInvokeDecorator from "$lib/__mocks__/invoke";
import SvelteStoresDecorator from "$lib/__mocks__/stores";
import MockFullPageLayout from "$lib/__mocks__/MockFullPageLayout.svelte";

export default {
  component: TerminalSession,
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
  Component: TerminalSession,
  props: args,
});

export const New: StoryObj = Template.bind({}) as any;
New.parameters = {
  sampleCallFiles: [
    "/api/sample-calls/run_command-bash.yaml",
    "/api/sample-calls/send_command_input-bash-interleaved.yaml",
  ],
};

export const NewOnWindows: StoryObj = Template.bind({}) as any;
NewOnWindows.parameters = {
  sampleCallFiles: [
    "/api/sample-calls/run_command-cmd.yaml",
    "/api/sample-calls/send_command_input-cmd-dir.yaml",
  ],
};
