import { styleKhmer } from "./MessageUI.svelte";

describe("Khmer styling", () => {
  it("should leave text without Khmer unchanged", () => {
    expect(styleKhmer("Hello, how are you?")).toEqual("Hello, how are you?");
  });

  it("should select the entire text if it is all Khmer", () => {
    expect(styleKhmer("ខ្ញុំសុខសប្បាយ")).toEqual(
      '<span class="khmer">ខ្ញុំសុខសប្បាយ</span>',
    );
  });

  it("should pick out Khmer text from within other text", () => {
    expect(
      styleKhmer("Hello, សួស្ដី, what languages do you speak? ចេះខ្មែរអត់?"),
    ).toEqual(
      'Hello, <span class="khmer">សួស្ដី</span>, what languages do you speak?' +
        ' <span class="khmer">ចេះខ្មែរអត់</span>?',
    );
  });
});
