#!/usr/bin/env bash
# Computes the next semantic version from the Conventional Commits made since
# the latest release tag (vX.Y.Z), and prints key=value lines:
#   release=true|false   is there anything to release?
#   version=X.Y.Z        the next version (empty when release=false)
#   bump=initial|major|minor|patch|none
#   previous=vX.Y.Z|none the tag the commits were counted from
#
# Rules, highest wins:
#   major  a "!" before the colon in a subject (feat!:, fix(ui)!:), or a
#          "BREAKING CHANGE:" footer line in a message body
#   minor  feat
#   patch  fix, perf, revert
#   none   anything else (docs, ci, test, chore, style, build, refactor)
# With no release tag yet, the first release is the version in Cargo.toml (or
# BASE_VERSION), as soon as one releasable commit exists.
set -euo pipefail

base_version="${BASE_VERSION:-}"
if [ -z "$base_version" ]; then
  base_version=$(sed -n 's/^version *= *"\([0-9]*\.[0-9]*\.[0-9]*\)".*/\1/p' Cargo.toml | head -n 1)
fi
: "${base_version:?no version in Cargo.toml and no BASE_VERSION}"

# Highest plain vX.Y.Z tag reachable from HEAD (pre-release tags are ignored).
last=$(git tag --merged HEAD --list 'v*' --sort=-v:refname \
  | grep -E '^v[0-9]+\.[0-9]+\.[0-9]+$' | head -n 1 || true)
range="HEAD"
[ -n "$last" ] && range="${last}..HEAD"

bump="none"
raise() {
  case "$1:$bump" in
    major:*) bump=major ;;
    minor:major) ;;
    minor:*) bump=minor ;;
    patch:none) bump=patch ;;
  esac
}

while IFS= read -r subject; do
  if [[ "$subject" =~ ^[a-z]+(\([^\)]*\))?!: ]]; then
    raise major
  elif [[ "$subject" =~ ^feat(\([^\)]*\))?: ]]; then
    raise minor
  elif [[ "$subject" =~ ^(fix|perf|revert)(\([^\)]*\))?: ]]; then
    raise patch
  fi
done < <(git log --no-merges --format=%s "$range")

if git log --no-merges --format=%b "$range" | grep -Eq '^BREAKING[ -]CHANGE: '; then
  raise major
fi

release=true
version=""
if [ "$bump" = "none" ]; then
  release=false
elif [ -z "$last" ]; then
  version="$base_version"
  bump="initial"
else
  IFS=. read -r major minor patch <<<"${last#v}"
  case "$bump" in
    major) version="$((major + 1)).0.0" ;;
    minor) version="${major}.$((minor + 1)).0" ;;
    patch) version="${major}.${minor}.$((patch + 1))" ;;
  esac
fi

echo "release=${release}"
echo "version=${version}"
echo "bump=${bump}"
echo "previous=${last:-none}"
