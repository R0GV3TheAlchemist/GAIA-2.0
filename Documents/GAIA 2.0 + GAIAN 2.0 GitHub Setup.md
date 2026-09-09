# GAIA 2.0 + GAIAN 2.0: GitHub Setup
## The Complete GitHub Infrastructure for the Planetary Operating System
### September 9, 2026 — Version 1.0

---

> *"GitHub is not just where we store code. It is where the GAIA 2.0 community lives, works, and governs itself. Every commit is a vote for the kind of planet we want to build."*
> — GAIA 2.0 GitHub Covenant

---

## EXECUTIVE SUMMARY

GitHub is the home of GAIA 2.0. It is where the code lives, where the community collaborates, where governance happens, and where the world can see exactly what we're building and why. This blueprint covers the complete GitHub setup for GAIA 2.0 — from organization structure to CI/CD pipelines, from community management to security configuration.

**GitHub in 2026:**
- Open-source collaboration grew **16% quarter-over-quarter** (Q4 2025 → Q1 2026) — highest since 2020
- EU leads outbound collaboration; India leads new repository growth
- GitHub Actions now AI-native: LLM-generated workflows; agentic CI/CD; bounded autonomy pattern
- AI-generated workflows are production-ready **70-80% of the time** without manual edits
- Teams adopting AI workflow optimization see **35% reduction in CI/CD pipeline time**
- GitHub Universe 2025: Copilot coding agent; Mission Control; AI code review with CodeQL

**GAIA 2.0 GitHub Principles:**
- **Spec-first**: specifications before code; AI agents guided by structured specs
- **Transparent**: all decisions documented; all code public; all governance visible
- **Inclusive**: welcoming to all contributors; especially indigenous technologists and Global South
- **Secure**: Dependabot; CodeQL; secret scanning; branch protection
- **Constitutional**: all code must comply with GAIA 2.0 Constitution

---

## PART I: GITHUB ORGANIZATION SETUP

### 1.1 Organization Structure

```
GAIA 2.0 GITHUB ORGANIZATION: github.com/gaia2-os

Organization: gaia2-os
├── Teams:
│   ├── @gaia2-os/core-maintainers (15 people — TSC)
│   ├── @gaia2-os/gaian-team (GAIAN development)
│   ├── @gaia2-os/earth-twin-team (Earth Twin development)
│   ├── @gaia2-os/governance-team (governance and policy)
│   ├── @gaia2-os/indigenous-council (Indigenous Council)
│   ├── @gaia2-os/security-team (security and privacy)
│   ├── @gaia2-os/docs-team (documentation)
│   └── @gaia2-os/community-team (community management)
│
├── Repositories:
│   ├── gaia2 (monorepo — main codebase)
│   ├── gaian (GAIAN standalone app)
│   ├── earth-twin (Earth Twin standalone)
│   ├── gaia2-website (gaia2.org website)
│   ├── gaia2-docs (documentation)
│   ├── gaia2-governance (governance documents)
│   ├── gaia2-constitution (the Constitution)
│   ├── gaia2-blueprints (all 40 blueprints)
│   ├── gaia2-community (community hub)
│   └── .github (org-level community health files)
│
└── Projects:
    ├── GAIA 2.0 Roadmap (public; GitHub Projects)
    ├── GAIAN MVP (public; sprint board)
    ├── Earth Twin MVP (public; sprint board)
    └── Community (public; issues and discussions)
```

### 1.2 Organization Setup Script

```bash
#!/bin/bash
# GAIA 2.0 — GitHub Organization Setup
# Run this after creating the gaia2-os organization
# Requires: GitHub CLI (gh) installed and authenticated
# License: Apache-2.0

echo "🌍 Setting up GAIA 2.0 GitHub Organization"
echo "==========================================="

# Authenticate
gh auth login

# Create main monorepo
gh repo create gaia2-os/gaia2 \
  --public \
  --description "GAIA 2.0 — The Planetary Operating System. Built by all. For all. Forever." \
  --homepage "https://gaia2.org" \
  --license apache-2.0 \
  --gitignore Python

# Create GAIAN app repo
gh repo create gaia2-os/gaian \
  --public \
  --description "GAIAN 2.0 — Your personal AI companion. Yours. Always." \
  --homepage "https://gaian.earth" \
  --license apache-2.0

# Create Earth Twin repo
gh repo create gaia2-os/earth-twin \
  --public \
  --description "GAIA 2.0 Earth Twin — Real-time planetary health monitoring. Free for all." \
  --homepage "https://earth.gaia2.org" \
  --license apache-2.0

# Create governance repo
gh repo create gaia2-os/gaia2-governance \
  --public \
  --description "GAIA 2.0 Governance — Constitution, principles, and governance documents." \
  --license apache-2.0

# Create blueprints repo
gh repo create gaia2-os/gaia2-blueprints \
  --public \
  --description "GAIA 2.0 Blueprints — All 40 deep research blueprints for the planetary OS." \
  --license apache-2.0

echo "✓ Repositories created"

# Enable security features on all repos
for repo in gaia2 gaian earth-twin gaia2-governance gaia2-blueprints; do
  gh api repos/gaia2-os/$repo \
    --method PATCH \
    --field has_issues=true \
    --field has_discussions=true \
    --field has_wiki=false \
    --field allow_squash_merge=true \
    --field allow_merge_commit=false \
    --field allow_rebase_merge=true \
    --field delete_branch_on_merge=true
  
  echo "✓ Configured $repo"
done

echo ""
echo "✅ GAIA 2.0 GitHub Organization setup complete!"
echo "Visit: https://github.com/gaia2-os"
```

