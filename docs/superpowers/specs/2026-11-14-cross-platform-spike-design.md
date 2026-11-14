# Cross-platform spike: design

Story: [#10](https://github.com/mederel/cuecard/issues/10). Decision under test: [ADR 0002](../../adr/0002-use-dioxus-for-the-ui.md).

## Goal

Find out, with the least code, whether Dioxus can carry Cuecard on Linux desktop, Android and the web. The spike answers yes or no for each target on four capabilities, and measures Kokoro on each. It ends with ADR 0002 confirmed or reopened.

## Scope

A throwaway app in `spikes/dioxus-platforms/`. It is its own Cargo project with an empty `[workspace]` table, so the main crate and its CI are unaffected. It is deleted, or kept for reference, once the ADR is updated.

Two crates inside the spike:

- `core`: pure Rust, no platform code. A function that normalises a French sentence (case, accents, punctuation) and counts matching words against an expected sentence. It proves the core compiles for all three targets, including `wasm32-unknown-unknown`.
- `app`: the Dioxus UI.

## What the app shows

One screen, the rehearsal screen from the wireframes (`docs/ui/wireframes.md`, screen 3), with fixed text from *Tartuffe* (public domain) and no script import. It has:

- the cue card and the hidden-line card;
- the three hiding-level buttons;
- a button that starts the microphone and shows a live input level;
- a button that plays a generated tone;
- a text field for typing accented French;
- a button that runs Kokoro on a fixed sentence (phase B).

Layout: a sidebar at 900 px and above, a single column with a bottom bar below, in plain CSS.

## Test matrix

Each cell is recorded as pass, fail or partial, with a one-line note.

| Capability | Linux | Android | Web |
| --- | --- | --- | --- |
| Builds and runs | | | |
| Both layouts render and switch | | | |
| Microphone with live level, after permission | | | |
| Generated tone plays | | | |
| Accents typed and shown (`é è ê à ç œ ’ « »`) | | | |
| Core crate compiles and its tests pass | | | |

Targets for testing: the developer's Linux machine; one real Android device plus the emulator; Firefox and Chromium on desktop, and Chrome on the Android device.

## Phase B: Kokoro on each target

Only after phase A passes on a target. Try to synthesize `Couvrez ce sein que je ne saurais voir.` with the French voice, and record:

- model size on disk and how it is delivered (downloaded at first launch, never committed);
- time to load, and time to synthesize after loading, on a warm run;
- which runtime was used (an ONNX runtime from Rust natively, a WebAssembly build in the browser);
- whether the French phonemizer pulled in GPL code (see ADR 0001), and how it was avoided or run.

Proposed bar for a usable result, to be adjusted once there are real numbers: synthesis of one short sentence in under 3 seconds on a mid-range phone after loading, and in under 1 second on desktop.

## Out of scope

Script import, character choice, speech recognition, storage, the other screens, styling beyond the wireframe, installers, and the Play Store.

Speech recognition is a separate decision. The spike only records microphone input and shows its level, which is the prerequisite for any recognition engine.

## Risks and how the spike exposes them

- **Android microphone through the WebView.** Declare `RECORD_AUDIO`, request it at runtime, and grant the page's media-capture request in the activity. If the generated project cannot be customised enough, record it as a fail and note what Dioxus or `wry` would need.
- **Linux WebKitGTK.** Note the minimum version needed for `getUserMedia`, and the package to install.
- **Browser secure context.** Microphone access needs HTTPS or `localhost`; test both ways.
- **Tooling.** Android SDK and NDK, the Dioxus CLI (`dx`) and the wasm target are needed. Versions used are pinned in the spike's README.

## Output

- The test matrix and the Kokoro measurements, in a new "Spike results" section of ADR 0002.
- ADR 0002 status set to Accepted, or to a new ADR that supersedes it.
- If Kokoro runs, a first outline for the Rust integration ADR flagged as open in ADR 0001.
