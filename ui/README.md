# ZAMM

## Dev setup

Follow the instructions in [`tauri.md`](/zamm/resources/tutorials/setup/dev/tauri.md) to set up Tauri.

Then, to avoid the issue mentioned in [`indradb.md`](/zamm/resources/tutorials/libraries/indradb.md), install this:

```bash
$ sudo apt install libclang-dev
```

And to avoid the issue mentioned in [new Tauri project setup](/zamm/resources/tutorials/setup/tauri/new-tauri-project.md), install this:

```bash
$ sudo apt install fuse
```

### For new repos

Follow these instructions to set the project up:

- [Setting up a new Tauri project](/zamm/resources/tutorials/setup/tauri/new-tauri-project.md)
- [Setting up a Python sidecar](/zamm/resources/tutorials/setup/tauri/python-sidecar.md)

### For existing repos

If you already have a version of this project built, then enter into its directories and start building:

```bash
$ cd src-python
$ poetry shell
$ poetry install
$ make sidecar
```

and

```bash
$ pre-commit install
```

## Feature engineering

### Singleton Graph DB

We define it as such

```rust
const DB_NAME: &str = "zamm.sqlite3";

struct ZammDatabase(Mutex<Option<SqliteConnection>>);

...

fn main() {
    let possible_db = get_db();

    tauri::Builder::default()
        .manage(ZammDatabase(Mutex::new(possible_db)))
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

where `get_db` is defined below.

Check that you have gotten this to compile.

### Saving the DB to the user's data dir

Instead of leaving a mess in the arbitrary current directory where the command is run, or preventing the user from accessing the same database again, use [`directories`](/zamm/resources/tutorials/libraries/directories.md) to pick the user's data folder for app data storage.

Requirements:

- Try to create the database in the user's data directory.
- If that fails for any reason, print out an error message to that effect, and default to creating the database in the current directory instead.
- Make sure to print out the eventual graph database path

Implementation details:

- Make sure to use a constant for the database name.

TODO: make database path a configurable commandline argument instead

This is a sample implementation of these requirements:

```rust
fn connect_to(db_path: PathBuf) -> Option<SqliteConnection> {
    let db_path_str = db_path.to_str().expect("Cannot convert DB path to str");
    match SqliteConnection::establish(db_path_str) {
        Ok(conn) => {
            println!("Connected to DB at {}", db_path_str);
            Some(conn)
        },
        Err(e) => {
            eprintln!("Failed to connect to DB: {}", e);
            None
        }
    }
}

/** Try to start SQLite database in user data dir. */
fn get_data_dir_db() -> Option<SqliteConnection> {
    if let Some(user_dirs) = ProjectDirs::from("dev", "zamm", "ZAMM") {
        let data_dir = user_dirs.data_dir();

        if !data_dir.exists() {
            match fs::create_dir_all(&data_dir) {
                Ok(()) => (),
                Err(e) => {
                    eprintln!("Failed to create data directory: {}", e);
                    return None;
                }
            }
        }

        connect_to(data_dir.join(DB_NAME))
    } else {
        eprintln!("Cannot find user home directory.");
        None
    }
}

fn get_db() -> Option<SqliteConnection> {
    get_data_dir_db().or_else(|| {
        eprintln!("Unable to create DB in user data dir, defaulting to current dir instead.");
        connect_to(env::current_dir().expect("Failed to get current directory").join(DB_NAME))
    })
}
```

### Frontend styling

Install fonts as described [here](/zamm/resources/tutorials/coding/frameworks/sveltekit.md). Then add CSS for the fonts, editing `src-svelte/src/routes/styles.css`:

```css
:root {
  --font-body: "Good Timing", sans-serif;
  --font-header: "Nasalization", sans-serif;
  --font-mono: "Jetbrains Mono", monospace;
  ...
  font-family: var(--font-body);
  color: var(--color-text);
  font-size: 18px;
  font-weight: 400;
}

...
```

If you do this, make sure to edit the font-family for `src-svelte/src/routes/Header.svelte` as well:

```css
  header {
    display: flex;
    justify-content: space-between;
    font-family: Arial, Helvetica, sans-serif;
  }
