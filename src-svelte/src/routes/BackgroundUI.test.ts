import { getDdjLineNumber, DDJ } from "./BackgroundUI.svelte";

describe("Dao De Jing positioning", () => {
  it("should put DDJ's first line on the right if there's one column", () => {
    expect(getDdjLineNumber(0, 1)).toEqual(0);
  });

  it("should put DDJ's first line on the right if there's two columns", () => {
    expect(getDdjLineNumber(0, 2)).toEqual(1);
    expect(getDdjLineNumber(1, 2)).toEqual(0);
  });

  it("should loop around if there's enough columns", () => {
    const numColumns = 25; // DDJ has 10 lines, so this will loop around twice
    // we start from the right of the screen and read line by line to the left
    expect(getDdjLineNumber(24, numColumns)).toEqual(0);
    expect(getDdjLineNumber(23, numColumns)).toEqual(1);
    // we check that it wraps around twice, each time after it finishes all 10 lines
    expect(getDdjLineNumber(15, numColumns)).toEqual(9);
    expect(getDdjLineNumber(14, numColumns)).toEqual(0);
    expect(getDdjLineNumber(5, numColumns)).toEqual(9);
    expect(getDdjLineNumber(4, numColumns)).toEqual(0);
    // the left-most columns end wherever it is in order
    expect(getDdjLineNumber(1, numColumns)).toEqual(3);
    expect(getDdjLineNumber(0, numColumns)).toEqual(4);
  });

  it("should always return last line for partial columns", () => {
    expect(getDdjLineNumber(1, 1)).toEqual(DDJ.length - 1);
    expect(getDdjLineNumber(25, 25)).toEqual(DDJ.length - 1);
  });
});