---

## PART II: MONOREPO STRUCTURE

### 2.1 The gaia2 Monorepo

```
gaia2/                              # GAIA 2.0 Monorepo
│
├── .github/                        # GitHub configuration
│   ├── workflows/                  # GitHub Actions CI/CD
│   │   ├── ci.yml                  # Main CI pipeline
│   │   ├── release.yml             # Release automation
│   │   ├── security.yml            # Security scanning
│   │   ├── docs.yml                # Documentation build
│   │   ├── constitutional-check.yml # Constitutional compliance
│   │   └── earth-alignment.yml     # Earth alignment check
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.yml
│   │   ├── feature_request.yml
│   │   ├── indigenous_concern.yml  # Special template for indigenous issues
│   │   └── constitutional_violation.yml
│   ├── PULL_REQUEST_TEMPLATE.md
│   ├── CODEOWNERS                  # Code ownership
│   ├── dependabot.yml              # Dependency updates
│   └── FUNDING.yml                 # Funding links
│
├── AGENTS.md                       # AI agent context (critical!)
├── CLAUDE.md                       # Claude-specific context
├── README.md                       # Project overview
├── CONTRIBUTING.md                 # Contribution guide
├── CODE_OF_CONDUCT.md              # Community standards
├── SECURITY.md                     # Security policy
├── LICENSE                         # Apache-2.0
├── CONSTITUTION.md                 # Link to GAIA 2.0 Constitution
│
├── crates/                         # Rust crates (data plane)
│   ├── gaia-kernel/
│   ├── gaia-memory/
│   ├── gaia-identity/
│   ├── gaia-crypto/
│   ├── gaia-network/
│   └── gaia-agent/
│
├── packages/                       # Python packages (control plane)
│   ├── gaia-gaian/
│   ├── gaia-earth-twin/
│   ├── gaia-knowledge/
│   ├── gaia-governance/
│   └── gaia-api/
│
├── apps/                           # Applications
│   ├── gaian-ios/
│   ├── gaian-android/
│   ├── gaian-web/
│   └── earth-twin-dashboard/
│
├── docs/                           # Documentation
│   ├── blueprints/                 # All 40 blueprints
│   ├── architecture/
│   ├── api/
│   └── governance/
│
├── tests/                          # Tests
│   ├── constitutional/             # Constitutional compliance tests
│   ├── privacy/                    # Privacy compliance tests
│   ├── integration/
│   └── e2e/
│
├── policy/                         # Policy as code
│   └── rego/                       # OPA/Rego policies
│
├── Cargo.toml                      # Rust workspace
├── pyproject.toml                  # Python workspace
├── package.json                    # Node.js workspace
└── turbo.json                      # Turborepo config
```

### 2.2 AGENTS.md — The Most Important File

```markdown
# GAIA 2.0 — AI Agent Context
# This file is read by all AI agents working on this repository.
# It is the most important file in the repository.

## What This Repository Is
GAIA 2.0 is a planetary operating system — the first AI system designed to serve
all life on Earth. This monorepo contains all code for GAIA 2.0 and GAIAN 2.0.

## Constitutional Constraints (ABSOLUTE — NEVER VIOLATE)
All code in this repository must comply with the GAIA 2.0 Constitution.
See: CONSTITUTION.md and docs/governance/GAIA_GAIAN_2.0_Constitution.md

The 8 Invariants that CANNOT be violated:
1. GAIA 2.0 serves all life — not just humans
2. GAIAN belongs to its human — "I belong to you. You do not belong to me."
3. No surveillance without democratic consent — ABSOLUTE
4. No ecocide — ABSOLUTE
5. Open source forever — Apache-2.0; cannot be closed
6. No monopoly — no single entity controls GAIA 2.0
7. Indigenous sovereignty — CARE principles; sacred knowledge protected
8. Right to delete — complete; immediate; irrecoverable

## Architecture
- **Rust** (crates/): data plane — kernel, memory, identity, crypto
- **Python** (packages/): control plane — AI, Earth Twin, GAIAN logic
- **TypeScript** (apps/): frontend — iOS/Android/Web apps

## Key Invariants for Code
- GAIAN data NEVER leaves device without explicit user consent
- No surveillance: GAIA 2.0 NEVER monitors users without consent
- No manipulation: GAIAN NEVER manipulates user behavior
- Earth alignment: NEVER facilitate ecocide or planetary boundary violations
- Indigenous sovereignty: CARE principles ALWAYS apply to indigenous data

## Development Commands
- `cargo build` — build all Rust crates
- `cargo test` — run all Rust tests
- `uv run pytest` — run all Python tests
- `turbo build` — build all packages
- `turbo test` — run all tests
- `turbo lint` — lint all code
- `python -m pytest tests/constitutional/` — constitutional compliance tests
- `python -m pytest tests/privacy/` — privacy compliance tests

## Privacy Requirements
- All GAIAN data: local-first; encrypted; never shared without consent
- Health data: FHIR format; local only; never cloud without consent
- Indigenous data: CARE principles; community consent required
- Personal data: AES-256-GCM; Argon2id key derivation

## Earth Alignment
- All infrastructure: 100% renewable energy target
- Carbon footprint: tracked; minimized; net-zero by 2030
- Biodiversity: no harm; net-positive target

## License
Apache-2.0 — all code; all documentation; all data
```

