#!/usr/bin/env bash
# Tests next-version.sh against throwaway git repositories.
set -euo pipefail

script="$(cd "$(dirname "$0")" && pwd)/next-version.sh"
failures=0

new_repo() {
  dir=$(mktemp -d)
  cd "$dir"
  git init -q -b main
  git config user.name test
  git config user.email test@example.com
  printf '[package]\nname = "x"\nversion = "0.1.0"\n' > Cargo.toml
  git add Cargo.toml
  git commit -q -m "chore: start"
}

commit() { git commit -q --allow-empty -m "$1"; }
tag() { git tag "$1"; }

expect() {
  local name="$1" want_release="$2" want_version="$3" want_bump="$4" out
  out=$(bash "$script")
  local got="release=$(sed -n 's/^release=//p' <<<"$out") version=$(sed -n 's/^version=//p' <<<"$out") bump=$(sed -n 's/^bump=//p' <<<"$out")"
  local want="release=${want_release} version=${want_version} bump=${want_bump}"
  if [ "$got" = "$want" ]; then
    echo "ok: $name"
  else
    echo "FAIL: $name"; echo "  want: $want"; echo "  got:  $got"
    failures=$((failures + 1))
  fi
}

(new_repo; commit "docs: add a page"; commit "ci: tweak"
 expect "no tag, nothing releasable" false "" none)
(new_repo; commit "feat: first feature"
 expect "no tag, first release is the Cargo.toml version" true 0.1.0 initial)
(new_repo; commit "docs: x"; tag v1.2.3
 expect "only docs since the tag" false "" none)
(new_repo; tag v1.2.3; commit "fix: a bug"
 expect "fix is a patch" true 1.2.4 patch)
(new_repo; tag v1.2.3; commit "perf: faster"
 expect "perf is a patch" true 1.2.4 patch)
(new_repo; tag v1.2.3; commit "feat: a feature"
 expect "feat is a minor" true 1.3.0 minor)
(new_repo; tag v1.2.3; commit "feat(ui): scoped (#12)"
 expect "scope and PR number are fine" true 1.3.0 minor)
(new_repo; tag v1.2.3; commit "fix: a"; commit "feat: b"; commit "docs: c"
 expect "highest bump wins" true 1.3.0 minor)
(new_repo; tag v1.2.3; commit "feat!: break it"
 expect "feat! is a major" true 2.0.0 major)
(new_repo; tag v1.2.3; commit "fix(ui)!: break it"
 expect "scoped fix! is a major" true 2.0.0 major)
(new_repo; tag v1.2.3; commit "refactor!: rename"
 expect "any type with ! is a major" true 2.0.0 major)
(new_repo; tag v1.2.3; git commit -q --allow-empty -m "feat: x" -m "BREAKING CHANGE: the format changed"
 expect "BREAKING CHANGE footer is a major" true 2.0.0 major)
(new_repo; tag v1.2.3; git commit -q --allow-empty -m "fix: x" -m "This is not BREAKING CHANGE: just prose"
 expect "BREAKING CHANGE mid-line is ignored" true 1.2.4 patch)
(new_repo; tag v1.2.3; commit "feat: a"; commit "fix!: b"
 expect "major beats minor" true 2.0.0 major)
(new_repo; tag v1.2.3; tag v2.0.0-rc.1; commit "fix: x"
 expect "pre-release tags are ignored" true 1.2.4 patch)
(new_repo; tag v1.2.3; tag v1.10.0; commit "fix: x"
 expect "tags sort numerically" true 1.10.1 patch)
(new_repo; tag v0.3.0; commit "feat!: first breaking change"
 expect "0.x follows the same rules" true 1.0.0 major)
(new_repo; tag v1.2.3; commit "feature: not conventional"; commit "Update stuff"
 expect "unknown subjects release nothing" false "" none)

if [ "$failures" -ne 0 ]; then
  echo "$failures test(s) failed" >&2
  exit 1
fi
echo "all next-version tests passed"
