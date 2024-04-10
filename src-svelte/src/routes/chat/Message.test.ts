import Message from "./Message.svelte";
import "@testing-library/jest-dom";
import { render, screen } from "@testing-library/svelte";
import { expect } from "vitest";

describe("Text messages", () => {
  it("should render without Khmer spans when no Khmer is present", () => {
    render(Message, {
      message: {
        role: "Human",
        text: "Hello, how are you?",
      },
    });
    expect(screen.getByRole("listitem")).toHaveTextContent(
      "Hello, how are you?",
    );
  });

  it("should pick out Khmer text embedded in other text", () => {
    const { container } = render(Message, {
      message: {
        role: "Human",
        text: "Hello, សួស្ដី, what languages do you speak? ចេះខ្មែរអត់?",
      },
    });
    expect(screen.getByRole("listitem")).toHaveTextContent(
      "Hello, សួស្ដី, what languages do you speak? ចេះខ្មែរអត់?",
    );

    const khmerSpans = container.getElementsByClassName("khmer");
    const khmerSpanText = Array.from(khmerSpans).map(
      (span) => span.textContent,
    );
    expect(khmerSpanText).toEqual(["សួស្ដី", "ចេះខ្មែរអត់"]);
  });
});
