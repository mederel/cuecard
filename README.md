# Cuecard

An open-source app to help actors learn their lines.

Import a script, pick your character, and rehearse: cues from your scene partners, your lines fading as you learn them, and a check on what you said.

> Early days. Nothing to run yet.

## Planned

- Import a script and choose your character
- Cue mode: the app gives the line before yours
- Progressive hiding: full text, first letters, blank
- Speech check against the script
- Spaced repetition for the shaky lines

## Development

You need a recent stable [Rust](https://rustup.rs) toolchain.

```sh
cargo run
cargo test
cargo fmt --check
cargo clippy -- -D warnings
```

## License

Apache-2.0. See [LICENSE](LICENSE).
