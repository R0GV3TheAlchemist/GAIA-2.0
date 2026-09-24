#!/usr/bin/env bash
# =============================================================================
# GAIA-2.0 local pre-flight — run this before every push / PR
#
# Replicates every check that runs in CI so failures are caught locally
# in seconds rather than after a 2-4 minute CI round-trip.
#
# Usage:
#   bash scripts/preflight.sh          # full check
#   bash scripts/preflight.sh --quick  # skip tarpaulin (fastest)
#
# Requirements: cargo, cargo-tarpaulin (optional), ripgrep (optional)
# Install tarpaulin once:  cargo install cargo-tarpaulin
# =============================================================================
set -euo pipefail

# ── colours ──────────────────────────────────────────────────────────────────
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
CYAN='\033[0;36m'; BOLD='\033[1m'; RESET='\033[0m'

pass() { echo -e "  ${GREEN}✓${RESET}  $1"; }
fail() { echo -e "  ${RED}✗${RESET}  $1"; FAILURES=$((FAILURES+1)); }
warn() { echo -e "  ${YELLOW}!${RESET}  $1"; }
head_() { echo -e "\n${BOLD}${CYAN}▶ $1${RESET}"; }

FAILURES=0
QUICK=false
[[ "${1:-}" == "--quick" ]] && QUICK=true

echo -e "${BOLD}GAIA-2.0 pre-flight$(${QUICK} && echo ' [quick]')${RESET}"
echo    "$(date '+%Y-%m-%d %H:%M:%S')  branch: $(git branch --show-current 2>/dev/null || echo unknown)"

# =============================================================================
# 1. CARGO CHECK — fastest compile gate; catches missing types, wrong field
#    names, unresolved imports.  This is the #1 source of CI failures.
# =============================================================================
head_ "cargo check --workspace"
if cargo check --workspace --message-format=short 2>&1; then
  pass "cargo check --workspace"
else
  fail "cargo check --workspace  ← fix compile errors before anything else"
fi

# =============================================================================
# 2. UNIT TESTS (--lib only, mirrors rust-unit-tests.yml)
#    Runs only #[cfg(test)] blocks inside lib.rs — no subprocess / IO tests.
#    Fast: typically < 10 s on a warm cache.
# =============================================================================
head_ "cargo test --workspace --lib"
if cargo test --workspace --lib --no-fail-fast 2>&1; then
  pass "cargo test --workspace --lib"
else
  fail "cargo test --workspace --lib"
fi

# =============================================================================
# 3. INTEGRATION TESTS (mirrors rust-workspace CI job)
#    Includes gaia-acp/tests/ (digital_parasite, deepfake_illusionist, …)
#    and gaia-cli/tests/cli_integration.rs.
#    Note: gaia-cli tests spawn the `gaia` binary via assert_cmd — ensure
#    the binary compiled cleanly in step 1 first.
# =============================================================================
head_ "cargo test --workspace (all targets)"
if cargo test --workspace --no-fail-fast 2>&1; then
  pass "cargo test --workspace"
else
  fail "cargo test --workspace  ← check gaia-acp/tests/ and gaia-cli/tests/"
fi

# =============================================================================
# 4. CLIPPY — warnings-as-errors (mirrors rust-unit-tests.yml clippy step)
#    Catches dead code, unused imports, bare .unwrap(), needless borrows.
#    Every warning that would be noise in --check becomes a hard error here.
# =============================================================================
head_ "cargo clippy --workspace --lib -- -D warnings"
if cargo clippy --workspace --lib -- -D warnings 2>&1; then
  pass "cargo clippy"
else
  fail "cargo clippy  ← fix all warnings (they are errors in CI)"
fi

# =============================================================================
# 5. CARGO.TOML CONSISTENCY — catches the class of failure where an AI agent
#    overwrites a Cargo.toml and silently drops production dependencies.
#
#    Checks:
#      a) Every workspace member directory actually exists on disk.
#      b) gaia-cli/Cargo.toml does NOT list ratatui/crossterm/futures under
#         [dependencies] (they are unused and cause resolver conflicts).
#      c) No Cargo.toml outside the workspace root defines its own [workspace].
# =============================================================================
head_ "Cargo.toml consistency"

