# ZAMM

![Dashboard](/webdriver/screenshots/baseline/desktop_wry/welcome-screen.png)

This is an incomplete, experimental AI chat app.

You can learn more about it on [the website](http://zamm.dev).

## Development

The most up-to-date dependencies and their versions can be seen at [`.github/workflows/tests.yaml`](/.github/workflows/tests.yaml). You can either build the project by running `make` or looking at the [Makefile](/Makefile) to see the steps needed to build the project.

To set a custom directory for Storybook tests (because the screenshots differ slightly on different machines), you can set

```bash
export SCREENSHOTS_BASE_DIR=screenshots/local
```

If tests are timing out, you may also want to set

```bash
export PLAYWRIGHT_TIMEOUT=20000
```
