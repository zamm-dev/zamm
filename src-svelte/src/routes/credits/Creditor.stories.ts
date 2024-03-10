import CreditorComponent from "./Creditor.svelte";
import type { StoryObj } from "@storybook/svelte";

export default {
  component: CreditorComponent,
  title: "Screens/Credits/Creditor",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: CreditorComponent,
  props: args,
});

export const Regular: StoryObj = Template.bind({}) as any;
Regular.args = {
  name: "Serde",
  url: "https://serde.rs/",
};

export const GithubContributor: StoryObj = Template.bind({}) as any;
GithubContributor.args = {
  name: "Amos Jun-yeung Ng",
  url: "https://github.com/amosjyng/",
};

export const TypodermicFont: StoryObj = Template.bind({}) as any;
TypodermicFont.args = {
  name: "Nasalization",
  url: "https://typodermicfonts.com/nasalization/",
};

export const DependencyWithIcon: StoryObj = Template.bind({}) as any;
DependencyWithIcon.args = {
  name: "Tauri",
  logo: "tauri.png",
  url: "https://tauri.app/",
};
