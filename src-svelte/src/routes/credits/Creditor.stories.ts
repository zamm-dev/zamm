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

export const GithubContributor: StoryObj = Template.bind({}) as any;
GithubContributor.args = {
  name: "Amos Jun-yeung Ng",
  url: "https://github.com/amosjyng/",
};

export const DependencyWithIcon: StoryObj = Template.bind({}) as any;
DependencyWithIcon.args = {
  name: "Tauri",
  logo: "tauri",
  url: "https://tauri.app/",
};
