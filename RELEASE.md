# Release

1. [Publish](https://github.com/zamm-dev/zamm/actions/workflows/publish.yaml) a new release of the app
2. [Edit](https://github.com/zamm-dev/zamm/releases) the latest release to include release notes
3. Add release notes in plaintext to the generated `latest.json`. Add a *single* newline in between each bullet point.
4. Edit the [latest version gist](https://gist.github.com/amosjyng/b3bbcb4ea176009732ea6898f87fe102/) with the updated contents of `latest.json`
5. Update the release blog post with the latest links to each release binary. The local file path should look something like `src/content/blog/v0.1.1.mdx`
6. Do a squash commit into the blog repo's main branch and push to origin
7. Download and install the latest version from the release webpage to ensure that everything works as expected
8. Submit release to HN, and update blog post with link to HN post
9. Bump version number in:
   - [`src-tauri/Cargo.toml`](/src-tauri/Cargo.toml)
   - [`src-tauri/Cargo.lock`](/src-tauri/Cargo.lock)
   - [`src-tauri/tauri.conf.json`](/src-tauri/tauri.conf.json)
   - [`src-tauri/api/sample-disk-writes/preferences/version-init/preferences.toml`](/src-tauri/api/sample-disk-writes/preferences/version-init/preferences.toml)
   - [`src-tauri/api/sample-disk-writes/preferences/version-update/preferences.toml`](/src-tauri/api/sample-disk-writes/preferences/version-update/preferences.toml)
   - [`src-tauri/api/sample-disk-writes/db-import-export/conflicting-llm-call/conflicting-db.yaml`](/src-tauri/api/sample-disk-writes/db-import-export/conflicting-llm-call/conflicting-db.yaml)
   - [`src-tauri/api/sample-disk-writes/db-import-export/conversation-edited-2/test-folder/exported-db.yaml`](/src-tauri/api/sample-disk-writes/db-import-export/conversation-edited-2/test-folder/exported-db.yaml)
   - [`src-tauri/api/sample-disk-writes/db-import-export/different-api-key/different.zamm.yaml`](/src-tauri/api/sample-disk-writes/db-import-export/different-api-key/different.zamm.yaml)
   - [`src-tauri/api/sample-disk-writes/db-import-export/terminal-sessions/exported-db.yaml`](/src-tauri/api/sample-disk-writes/db-import-export/terminal-sessions/exported-db.yaml)
10. Update the end-to-end screenshot
