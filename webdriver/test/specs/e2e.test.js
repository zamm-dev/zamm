const maxMismatch = getEnvMismatchTolerance() ?? 0.0;
const DEFAULT_TIMEOUT = 5_000;

function getEnvMismatchTolerance() {
  return process.env.MISMATCH_TOLERANCE === undefined
    ? undefined
    : parseFloat(process.env.MISMATCH_TOLERANCE);
}

async function findAndClick(selector, timeout) {
  const button = await $(selector);
  await button.waitForClickable({
    timeout: timeout ?? DEFAULT_TIMEOUT,
  });
  await browser.execute("arguments[0].click();", button);
}

describe("App", function () {
  it("should render the welcome screen correctly", async function () {
    this.retries(2);
    await $("table"); // ensure page loads before taking screenshot
    await browser.pause(500); // for CSS transitions to finish
    expect(
      await browser.checkFullPageScreen("welcome-screen", {}),
    ).toBeLessThanOrEqual(maxMismatch);
  });

  it("should allow navigation to the chat page", async function () {
    this.retries(2);
    await findAndClick('a[title="Chat"]');
    await findAndClick('a[title="Dashboard"]');
    await findAndClick('a[title="Chat"]');
    await browser.pause(2500); // for page to finish rendering
    expect(
      await browser.checkFullPageScreen("chat-screen", {}),
    ).toBeLessThanOrEqual(maxMismatch);
  });

  it("should allow navigation to the API calls page", async function () {
    this.retries(2);
    await findAndClick('a[title="API Calls"]');
    await findAndClick('a[title="Dashboard"]');
    await findAndClick('a[title="API Calls"]');
    await browser.pause(2500); // for page to finish rendering
    expect(
      await browser.checkFullPageScreen("api-calls", {}),
    ).toBeLessThanOrEqual(maxMismatch);
  });

  it("should allow navigation to the settings page", async function () {
    this.retries(2);
    await findAndClick('a[title="Settings"]');
    await findAndClick('a[title="Dashboard"]');
    await findAndClick('a[title="Settings"]');
    await browser.pause(2500); // for page to finish rendering
    expect(
      await browser.checkFullPageScreen("settings-screen", {}),
    ).toBeLessThanOrEqual(maxMismatch);
  });

  it("should allow navigation to the credits page", async function () {
    this.retries(2);
    await findAndClick('a[title="Credits"]');
    await findAndClick('a[title="Dashboard"]');
    await findAndClick('a[title="Credits"]');
    await browser.pause(2500); // for page to finish rendering
    await browser.execute(
      `document.querySelector('.growable .scroll-contents')
        .dispatchEvent(new Event('mousedown'));`,
    );
    await browser.execute(
      "document.querySelector('.growable .scroll-contents').scrollTop = 0;",
    );
    expect(
      await browser.checkFullPageScreen("credits-screen", {}),
    ).toBeLessThanOrEqual(maxMismatch);
  });
});
