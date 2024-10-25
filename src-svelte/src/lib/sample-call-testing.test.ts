import { TauriInvokePlayback } from "./sample-call-testing";

describe("TauriInvokePlayback", () => {
  it("should mock matched calls", async () => {
    const playback = new TauriInvokePlayback();
    playback.addCalls({
      request: ["command", { inputArg: "input" }],
      response: { outputKey: "output" },
      succeeded: true,
    });
    const result = await playback.mockCall("command", { inputArg: "input" });
    expect(result).toEqual({ outputKey: "output" });
  });

  it("should throw an error for unmatched calls", async () => {
    const playback = new TauriInvokePlayback();
    playback.addCalls({
      request: ["command", { inputArg: "input" }],
      response: { outputKey: "output" },
      succeeded: true,
    });
    expect(() => playback.mockCall("command", { inputArg: "wrong" })).toThrow(
      'No matching call found for ["command",{"inputArg":"wrong"}].\n' +
        'Candidates are ["command",{"inputArg":"input"}]',
    );
  });
});
