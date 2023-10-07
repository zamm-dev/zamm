# CSS shenanigans

## Adjusting table width to fit content

See [this answer](https://stackoverflow.com/a/43615091).

## Rounded cut corners

There are [a variety](https://stackoverflow.com/questions/7324722/cut-corners-using-css/65759042) [of ways](https://blog.logrocket.com/how-to-create-fancy-corners-in-css/) [to produce](https://stackoverflow.com/questions/10349867/how-do-i-bevel-the-corners-of-an-element) a cut corner effect. There is also a really simple way to create a rounded corner effect using `border-radius`, and a less straightforward way to do so to arbitrary shapes using [an SVG filter](https://stackoverflow.com/questions/31765345/how-to-round-out-corners-when-using-css-clip-path). What combines the two is [this technique](https://stackoverflow.com/a/65759042) in particular. We follow the provided example to create a rounded cut corner effect on a `div` element:

```svelte
<div class="container">
  <div class="border-box"></div>
  <svg style="visibility: hidden; position: absolute;" width="0" height="0" xmlns="http://www.w3.org/2000/svg" version="1.1">
    <defs>
        <filter id="round">
            <feGaussianBlur in="SourceGraphic" stdDeviation="3" result="blur" />    
            <feColorMatrix in="blur" mode="matrix" values="1 0 0 0 0  0 1 0 0 0  0 0 1 0 0  0 0 0 19 -9" result="goo" />
            <feComposite in="SourceGraphic" in2="goo" operator="atop"/>
        </filter>
    </defs>
  </svg>

  <div class="info-box">
    Actual content.
  </div>
</div>

<style>
  .border-box {
    width: 100%;
    height: 100%;
    position: absolute;
    top: 0;
    left: 0;
    filter: url(#round);
    z-index: 0;
  }

  .border-box::before {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: var(--color-border);
    mask:
      linear-gradient(-45deg, transparent 0 1rem, #fff 0) bottom right,
      linear-gradient(135deg, transparent 0 1rem, #fff 0) top left;
    mask-size: 51% 100%;
    mask-repeat: no-repeat;
  }

  .info-box {
    position: relative;
    z-index: 2;
    padding: 1rem;
  }

  .info-box h2 {
    margin: -0.25rem 0 0 1rem
  }
</style>
```

Since there's only two cut corners, we can produce that effect with only two linear gradients instead of the 4 in the StackOverflow answer.

If we want it to just be the border instead of a solid background, we can do this:

```svelte
  <div class="background-box"></div>

<style>
  .border-box::before {
    mask:
      linear-gradient(-45deg, transparent 0 calc(1rem + 1px), #fff 0) bottom right,
      linear-gradient(135deg, transparent 0 calc(1rem + 1px), #fff 0) top left;
  }

  .background-box {
    width: calc(100% - 1px);
    height: calc(100% - 1px);
    position: absolute;
    top: 1;
    left: 1;
    filter: url(#round);
    z-index: 1;
  }

  .background-box::before {
    content: "";
    position: absolute;
    top: 1px;
    left: 1px;
    right: 0;
    bottom: 0;
    background: white;
    mask:
      linear-gradient(-45deg, transparent 0 1rem, #fff 0) bottom right,
      linear-gradient(135deg, transparent 0 1rem, #fff 0) top left;
    mask-size: 51% 100%;
    mask-repeat: no-repeat;
  }
</style>
```

This new item is just a tiny bit smaller than the border box, allowing the edges to peek out by one single pixel. Note that we've also changed the border box cut to `calc(1rem + 1px)` to advance the cut by 1px in both corners, because otherwise the diagonal cut border would be thicker than the sides. 

Unfortunately, we see in our storybook screenshot tests that this is not working in headless Chrome. As it turns out, Firefox renders this fine, but not Chrome. It turns out Chrome only partially supports this feature with the `-webkit-` mask, so we add a copy of all these properties with the `-webkit-` prefix. We see that the lower right diagonal edge of the border is missing in Chrome, so we revert the `calc(1rem + 1px)` back to `1rem` specifically for `-webkit-`. Remember to put vendor prefixes [*before*](https://stackoverflow.com/a/7080674) the regular ones.

### Refining

By adding a drop shadow with a very small spread, we can generate the border effect without the use of the `border-box` div:

```css
  .container {
    filter: drop-shadow(0px 0px 1px rgba(0, 0, 0, 0.4));
  }
```

Now we can simply use one div with a white background (or else the drop shadow will apply to the elements inside the div and not on the whole div), and call it the `border-box`. Using a single element will make the rendering much more robust, as we no longer have to deal with precisely positioning one div on top of another.

Also note that we cannot use `var(--color-border)` here because the drop shadow effect will only further attenuate the color.

Next, we can parameterize how deep the cut is with CSS variables.

```css
  .container {
    --cut: 1rem;
  }

  .border-box {
    width: 100%;
    height: 100%;
    position: absolute;
    filter: url(#round);
    z-index: 1;
  }

  .border-box::before {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: white;
    -webkit-mask:
      linear-gradient(-45deg, transparent 0 var(--cut), #fff 0) bottom right,
      linear-gradient(135deg, transparent 0 var(--cut), #fff 0) top left;
    -webkit-mask-size: 51% 100%;
    -webkit-mask-repeat: no-repeat;
    mask:
      linear-gradient(-45deg, transparent 0 var(--cut), #fff 0) bottom right,
      linear-gradient(135deg, transparent 0 var(--cut), #fff 0) top left;
    mask-size: 51% 100%;
    mask-repeat: no-repeat;
  }
```

Unfortunately, our Storybook tests are failing now because the screenshot of the element itself doesn't include its drop shadow. To be fair, the fact that our container casts a 1px shadow outside of its boundaries means that we're not containing the entire visual display of the container inside the component. We contain everything inside the element, accounting for every single pixel of its visual display:

```svelte
<div class="container">
  <svg
    style="visibility: hidden; position: absolute;"
    width="0"
    height="0"
    xmlns="http://www.w3.org/2000/svg"
    version="1.1"
  >
    ...
  </svg>

  <div class="border-container">
    <div class="border-box"></div>
    <div class="info-box">
      <h2>{title}</h2>
      <slot />
    </div>
  </div>
</div>

<style>
  .container {
    position: relative;
    flex: 1;
    --cut: 1rem;
    padding: 1px;
  }

  .border-container {
    filter: drop-shadow(0px 0px 1px rgba(0, 0, 0, 0.4));
  }

  ...
</style>
```

### Alternative implementations

Another (untested) way of doing this is by using the [border-image-slice](https://www.w3schools.com/cssref/css3_pr_border-image-slice.php) CSS property.

Alternatively, the [clip-path](https://medium.com/headstorm/beveled-corners-with-css-react-3e385f50b6) CSS property can also be used. A quick test [here](https://codesandbox.io/s/beveled-corners-with-clip-path-forked-wcnxc8?file=/src/index.js) confirms that it is indeed possible to get a rounded cut corner effect with this property using the following React code:

```js
import React from "react";
import ReactDOM from "react-dom";

import styled from "styled-components";

import "./styles.css";

const SquareDiv = styled.div`
  display: inline-block;
  background-color: white;
  padding: 2rem;
  margin: 2rem;
`;

const RoundedDiv = styled.div`
  filter: url(#round);
`;

const BeveledDiv = styled(SquareDiv)`
  ${({ cornerAngleRadians, depth }) => `
  clip-path: polygon(
    ${Math.sin(cornerAngleRadians[0]) * depth}rem 0%,
    0% ${Math.cos(cornerAngleRadians[0]) * depth}rem,
  
    0% calc(100% - ${Math.cos(cornerAngleRadians[3]) * depth}rem),
    ${Math.sin(cornerAngleRadians[3]) * depth}rem 100%,
  
    calc(100% - ${Math.sin(cornerAngleRadians[2]) * depth}rem) 100%,
    100% calc(100% - ${Math.cos(cornerAngleRadians[2]) * depth}rem),
  
    100% ${Math.cos(cornerAngleRadians[1]) * depth}rem,
    calc(100% - ${Math.sin(cornerAngleRadians[1]) * depth}rem) 0%
  );
`}
`;

const ShadowCaster = styled.div`
  filter: drop-shadow(0px 2px 4px rgba(0, 0, 0, 0.2));
`;

function App() {
  const cornerAngles = [0, 45, 0, 45];

  return (
    <div className="App">
      <svg
        style={{ visibility: "hidden", position: "absolute" }}
        width="0"
        height="0"
        xmlns="http://www.w3.org/2000/svg"
        version="1.1"
      >
        <defs>
          <filter id="round">
            <feGaussianBlur in="SourceGraphic" stdDeviation="3" result="blur" />
            <feColorMatrix
              in="blur"
              mode="matrix"
              values="1 0 0 0 0  0 1 0 0 0  0 0 1 0 0  0 0 0 19 -9"
              result="goo"
            />
            <feComposite in="SourceGraphic" in2="goo" operator="atop" />
          </filter>
        </defs>
      </svg>
      <h2>The (SciFi) future is beveled</h2>
      <ShadowCaster>
        <SquareDiv>Past.</SquareDiv>

        <RoundedDiv>
          <SquareDiv>Present.</SquareDiv>
        </RoundedDiv>

        <RoundedDiv>
          <BeveledDiv
            cornerAngleRadians={cornerAngles.map((a) => (a * Math.PI) / 180)}
            depth={2}
          >
            Future.
          </BeveledDiv>
        </RoundedDiv>
      </ShadowCaster>
    </div>
  );
}

const rootElement = document.getElementById("root");
ReactDOM.render(<App />, rootElement);

```

## Refactoring variables

Notice in commit b784423 how the same color is refactored into a common variable.

## Dynamic background

We take inspiration from [Chris Smith](https://codepen.io/chris22smith/pen/RZogMa) and achieve a dynamic sliding background as such:

```svelte
<script lang="ts">
  export let animated = false;
  const duration = animated ? 15 : 0;
</script>

<div class="background" style="--base-duration: {duration}s;">
  <div class="bg"></div>
  <div class="bg bg2"></div>
  <div class="bg bg3"></div>
</div>

<style>
  .background {
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
    z-index: -100;
    --base-position: 55%;
    --base-duration: 0;
    --max-left: 40%;
    --max-right: 65%;
  }

  .bg {
    --position: var(--base-position);
    --duration: var(--base-duration);
    --color-overlay: #49D8D7;
    --go-left: calc(var(--max-left) - var(--position));
    --go-right: calc(var(--max-right) - var(--position));
    animation:slide var(--duration) ease-in-out infinite alternate;
    background-image: linear-gradient(
      120deg,
      transparent var(--position),
      var(--color-overlay) var(--position),
      transparent calc(var(--position) + 10%));
    bottom:0;
    left:-100%;
    opacity:0.1;
    position:fixed;
    right:-100%;
    top:0%;
    z-index:-1;
  }

  .bg2 {
    --duration: calc(1.33 * var(--base-duration));
    --color-overlay: #4949d8;
    --position: calc(var(--base-position) + 30vw);
  }

  .bg3 {
    --duration: calc(1.66 * var(--base-duration));
    --color-overlay: #49d849;
    --position: calc(var(--base-position) - 30vw);
  }

  @keyframes slide {
    0% {
      transform:translateX(var(--go-left));
    }
    100% {
      transform:translateX(var(--go-right));
    }
  }
</style>

```

You'll want to make sure this goes below other elements. For example, if you're using Svelte, edit `src-svelte/src/routes/+layout.svelte`:

```svelte
<script>
  ...
  import Background from "./Background.svelte";
</script>

<div class="app">
  <div class="background">
    <Background />
  </div>
  ...
</div>

<style>
  .background {
    z-index: -100;
  }

  main {
    ...
    z-index: 100;
  }
</style>
```

Note that this produces a jerky effect when switching from animated to not animated. To make it smoother, we can dynamically set the `animation-play-state` instead, with a [negative animation-delay](https://developer.mozilla.org/en-US/docs/Web/CSS/animation-delay#values) to separate the different slides instead of doing it with an edited gradient position:

```svelte
<script lang="ts">
  export let animated = false;
  let animationState: string;

  $: animationState = animated ? "running" : "paused";
</script>

<div class="background" style="--animation-state: {animationState};">
  <div class="bg"></div>
  <div class="bg bg2"></div>
  <div class="bg bg3"></div>
</div>

<style>
  .background {
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
    z-index: -100;
    --position: 55%;
    --base-duration: 15s;
    --max-left: -15%;
    --max-right: +10%;
    --animation-state: paused;
  }

  .bg {
    --duration: var(--base-duration);
    --color-overlay: #49d8d7;
    animation: slide var(--duration) ease-in-out infinite alternate;
    animation-play-state: var(--animation-state);
    background-image: linear-gradient(
      120deg,
      transparent var(--position),
      var(--color-overlay) var(--position),
      transparent calc(var(--position) + 10%)
    );
    bottom: 0;
    left: -100%;
    opacity: 0.1;
    position: fixed;
    right: -100%;
    top: 0%;
    z-index: -1;
  }

  .bg2 {
    --duration: calc(1.33 * var(--base-duration));
    --color-overlay: #4949d8;
    animation-direction: alternate-reverse;
    animation-delay: calc(-0.5 * var(--base-duration));
  }

  .bg3 {
    --duration: calc(1.66 * var(--base-duration));
    --color-overlay: #49d849;
    animation-delay: calc(-0.7 * var(--base-duration));
  }

  @keyframes slide {
    0% {
      transform: translateX(var(--max-left));
    }
    100% {
      transform: translateX(var(--max-right));
    }
  }
</style>

```
