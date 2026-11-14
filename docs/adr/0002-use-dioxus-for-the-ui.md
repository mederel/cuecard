# 2. Use Dioxus for the user interface

- Status: Accepted, to be confirmed by a spike (see Consequences)
- Date: 2026-11-14

## Context

Cuecard targets Linux desktop, Android and the web with one adaptive UI (`docs/ui/wireframes.md`). The UI toolkit must:

- run on all three targets from one Rust codebase;
- have a permissive license (MIT, Apache-2.0, BSD), as required for dependencies in `AGENTS.md`;
- handle French text well: accents, text input on a phone, screen readers;
- reach the microphone, since the app listens to the actor.

## Decision

Use **Dioxus** (0.7 at the time of writing) for the UI, and keep everything that is not UI in a separate pure-Rust core crate (scripts, matching, rehearsal logic) with no platform code.

- One component model renders to the browser (WebAssembly), to desktop (a system WebView) and to Android (the system WebView). The UI is written in HTML and CSS, so the responsive layouts in the wireframes (a 900 px breakpoint, navigation moving to a bottom bar) are plain CSS.
- It is licensed MIT, per its crate READMEs.
- Android and iOS are first-class targets since 0.7, with direct access to Android APIs through JNI when needed.
- The TTS and speech engines stay behind their own interfaces (see ADR 0001), outside the UI crate.

## Consequences

Good:

- A single UI codebase for the three targets, in Rust, with CSS for layout and theming.
- Native text inputs and browser accessibility (the DOM exposes the real accessibility tree), which matters for a text-heavy app in French.
- Calls into the platform are possible where the framework falls short (`web-sys` in the browser, JNI on Android).

Bad or to watch:

- **Dioxus is pre-1.0.** Expect breaking changes between minor versions; pin the version.
- **Desktop and Android depend on a system WebView.** On Linux this means WebKitGTK must be installed. On Android, microphone access from a WebView needs the app to bridge the OS permission to the page; Tauri users on the same WebView layer report needing manual work for this, and `wry` merged a permission handler API for it in June 2026. This is the main technical risk.
- **Offline use on the web** needs a service worker (PWA) and downloaded models. Model weights are never committed.
- **TTS and speech recognition on Android and in the browser are unverified.** ADR 0001 only covered Linux, and the Rust integration of Kokoro is still open.

Spike before building features: a minimal Dioxus app that, on Linux, Android and in a browser, (1) shows the rehearsal screen at both breakpoints, (2) records from the microphone, and (3) plays one synthesized sentence. If microphone access or audio fails on a target, reopen this decision. The first fallback to evaluate is egui, with the limits below.

## Alternatives considered

- **Slint**: native rendering for desktop, mobile and web. Its license is GPL-3.0, a royalty-free license for proprietary apps, or a paid one. None is permissive, so it conflicts with our dependency rule, and GPL would need an explicit decision. I did not confirm its current Android and web maturity.
- **egui with eframe** (MIT or Apache-2.0): runs on Linux, Android and web. Its own README recommends it where performance matters more than accessibility and mobile text editing, and says the web screen reader is experimental and must be enabled explicitly. Cuecard is text input and reading, so this is the wrong trade. Kept as the fallback.
- **Tauri 2**: supports desktop and Android with a web frontend. It would still need a separate web build, and a JavaScript or Rust-to-WASM frontend on top. Dioxus gives the three targets from one toolchain. The Android microphone caveat above applies equally.
- **Iced**: not evaluated in depth, since its Android support is not first-class.

## References

- Dioxus: <https://dioxuslabs.com/learn/0.7>
- Dioxus 0.7 release notes: <https://github.com/DioxusLabs/dioxus/releases/tag/v0.7.0>
- Slint license: <https://github.com/slint-ui/slint/blob/master/LICENSE.md>
- eframe README (limits): <https://docs.rs/crate/eframe/0.34.0/source/README.md>
- wry permission handler API: <https://github.com/tauri-apps/wry/pull/1654>
- Tauri Android microphone report: <https://github.com/orgs/tauri-apps/discussions/12732>
