import CreditsComponent from "./Credits.svelte";
import MockPageTransitions from "$lib/__mocks__/MockPageTransitions.svelte";
import type { StoryObj, StoryFn } from "@storybook/svelte";

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
FullPage.decorators = [
  (story: StoryFn) => {
    return {
      Component: MockPageTransitions,
      slot: story,
    };
  },
];