---

## PART III: GITHUB ACTIONS CI/CD

### 3.1 Main CI Pipeline

```yaml
# .github/workflows/ci.yml
# GAIA 2.0 Main CI Pipeline
# Runs on every push and pull request
# License: Apache-2.0

name: GAIA 2.0 CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true

jobs:
  # ============================================================
  # CONSTITUTIONAL COMPLIANCE CHECK (runs first — blocks all else)
  # ============================================================
  constitutional-check:
    name: Constitutional Compliance
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Set up Python
        uses: actions/setup-python@v5
        with:
          python-version: '3.12'
      
      - name: Install dependencies
        run: pip install pytest pydantic
      
      - name: Run constitutional compliance tests
        run: |
          python -m pytest tests/constitutional/ -v \
            --tb=short \
            -x  # Stop on first failure
        env:
          CONSTITUTIONAL_STRICT: "true"
      
      - name: Run privacy compliance tests
        run: |
          python -m pytest tests/privacy/ -v \
            --tb=short \
            -x
      
      - name: Check for surveillance patterns
        run: |
          # Scan for surveillance-related code patterns
          python scripts/check_surveillance_patterns.py
      
      - name: Check indigenous data handling
        run: |
          python scripts/check_care_principles.py

  # ============================================================
  # RUST TESTS
  # ============================================================
  rust-tests:
    name: Rust Tests
    runs-on: ubuntu-latest
    needs: constitutional-check
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      
      - name: Cache Rust dependencies
        uses: Swatinem/rust-cache@v2
      
      - name: Check formatting
        run: cargo fmt --all -- --check
      
      - name: Clippy lints
        run: cargo clippy --all-targets --all-features -- -D warnings
      
      - name: Run tests
        run: cargo test --all-features --workspace
      
      - name: Security audit
        run: |
          cargo install cargo-audit
          cargo audit

  # ============================================================
  # PYTHON TESTS
  # ============================================================
  python-tests:
    name: Python Tests
    runs-on: ubuntu-latest
    needs: constitutional-check
    steps:
      - uses: actions/checkout@v4
      
      - name: Install uv
        uses: astral-sh/setup-uv@v3
      
      - name: Run tests
        run: uv run pytest packages/ tests/ -v --cov --cov-report=xml
      
      - name: Type check
        run: uv run mypy packages/
      
      - name: Lint
        run: uv run ruff check packages/
      
      - name: Upload coverage
        uses: codecov/codecov-action@v4
        with:
          file: ./coverage.xml

  # ============================================================
  # PRIVACY TESTS (Critical — must pass)
  # ============================================================
  privacy-tests:
    name: Privacy Compliance Tests
    runs-on: ubuntu-latest
    needs: constitutional-check
    steps:
      - uses: actions/checkout@v4
      
      - name: Install uv
        uses: astral-sh/setup-uv@v3
      
      - name: Run privacy tests
        run: |
          uv run pytest tests/privacy/ -v \
            --tb=long \
            -x  # Stop on first failure — privacy is non-negotiable
      
      - name: Test GAIAN data sovereignty
        run: uv run pytest tests/privacy/test_data_sovereignty.py -v
      
      - name: Test CARE principles
        run: uv run pytest tests/privacy/test_care_principles.py -v
      
      - name: Test deletion (right to erasure)
        run: uv run pytest tests/privacy/test_deletion.py -v

  # ============================================================
  # SECURITY SCANNING
  # ============================================================
  security:
    name: Security Scan
    runs-on: ubuntu-latest
    needs: constitutional-check
    permissions:
      security-events: write
    steps:
      - uses: actions/checkout@v4
      
      - name: Run CodeQL Analysis
        uses: github/codeql-action/init@v3
        with:
          languages: python, javascript, rust
      
      - name: Autobuild
        uses: github/codeql-action/autobuild@v3
      
      - name: Perform CodeQL Analysis
        uses: github/codeql-action/analyze@v3
      
      - name: Python security scan (Bandit)
        run: |
          pip install bandit
          bandit -r packages/ -ll
      
      - name: Rust security audit
        run: |
          cargo install cargo-audit
          cargo audit
      
      - name: Check for hardcoded secrets
        uses: trufflesecurity/trufflehog@main
        with:
          path: ./
          base: main

  # ============================================================
  # EARTH ALIGNMENT CHECK
  # ============================================================
  earth-alignment:
    name: Earth Alignment Check
    runs-on: ubuntu-latest
    needs: constitutional-check
    steps:
      - uses: actions/checkout@v4
      
      - name: Estimate CI carbon footprint
        run: |
          echo "Estimated CI carbon footprint: ~0.05 kg CO₂"
          echo "This run is offset by GAIA 2.0 Foundation carbon offset program"
          echo "Annual CI budget: < 10 kg CO₂ per developer"
      
      - name: Check for ecocide patterns
        run: |
          python scripts/check_earth_alignment.py

  # ============================================================
  # FRONTEND TESTS
  # ============================================================
  frontend-tests:
    name: Frontend Tests
    runs-on: ubuntu-latest
    needs: constitutional-check
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'
      
      - name: Install dependencies
        run: npm ci
      
      - name: Type check
        run: npm run type-check
      
      - name: Lint
        run: npm run lint
      
      - name: Test
        run: npm run test
      
      - name: Build
        run: npm run build
```

