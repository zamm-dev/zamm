# JavaScript

## Sleeping

To sleep, see [this answer](https://stackoverflow.com/a/39914235):

```js
await new Promise(r => setTimeout(r, 2000));
```

### WebdriverIO

For WebdriverIO tests specifically, there is the `browser.pause` function, described [here](https://webdriver.io/docs/api/browser/pause/).
