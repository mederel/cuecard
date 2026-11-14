#!/usr/bin/env bash
# Checks that each argument is a Conventional Commit subject:
#   <type>[(scope)][!]: <description>
#
# Usage: check-conventional.sh "feat: add x" "fix(ui): y"
# Subjects come from the caller as arguments (never interpolated into code),
# because pull request titles are untrusted input.
set -uo pipefail

types='feat|fix|docs|ci|test|refactor|chore|perf|build|style|revert'
pattern="^(${types})(\\([a-z0-9._/-]+\\))?!?: [^ ].*$"
max=100
status=0

if [ "$#" -eq 0 ]; then
  echo "usage: $0 <subject>..." >&2
  exit 2
fi

for subject in "$@"; do
  if [[ ! "$subject" =~ $pattern ]]; then
    echo "::error::Not a Conventional Commit: ${subject}"
    status=1
  elif [ "${#subject}" -gt "$max" ]; then
    echo "::error::Longer than ${max} characters: ${subject}"
    status=1
  else
    echo "ok: ${subject}"
  fi
done

if [ "$status" -ne 0 ]; then
  cat >&2 <<EOF

Expected: <type>[(scope)][!]: <description>
Types: ${types//|/, }
Example: fix: keep the cue visible when a line is hidden
EOF
fi
exit "$status"