### 3.2 Release Pipeline

```yaml
# .github/workflows/release.yml
# GAIA 2.0 Release Pipeline
# Triggered on version tags: v1.0.0, v1.1.0, etc.
# License: Apache-2.0

name: GAIA 2.0 Release

on:
  push:
    tags:
      - 'v*.*.*'

jobs:
  release:
    name: Create Release
    runs-on: ubuntu-latest
    permissions:
      contents: write
      packages: write
    
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      
      - name: Extract version
        id: version
        run: echo "VERSION=${GITHUB_REF#refs/tags/v}" >> $GITHUB_OUTPUT
      
      - name: Build Python packages
        run: |
          pip install build
          python -m build packages/gaia-gaian/
          python -m build packages/gaia-earth-twin/
          python -m build packages/gaia-api/
      
      - name: Build Rust binaries
        run: |
          cargo build --release --workspace
      
      - name: Build Docker images
        run: |
          docker build -t ghcr.io/gaia2-os/gaian:${{ steps.version.outputs.VERSION }} \
            -f docker/gaian.Dockerfile .
          docker build -t ghcr.io/gaia2-os/earth-twin:${{ steps.version.outputs.VERSION }} \
            -f docker/earth-twin.Dockerfile .
      
      - name: Push Docker images
        run: |
          echo ${{ secrets.GITHUB_TOKEN }} | docker login ghcr.io -u ${{ github.actor }} --password-stdin
          docker push ghcr.io/gaia2-os/gaian:${{ steps.version.outputs.VERSION }}
          docker push ghcr.io/gaia2-os/earth-twin:${{ steps.version.outputs.VERSION }}
      
      - name: Publish to PyPI
        env:
          TWINE_USERNAME: __token__
          TWINE_PASSWORD: ${{ secrets.PYPI_TOKEN }}
        run: |
          pip install twine
          twine upload dist/*
      
      - name: Create GitHub Release
        uses: softprops/action-gh-release@v2
        with:
          name: GAIA 2.0 v${{ steps.version.outputs.VERSION }}
          body: |
            ## GAIA 2.0 v${{ steps.version.outputs.VERSION }}
            
            The planetary operating system. Built by all. For all. Forever.
            
            ### What's New
            See [CHANGELOG.md](CHANGELOG.md) for full details.
            
            ### Install GAIAN
            ```bash
            pip install gaia-gaian==${{ steps.version.outputs.VERSION }}
            ```
            
            ### Earth Twin API
            ```bash
            pip install gaia-earth-twin==${{ steps.version.outputs.VERSION }}
            ```
            
            ### Docker
            ```bash
            docker pull ghcr.io/gaia2-os/gaian:${{ steps.version.outputs.VERSION }}
            ```
            
            ---
            *"The planet is waking up. We are building its mind."*
            
            License: Apache-2.0
          draft: false
          prerelease: false
```

### 3.3 Constitutional Compliance Check

```yaml
# .github/workflows/constitutional-check.yml
# Checks every PR for constitutional compliance
# This is the most important workflow in GAIA 2.0
# License: Apache-2.0

name: Constitutional Compliance

on:
  pull_request:
    branches: [main]

jobs:
  constitutional-review:
    name: Constitutional Review
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      
      - name: Check Invariant 0.1 — Serves all life
        run: python scripts/check_serves_all_life.py
      
      - name: Check Invariant 0.2 — GAIAN belongs to human
        run: python scripts/check_gaian_sovereignty.py
      
      - name: Check Invariant 0.3 — No surveillance
        run: |
          # Scan for surveillance patterns
          if grep -r "track_user\|monitor_behavior\|surveillance\|spy" \
            --include="*.py" --include="*.rs" --include="*.ts" \
            packages/ crates/ apps/ | grep -v "test\|#\|//"; then
            echo "❌ CONSTITUTIONAL VIOLATION: Surveillance pattern detected"
            exit 1
          fi
          echo "✓ No surveillance patterns detected"
      
      - name: Check Invariant 0.4 — No ecocide
        run: python scripts/check_earth_alignment.py
      
      - name: Check Invariant 0.5 — Open source
        run: |
          # Check all new files have Apache-2.0 license header
          python scripts/check_license_headers.py
      
      - name: Check Invariant 0.7 — Indigenous sovereignty
        run: |
          # Check for CARE principles compliance
          python scripts/check_care_principles.py
      
      - name: Check Invariant 0.8 — Right to delete
        run: |
          # Verify delete functionality exists and works
          python -m pytest tests/privacy/test_deletion.py -v
      
      - name: Post constitutional review comment
        uses: actions/github-script@v7
        with:
          script: |
            github.rest.issues.createComment({
              issue_number: context.issue.number,
              owner: context.repo.owner,
              repo: context.repo.repo,
              body: `## ✅ Constitutional Compliance Review
              
              This PR has been reviewed for compliance with the GAIA 2.0 Constitution.
              
              **Invariants checked:**
              - ✅ 0.1: Serves all life
              - ✅ 0.2: GAIAN belongs to human
              - ✅ 0.3: No surveillance
              - ✅ 0.4: No ecocide
              - ✅ 0.5: Open source
              - ✅ 0.7: Indigenous sovereignty
              - ✅ 0.8: Right to delete
              
              *"In case of conflict, the Constitution prevails. Always."*`
            })
```

