import CreditsComponent from "./Credits.svelte";
import type { StoryObj } from "@storybook/svelte";
import {
  MockAppLayoutDecorator,
  MockPageTransitionsDecorator,
} from "$lib/__mocks__/decorators";

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
  fullHeight: true,
  viewport: {
    defaultViewport: "smallTablet",
  },
};
Tablet.decorators = [MockAppLayoutDecorator];

export const FullPage: StoryObj = Template.bind({}) as any;
FullPage.decorators = [MockPageTransitionsDecorator];
