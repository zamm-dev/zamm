import SidebarUI from "./SidebarUI.svelte";
import type { StoryFn, StoryObj } from "@storybook/svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";
import MockAppLayout from "$lib/__mocks__/MockAppLayout.svelte";

export default {
  component: SidebarUI,
  title: "Layout/Sidebar",
  argTypes: {},
  parameters: {
    backgrounds: {
      default: "ZAMM background",
      values: [{ name: "ZAMM background", value: "#f4f4f4" }],
    },
  },
  decorators: [
    SvelteStoresDecorator,
    (story: StoryFn) => {
      return {
        Component: MockAppLayout,
        slot: story,
      };
    },
  ],
};

const Template = ({ ...args }) => ({
  Component: SidebarUI,
  props: args,
});

export const DashboardSelected: StoryObj = Template.bind({}) as any;
DashboardSelected.args = {
  currentRoute: "/",
  dummyLinks: true,
};

export const SettingsSelected: StoryObj = Template.bind({}) as any;
SettingsSelected.args = {
  currentRoute: "/settings",
  dummyLinks: true,
};

export const CreditsSelected: StoryObj = Template.bind({}) as any;
CreditsSelected.args = {
  currentRoute: "/credits",
  dummyLinks: true,
};

export const SlowMotion: StoryObj = Template.bind({}) as any;
SlowMotion.args = {
  currentRoute: "/",
  dummyLinks: true,
};
SlowMotion.parameters = {
  preferences: {
    animationSpeed: 0.1,
  },
};
