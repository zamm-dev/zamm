import SettingsComponent from "./Settings.svelte";
import type { StoryObj } from "@storybook/svelte";
import {
  MockAppLayoutDecorator,
  MockPageTransitionsDecorator,
} from "$lib/__mocks__/decorators";

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
TinyPhoneScreen.decorators = [MockAppLayoutDecorator];

export const LargePhoneScreen: StoryObj = Template.bind({}) as any;
LargePhoneScreen.parameters = {
  viewport: {
    defaultViewport: "mobile2",
  },
};
LargePhoneScreen.decorators = [MockAppLayoutDecorator];

export const Tablet: StoryObj = Template.bind({}) as any;
Tablet.parameters = {
  viewport: {
    defaultViewport: "tablet",
  },
};
Tablet.decorators = [MockAppLayoutDecorator];

export const FullPage: StoryObj = Template.bind({}) as any;
FullPage.decorators = [MockPageTransitionsDecorator];
