import CreditsComponent from "./Credits.svelte";
import type { StoryObj } from "@storybook/svelte";
import { MockPageTransitionsDecorator } from "$lib/__mocks__/decorators";

export default {
  component: CreditsComponent,
  title: "Screens/Credits",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: CreditsComponent,
  props: args,
});

export const Tablet: StoryObj = Template.bind({}) as any;
Tablet.parameters = {
  viewport: {
    defaultViewport: "tablet",
  },
};

export const FullPage: StoryObj = Template.bind({}) as any;
FullPage.decorators = [MockPageTransitionsDecorator];
