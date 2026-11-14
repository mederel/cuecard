<p align="center">
  <img src="assets/logo/cuecard-mark.svg" alt="Cuecard logo: a card whose lines fade out" width="96" height="96">
</p>

<h1 align="center">Cuecard</h1>

<p align="center">An open-source app to help actors learn their lines.</p>

Import a script, pick your character, and rehearse: cues from your scene partners, your lines fading as you learn them, and a check on what you said. Scripts are mostly French. The app is planned for Linux, Android and the web, with one adaptive interface.

> Early days. Nothing to run yet: the design is done and a technical spike is under way.

## Planned

- Import a script and choose your character
- Cue mode: the app gives the line before yours, read aloud by a synthetic voice
- Progressive hiding: full text, first letters, blank
- Speech check against the script
- Spaced repetition for the shaky lines

## Where things are

- [Wireframes](docs/ui/wireframes.md): six screens, desktop and phone layouts
- [Architecture decisions](docs/adr/): Kokoro for speech synthesis, Dioxus for the interface
- [Specs and plans](docs/superpowers/): one pair per story
- [Releases](https://github.com/mederel/cuecard/releases): versions follow semantic versioning, computed from Conventional Commit titles
- [`AGENTS.md`](AGENTS.md): conventions for contributors and coding agents

## Development

You need a recent stable [Rust](https://rustup.rs) toolchain.

```sh
cargo run
cargo test
cargo fmt --check
cargo clippy -- -D warnings
```

The cross-platform spike lives in [`spikes/dioxus-platforms/`](spikes/dioxus-platforms/); its README says how to run it.

Pull request titles and commit messages must be [Conventional Commits](https://www.conventionalcommits.org), for example `feat: choose a character` or `fix: keep the cue visible`.

## Credits

Speech synthesis is planned with [Kokoro-82M](https://huggingface.co/hexgrad/Kokoro-82M) (Apache-2.0). Its training data includes the SIWIS French speech dataset, licensed CC BY 4.0. Scene text used in examples comes from public-domain plays, such as Molière's *Tartuffe*.

## License

Apache-2.0. See [LICENSE](LICENSE).