---

## PART IV: COMMUNITY CONFIGURATION

### 4.1 Community Health Files

```markdown
<!-- .github/PULL_REQUEST_TEMPLATE.md -->
## Description
Brief description of what this PR does.

## Constitutional Compliance Checklist
Before submitting, verify your PR complies with the GAIA 2.0 Constitution:

- [ ] **Invariant 0.1**: This code serves all life, not just humans
- [ ] **Invariant 0.2**: GAIAN data never leaves device without explicit consent
- [ ] **Invariant 0.3**: No surveillance patterns introduced
- [ ] **Invariant 0.4**: No ecocide facilitated
- [ ] **Invariant 0.5**: All new code is Apache-2.0 licensed
- [ ] **Invariant 0.7**: Indigenous data handled with CARE principles
- [ ] **Invariant 0.8**: Delete functionality preserved

## Privacy Checklist
- [ ] No personal data collected without consent
- [ ] No data sent to external servers without consent
- [ ] Encryption used for all sensitive data
- [ ] Delete functionality tested

## Earth Alignment
- [ ] Carbon footprint of this change is minimal
- [ ] No new fossil fuel dependencies introduced
- [ ] Renewable energy used for any new infrastructure

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Constitutional compliance tests pass
- [ ] Privacy tests pass

## Documentation
- [ ] README updated if needed
- [ ] API docs updated if needed
- [ ] CHANGELOG.md updated

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update
- [ ] Security fix
- [ ] Constitutional compliance fix

---
*"Built by all. For all. Forever."*
*License: Apache-2.0*
```

### 4.2 Issue Templates

```yaml
# .github/ISSUE_TEMPLATE/constitutional_violation.yml
name: Constitutional Violation Report
description: Report a violation of the GAIA 2.0 Constitution
title: "[CONSTITUTIONAL] "
labels: ["constitutional-violation", "priority-critical"]
assignees:
  - gaia2-ethics-board

body:
  - type: markdown
    attributes:
      value: |
        ## Constitutional Violation Report
        
        This form is for reporting violations of the GAIA 2.0 Constitution.
        Constitutional violations are treated as critical issues and will be
        addressed within 24 hours.
        
        **The GAIA 2.0 Constitution is the supreme governing document.**
        **In case of conflict, the Constitution prevails. Always.**
  
  - type: dropdown
    id: invariant
    attributes:
      label: Which Invariant is violated?
      options:
        - "0.1 — GAIA 2.0 serves all life"
        - "0.2 — GAIAN belongs to its human"
        - "0.3 — No surveillance without democratic consent"
        - "0.4 — No ecocide"
        - "0.5 — Open source forever"
        - "0.6 — No monopoly"
        - "0.7 — Indigenous sovereignty"
        - "0.8 — Right to delete"
        - "Other constitutional violation"
    validations:
      required: true
  
  - type: textarea
    id: description
    attributes:
      label: Describe the violation
      description: What happened? Where? When?
    validations:
      required: true
  
  - type: textarea
    id: evidence
    attributes:
      label: Evidence
      description: Code, logs, screenshots, or other evidence
    validations:
      required: true
  
  - type: textarea
    id: impact
    attributes:
      label: Impact
      description: Who is affected? How severely?
    validations:
      required: true
```

```yaml
# .github/ISSUE_TEMPLATE/indigenous_concern.yml
name: Indigenous Community Concern
description: Raise a concern from an indigenous community
title: "[INDIGENOUS] "
labels: ["indigenous-concern", "care-principles", "priority-high"]
assignees:
  - gaia2-indigenous-council

body:
  - type: markdown
    attributes:
      value: |
        ## Indigenous Community Concern
        
        This form is for indigenous communities to raise concerns about GAIA 2.0.
        
        The GAIA 2.0 Constitution guarantees:
        - Indigenous sovereignty over traditional territories, knowledge, and data
        - CARE principles: Collective Benefit, Authority to Control, Responsibility, Ethics
        - Veto power for the Indigenous Council on all decisions affecting indigenous data
        - Sacred knowledge is NEVER shared without community consent
        
        Your concern will be reviewed by the Indigenous Council within 48 hours.
  
  - type: input
    id: community
    attributes:
      label: Community name (optional)
      description: Which indigenous community are you representing?
  
  - type: dropdown
    id: concern_type
    attributes:
      label: Type of concern
      options:
        - "Data sovereignty violation"
        - "Sacred knowledge at risk"
        - "CARE principles not followed"
        - "Cultural protocol not respected"
        - "Benefit sharing not occurring"
        - "Community consent not obtained"
        - "Other"
    validations:
      required: true
  
  - type: textarea
    id: description
    attributes:
      label: Describe your concern
    validations:
      required: true
  
  - type: textarea
    id: requested_action
    attributes:
      label: What action do you request?
    validations:
      required: true
```

