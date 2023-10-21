# Customizing Firefox

## Disabling password saving

Go to `about:preferences#privacy` and uncheck "Ask to save logins and passwords for websites".

## Tree-style tabs

Install the [Tree Style Tab](https://addons.mozilla.org/en-US/firefox/addon/tree-style-tab/) extension.

Then, follow the instructions [here](https://medium.com/@Aenon/firefox-hide-native-tabs-and-titlebar-f0b00bdbb88b) to hide the regular tab bar:

- Go to `about:config`
- Click the button `Accept the Risk and Continue`
- Set `toolkit.legacyUserProfileCustomizations.stylesheets` to true
- Go to `about:support`
- Get the "Profile Directory", which looks something like ` /home/amos/.mozilla/firefox/5dmcc2q7.default-release`
- Edit the file `chrome/userChrome.css` in that directory, and add the following:

```css
/* hides the native tabs */
#TabsToolbar {
  visibility: collapse;
}

#sidebar-header {
  visibility: collapse !important;
}
```

- Restart Firefox
