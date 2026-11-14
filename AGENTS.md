# AGENTS.md

Guidance for AI coding agents working on Cuecard. Humans: see README.md.

## Project status

Cuecard is an open-source app that helps actors learn their lines. You import a script, pick your character, and rehearse: the app gives you the cue (the line before yours), hides your lines progressively, and checks what you said. Scripts are mostly French. Linux first.

The project is at the very beginning: one Cargo crate (`src/`), architecture decisions in `docs/adr/`, CI in `.github/workflows/ci.yml`, and the `main` branch ruleset in `.github/rulesets/`. Update this section as the layout grows.

## Build, test and format

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run
```

CI (`.github/workflows/ci.yml`) runs the first three in a job named `check`. It must pass before anything merges.

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

## Code conventions

- Rust 2024 edition, stable toolchain. Keep `cargo fmt` and clippy clean.
- Prefer clear, idiomatic code, small modules, and unit tests next to the code (`#[cfg(test)] mod tests`). Work test-first when adding behavior.
- Keep dependencies few. Before adding one, check its license: permissive only (MIT, Apache-2.0, BSD). Ask before adding anything GPL, AGPL or non-commercial.
- Everything must work offline. No network calls in tests or at runtime unless the user opts in.
- Text is French by default: handle UTF-8 properly (accents, apostrophes, guillemets). When comparing what the user said to the script, normalise case, accents and punctuation first.
- Do not modify generated files unless the task explicitly requires it.
- Never commit model weights, audio files or full copyrighted plays. Test fixtures should be short excerpts of public-domain texts.
- No secrets in the repo: it is public.

## Licensing

The repository license has not been chosen yet (MIT or Apache-2.0 are the candidates). Do not add a LICENSE file without asking.

## Working conventions

- Keep changes focused on the requested task.
- The Superpowers plugin is enabled for this repo (`.claude/settings.json`). If Claude Code says it is enabled but not installed, run `/plugin install superpowers@claude-plugins-official` once. Use its skills: brainstorm before building something new, write a plan, then test-driven development, and verify before declaring a task finished.
- Every feature has a spec and an implementation plan, both committed with it: the design in `docs/superpowers/specs/YYYY-MM-DD-<topic>-design.md` and the plan in `docs/superpowers/plans/YYYY-MM-DD-<topic>.md`. Write them before the code. Bug fixes and small chores need neither.
- Make small, reviewable commits with clear messages.
- Preserve existing user changes. Avoid destructive Git operations (no force-push, no `reset --hard` on work you did not create).

## Pull requests

- `main` is protected: no direct pushes. Work on a branch, open a pull request, and turn on auto-merge (`gh pr merge --auto --squash`). The `check` job must pass before anything merges.
- Pull request titles are Conventional Commits (`feat:`, `fix:`, `docs:`, `ci:`, `test:`, `refactor:`, `chore:`). PRs are squash-merged, so the title becomes the commit on `main`.
- Before opening a pull request, refresh the local `main` from upstream (`git fetch origin main:main`, or `git switch main && git pull --ff-only`) and base the branch on it: the ruleset requires branches to be up to date with `main`.

## Bug fixes

- A bug fix is a `fix:` commit, and its pull request title uses the same prefix.
- Prefer one commit per bug fix. The commit holds the fix and its regression test together, and the test fails before the fix.
- Name the bug in the title in plain words, for example `fix: keep the cue visible when a line is hidden`.
- The pull request description gives the behaviour seen and the behaviour expected.

## Before submitting changes

- Run the formatter, clippy and the tests.
- Report the commands that were run and any checks that could not be run.
- Document new setup steps or developer workflows in the README when you add them.
