#!/usr/bin/env bash
# =============================================================================
# GAIA-2.0 agent-validate — machine-readable validation for AI-assisted
# correction loops.
#
# Runs the same checks as CI and emits a structured JSON result so an AI
# assistant can read exactly what failed, propose a targeted repair, and
# request human approval before applying it.
#
# Usage:
#   bash scripts/agent-validate.sh [MODE] [--attempt N] [--fingerprint FILE]
#
#   MODE        full (default) | changed | targeted
#
#   --attempt N
#               Record attempt number N in agent-validation.json (1-indexed).
#               Used by the correction loop to track and enforce the
#               max-attempt safety limit.
#
#   --fingerprint FILE
#               Path to a JSON file containing a previous diagnostics array
#               (e.g. the "diagnostics" field of the prior run's
#               agent-validation.json).  If every fingerprint in the new
#               result already appeared in that file the script exits with
#               code 3 (NO_PROGRESS) — the loop must stop and escalate.
#
# Output:
#   agent-validation.json    written at repo root (gitignored)
#   stdout                   human-readable progress (mirrors preflight.sh)
#
# Exit codes:
#   0  all stages passed
#   1  one or more stages failed (new failures; retry is permitted)
#   2  usage error
#   3  no progress — all failure fingerprints identical to previous attempt;
#                    the correction loop MUST escalate instead of retrying
#
# Safety contract (GAIA-2.0 governance):
#   - Read-only on the working tree — never modifies source files.
#   - Writes ONLY agent-validation.json and /tmp/av-*.json scratch files.
#   - Every result is SHA-bound; a stale result (wrong head_sha) MUST be
#     rejected by the consumer before any repair is proposed.
#   - Max repair attempts enforced by the caller, not this script.
#   - exit 3 is a hard stop for the correction loop — do not retry.
# =============================================================================
set -euo pipefail

# ── colours (same palette as preflight.sh) ───────────────────────────────────
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
CYAN='\033[0;36m'; BOLD='\033[1m'; RESET='\033[0m'

pass()  { echo -e "  ${GREEN}✓${RESET}  $1"; }
fail_()  { echo -e "  ${RED}✗${RESET}  $1"; }
warn()  { echo -e "  ${YELLOW}!${RESET}  $1"; }
head_() { echo -e "\n${BOLD}${CYAN}▶ $1${RESET}"; }

# ── argument parsing ──────────────────────────────────────────────────────────
# Supports:
#   agent-validate.sh [full|changed|targeted] [--attempt N] [--fingerprint FILE]
# All flags are optional and order-independent (mode must come first if present).
MODE="full"
ATTEMPT=1
FINGERPRINT_FILE=""

