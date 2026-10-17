#!/usr/bin/env bash
# Renumber the prompt files once the deck's slide order is known.
#
# Write the mapping into map.tsv, one line per file, old number then new number:
#
#   15.1<TAB>14.1
#   16.1<TAB>15.1
#   17.1<TAB>16.1
#   48.1<TAB>41.2
#
# Then run ./renumber.sh. Files are moved with git mv, so the history follows.
# The script stops on the first problem and changes nothing until the whole map is valid.

set -euo pipefail
cd "$(dirname "$0")"

MAP=${1:-map.tsv}
[ -f "$MAP" ] || { echo "no $MAP — see the comment at the top of this script" >&2; exit 2; }

# 1. validate everything first
while IFS=$'\t' read -r old new rest; do
  [ -z "${old:-}" ] && continue
  case "$old" in \#*) continue ;; esac
  [ -n "${new:-}" ] || { echo "line for $old has no new number" >&2; exit 2; }
  src=$(ls "$old"-*.md 2>/dev/null | head -1) || true
  [ -n "$src" ] || { echo "no file for $old" >&2; exit 2; }
  slug=${src#"$old"-}
  echo "  $src  ->  $new-$slug"
done < "$MAP"

echo
read -r -p "apply? [y/N] " ok
[ "$ok" = y ] || { echo "nothing changed"; exit 0; }

# 2. then move
while IFS=$'\t' read -r old new rest; do
  [ -z "${old:-}" ] && continue
  case "$old" in \#*) continue ;; esac
  src=$(ls "$old"-*.md 2>/dev/null | head -1)
  git mv "$src" "$new-${src#"$old"-}"
done < "$MAP"

echo "done. Update the table in README.md."
