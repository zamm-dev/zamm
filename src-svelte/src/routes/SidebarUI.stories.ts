import SidebarUI from "./SidebarUI.svelte";
import type { StoryObj } from "@storybook/svelte";
import SvelteStoresDecorator from "$lib/__mocks__/stores";
import {
  MockAppLayoutDecorator,
  MockPageTransitionsDecorator,
} from "$lib/__mocks__/decorators";

export default {
  component: SidebarUI,
  title: "Layout/Sidebar",
  argTypes: {},
  parameters: {
    fullHeight: true,
    backgrounds: {
      default: "ZAMM background",
      values: [{ name: "ZAMM background", value: "#f4f4f4" }],
    },
  },
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
DashboardSelected.decorators = [SvelteStoresDecorator, MockAppLayoutDecorator];

export const SettingsSelected: StoryObj = Template.bind({}) as any;
SettingsSelected.args = {
  currentRoute: "/settings",
  dummyLinks: true,
};
SettingsSelected.decorators = [SvelteStoresDecorator, MockAppLayoutDecorator];

export const CreditsSelected: StoryObj = Template.bind({}) as any;
CreditsSelected.args = {
  currentRoute: "/credits",
  dummyLinks: true,
};
CreditsSelected.decorators = [SvelteStoresDecorator, MockAppLayoutDecorator];

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
SlowMotion.decorators = [SvelteStoresDecorator, MockPageTransitionsDecorator];
