import type {
  DecoratorFunction,
  PartialStoryFn,
} from "storybook/internal/types";
import MockTransitions from "./MockTransitions.svelte";
import MockPageTransitions from "./MockPageTransitions.svelte";
import type { SvelteRenderer } from "@storybook/svelte";

const mockPageTransitionFn = (story: PartialStoryFn) => {
  return {
    Component: MockPageTransitions,
    slot: story,
  };
};
export const MockPageTransitionsDecorator: DecoratorFunction<SvelteRenderer> =
  mockPageTransitionFn as unknown as DecoratorFunction<SvelteRenderer>;

const mockTransitionFn = (story: PartialStoryFn) => {
  return {
    Component: MockTransitions,
    slot: story,
  };
};
export const MockTransitionsDecorator: DecoratorFunction<SvelteRenderer> =
  mockTransitionFn as unknown as DecoratorFunction<SvelteRenderer>;
