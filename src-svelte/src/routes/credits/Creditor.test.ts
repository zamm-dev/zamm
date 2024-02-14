import { expect, test } from "vitest";
import { formatUrl } from "./Creditor.svelte";

describe("URL formatter", () => {
  test("formats HTTP(S) URL correctly", () => {
    expect(formatUrl("http://yahoo.com")).toEqual("yahoo.com");
    expect(formatUrl("https://google.com")).toEqual("google.com");
  });

  test("formats Github username URLs correctly", () => {
    expect(formatUrl("https://github.com/amosjyng/")).toEqual("amosjyng");
  });

  test("formats Github project URLs correctly", () => {
    expect(formatUrl("https://github.com/ai/nanoid")).toEqual("ai/nanoid");
  });

  test("strips ending slash from URL", () => {
    expect(formatUrl("https://tauri.app/")).toEqual("tauri.app");
  });

  test("strips www from URL", () => {
    expect(formatUrl("https://www.neodrag.dev/")).toEqual("neodrag.dev");
  });

  test("preserves path", () => {
    expect(formatUrl("https://www.a.com/b/c")).toEqual("a.com/b/c");
  });

  test("truncates long URLs", () => {
    expect(formatUrl("https://www.jacklmoore.com/autosize/")).toEqual(
      "jacklmoore.c...",
    );
  });
});