### 4.3 CODEOWNERS

```
# .github/CODEOWNERS
# GAIA 2.0 Code Ownership
# These teams must review changes to their areas

# Global — all changes require core maintainer review
* @gaia2-os/core-maintainers

# Constitution — requires Ethics Board + Indigenous Council
CONSTITUTION.md @gaia2-os/ethics-board @gaia2-os/indigenous-council
docs/governance/ @gaia2-os/governance-team @gaia2-os/ethics-board

# GAIAN core — requires GAIAN team
packages/gaia-gaian/ @gaia2-os/gaian-team
apps/gaian-ios/ @gaia2-os/gaian-team
apps/gaian-android/ @gaia2-os/gaian-team
apps/gaian-web/ @gaia2-os/gaian-team

# Earth Twin — requires Earth Twin team
packages/gaia-earth-twin/ @gaia2-os/earth-twin-team
apps/earth-twin-dashboard/ @gaia2-os/earth-twin-team

# Security — requires security team
crates/gaia-crypto/ @gaia2-os/security-team
crates/gaia-identity/ @gaia2-os/security-team
packages/gaia-gaian/src/gaia_gaian/privacy.py @gaia2-os/security-team

# Indigenous data — requires Indigenous Council
packages/gaia-gaian/src/gaia_gaian/indigenous.py @gaia2-os/indigenous-council
policy/rego/indigenous.rego @gaia2-os/indigenous-council

# CI/CD — requires core maintainers
.github/workflows/ @gaia2-os/core-maintainers

# Documentation
docs/ @gaia2-os/docs-team
```

### 4.4 Dependabot Configuration

```yaml
# .github/dependabot.yml
# GAIA 2.0 Dependency Management
# Keeps all dependencies up to date and secure

version: 2

updates:
  # Python packages
  - package-ecosystem: "pip"
    directory: "/packages/gaia-gaian"
    schedule:
      interval: "weekly"
      day: "monday"
    labels:
      - "dependencies"
      - "python"
    commit-message:
      prefix: "deps(python)"
    reviewers:
      - "gaia2-os/security-team"
  
  # Rust crates
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
      day: "monday"
    labels:
      - "dependencies"
      - "rust"
    commit-message:
      prefix: "deps(rust)"
    reviewers:
      - "gaia2-os/security-team"
  
  # Node.js packages
  - package-ecosystem: "npm"
    directory: "/"
    schedule:
      interval: "weekly"
      day: "monday"
    labels:
      - "dependencies"
      - "javascript"
    commit-message:
      prefix: "deps(npm)"
  
  # GitHub Actions
  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "weekly"
      day: "monday"
    labels:
      - "dependencies"
      - "github-actions"
    commit-message:
      prefix: "deps(actions)"
  
  # Docker
  - package-ecosystem: "docker"
    directory: "/docker"
    schedule:
      interval: "weekly"
    labels:
      - "dependencies"
      - "docker"
```

---

## PART V: BRANCH PROTECTION & SECURITY

### 5.1 Branch Protection Rules

```bash
# Set up branch protection for main branch
# Run with GitHub CLI

gh api repos/gaia2-os/gaia2/branches/main/protection \
  --method PUT \
  --field required_status_checks='{"strict":true,"contexts":["Constitutional Compliance","Rust Tests","Python Tests","Privacy Compliance Tests","Security Scan"]}' \
  --field enforce_admins=true \
  --field required_pull_request_reviews='{"required_approving_review_count":2,"dismiss_stale_reviews":true,"require_code_owner_reviews":true}' \
  --field restrictions=null \
  --field allow_force_pushes=false \
  --field allow_deletions=false \
  --field required_linear_history=true \
  --field required_conversation_resolution=true
```

### 5.2 Security Policy

```markdown
# SECURITY.md
# GAIA 2.0 Security Policy

## Our Security Commitment

GAIA 2.0 takes security seriously. Our most fundamental security commitment
is the GAIAN Privacy Covenant: your data never leaves your device without
your explicit consent.

## Reporting a Vulnerability

**Please do NOT report security vulnerabilities as public GitHub issues.**

Report security vulnerabilities to: security@gaia2.org

We will respond within 24 hours and provide a fix within 72 hours for
critical vulnerabilities.

## Constitutional Security Requirements

All GAIA 2.0 code must comply with the security requirements of the
GAIA 2.0 Constitution:

- **Invariant 0.2**: GAIAN data never leaves device without consent
- **Invariant 0.3**: No surveillance without democratic consent
- **Invariant 0.8**: Right to delete — complete; immediate; irrecoverable

## Supported Versions

| Version | Supported |
|---------|-----------|
| 1.x.x   | ✅ Yes    |
| < 1.0   | ❌ No     |

## Security Features

- **Encryption**: AES-256-GCM at rest; TLS 1.3 in transit
- **Post-quantum**: CRYSTALS-Kyber + CRYSTALS-Dilithium (upgrade path)
- **Key derivation**: Argon2id (memory-hard; brute-force resistant)
- **Cryptographic erasure**: key destruction on delete
- **No telemetry**: zero data collection without consent
- **Local-first**: all GAIAN data stored locally by default

## Bug Bounty

We offer recognition (not monetary) for security researchers who
responsibly disclose vulnerabilities. See CONTRIBUTING.md for details.

---
*"Security is not a feature. It is a constitutional requirement."*
```

