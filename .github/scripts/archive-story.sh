#!/usr/bin/env bash
# Writes docs/stories/<number>-<slug>.md from a closed story issue.
#
# Everything comes from environment variables, never from inline workflow
# expressions, because issue titles and bodies are untrusted input.
#
# Required: ISSUE_NUMBER ISSUE_TITLE ISSUE_URL
# Optional: ISSUE_BODY STATE_REASON CLOSED_AT CLOSING_PR OUT_DIR
set -euo pipefail

: "${ISSUE_NUMBER:?missing ISSUE_NUMBER}"
: "${ISSUE_TITLE:?missing ISSUE_TITLE}"
: "${ISSUE_URL:?missing ISSUE_URL}"
ISSUE_BODY="${ISSUE_BODY:-}"
STATE_REASON="${STATE_REASON:-completed}"
CLOSED_AT="${CLOSED_AT:-}"
CLOSING_PR="${CLOSING_PR:-}"
OUT_DIR="${OUT_DIR:-docs/stories}"

case "$ISSUE_NUMBER" in
  '' | *[!0-9]*) echo "ISSUE_NUMBER must be a number" >&2; exit 1 ;;
esac

# ASCII slug: strip accents, lowercase, collapse everything else to dashes.
slug=$(python3 -I -c '
import re, sys, unicodedata
s = unicodedata.normalize("NFKD", sys.argv[1])
s = "".join(c for c in s if not unicodedata.combining(c)).lower()
s = re.sub(r"[^a-z0-9]+", "-", s).strip("-")[:50].strip("-")
print(s or "story")
' "$ISSUE_TITLE")

mkdir -p "$OUT_DIR"
# A re-closed issue may have a new title, so drop any earlier file for it.
rm -f "$OUT_DIR/${ISSUE_NUMBER}-"*.md
file="$OUT_DIR/${ISSUE_NUMBER}-${slug}.md"

reason="completed"
if [ "$STATE_REASON" = "not_planned" ]; then
  reason="not planned"
fi

{
  printf '# %s\n\n' "$ISSUE_TITLE"
  printf -- '- Issue: %s\n' "$ISSUE_URL"
  if [ -n "$CLOSED_AT" ]; then
    printf -- '- Closed: %s (%s)\n' "${CLOSED_AT%%T*}" "$reason"
  else
    printf -- '- Closed: %s\n' "$reason"
  fi
  if [ -n "$CLOSING_PR" ]; then
    printf -- '- Closing pull request: #%s\n' "$CLOSING_PR"
  fi
  printf '\n---\n\n%s\n' "${ISSUE_BODY//$'\r'/}"
} > "$file"

echo "$file"
