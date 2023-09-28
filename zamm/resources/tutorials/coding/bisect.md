# Using Git bisect

We realize at some point that `src-svelte/screenshots/baseline/dashboard/api-keys-display/known.png`, along with `unknown`, have both broken at some point. We want to find out when this happened because it will give us clues as to what the problem is.

Using Git, we see that the screenshot file changed in commit `752603c`. This is part of PR #19. We do a git bisect on the commits inside that PR, and find that it is in fact commit `752603c` that broke it. We look at all the relevant changes in that commit and find:

```ts
export const Unknown: StoryObj = Template.bind({}) as any;
Unknown.parameters = {
  resolution: unknownKeys,
};
Unknown.parameters = {
  viewport: {
    defaultViewport: "mobile2",
  },
};
```

It turns out we have been overwriting the parameters! This is an easy fix. We apply the fix, check that it works on the offending commit, then Git stash and pop it right back onto the branch we were previously on.
