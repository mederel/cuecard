# Cross-platform spike

Throwaway code that tests [ADR 0002](../../docs/adr/0002-use-dioxus-for-the-ui.md) (Dioxus on Linux, Android and the web). Story [#10](https://github.com/mederel/cuecard/issues/10), spec and plan in `docs/superpowers/`.

This folder is its own Cargo workspace, separate from the main crate and from CI. Run everything from here.

## Layout

- `core/`: package `cuecard-spike-core`, pure Rust with no dependencies. Normalises French text and counts matching words. (The folder is `core`, but the package cannot be named `core`, which is the standard library crate.)
- `app/`: the Dioxus UI. Not added yet: it is created in plan task 3, on a machine that can download Dioxus and has the Android tooling.

## Commands

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Tool versions

Fill in as the spike progresses.

| Tool | Version |
| --- | --- |
| Rust | 1.97.0 (stable) |
| Dioxus and `dx` | not chosen yet |
| Android SDK, NDK | not chosen yet |
| WebKitGTK | not checked yet |

## Not yet checked

The core crate has only been built for the host (`x86_64-unknown-linux-gnu`). Building it for `wasm32-unknown-unknown` and `aarch64-linux-android` is left for a machine that can install those targets (`rustup target add ...`).
