#!/usr/bin/env bash
set -euo pipefail

TITLE="${1:-}"
RE='^\[(Add|Change|Fix|Remove|Refactor|Docs|Test|Chore)\] ?[^[:space:]]'

if [[ "$TITLE" =~ $RE ]]; then
  echo "OK: $TITLE"
else
  echo "::error::Invalid title PR: '$TITLE'"
  echo "Format: [Type] Description"
  echo "Types: Add, Change, Fix, Remove, Refactor, Docs, Test, Chore"
  exit 1
fi