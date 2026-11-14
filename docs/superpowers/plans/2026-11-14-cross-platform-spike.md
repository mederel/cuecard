# Cross-platform spike: implementation plan

Spec: `docs/superpowers/specs/2026-11-14-cross-platform-spike-design.md`. Story: [#10](https://github.com/mederel/cuecard/issues/10).

Work in `spikes/dioxus-platforms/`, one commit per task, Conventional Commit messages. Run `cargo fmt` and clippy in the spike too. The main crate and CI stay untouched.

## Task 1: Scaffold the spike

- [x] Create `spikes/dioxus-platforms/` with its own `Cargo.toml` containing an empty `[workspace]` table. Only `core` is a member for now (package `cuecard-spike-core`, since `core` is the standard library crate). `app` is added in task 3, because Dioxus cannot be downloaded in the Claude workspace.
- [x] Write `spikes/dioxus-platforms/README.md` with the tool versions (Rust, `dx`, Android SDK and NDK, WebKitGTK), filled in as they are chosen.
- [ ] Pin the Dioxus version in `app` (task 3).
- [x] Check that the root `cargo test` and CI still do not see the spike.
- Commit: `chore: scaffold the cross-platform spike`

## Task 2: Core crate, test first

- [x] Write failing tests for `normalise` (case, accents, curly and straight apostrophes, guillemets, punctuation) and for `matching_words(expected, said)`, using `Vous êtes donc bien tendre à la tentation ?` as the fixture.
- [x] Implement them without platform code or heavy dependencies. Matching is a longest common subsequence of words, so a replaced or missing word is missed and extra words are free.
- [ ] Check it builds for `wasm32-unknown-unknown` and `aarch64-linux-android`. Not done: those targets are not installed in the Claude workspace.
- Commit: `feat: add word matching to the spike core`

## Task 3: Rehearsal screen and both layouts

- [ ] Build the rehearsal screen from the wireframe with the three hiding-level buttons, using the core crate for the hidden text.
- [ ] Write the CSS: sidebar at 900 px and above, one column and a bottom bar below. Touch targets of 48 px.
- [ ] Run it on Linux with `dx serve --desktop` and in a browser with `dx serve --web`. Fill the "builds", "layouts" and "accents" rows for both.
- Commit: `feat: add the rehearsal screen to the spike`

## Task 4: Microphone level and generated tone

- [ ] Add the microphone button: request access, show a live level. Use the browser media APIs through `web-sys` on web and in the WebViews.
- [ ] Add the tone button, generated in code. Do not commit any audio file.
- [ ] Test in the browser over `localhost` and over HTTPS, and on Linux desktop. Fill the microphone and tone rows.
- Commit: `feat: add microphone level and tone to the spike`

## Task 5: Android

- [ ] Generate the Android project with `dx`, and run it in the emulator, then on a real device.
- [ ] Add `RECORD_AUDIO` and `MODIFY_AUDIO_SETTINGS` to the manifest, request the permission at runtime, and grant the WebView's media-capture request in the activity.
- [ ] Fill the whole Android column. For any fail, write down what blocks it and which upstream change would fix it.
- Commit: `feat: run the spike on Android`

## Task 6: Kokoro on each target (phase B)

- [ ] For each target that passed task 4, try Kokoro on `Couvrez ce sein que je ne saurais voir.`. Linux and Android: an ONNX runtime from Rust. Web: a WebAssembly build of an ONNX runtime.
- [ ] Download the model at first launch into a cache folder. Never commit weights.
- [ ] Record model size, load time, warm synthesis time and the runtime used.
- [ ] Check which phonemizer is used for French and whether it brings GPL code.
- Commit: `feat: try Kokoro in the spike`

## Task 7: Write the results and decide

- [ ] Add a "Spike results" section to ADR 0002 with the filled matrix and the Kokoro numbers.
- [ ] Set its status to Accepted, or write a superseding ADR if a target failed with no workaround.
- [ ] If Kokoro ran, add an outline for the Rust integration ADR that ADR 0001 left open.
- [ ] Decide whether to keep `spikes/dioxus-platforms/` or delete it.
- Pull request title: `docs: record the cross-platform spike results`. Closes #10.

## Before each commit

`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test` in `spikes/dioxus-platforms/`, plus the same at the repo root.

## What needs a real machine

Tasks 3 to 6 need an Android SDK and NDK, an emulator or device, and a desktop with a microphone and speakers. They cannot be done in the Claude workspace, which has no network access for these tools and no audio devices.
