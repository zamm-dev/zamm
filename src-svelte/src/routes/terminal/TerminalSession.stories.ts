import TerminalSession from "./TerminalSession.svelte";
import type { StoryFn, StoryObj } from "@storybook/svelte";
import TauriInvokeDecorator from "$lib/__mocks__/invoke";
import SvelteStoresDecorator from "$lib/__mocks__/stores";
import MockFullPageLayout from "$lib/__mocks__/MockFullPageLayout.svelte";

export default {
  component: TerminalSession,
  title: "Screens/Terminal/Session",
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

export const InProgress: StoryObj = Template.bind({}) as any;
InProgress.args = {
  sessionId: "3717ed48-ab52-4654-9f33-de5797af5118",
  command: "bash",
  output:
    // eslint-disable-next-line max-len
    "The default interactive shell is now zsh.\r\nTo update your account to use zsh, please run `chsh -s /bin/zsh`.\r\nFor more details, please visit https://support.apple.com/kb/HT208050.\r\nbash-3.2$ ",
};
InProgress.parameters = {
  sampleCallFiles: [
    "/api/sample-calls/send_command_input-bash-interleaved.yaml",
  ],
};
