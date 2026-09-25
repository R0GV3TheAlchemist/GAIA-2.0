#!/usr/bin/env bash
# scripts/agent-validate.sh
#
# Run workspace validation for the GAIA-2.0 human-gated correction loop.
#
# Usage:
#   agent-validate.sh <mode> [--attempt N] [--fingerprint <prior-result.json>]
#
# Modes:
#   changed   — fmt + clippy + test (default, fastest)
#   targeted  — fmt + clippy + test (same as changed; targeted file list TBD)
#   full      — fmt + clippy + test across the whole workspace
#
# Exit codes:
#   0  passed       — all stages green
#   1  failed       — one or more stages failed
#   2  usage error  — bad arguments
#   3  no_progress  — failure fingerprints identical to prior attempt
#
# Output:
#   Writes agent-validation.json to the workspace root.
#   Writes agent-validation-N.json when --attempt N is supplied.

set -euo pipefail

# ── argument parsing ──────────────────────────────────────────────────────────

MODE="${1:-changed}"
shift || true

ATTEMPT=1
PRIOR_RESULT=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --attempt)
      ATTEMPT="$2"; shift 2 ;;
    --fingerprint)
      PRIOR_RESULT="$2"; shift 2 ;;
    *)
      echo "Unknown argument: $1" >&2; exit 2 ;;
  esac
done

case "$MODE" in
  changed|targeted|full) ;;
  *) echo "Unknown mode: $MODE (expected changed, targeted, or full)" >&2; exit 2 ;;
esac

# ── setup ─────────────────────────────────────────────────────────────────────

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]:-$0}")/.." && pwd)"
cd "$ROOT"

RESULT_FILE="$ROOT/agent-validation.json"
NUMBERED_FILE="$ROOT/agent-validation-${ATTEMPT}.json"
TIMESTAMP="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
HEAD_SHA="$(git rev-parse HEAD 2>/dev/null || echo unknown)"

FAILED_STAGE=""
STATUS="passed"
NO_PROGRESS=false
STAGES_JSON="[]"
DIAGNOSTICS_JSON="[]"

# ── fallback trap — guarantees agent-validation.json always exists ────────────
#
# Registered after ROOT, ATTEMPT, and HEAD_SHA are set so the function
# can reference them. Fires on ERR (unexpected non-zero command) or on
# EXIT before the normal write block has run.

write_fallback_result() {
  # No-op if the normal result block already wrote the file
  [[ -f "$RESULT_FILE" ]] && return 0
  local ts
  ts="$(date -u +%Y-%m-%dT%H:%M:%SZ 2>/dev/null || echo unknown)"
  cat > "$RESULT_FILE" <<FALLBACK
{
  "schema_version": "1.1",
  "head_sha": "$HEAD_SHA",
  "timestamp": "$ts",
  "mode": "$MODE",
  "attempt": $ATTEMPT,
  "status": "failed",
  "failed_stage": "script-error",
  "no_progress": false,
  "stages": [],
  "diagnostics": [{"level":"error","code":"script-error","path":"","message":"agent-validate.sh exited unexpectedly before writing result","fingerprint":"script-error:unexpected-exit"}]
}
FALLBACK
  cp "$RESULT_FILE" "$NUMBERED_FILE" 2>/dev/null || true
}

trap 'write_fallback_result' ERR EXIT

# ── helpers ───────────────────────────────────────────────────────────────────

stage_result() {
  local name="$1" status="$2" exit_code="$3" duration="$4"
  printf '{"name":"%s","status":"%s","exit_code":%d,"duration_s":%d}' \
    "$name" "$status" "$exit_code" "$duration"
}

json_escape() {
  # minimal escaping sufficient for compiler messages
  printf '%s' "$1" \
    | sed 's/\\/\\\\/g' \
    | sed 's/"/\\"/g' \
    | sed 's/$/\\n/' \
    | tr -d '\n' \
    | sed 's/\\n$//'
}

# ── stage: cargo fmt ─────────────────────────────────────────────────────────

STAGE_STAGES=()

FMT_START=$(date +%s)
cargo fmt --all -- --check > /tmp/gaia-fmt.out 2>&1 || FMT_EXIT=$?
FMT_EXIT=${FMT_EXIT:-0}
FMT_DUR=$(( $(date +%s) - FMT_START ))

if [[ $FMT_EXIT -ne 0 ]]; then
  STATUS="failed"
  FAILED_STAGE="cargo-fmt"
  FMT_MSG="$(json_escape "$(head -40 /tmp/gaia-fmt.out)")"
  DIAGNOSTICS_JSON=$(printf '[{"level":"error","code":"fmt","path":"","message":"%s","fingerprint":"fmt:%s"}]' \
    "$FMT_MSG" "$HEAD_SHA")
fi
STAGE_STAGES+=("$(stage_result cargo-fmt "$([ $FMT_EXIT -eq 0 ] && echo passed || echo failed)" $FMT_EXIT $FMT_DUR)")

# ── stage: cargo clippy ───────────────────────────────────────────────────────

