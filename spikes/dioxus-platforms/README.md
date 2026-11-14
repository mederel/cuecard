# Cross-platform spike

Throwaway code that tests [ADR 0002](../../docs/adr/0002-use-dioxus-for-the-ui.md) (Dioxus on Linux, Android and the web). Story [#10](https://github.com/mederel/cuecard/issues/10), spec and plan in `docs/superpowers/`.

This folder is its own Cargo workspace, separate from the main crate and from CI. Run everything from here.

## Layout

- `core/`: package `cuecard-spike-core`, pure Rust with no dependencies. Normalises French text and counts matching words. (The folder is `core`, but the package cannot be named `core`, which is the standard library crate.)
- `app/`: the Dioxus UI, the rehearsal screen from the wireframes with the three hiding levels, a microphone level meter, a generated tone, and a text field that counts matching words. It is its own Cargo workspace so that `core` can be tested without downloading Dioxus. **Written but never compiled**: the Claude workspace has no network access to fetch Dioxus. Expect to fix compile errors on the first build.

## Commands

In `spikes/dioxus-platforms/` (the `core` crate):

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

In `spikes/dioxus-platforms/app/` (needs the Dioxus CLI: `cargo install dioxus-cli --version 0.7.9 --locked`, and `rustup target add wasm32-unknown-unknown`):

```sh
dx serve --web                      # browser, http://localhost:8080
dx serve --desktop --no-default-features --features desktop
dx serve --android --no-default-features --features mobile
```

The microphone needs `localhost` or HTTPS in the browser. On Android it also needs the `RECORD_AUDIO` permission and a handler that grants the WebView's capture request (plan task 5): this is not written yet, and the app will report `NotAllowedError` until it is.

## Tool versions

Fill in as the spike progresses.

| Tool | Version |
| --- | --- |
| Rust | 1.97.0 (stable) |
| Dioxus and `dx` | 0.7.9 (pinned in `app/Cargo.toml`, same as `DX_VERSION` in `release.yml`) |
| Android SDK, NDK | not chosen yet |
| WebKitGTK | not checked yet |

## Not yet checked

The core crate has only been built for the host (`x86_64-unknown-linux-gnu`). Building it for `wasm32-unknown-unknown` and `aarch64-linux-android` is left for a machine that can install those targets (`rustup target add ...`).
