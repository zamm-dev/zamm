import SettingsComponent from "./Settings.svelte";
import type { StoryObj } from "@storybook/svelte";
import { MockPageTransitionsDecorator } from "$lib/__mocks__/decorators";

export default {
  component: SettingsComponent,
  title: "Screens/Settings",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: SettingsComponent,
  props: args,
});

export const TinyPhoneScreen: StoryObj = Template.bind({}) as any;
TinyPhoneScreen.parameters = {
  viewport: {
    defaultViewport: "tallerSmallMobile",
  },
};

export const LargePhoneScreen: StoryObj = Template.bind({}) as any;
LargePhoneScreen.parameters = {
  viewport: {
    defaultViewport: "mobile2",
  },
};

export const Tablet: StoryObj = Template.bind({}) as any;
Tablet.parameters = {
  viewport: {
    defaultViewport: "tablet",
  },
};

export const FullPage: StoryObj = Template.bind({}) as any;
FullPage.decorators = [MockPageTransitionsDecorator];