if [[ "$STATUS" == "passed" ]]; then
  CLIPPY_START=$(date +%s)
  cargo clippy --workspace --exclude gaia-cli --exclude gaia-agents \
    --message-format=json -- -D warnings \
    > /tmp/gaia-clippy.json 2>/tmp/gaia-clippy.err || true
  CLIPPY_EXIT=${PIPESTATUS[0]:-$?}
  CLIPPY_DUR=$(( $(date +%s) - CLIPPY_START ))

  # Extract error diagnostics from JSON output
  CLIPPY_DIAGS=$(grep '"level":"error"' /tmp/gaia-clippy.json \
    | python3 -c "
import sys, json
diags = []
for line in sys.stdin:
    try:
        msg = json.loads(line)
        if msg.get('reason') == 'compiler-message':
            m = msg.get('message', {})
            if m.get('level') == 'error':
                spans = m.get('spans', [{}])
                primary = next((s for s in spans if s.get('is_primary')), spans[0] if spans else {})
                path = primary.get('file_name', '')
                line_no = primary.get('line_start', 0)
                code = (m.get('code') or {}).get('code', '')
                text = m.get('rendered', m.get('message', ''))
                fp = f\"{code}:{path}:{line_no}\"
                diags.append({'level':'error','code':code,'path':f\"{path}:{line_no}\",'message':text[:200],'fingerprint':fp})
    except Exception:
        pass
print(json.dumps(diags))
" 2>/dev/null || echo "[]")

  if [[ "$CLIPPY_EXIT" -ne 0 ]]; then
    STATUS="failed"
    FAILED_STAGE="cargo-clippy"
    DIAGNOSTICS_JSON="$CLIPPY_DIAGS"
  fi
  STAGE_STAGES+=("$(stage_result cargo-clippy "$([ $CLIPPY_EXIT -eq 0 ] && echo passed || echo failed)" $CLIPPY_EXIT $CLIPPY_DUR)")
fi

# ── stage: cargo test ─────────────────────────────────────────────────────────

if [[ "$STATUS" == "passed" ]]; then
  TEST_START=$(date +%s)
  cargo test --workspace --exclude gaia-cli --exclude gaia-agents \
    -- --test-threads=4 > /tmp/gaia-test.out 2>&1 || TEST_EXIT=$?
  TEST_EXIT=${TEST_EXIT:-0}
  TEST_DUR=$(( $(date +%s) - TEST_START ))

  if [[ $TEST_EXIT -ne 0 ]]; then
    STATUS="failed"
    FAILED_STAGE="cargo-test"
    TEST_MSG="$(json_escape "$(grep -E 'FAILED|error\[' /tmp/gaia-test.out | head -20)")"
    DIAGNOSTICS_JSON=$(printf '[{"level":"error","code":"test","path":"","message":"%s","fingerprint":"test-fail:%s"}]' \
      "$TEST_MSG" "$HEAD_SHA")
  fi
  STAGE_STAGES+=("$(stage_result cargo-test "$([ $TEST_EXIT -eq 0 ] && echo passed || echo failed)" $TEST_EXIT $TEST_DUR)")
fi

# ── no-progress detection ─────────────────────────────────────────────────────

if [[ "$STATUS" == "failed" && -n "$PRIOR_RESULT" && -f "$PRIOR_RESULT" ]]; then
  PRIOR_FPS=$(python3 -c "
import sys, json
try:
    data = json.load(open('$PRIOR_RESULT'))
    fps = [d.get('fingerprint','') for d in data.get('diagnostics',[])]
    print(','.join(sorted(fps)))
except Exception:
    print('')
" 2>/dev/null || echo "")

  CUR_FPS=$(echo "$DIAGNOSTICS_JSON" | python3 -c "
import sys, json
try:
    data = json.load(sys.stdin)
    fps = [d.get('fingerprint','') for d in data]
    print(','.join(sorted(fps)))
except Exception:
    print('')
" 2>/dev/null || echo "")

  if [[ -n "$PRIOR_FPS" && "$PRIOR_FPS" == "$CUR_FPS" ]]; then
    STATUS="no_progress"
    NO_PROGRESS=true
  fi
fi

# ── assemble stages JSON array ────────────────────────────────────────────────

STAGES_JSON="[$(IFS=,; echo "${STAGE_STAGES[*]}")]"

# ── write result ──────────────────────────────────────────────────────────────
#
# Disable the trap before writing so it does not fire on the EXIT
# that follows the normal successful completion path.

trap - ERR EXIT

FAILED_STAGE_JSON="null"
[[ -n "$FAILED_STAGE" ]] && FAILED_STAGE_JSON="\"$FAILED_STAGE\""

cat > "$RESULT_FILE" <<EOF
{
  "schema_version": "1.1",
  "head_sha": "$HEAD_SHA",
  "timestamp": "$TIMESTAMP",
  "mode": "$MODE",
  "attempt": $ATTEMPT,
  "status": "$STATUS",
  "failed_stage": $FAILED_STAGE_JSON,
  "no_progress": $NO_PROGRESS,
  "stages": $STAGES_JSON,
  "diagnostics": $DIAGNOSTICS_JSON
}
EOF

cp "$RESULT_FILE" "$NUMBERED_FILE"

# ── exit ──────────────────────────────────────────────────────────────────────

case "$STATUS" in
  passed)      exit 0 ;;
  no_progress) exit 3 ;;
  *)           exit 1 ;;
esac