```


### Exposing API keys to the frontend

First [refactor](/zamm/resources/tutorials/coding/refactoring_rust_module.md), then instructions at [`environment_variables.md`](/zamm/resources/tutorials/systems/environment_variables.md) to read them from the environment.

Then add a command for exposing API keys, as described below. Pipe that data through to the frontend. The keys should be displayed in a table with a single header spanning all columns, named "API Keys". The service name should be displayed on the left column of the table, and the key should be displayed on the right. The key should be marked as "not set" if it's undefined.

Entire page:

```css
<script lang="ts">
  import { getApiKeys } from "$lib/bindings";

  let api_keys = getApiKeys();
</script>

<section>
  <table>
    <tr>
      <th colspan="2">API keys</th>
    </tr>
    <tr>
      <td>OpenAI</td>
      <td class="key">
        {#await api_keys}
          ...loading
        {:then keys}
          {#if keys.openai !== undefined && keys.openai !== null}
            {keys.openai.value}
          {:else}
            <span class="unset">not set</span>
          {/if}
        {:catch error}
          <span style="color: red">{error.message}</span>
        {/await}
      </td>
    </tr>
  </table>
</section>

<style>
  section {
    display: flex;
    flex-direction: column;
    flex: 0.6;
  }

  table {
    width: 0.1%;
    white-space: nowrap;
  }

  th, td {
    padding: 0 0.5rem;
    text-align: left;
  }

  td {
    color: #000;
  }

  .key {
    font-weight: bold;
    text-transform: lowercase;
  }

  .unset {
    color: #888;
  }
</style>

```

## Actions

### Adding a new command

#### Example: command for exposing API keys

Create `src-tauri/src/commands/keys.rs`:

```rust
use crate::setup::api_keys::ApiKeys;
use crate::ZammApiKeys;
use specta::specta;
use std::clone::Clone;
use tauri::State;

#[tauri::command]
#[specta]
pub fn get_api_keys(api_keys: State<ZammApiKeys>) -> ApiKeys {
    api_keys.0.lock().unwrap().clone()
}

```

Then edit `src-tauri/src/commands/mod.rs` to include the new command:

```rust
mod api;
mod errors;
mod greet;
mod keys;

pub use greet::greet;
pub use keys::get_api_keys;

```

Now, in `src-tauri/src/main.rs`, check if the name for the new command already exists for a different function or variable. If it does, choose to either:

1. Rename the command to something else
2. Rename the existing function to something else
3. Qualify each use of the existing function or new command with the module name

In this case, `get_api_keys` was already defined for initializing the API keys at app startup. We decide to rename it to `setup_api_keys` instead. Now:

1. Add the new command to Specta types export

```rust
use commands::{get_api_keys, greet};

fn main() {
    #[cfg(debug_assertions)]
    ts::export(
        collect_types![greet, get_api_keys],
        "../src-svelte/src/lib/bindings.ts",
    )
    .unwrap();
```

2. Add the new command to the Tauri invoke handler

```rust
    tauri::Builder::default()
        ...
        .invoke_handler(tauri::generate_handler![greet, get_api_keys])
        ...
```

### Using the new command on the frontend

#### Displaying the API keys

Continuing the example from above, in the Svelte component you want to edit, get the promise:

```ts
  import { getApiKeys } from "$lib/bindings";

  let api_keys = getApiKeys();
```

then edit the HTML for Svelte:

```svelte
<section>
  <p>
    Your OpenAI API key:
    <span class="key">
    {#await api_keys}
      ...loading
    {:then keys}
      {#if keys.openai !== undefined && keys.openai !== null}
        {keys.openai.value}
      {:else}
        <span class="unset">not set</span>
      {/if}
    {:catch error}
      <span style="color: red">{error.message}</span>
    {/await}
    </span>
  </p>
</section>
```

An alternative would be to wrap a larger part in "loading...". However, note that this would result in layout changes when the promise resolves:

```svelte
<section>
  <table>
    <tr>
      <th class="header-text" colspan="2">API Keys</th>
    </tr>
    {#await api_keys}
      <tr><td colspan="2">...loading</td></tr>
    {:then keys}
      <tr>
        <td>OpenAI</td>
        <td class="key">
          {#if keys.openai !== undefined && keys.openai !== null}
            {keys.openai.value}
          {:else}
            <span class="unset">not set</span>
          {/if}
        </td>
      </tr>
    {:catch error}
      <tr><td colspan="2">{error.message}</td></tr>
    {/await}
  </table>
</section>
```

Because the function signature of the Rust command indicates that it should always return successfully, we don't actually need the `catch` block because the main other failure mode for frontend API calls is a network error, which is not relevant here for a local app. However, it may be relevant again should we ever want to port this to the web.

Use [this trick](https://doc.rust-lang.org/std/thread/fn.sleep.html) to make the API call slower so that we can actually see the wait in action:

```rust
use crate::setup::api_keys::ApiKeys;
use crate::ZammApiKeys;
use specta::specta;
use std::clone::Clone;
use std::{thread, time};
use tauri::State;

#[tauri::command]
#[specta]
pub fn get_api_keys(api_keys: State<ZammApiKeys>) -> ApiKeys {
    let ten_seconds = time::Duration::from_secs(10);
    thread::sleep(ten_seconds);
    api_keys.0.lock().unwrap().clone()
}

```

From this we can observe that the screen is not rendering at all until the API call finishes, unlike what SvelteKit should be doing with `await`. At first it is unclear if this is not working as intended due to Tauri or SvelteKit behavior, but with more testing we realize it is definitely on the Tauri side, as not a single way to implement async function on Svelte works.

From further searching, we find [this discussion](https://github.com/tauri-apps/tauri/discussions/4191) where we realize that we need to say `#[tauri::command(async)]`. After this change, the wait works as expected. Make sure to undo the wait before committing. **Note: `tauri::command(async)` should almost always be used. Go back and add it in to whichever commands you have forgotten to do this with.

## Sidebar

There shold be a sidebar next to the main content. `src-svelte/src/routes/+layout.svelte` should look like this:

```svelte
<div class="app">
  <Sidebar />

  <main class="h-screen">
    <slot />
  </main>
</div>

<style>
  main {
    padding: 0.5em;
    margin-left: var(--sidebar-width);
  }
</style>
```

Now for the sidebar itself, add to `src-svelte/src/routes/styles.css`:

```css
...

:root {
  ...
  --shadow-offset: 4px;
  --shadow-blur: 8px;
}
```

This will allow us to control the shadow intensity across multiple elements at the same time. Note that we are using pixels here instead of `em` or `rem` so that shadow intensity does not increase with font size. However, the case can be made that if font size needs to be increased as a proxy for zoom on a larger screen, then the shadow intensity should be made more obvious as a visual element as well.

Then, add to `src-svelte/src/routes/Sidebar.svelte`:

```svelte
<script>
  let icons = ["⚙️", "?"];
  let selectedIcon = "⚙️";
</script>

<header>
  <nav>
    {#each icons as icon}
      <div class="{icon === selectedIcon ? 'selected' : ''} icon rounded-l-md">
        {icon}
      </div>
    {/each}
  </nav>
</header>
```

Now for the style section, the sidebar should always be on the left:

```css
  header {
    position: fixed;
    top: 0;
    left: 0;
  }
```

It should always span the entire left of the page:

```css
  header {
    height: 100vh;
    width: var(--sidebar-width);
  }
```

We'll want an empty pseudo-element casting a shadow on it from the right, as if it were the main content casting a shadow:

```css
  header::before {
    content: "";
    pointer-events: none;
  }
```

This will be located just to the right of the header:

```css
  header::before {
    position: fixed;
    top: 0;
    left: var(--sidebar-width);
  }
```

It will run the entire left of the header. It needs a sizable width, or else its shadow will be diminished.

```css
  header::before {
    width: 50px;
    height: 100vh;
  }
```

But it should only cast a shadow on the left, not to the right, or else it would be casting a shadow on the main content as well and making its presence absolutely clear. To do this, we use `clip-path` as mentioned [here](https://stackoverflow.com/a/62366856):

```css
  header::before {
    box-shadow: calc(-1 * var(--shadow-offset)) 0 var(--shadow-blur) 0 #ccc;
    clip-path: inset(0px 0 0px -15px);
  }
```

As for the icons, they should be squares that take up the entire width of the sidebar:

```css
  .icon {
    width: var(--sidebar-width);
    height: var(--sidebar-width);
  }
```

Their contents should be centered both vertically and horiontally:

```css
  .icon {
    display: flex;
    align-items: center;
    justify-content: center;
  }
```

Now, we should make selected items stand out. We'll make them bright white while the sidebar is made slightly gray:

```css
  header {
    background-color: #f4f4f4;
  }

  .selected {
    background-color: white;
  }
```

We will also raise the selected items up so that they look as if they're flush with the main content. To do this, first make sure no shadow is cast on them. We achieve this using `z-index`. Note that `position` has to be relative, or else the `z-index` [won't apply](https://stackoverflow.com/a/3259067).

```css
  header {
    z-index: 1;
  }

  header::before {
    z-index: 1;
  }

  .selected {
    position: relative;
    z-index: 2;
  }
```

Next, let's make sure that this element also applies a shadow like the main content does. We'll reuse the same trick from before to make sure that the shadow is cast on every side except for the right, because the selected element is flush with the main content and therefore shouldn't cast a shadow on it:

```css
  .selected {
    box-shadow: 0 var(--shadow-offset) var(--shadow-blur) 0 #ccc;
    clip-path: inset(-15px 0 -15px -15px);
  }
```

To be consistent with the rounded nature of the Nasalization fonts, let's smooth out the corners a bit by adding `rounded-l-md` as a Tailwind class to the div. We notice that it is slightly weird to have this flush at the very top left corner of the screen, so we move all the icons slightly down:

```css
  header {
    padding-top: 0.75rem;
  }
```

Now we can fully appreciate its physicality through the shadow cast by this on the sidebar. However, if we are to pay attention, we notice that the corners on the right that connect with the rest of the main content are at jarring right angles. Let's use [this trick](https://itnext.io/how-to-make-a-fancy-inverted-border-radius-in-css-5db048a53f95) to create fancy inverted borders:

```css
  .selected::before, .selected::after {
    content: '';
    height: 1rem;
    width: 1rem;
    position: absolute;
    right: 0;
  }

  .selected::before {
    bottom: var(--sidebar-width);
    border-radius: 0 0 5px 0;
    box-shadow: 0 5px 0 0 white;
  }

  .selected::after {
    top: var(--sidebar-width);
    border-radius: 0 5px 0 0;
    box-shadow: 0 -5px 0 0 white;
  }
```

If you wish to debug where the elements are, simply set `background-color` or `border` to something obvious like red, and the box-shadow color to something like blue.

If we run the visual Storybook test again now:

```
Serialized Error: { matcherResult: { message: 'Expected image to match or be a close match to snapshot but was 0.05102040816326531% different from snapshot (12 differing pixels).\n\u001b[1m\u001b[31mSee diff for details:\u001b[39m\u001b[22m \u001b[31mscreenshots/testing/diff/navigation/sidebar/settings-selected-diff.png\u001b[39m', pass: false } }
```

It's amazing what a difference 12 pixels can make.

Note that if we blow up the font size to 38px, for example, the corners start looking off. This is because we're using absolute pixel units to define the shadow offset. Instead, adjust it using `rem` until it fits:

```css
  .selected::before {
    box-shadow: 0 0.375rem 0 0 white;
  }

  .selected::after {
    box-shadow: 0 -0.375rem 0 0 white;
  }
```

Note if we increase the size of the elements by a lot, it makes it more obvious that the rounded corner is incongruously casting a square shadow. To make the inverted border radius shadow rounded is hard to do with CSS alone, and might call for custom graphics. However, this is really only noticeable at large font sizes, especially when the shadow is a very intense one.

### No shadows on older browsers

However, note that on some older browsers, negative values for `clip-path` will not show the shadow being cast outside of the element's usual bounding box, resulting in problems such as [this](https://css-tricks.com/using-box-shadows-and-clip-path-together/). The trick noted in that article won't work because we already have the shape we want to cast a shadow with, we just want to truncate the shadow on one side. Instead, we shift the `clip-path` property out of `header::before` and `.selected`, and into `header` instead:

```css
  header {
    ...
    clip-path: inset(0 0 0 0);
  }

  header::before {
    ...
    box-shadow: calc(-1 * var(--shadow-offset)) 0 var(--shadow-blur) 0 #ccc;
  }

  .selected {
    ...
  }
```

### Making inverted round corner scalable with corner roundness

The inverted corner effect might disappear entirely if it is too small in relation to the corner roundness. On the other hand, the selected tab might appear over the rounded corner if it is not up enough. To scale it up, we make all units relative to `--corner-roundness`:

```css
  header {
    --icons-top-offset: calc(2 * var(--corner-roundness));
  }

  .indicator::before,
  .indicator::after {
    height: calc(2 * var(--corner-roundness));
    width: var(--corner-roundness);
  }

  .indicator::before {
    box-shadow: 0 var(--corner-roundness) 0 0 var(--color-foreground);
  }

  .indicator::after {
    box-shadow: 0 calc(-1 * var(--corner-roundness)) 0 0 var(--color-foreground);
  }
```

We notice that this works on the Tauri app but not on the latest Firefox, so we make our CSS more appropriate by adding

```css
  nav {
    position: relative;
  }
```

which makes the `nav` a positioned element and makes the calculation of `--icons-top-offset` entirely irrelevant. We update our TypeScript code accordingly:

```ts
  function setIndicatorPosition(newRoute: string) {
    ...
    indicatorPosition = `calc(${routeIndex} * var(--sidebar-icon-size))`;
    ...
  }
```

### Embossing icons

To use icons, follow the instructions for [unplugin-icons](/zamm/resources/tutorials/setup/dev/unplugin.md). Then we follow the instructions [here](https://css-tricks.com/adding-shadows-to-svg-icons-with-css-and-svg-filters/) to define a filter that will emboss our icons:

```svelte
...

<header>
  <svg version="1.1" width="0" height="0">
    <filter id="inset-shadow">
      <feOffset dx="0" dy="0" />
      <feGaussianBlur stdDeviation="1" result="offset-blur" />
      <feComposite
        operator="out"
        in="SourceGraphic"
        in2="offset-blur"
        result="inverse"
      />
      <feFlood flood-color="#555" flood-opacity=".95" result="color" />
      <feComposite operator="in" in="color" in2="inverse" result="shadow" />
      <feComposite operator="over" in="shadow" in2="SourceGraphic" />
    </filter>
  </svg>

  <nav>
    <div class="selected icon">
      <IconSettings />
    </div>
    <div class="icon">
      <IconChat />
    </div>
  </nav>
</header>
```

We make our icons bigger and apply the filter to them. We use `:global` here to make the CSS actually apply to the externally defined component, as described [here](https://learn.svelte.dev/tutorial/component-styles).

```css
  .icon > :global(:only-child) {
    font-size: calc(0.5 * var(--sidebar-width));
    color: #aaa;
    filter: url(#inset-shadow);
  }
```

Next, we make the selected icon stand out by applying an activation color to it. Note that there is [no way](https://stackoverflow.com/a/31847031) to pass parameters to SVG filters, so we copy-paste the code from before and tweak it:

```svelte

<header>
  <svg version="1.1" width="0" height="0">
    ...

    <filter id="inset-shadow-selected">
      <feOffset dx="0" dy="0" />
      <feGaussianBlur stdDeviation="2" result="offset-blur" />
      <feComposite
        operator="out"
        in="SourceGraphic"
        in2="offset-blur"
        result="inverse"
      />
      <feFlood flood-color="#002966" flood-opacity=".95" result="color" />
      <feComposite operator="in" in="color" in2="inverse" result="shadow" />
      <feComposite operator="over" in="shadow" in2="SourceGraphic" />
    </filter>

<style>
  ...

  .icon.selected > :global(:only-child) {
    color: #1a75ff;
    filter: url(#inset-shadow-selected) url(#sofGlow);
  }

  ...
</style>
```

Note the increase in `stdDeviation`, to make the embossed effect more obvious when the colors involved are darker.

Finally, we make this SVG completely invisible, because we are after all only using it for its filter:

```svelte
  <svg version="1.1" style="visibility: hidden; position: absolute;" width="0" height="0">
    ...
  </svg>
```

#### Refactoring width

Remember what we did above with `margin-top` for the header? Let's shift the icons a little to the left as well so that we can appreciate the shadow on the left too. Note that here, we'll have to take into account the item size as well; it used to be synonymous with sidebar width, but this is no longer the case. This calls for a little refactoring challenge that will test the LLM's ability to keep code clean in the face of new demands for features.

First define new constants in `src-svelte/src/routes/styles.css`:

```css
...

:root {
  ...
  --sidebar-width: 68px;
  --sidebar-left-padding: 0.5rem;
  --sidebar-icon-size: calc(var(--sidebar-width) - var(--sidebar-left-padding));
  ...
}
```

Now in `src-svelte/src/routes/Sidebar.svelte`:

```css
  header {
    ...
    /* this is the icon size, not the sidebar-width, because
    sidebar-width is supposed to control the total width of the sidebar,
    whereas CSS width only controls the sidebar's content area */
    width: var(--sidebar-icon-size);
    ...
  }

  ...

  .icon {
    width: var(--sidebar-icon-size);
    height: var(--sidebar-icon-size);
    ...
  }

  .icon > :global(:only-child) {
    font-size: calc(0.5 * var(--sidebar-icon-size));
    ...
  }

  ...


  .selected::before {
    bottom: var(--sidebar-icon-size);
    ...
  }

  .selected::after {
    top: var(--sidebar-icon-size);
    ...
  }
```

### Refactoring Tailwind parameters

Let's put corner roundedness into a single CSS variable so that we can control all of it at once. To do that, we need to either stop using the `rounded-l-md` class and define everything using our own CSS, or else define a new custom Tailwind class that will be compatible with the existing ones. We choose the former to maintain separation of content from presentation and allow for cleaner HTML code.

We see from the [Tailwind documentation](https://tailwindcss.com/docs/border-radius) that `rounded-l-md` is equivalent to

```css
border-top-left-radius: 0.375rem; /* 6px */
border-bottom-left-radius: 0.375rem; /* 6px */
```

We therefore edit `src-svelte/src/routes/styles.css` to define:

```css
:root {
  ...
  --corner-roundness: 0.375rem;
  ...
}
```

Next, edit `src-svelte/src/routes/Sidebar.svelte` to remove the class and add the CSS:

```svelte
<header>
  ...
      <div class="{icon === selectedIcon ? 'selected' : ''} icon">
  ...
</header>

<style>
  ...

  .selected {
    border-top-left-radius: var(--corner-roundness);
    border-bottom-left-radius: var(--corner-roundness);
    ...
  }

  ...

  .selected::before {
    border-radius: 0 0 var(--corner-roundness) 0;
    ...
  }

  .selected::after {
    border-radius: 0 var(--corner-roundness) 0 0;
    ...
  }
</style>
```

### Adding a story to the Storybook

We add this new component to the Storybook at `src-svelte/src/routes/sidebar.stories.ts`:

```ts
import Sidebar from "./Sidebar.svelte";
import type { StoryObj } from "@storybook/svelte";

export default {
  component: Sidebar,
  title: "Navigation/Sidebar",
  argTypes: {},
};

const Template = ({ ...args }) => ({
  Component: Sidebar,
  props: args,
});

export const SettingsSelected: StoryObj = Template.bind({}) as any;

```

We add a corresponding entry to `src-svelte/src/routes/storybook.test.ts`:

```ts
...

const components: ComponentTestConfig[] = [
  ...
  {
    path: ["navigation", "sidebar"],
    variants: ["settings-selected"],
  },
];

...
```

Check that all tests pass.

## Logo

### Creating the typographic SVG

To create a typographic SVG version of the logo, open Gimp up and type in "ZAMM" in 24 point Ropa Sans font. Then, follow [these instructions](https://www.techwalla.com/articles/text-to-path-in-gimp) to convert it to a path. Create a long line that runs parallel to the sloped middle edges of the Z. It may be easier if you press `Edit Mode > Edit (Ctrl)` in the `Paths` palette. Now use that line to extend the top and bottom of the Z to the line, so that the Z becomes a zig-zag. Delete the two intermediate points that are now redundant.

Next, extend the horizontal top and bottom of the Z to cover the whole word. To select multiple points to move at once, select `Edit Mode > Design` and shift-click on each node of the path.

If you see a dotted yellow border, that's the layer boundary. If it doesn't cover the whole word, you can [resize](https://docs.gimp.org/en/gimp-layer-resize.html) the layer as mentioned in the link, or resize it by first doing "Fill Path" in the Paths Tool palette, and then `Layer > Crop to Content` and `Image > Crop to Content` from the menu bar.

You can export the path as an SVG by right-clicking the path on the Paths Tool palette and selecting "Export Path...".

### Creating the image logo

Use the prompt "a logo of a robot hand typing on a keyboard, simple, vector, futuristic" on Midjourney. Then, for the image we have chosen, use the fuzzy select tool as described [here](https://logosbynick.com/gimp-delete-background-to-transparent/) to select the back background. This will also end up selecting dark parts of the robot itself, so subtract parts of the selection as described [here](http://gimpchat.com/viewtopic.php?f=4&t=15616) to make sure that it's only the background that is being selected.

Then, export the PNG. Because there's a lot of whitespace in the logo, we'll want a zoomed-in version of the logo for the app icon so that the icon will look more filled-up. Use the crop tool in Gimp, but lock the aspect ratio to be square. Then, export the cropped PNG as well.

Copy the cropped PNG over to `src-tauri/icons/icon.png`, and add a Makefile rule to generate all the app icons that Tauri expects:

```Makefile
icon:
	yarn tauri icon src-tauri/icons/icon.png
```

Copy the uncropped logo over to `src-svelte/static/logo.png`. Edit `src-svelte/src/routes/+page.svelte` to display it on the main page, above the other stuff:

```svelte
...

<img src="/logo.png" alt="Robot typing on keyboard" />

...
```

#### Main page banner

The image creates a lot of whitespace to the right. To fill it up, we add some system information:

```svelte
...

<section class="homepage-banner">
  <img src="/logo.png" alt="Robot typing on keyboard" />
  <div class="zamm-metadata info-box">
    <h2>System Information</h2>
    <table>
      <tr>
        <th colspan="2">ZAMM</th>
      </tr>
      <tr>
        <td>Version</td>
        <td class="version-value">0.0.0</td>
      </tr>
      <tr>
        <td>Stability</td>
        <td class="stability-value">Unstable (Alpha)</td>
      </tr>
      <tr>
        <td>Fork</td>
        <td>Original</td>
      </tr>
    </table>

    <table>
      <tr>
        <th colspan="2">Computer</th>
      </tr>
      <tr>
        <td>OS</td>
        <td>Linux</td>
      </tr>
      <tr>
        <td>Release</td>
        <td>Ubuntu 18.04</td>
      </tr>
    </table>
  </div>
</section>

...
```

Let's style this. The homepage banner should be one horizontal row, separated from any content below it by a little gap:

```css
  .homepage-banner {
    display: flex;
    flex-direction: row;
    margin-bottom: 1rem;
  }
```

Every element in it should have some space from the one to its left:

```css
  .homepage-banner > * {
    margin-left: 1rem;
  }
```

The system information div to the right should take up all the space that the logo doesn't take up:

```css
  .zamm-metadata {
    flex: 1;
  }
```

Each table should have some space from the element above it:

```css
  .zamm-metadata table {
    margin-top: 0.5rem;
  }
```

All cells in the table should have their text be flush to the left:

```css
  .zamm-metadata th, td {
    text-align: left;
    padding-left: 0;
  }
```

Cell text should be aligned to the top, in case the window width is is small enogh that the value line wraps around:

```css
  td {
    vertical-align: text-top;
  }
```

Table keys should be faded:

```css
  .zamm-metadata td:first-child {
    color: var(--color-faded);
    padding-right: 1rem;
  }
```

The value for system stability we highlight in orange for now:

```css
  .stability-value {
    color: var(--color-caution);
  }
```

with the accompanying var declaration in `src-svelte/src/routes/styles.css`:

```css
:root {
  --color-caution: #FF8C00;
}
```

Finally, we give the logo this size:

```css
  img {
    width: 19rem;
  }
```

This is just for the elements specific to this page. To complete the styling, we'll make use of the [rounded cut-corner CSS trick](/zamm/resources/tutorials/coding/css.md).

Afterwards, we notice that because the last line of the table, "Release    Ubuntu 18.04", contains no descenders in any of its characters, it looks as if there's extra padding at the bottom of the div, between the text baseline and the div border. To fix this, we add the class `.less-space` to the last table, and define the CSS for this class as:

```css
  .less-space {
    margin-bottom: -0.33rem;
  }
```
