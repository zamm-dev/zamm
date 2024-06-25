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

const SAMPLE_DB_PATH =
  "../src-tauri/api/sample-database-writes/conversation-edited-2/dump.yaml";

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

  it("should allow navigation to the new API call page", async function () {
    this.retries(2);
    await findAndClick('a[title="API Calls"]');
    await findAndClick("a=from scratch");
    await findAndClick('a[title="API Calls"]');
    await findAndClick("a=from scratch");
    await browser.pause(2500); // for page to finish rendering
    expect(
      await browser.checkFullPageScreen("new-api-call", {}),
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

  it("should be able to import data", async function () {
    await findAndClick('a[title="Settings"]');
    await findAndClick('a[title="Dashboard"]');
    await findAndClick('a[title="Settings"]');
    await browser.execute(`window.WEBDRIVER_FILE_PATH = '${SAMPLE_DB_PATH}';`);
    await findAndClick("button=Import data");
    await browser.pause(1000); // for data to be imported
    await findAndClick('a[title="API Calls"]');
    await findAndClick('a[title="Dashboard"]');
    // click twice to reset the saved navigation to the "New API Call" page
    await findAndClick('a[title="API Calls"]');
    await findAndClick('a[title="API Calls"]');
    await browser.pause(500); // for API calls to load
    expect(
      await browser.checkFullPageScreen("api-calls-populated", {}),
    ).toBeLessThanOrEqual(maxMismatch);
  });

  it("should be able to view single LLM call", async function () {
    this.retries(2);
    await findAndClick('a[title="API Calls"]');
    await browser.pause(500); // for API calls to load
    // second link is the first in the list because the first link is the + sign
    await findAndClick(".api-calls-page a:nth-child(2)");
    await findAndClick('a[title="API Calls"]');
    await browser.pause(500); // for API calls to load
    await findAndClick(".api-calls-page a:nth-child(2)");
    await browser.pause(4_000); // for snackbar messages from previous tests to go away
    expect(
      await browser.checkFullPageScreen("api-call-individual", {}),
    ).toBeLessThanOrEqual(maxMismatch);
  });
});