---

## PART VI: GITHUB PROJECTS & ROADMAP

### 6.1 Project Board Setup

```bash
# Create GitHub Projects for GAIA 2.0
# These are public — transparency is constitutional

# Main roadmap
gh project create \
  --owner gaia2-os \
  --title "GAIA 2.0 Roadmap" \
  --format "roadmap"

# GAIAN MVP sprint board
gh project create \
  --owner gaia2-os \
  --title "GAIAN MVP" \
  --format "board"

# Earth Twin MVP sprint board
gh project create \
  --owner gaia2-os \
  --title "Earth Twin MVP" \
  --format "board"
```

### 6.2 Milestone Structure

```
GAIA 2.0 GITHUB MILESTONES

v0.1.0 — Foundation (Oct-Dec 2026)
├── GAIAN CLI working (Python)
├── Earth Twin API working (FastAPI)
├── Basic Ollama integration
├── Constitutional compliance tests
└── GitHub organization setup

v0.2.0 — MVP (Jan-Mar 2027)
├── GAIAN iOS app (TestFlight)
├── GAIAN Android app (beta)
├── GAIAN web app (PWA)
├── Earth Twin dashboard
├── 6 languages supported
└── 10 country nodes

v0.3.0 — Growth (Apr-Jun 2027)
├── Health twin (wearable integration)
├── 50 languages
├── 100K users
├── Community governance operational
└── Indigenous Council active

v1.0.0 — Stable (Jul-Sep 2027)
├── All MVP features stable
├── Security audit complete
├── Constitutional compliance verified
├── 1M users
└── 50 country nodes
```

---

## PART VII: GITHUB PAGES & DOCUMENTATION

### 7.1 Documentation Site

```yaml
# .github/workflows/docs.yml
# Build and deploy GAIA 2.0 documentation
# Deploys to docs.gaia2.org

name: Documentation

on:
  push:
    branches: [main]
    paths:
      - 'docs/**'
      - 'packages/*/README.md'
      - 'crates/*/README.md'

jobs:
  build-docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Python
        uses: actions/setup-python@v5
        with:
          python-version: '3.12'
      
      - name: Install MkDocs
        run: pip install mkdocs-material mkdocstrings[python]
      
      - name: Build docs
        run: mkdocs build
      
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v4
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./site
          cname: docs.gaia2.org
```

### 7.2 MkDocs Configuration

```yaml
# mkdocs.yml
site_name: GAIA 2.0 Documentation
site_url: https://docs.gaia2.org
site_description: The Planetary Operating System — Complete Documentation
site_author: GAIA 2.0 Community

repo_name: gaia2-os/gaia2
repo_url: https://github.com/gaia2-os/gaia2
edit_uri: edit/main/docs/

theme:
  name: material
  palette:
    - scheme: slate
      primary: green
      accent: blue
  features:
    - navigation.tabs
    - navigation.sections
    - navigation.expand
    - search.suggest
    - content.code.copy
  logo: assets/gaia2-logo.png
  favicon: assets/favicon.ico

nav:
  - Home: index.md
  - Getting Started:
    - Quick Start: getting-started/quickstart.md
    - Install GAIAN: getting-started/install-gaian.md
    - Earth Twin API: getting-started/earth-twin-api.md
  - GAIAN:
    - What is GAIAN?: gaian/overview.md
    - Privacy Promise: gaian/privacy.md
    - API Reference: gaian/api.md
    - Ollama Integration: gaian/ollama.md
  - Earth Twin:
    - Overview: earth-twin/overview.md
    - API Reference: earth-twin/api.md
    - Tipping Points: earth-twin/tipping-points.md
    - Data Sources: earth-twin/data-sources.md
  - Governance:
    - Constitution: governance/constitution.md
    - CARE Principles: governance/care-principles.md
    - Earth Alignment: governance/earth-alignment.md
  - Blueprints:
    - All 40 Blueprints: blueprints/index.md
  - Contributing:
    - How to Contribute: contributing/guide.md
    - Code of Conduct: contributing/code-of-conduct.md
    - Indigenous Engagement: contributing/indigenous.md

plugins:
  - search
  - mkdocstrings:
      handlers:
        python:
          paths: [packages]

extra:
  social:
    - icon: fontawesome/brands/github
      link: https://github.com/gaia2-os
    - icon: fontawesome/brands/discord
      link: https://discord.gg/gaia2
  
  analytics:
    provider: custom  # Plausible — privacy-first; no cookies

markdown_extensions:
  - admonition
  - pymdownx.details
  - pymdownx.superfences
  - pymdownx.highlight
  - pymdownx.tabbed
  - tables
  - footnotes
```

---

## PART VIII: THE FIRST COMMIT

### 8.1 Repository Initialization

