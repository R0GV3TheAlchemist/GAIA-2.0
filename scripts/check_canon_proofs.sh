#!/usr/bin/env bash
# Canon Proof Gate — Author: Kyle Steen (R0GV3TheAlchemist)
# C77 / THE ORDER: no canon without a proof artefact.
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT"

BASE_REF="${GITHUB_BASE_REF:-}"
CHANGED=""

if [[ -n "$BASE_REF" ]] && git rev-parse --verify "origin/${BASE_REF}" >/dev/null 2>&1; then
  CHANGED=$(git diff --name-only "origin/${BASE_REF}...HEAD" -- 'docs/canon/' || true)
elif git rev-parse --verify origin/main >/dev/null 2>&1; then
  CHANGED=$(git diff --name-only origin/main...HEAD -- 'docs/canon/' || true)
fi

if [[ -z "${CHANGED}" ]]; then
  CHANGED=$(git ls-files 'docs/canon/' || true)
fi

if [[ -z "${CHANGED}" ]]; then
  echo "No canon documents to check."
  exit 0
fi

fail=0
while IFS= read -r file; do
  [[ -z "$file" ]] && continue
  [[ -f "$file" ]] || continue
  [[ "$file" == *.md ]] || continue

  echo "Checking $file"

  if ! grep -Eq 'PROOF-[A-Z0-9][A-Z0-9_-]*' "$file"; then
    echo "  FAIL: no PROOF- id referenced"
    fail=1
    continue
  fi

  mapfile -t ids < <(grep -Eo 'PROOF-[A-Z0-9][A-Z0-9_-]*' "$file" | sort -u)
  found_proof=0
  for id in "${ids[@]}"; do
    proof_path="proofs/${id}.md"
    if [[ -f "$proof_path" ]]; then
      found_proof=1
      if ! grep -Eqi '(^##[[:space:]]*Method|\*\*Type:\*\*|\*\*Status:\*\*|\*\*Method:\*\*)' "$proof_path"; then
        echo "  FAIL: $proof_path missing Type/Status/Method"
        fail=1
      else
        echo "  OK: $id -> $proof_path"
      fi
    fi
  done

  if [[ "$found_proof" -eq 0 ]]; then
    echo "  FAIL: referenced PROOF- id has no proofs/*.md artefact"
    fail=1
  fi
done <<< "$CHANGED"

if [[ "$fail" -ne 0 ]]; then
  echo "Canon Proof Gate failed. C77: no canon without proof."
  exit 1
fi

echo "Canon Proof Gate passed."
