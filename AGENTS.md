# AGENTS.md

Guidance for AI coding agents working on Cuecard. Humans: see README.md.

## What this is

Cuecard is an open-source app that helps actors learn their lines. You import a script, pick your character, and rehearse: the app gives you the cue (the line before yours), hides your lines progressively, and checks what you said. Scripts are mostly French. Linux first. The project is at the very beginning: expect a nearly empty codebase.

## Commands

```sh
cargo run
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

CI runs the last three. Run them before saying a task is done.

## Vocabulary

Use these words consistently in code and docs:

- **script**: the whole play, imported by the user.
- **character**: a role in the script. The user plays one.
- **line**: one speech by one character.
- **cue**: the line spoken just before the user's line.
- **hiding level**: how much of the user's line is shown (full, first letters, blank).

## Decisions already made

Read `docs/adr/` before changing architecture. Add a new numbered ADR for any significant decision.

- **TTS: Kokoro** (ADR 0001), behind a small interface so the engine can be swapped. Rust integration is not decided yet.
- Do not use Breeze TTS 2 (non-commercial weights, no French) or Piper (GPL engine).

## Conventions

- Rust 2024 edition, stable toolchain. Keep `cargo fmt` and clippy clean.
- Prefer small modules and unit tests next to the code (`#[cfg(test)] mod tests`). Work test-first when adding behavior.
- Keep dependencies few. Before adding one, check its license: permissive only (MIT, Apache-2.0, BSD). Ask before adding anything GPL, AGPL or non-commercial.
- Everything must work offline. No network calls in tests or at runtime unless the user opts in.
- Text is French by default: handle UTF-8 properly (accents, apostrophes, guillemets). When comparing what the user said to the script, normalise case, accents and punctuation first.
- Never commit model weights, audio files or full copyrighted plays. Test fixtures should be short excerpts of public-domain texts.
- No secrets in the repo: it is public.

## Licensing

The repository license has not been chosen yet (MIT or Apache-2.0 are the candidates). Do not add a LICENSE file without asking.

## Workflow

The Superpowers plugin is enabled for this repo (`.claude/settings.json`). If Claude Code says it is enabled but not installed, run `/plugin install superpowers@claude-plugins-official` once. Use its skills: brainstorm before building something new, write a plan, then test-driven development, and verify before declaring a task finished. Commit in small steps with clear messages.