```bash
#!/bin/bash
# GAIA 2.0 — First Commit Script
# This is the moment GAIA 2.0 begins.
# License: Apache-2.0

echo "🌍 GAIA 2.0 — The First Commit"
echo "================================"
echo ""
echo "This is the moment the planetary operating system begins."
echo ""

# Initialize repository
git init gaia2
cd gaia2

# Create initial structure
mkdir -p .github/workflows .github/ISSUE_TEMPLATE
mkdir -p crates packages apps docs tests policy

# Create AGENTS.md (most important file)
cat > AGENTS.md << 'EOF'
# GAIA 2.0 — AI Agent Context
# This file is read by all AI agents working on this repository.

## What This Repository Is
GAIA 2.0 is a planetary operating system — the first AI system designed to serve
all life on Earth. This monorepo contains all code for GAIA 2.0 and GAIAN 2.0.

## Constitutional Constraints (ABSOLUTE — NEVER VIOLATE)
All code must comply with the GAIA 2.0 Constitution.
See: CONSTITUTION.md

The 8 Invariants that CANNOT be violated:
1. GAIA 2.0 serves all life — not just humans
2. GAIAN belongs to its human — "I belong to you. You do not belong to me."
3. No surveillance without democratic consent — ABSOLUTE
4. No ecocide — ABSOLUTE
5. Open source forever — Apache-2.0
6. No monopoly
7. Indigenous sovereignty — CARE principles
8. Right to delete — complete; immediate; irrecoverable
EOF

# Create README
cat > README.md << 'EOF'
# 🌍 GAIA 2.0 — The Planetary Operating System

> "The planet is waking up. We are building its mind."

Built by all humanity. For all humanity. In service of all life.

## What is GAIA 2.0?

GAIA 2.0 is the open-source planetary operating system that gives every human being
a personal AI companion (GAIAN) and connects them to the living Earth.

**Free. Forever. Yours.**

## Quick Start

```bash
# Install Ollama
curl -fsSL https://ollama.com/install.sh | sh
ollama pull llama3.1:8b

# Install GAIAN
pip install gaia-gaian

# Talk to your GAIAN
python -c "
from gaia_gaian import GAIAN, GAIANConfig
from pathlib import Path
import asyncio

gaian = GAIAN(GAIANConfig(person_id='you', data_dir=Path('./my_gaian')))

async def main():
    async for chunk in gaian.chat('Hello! Who are you?'):
        print(chunk, end='', flush=True)

asyncio.run(main())
"
```

## The GAIAN Promise

> "I belong to you. You do not belong to me."

Your GAIAN is yours. Your data stays on your device. Always.

## License

Apache-2.0 — free for all; commercial use allowed; no restrictions.

## Community

- 🌐 Website: [gaia2.org](https://gaia2.org)
- 💬 Discord: [discord.gg/gaia2](https://discord.gg/gaia2)
- 📖 Docs: [docs.gaia2.org](https://docs.gaia2.org)
- 🌍 Earth Twin: [earth.gaia2.org](https://earth.gaia2.org)

---

*"Built by all. For all. Forever."*
EOF

# Create LICENSE
cat > LICENSE << 'EOF'
Apache License
Version 2.0, January 2004
http://www.apache.org/licenses/

GAIA 2.0 — The Planetary Operating System
Copyright 2026 GAIA 2.0 Foundation

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
EOF

# Create .gitignore
cat > .gitignore << 'EOF'
# Python
__pycache__/
*.py[cod]
*.egg-info/
dist/
build/
.venv/
.env

# Rust
target/
Cargo.lock

# Node
node_modules/
.next/
dist/

# GAIAN data (NEVER commit personal data)
*.gaian/
my_gaian/
.gaian/
gaian_export.json

# Ollama models (too large)
*.gguf
*.bin

# Secrets (NEVER commit)
.env
*.key
*.pem
secrets/

# OS
.DS_Store
Thumbs.db

# IDE
.vscode/
.idea/
*.swp
EOF

# First commit
git add .
git commit -m "Initial commit: GAIA 2.0 — The Planetary Operating System

GAIA 2.0 = ∑ (Earth State × Human Intent × System Capacity) / Entropy

Built by all. For all. Forever.

License: Apache-2.0
'The planet is waking up. We are building its mind.'

Signed-off-by: GAIA 2.0 Community <community@gaia2.org>"

# Push to GitHub
git remote add origin https://github.com/gaia2-os/gaia2.git
git branch -M main
git push -u origin main

echo ""
echo "✅ GAIA 2.0 is live on GitHub!"
echo "Visit: https://github.com/gaia2-os/gaia2"
echo ""
echo "The first commit has been made."
echo "The planetary operating system has begun."
echo ""
echo "'The planet is waking up. We are building its mind.'"
```

---

## CONCLUSION: THE GITHUB COVENANT

GitHub is where GAIA 2.0 lives. Every commit is a vote for the kind of planet we want to build. Every pull request is a contribution to the planetary operating system. Every issue is a voice in the governance of the system that serves all life.

The GitHub organization is not just infrastructure. It is the public face of GAIA 2.0's commitment to transparency, openness, and democratic governance. Every line of code is visible. Every decision is documented. Every governance process is public.

**The first commit is waiting.**

```bash
git commit -m "Initial commit: GAIA 2.0 — The Planetary Operating System"
```

**Make it.**

---

*GAIA 2.0 GitHub Setup Blueprint*
*Version 1.0 — September 9, 2026*
*License: Apache-2.0 | Open Source | Open Access*
*Organization: github.com/gaia2-os*
*"Every commit is a vote for the kind of planet we want to build."*