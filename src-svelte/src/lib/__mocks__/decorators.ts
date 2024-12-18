import type {
  DecoratorFunction,
  PartialStoryFn,
} from "storybook/internal/types";
import MockPageTransitions from "./MockPageTransitions.svelte";
import MockAppLayout from "./MockAppLayout.svelte";
import type { StoryContext, SvelteRenderer } from "@storybook/svelte";
import MockFullPageLayout from "./MockFullPageLayout.svelte";
import MockTransitionUsingStore from "./MockTransitionUsingStore.svelte";

const mockPageTransitionFn = (
  story: PartialStoryFn,
  { parameters }: StoryContext,
) => {
  const component =
    parameters.preferences?.animationSpeed !== undefined
      ? MockTransitionUsingStore
      : MockPageTransitions;
  return {
    Component: component,
    slot: story,
  };
};
export const MockPageTransitionsDecorator: DecoratorFunction<SvelteRenderer> =
  mockPageTransitionFn as unknown as DecoratorFunction<SvelteRenderer>;

const mockAppLayoutFn = (
  story: PartialStoryFn,
  { parameters }: StoryContext,
) => {
  const component = parameters.fullHeight ? MockFullPageLayout : MockAppLayout;
  return {
    Component: component,
    slot: story,
  };
};
export const MockAppLayoutDecorator: DecoratorFunction<SvelteRenderer> =
  mockAppLayoutFn as unknown as DecoratorFunction<SvelteRenderer>;
