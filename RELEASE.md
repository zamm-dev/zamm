# Release

1. [Publish](https://github.com/zamm-dev/zamm/actions/workflows/publish.yaml) a new release of the app
2. [Edit](https://github.com/zamm-dev/zamm/releases) the latest release to include release notes
3. Add release notes in plaintext to the generated `latest.json`
4. Edit the [latest version gist](https://gist.github.com/amosjyng/b3bbcb4ea176009732ea6898f87fe102/) with the updated contents of `latest.json`
5. Update the release blog post with the latest links to each release binary. The local file path should look something like `src/content/blog/v0.1.1.mdx`
6. Do a squash commit into the blog repo's main branch and push to origin
7. Download and install the latest version from the release webpage to ensure that everything works as expected
8. Submit release to HN, and update blog post with link to HN post
9. Bump version number in `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, and `src-tauri/tauri.conf.json`, and update the end-to-end screenshot