# 5a: all workspace members exist
ROOT_CARGO="Cargo.toml"
if command -v python3 &>/dev/null; then
  python3 - <<'PYEOF'
import subprocess, sys, re, pathlib

with open("Cargo.toml") as f:
    text = f.read()

members = re.findall(r'"([^"]+)"', text[text.find('[workspace]'):])
for m in members:
    p = pathlib.Path(m)
    if not p.exists():
        print(f"  MISSING workspace member: {m}")
        sys.exit(1)
print(f"  All {len(members)} workspace member directories present.")
PYEOF
  if [[ $? -eq 0 ]]; then pass "all workspace member dirs present"
  else fail "workspace member directory missing — check Cargo.toml [workspace]"; fi
else
  warn "python3 not found — skipping workspace-member check"
fi

# 5b: gaia-cli/Cargo.toml must NOT contain ratatui/crossterm/futures under [dependencies]
if [[ -f gaia-cli/Cargo.toml ]]; then
  # Extract only the [dependencies] block (stop at next section header)
  DEP_BLOCK=$(awk '/^\[dependencies\]/{found=1; next} found && /^\[/{found=0} found{print}' gaia-cli/Cargo.toml)
  BAD_DEPS=""
  for dep in ratatui crossterm futures; do
    if echo "$DEP_BLOCK" | grep -q "^${dep}"; then
      BAD_DEPS="$BAD_DEPS $dep"
    fi
  done
  if [[ -z "$BAD_DEPS" ]]; then
    pass "gaia-cli/Cargo.toml has no forbidden prod deps"
  else
    fail "gaia-cli/Cargo.toml lists unused prod deps:$BAD_DEPS — move to [dev-dependencies] or remove"
  fi
fi

# 5c: no nested [workspace] definitions
NESTED=$(grep -rл '^\[workspace\]' --include='Cargo.toml' . 2>/dev/null \
  | grep -v '^./Cargo.toml$' || true)
if [[ -z "$NESTED" ]]; then
  pass "no nested [workspace] definitions"
else
  fail "nested [workspace] found in: $NESTED"
fi

# =============================================================================
# 6. DEAD FIELD / MISSING FIELD GUARD — the most common AI-agent mistake:
#    writing a struct with the wrong field names against a test that calls
#    Struct { field_name: value }.
#
#    Strategy: compile the test crate in isolation with --tests so rustc
#    reports E0560 (unknown field) or E0063 (missing field) immediately.
# =============================================================================
head_ "gaia-acp test compilation (field-shape guard)"
if cargo test -p gaia-acp --no-run 2>&1; then
  pass "gaia-acp test binary compiles"
else
  fail "gaia-acp test binary failed to compile — check struct field names in claim.rs / quota.rs against tests/"
fi

# =============================================================================
# 7. TARPAULIN (optional — skipped with --quick)
#    Mirrors coverage.yml. Excluded: gaia-cli (binary crate, no lib target).
#    Only run this locally to spot coverage regressions — CI always runs it.
# =============================================================================
if ! $QUICK; then
  head_ "cargo tarpaulin --workspace --exclude gaia-cli"
  if command -v cargo-tarpaulin &>/dev/null || cargo tarpaulin --version &>/dev/null 2>&1; then
    if cargo tarpaulin --workspace --exclude gaia-cli --timeout 120 --out Stdout \
         --exclude-files 'gaia-cli/src/main.rs' -- --test-threads=1 2>&1; then
      pass "cargo tarpaulin"
    else
      fail "cargo tarpaulin"
    fi
  else
    warn "cargo-tarpaulin not installed — skipping (install: cargo install cargo-tarpaulin)"
  fi
fi

# =============================================================================
# SUMMARY
# =============================================================================
echo ""
echo -e "${BOLD}─────────────────────────────────────────${RESET}"
if [[ $FAILURES -eq 0 ]]; then
  echo -e "${GREEN}${BOLD}✓ All checks passed — safe to push.${RESET}"
else
  echo -e "${RED}${BOLD}✗ $FAILURES check(s) failed — do not push until fixed.${RESET}"
  exit 1
fi