# First positional arg may be the mode
if [[ $# -gt 0 && "$1" != --* ]]; then
  MODE="$1"
  shift
fi

while [[ $# -gt 0 ]]; do
  case "$1" in
    --attempt)
      [[ $# -lt 2 ]] && { echo "--attempt requires a value" >&2; exit 2; }
      ATTEMPT="$2"; shift 2
      # Validate numeric
      [[ "$ATTEMPT" =~ ^[0-9]+$ ]] || { echo "--attempt must be a positive integer" >&2; exit 2; }
      ;;
    --fingerprint)
      [[ $# -lt 2 ]] && { echo "--fingerprint requires a file path" >&2; exit 2; }
      FINGERPRINT_FILE="$2"; shift 2
      [[ -f "$FINGERPRINT_FILE" ]] || { echo "--fingerprint file not found: $FINGERPRINT_FILE" >&2; exit 2; }
      ;;
    *)
      echo "Unknown option: $1" >&2
      echo "Usage: $0 [full|changed|targeted] [--attempt N] [--fingerprint FILE]" >&2
      exit 2
      ;;
  esac
done

case "$MODE" in
  full|changed|targeted) ;;
  *)
    echo "Usage: $0 [full|changed|targeted] [--attempt N] [--fingerprint FILE]" >&2
    exit 2
    ;;
esac

# Schema version: 1.1 when new flags are in use, else 1.0 for back-compat
SCHEMA_VERSION="1.0"
if [[ $ATTEMPT -ne 1 || -n "$FINGERPRINT_FILE" ]]; then
  SCHEMA_VERSION="1.1"
fi

# ── state ────────────────────────────────────────────────────────────────────
HEAD_SHA=$(git rev-parse HEAD 2>/dev/null || echo "unknown")
TIMESTAMP=$(date -u '+%Y-%m-%dT%H:%M:%SZ')
RESULT_FILE="agent-validation.json"
SCRATCH="/tmp/av-cargo-check.json"
FAILURES=0
FAILED_STAGE=""
STAGES_JSON=""

echo -e "${BOLD}GAIA-2.0 agent-validate [mode: $MODE | attempt: $ATTEMPT]${RESET}"
echo    "$TIMESTAMP  sha: $HEAD_SHA"
[[ -n "$FINGERPRINT_FILE" ]] && echo "  Prior fingerprints: $FINGERPRINT_FILE"

# ── stage runner ─────────────────────────────────────────────────────────────
run_stage() {
  local name="$1" cmd="$2"
  local start exit_code=0 duration
  head_ "$name"
  start=$(date +%s)
  eval "$cmd" 2>&1 && exit_code=0 || exit_code=$?
  duration=$(( $(date +%s) - start ))

  local status="passed"
  if [[ $exit_code -ne 0 ]]; then
    status="failed"
    FAILURES=$(( FAILURES + 1 ))
    [[ -z "$FAILED_STAGE" ]] && FAILED_STAGE="$name"
    fail_ "$name (exit $exit_code)"
  else
    pass "$name"
  fi

  STAGES_JSON+=$(printf '{"name":"%s","status":"%s","exit_code":%d,"duration_s":%d}' \
    "$name" "$status" "$exit_code" "$duration")
  STAGES_JSON+=$'\n'
}

# ── stages ───────────────────────────────────────────────────────────────────

# Stage 1 — cargo check (always; fastest signal; JSON output captured for diagnostics)
run_stage "cargo-check" \
  "cargo check --workspace --message-format=json 2>&1 | tee $SCRATCH"

# Stage 2 — cargo fmt check
if [[ "$MODE" == "full" || "$MODE" == "changed" ]]; then
  run_stage "cargo-fmt" "cargo fmt --all --check"
fi

# Stage 3 — cargo clippy (warnings as errors)
if [[ "$MODE" == "full" || "$MODE" == "changed" ]]; then
  run_stage "cargo-clippy" \
    "cargo clippy --workspace --lib -- -D warnings"
fi

# Stage 4 — rust workspace tests (mirrors CI rust-workspace job)
run_stage "rust-workspace" \
  "cargo test --workspace --exclude gaia-cli"

# Stage 5 — gaia-cli integration tests (mirrors CI rust-cli-integration job)
if [[ "$MODE" == "full" ]]; then
  run_stage "rust-cli-integration" \
    "cargo build -p gaia-cli && cargo test -p gaia-cli"
fi

# Stage 6 — spec/schema validation
if [[ "$MODE" == "full" || "$MODE" == "changed" ]]; then
  run_stage "spec-schemas" \
    "python3 gaia-spec/tools/validate_aip.py && python3 gaia-spec/tools/validate_identity.py"
  run_stage "claim-tags" \
    "python3 gaia-spec/tools/check_claim_tags.py"
fi

# Stage 7 — markdown presence check
if [[ "$MODE" == "full" ]]; then
  run_stage "markdown" \
    "test -f README.md && test -f gaia-spec/README.md && test -f CONTRIBUTING.md && test -f SECURITY.md"
fi

# ── extract diagnostics from cargo check JSON ─────────────────────────────────
DIAGNOSTICS_JSON="[]"
if [[ -f "$SCRATCH" ]]; then
  DIAGNOSTICS_JSON=$(python3 - <<'PYEOF'
import json, sys

entries = []
try:
    with open("/tmp/av-cargo-check.json") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            try:
                msg = json.loads(line)
            except Exception:
                continue
            if msg.get("reason") != "compiler-message":
                continue
            m   = msg.get("message", {})
            lvl = m.get("level", "")
            if lvl not in ("error", "warning"):
                continue
            spans = m.get("spans", [])
            loc   = ""
            if spans:
                s   = spans[0]
                loc = f"{s.get('file_name','')}:{s.get('line_start','')}"
            code = (m.get("code") or {}).get("code", "")
            text = m.get("message", "")[:200]
            fp   = f"{code}:{loc}"
            entries.append({
                "level":       lvl,
                "code":        code,
                "path":        loc,
                "message":     text,
                "fingerprint": fp
            })
except Exception:
    pass

print(json.dumps(entries))
PYEOF
)
fi

# ── no-progress detection ─────────────────────────────────────────────────────
# If a prior fingerprint file was provided, check whether this run produced
# any new failure fingerprints.  If not, exit 3 so the loop stops.
NO_PROGRESS=0
if [[ -n "$FINGERPRINT_FILE" && $FAILURES -gt 0 ]]; then
  NO_PROGRESS=$(python3 - <<PYEOF
import json, sys

def load_fps(path):
    try:
        with open(path) as f:
            data = json.load(f)
        # Accept either a diagnostics array or a full agent-validation.json
        if isinstance(data, list):
            return {e.get("fingerprint", "") for e in data if e.get("fingerprint")}
        elif isinstance(data, dict) and "diagnostics" in data:
            return {e.get("fingerprint", "") for e in data["diagnostics"] if e.get("fingerprint")}
    except Exception:
        pass
    return set()

prev_fps = load_fps("$FINGERPRINT_FILE")

try:
    current = json.loads('''$DIAGNOSTICS_JSON''')
except Exception:
    current = []

current_fps = {e.get("fingerprint", "") for e in current if e.get("fingerprint")}

# No progress if every current fingerprint was already in the previous set
# and both sets are non-empty
if current_fps and prev_fps and current_fps.issubset(prev_fps):
    print(1)
else:
    print(0)
PYEOF
)
fi

# ── build stages array ────────────────────────────────────────────────────────
STAGES_ARRAY=$(printf '%s' "$STAGES_JSON" | python3 -c "
import sys, json
lines = [l.strip() for l in sys.stdin if l.strip()]
objs  = [json.loads(l) for l in lines]
print(json.dumps(objs))
")

# ── determine overall status ──────────────────────────────────────────────────
OVERALL="passed"
[[ $FAILURES -gt 0 ]] && OVERALL="failed"
[[ $NO_PROGRESS -eq 1 ]] && OVERALL="no_progress"

# ── write agent-validation.json ───────────────────────────────────────────────
python3 - <<PYEOF
import json

result = {
    "schema_version": "$SCHEMA_VERSION",
    "head_sha":        "${HEAD_SHA}",
    "timestamp":       "${TIMESTAMP}",
    "mode":            "${MODE}",
    "attempt":         ${ATTEMPT},
    "status":          "${OVERALL}",
    "failed_stage":    "${FAILED_STAGE}" or None,
    "no_progress":     bool(${NO_PROGRESS}),
    "stages":          ${STAGES_ARRAY},
    "diagnostics":     ${DIAGNOSTICS_JSON}
}

with open("${RESULT_FILE}", "w") as f:
    json.dump(result, f, indent=2)

print(f"\nWrote {len(json.dumps(result))} bytes → ${RESULT_FILE}")
PYEOF

# ── summary ───────────────────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}─────────────────────────────────────────${RESET}"
if [[ $NO_PROGRESS -eq 1 ]]; then
  echo -e "${RED}${BOLD}⊘ NO PROGRESS — all failure fingerprints identical to previous attempt.${RESET}"
  echo -e "  The correction loop MUST escalate to a human. Do not retry."
  echo -e "  status written → ${RESULT_FILE}  (status: no_progress)"
  exit 3
elif [[ $FAILURES -eq 0 ]]; then
  echo -e "${GREEN}${BOLD}✓ All stages passed — agent-validation.json is green.${RESET}"
else
  echo -e "${RED}${BOLD}✗ $FAILURES stage(s) failed.  First failure: ${FAILED_STAGE}${RESET}"
  echo -e "  Read agent-validation.json for structured diagnostics."
  exit 1
fi
