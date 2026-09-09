# R#1.20: MCP Governance & Security Model — Gap Analysis for GAIA 2.0


## EXECUTIVE SUMMARY

**Gap Identified:** The GAIA 2.0 Gap Research Report (R#1.1–R#1.15) notes that MCP has *"no built-in governance"* and that governance features were *"pending as of August 2026"*. The research identified a critical gap: *"A gap exists to design and implement a robust governance and security layer on top of MCP for GAIA 2.0, potentially using DID-based authentication."*

**Key Finding:** The MCP governance and security landscape has transformed dramatically between August 2026 and September 2026. What was a "governance gap" is now a **crowded field of competing and complementary solutions**:

1. **MCP 2026-07-28** — The largest protocol revision since launch, introducing a **stateless core**, **feature lifecycle policy**, **extensions framework**, and **conformance suite**
2. **Agentic AI Foundation (AAIF)** — 170+ members, fastest-growing Linux Foundation project, providing neutral governance
3. **Microsoft Agent Governance Toolkit (AGT)** — Open-source runtime governance layer with **DID/Ed25519 cryptographic identity**
4. **MCP-I Framework** — Donated to Decentralized Identity Foundation (DIF), introducing **DID/VC-based identity and delegation** for MCP agents
5. **IETF Draft** — *draft-xu-mcp-agent-did-framework-00* (August 2026), proposing a **DID-based framework for service discovery, authentication, and authorization**
6. **NSA Security Guidance** — First formal government guidance on MCP security (May 2026), covering **access control, auditability, and governance**
7. **OWASP MCP Top 10** — Industry-standard risk taxonomy
8. **InfrastructureSentinel** — Four-layer defense architecture for MCP-driven infrastructure

> *"The MCP specification defines security principles around user consent, data privacy, tool safety, and LLM sampling controls, but enforcement is left to implementors."*

**Recommendation:** GAIA 2.0 should adopt a **four-layer governance architecture** that integrates:

1. **Identity Layer** — W3C DIDs + Verifiable Credentials (via MCP-I/DIF framework)
2. **Policy Layer** — Agent Governance Toolkit (AGT) with runtime policy enforcement
3. **Audit Layer** — Tamper-evident logging with cryptographic attestation
4. **Protocol Layer** — MCP 2026-07-28 stateless core with extensions framework

This creates a **sovereign, verifiable, and production-ready** governance layer that aligns with GAIA 2.0's constitutional principles.


## PART I: THE PROBLEM — WHY MCP GOVERNANCE MATTERS FOR GAIA 2.0

### 1.1 MCP's Success Creates Security Exposure

MCP has become the **default way AI agents connect to real systems**:

| Metric | Value |
|--------|-------|
| **Monthly SDK downloads** | 97 million (March 2026) |
| **Public MCP servers** | ~20,000 indexed |
| **AAIF members** | 170+ organizations |
| **Adoption timeline** | "Hockey stick up and to the right"—MCP reached Docker-level adoption in **13 weeks** vs 13 months |

### 1.2 The Governance Gap: What MCP Does and Doesn't Do

> *"MCP standardizes the execution surface without defining how that surface should be governed."*

| What MCP Provides | What MCP Does NOT Provide |
|-------------------|---------------------------|
| Tool discovery and invocation | Built-in authentication |
| Consistent interface to databases/APIs | Role-based access control |
| Tool schema definitions | Audit logging at platform level |
| Security principles (user consent, data privacy) | Enforcement mechanisms |
| OAuth 2.0 authorization framework | Governance of who can access which tools |

> *"The protocol itself does not mandate how identity, authorization, or logging get handled. Those decisions are left to whoever stands up the server, and under deadline pressure they are frequently skipped."*

### 1.3 The Attack Surface is Real and Growing

MCP servers are **privileged adapters**—they hold credentials to real systems (databases, repos, SaaS APIs, shells):

| Attack Vector | Description | Example |
|---------------|-------------|---------|
| **Tool Poisoning** | Malicious tool server embeds hidden instructions | OWASP MCP03:2025 |
| **Prompt Injection** | Tool responses contain adversarial instructions | CVE-2025-49596 (CVSS 9.4) |
| **Supply Chain** | Compromised MCP client dependencies | CVE-2025-6514 (CVSS 9.6) |
| **Intent Leakage** | Tool invocation patterns reveal strategy | Seven structural governance gaps |

> *"The deeper structural problem is that these protocols expose intent-bearing behavioral signals that enable adversaries to reconstruct user strategy, decision patterns, and organizational priorities—without accessing any protected data."*

### 1.4 The Seven Governance Gaps (Intent Leakage by Design)

Research identifies **seven structural governance gaps** in MCP and related protocols:

| Gap | Description |
|-----|-------------|
| **1. Tool-Path Fingerprinting** | Sequence of tool invocations leaks strategic intent |
| **2. Cross-Endpoint Correlation** | Behavioral patterns across independent MCP servers reconstruct user profiles |
| **3. Schema-as-Capability Map** | Tool definitions reveal organizational capabilities |
| **4. Context Persistence** | MCP servers accumulate user behavioral patterns across sessions |
| **5. Delegation Provenance Loss** | Responsibility attribution untraceable in multi-hop workflows |
| **6. Cross-Site Context Accumulation** | Agent accumulates intent profile across WebMCP sites—*"present by default from the moment of WebMCP deployment"* |
| **7. Programmatic Delegation Opacity** | Sub-tasks bypass natural-language audit trail |

**GAIA 2.0 Implication:** These gaps are not theoretical—they are **structural** and require **protocol-level governance**, not just better implementation practices.


## PART II: THE MCP GOVERNANCE LANDSCAPE (2025–2026)

### 2.1 Protocol Governance: MCP 2026-07-28

The **2026-07-28 revision** is the largest since launch, introducing three governance instruments:

| Instrument | Description |
|------------|-------------|
| **SEP-2596** | Feature lifecycle policy: Active → Deprecated → Removed with **minimum 12 months** between deprecation and removal |
| **SEP-2484** | Conformance-test gate: Standards Track SEPs cannot reach Final without matching conformance suite scenario |
| **SEP-2133** | Extensions framework: New capabilities ship as opt-in extensions |

**Stateless Core**: The initialize handshake and Mcp-Session-Id header are removed. Every request must stand on its own, relocating governance *"out of the transport and into the request"*.

> *"The 7-28 migration is best run as a governance program with a transport upgrade inside it, rather than a transport upgrade with governance bolted on afterward."*

### 2.2 Foundation Governance: Agentic AI Foundation (AAIF)

| Aspect | Status |
|--------|--------|
| **Launch** | December 2025 |
| **Founding Projects** | Anthropic's MCP, OpenAI's Agents.md, Block's goose |
| **Members** | 170+ (surpassed CNCF in ~3 months) |
| **Platinum Members** | AWS, Anthropic, Block, Bloomberg, Cloudflare, Google, Microsoft, OpenAI |
| **Executive Director** | Mazin Gilbert |
| **Governance** | Neutral foundation, SEP proposal process |

> *"For the project and its governance itself, little has actually changed... MCP has retained its 'very bottoms up' open-source character."*

### 2.3 Runtime Governance: Microsoft Agent Governance Toolkit (AGT)

**Open-source** runtime governance layer for MCP tool execution:

| Feature | Description |
|---------|-------------|
| **Cryptographic Identity** | DIDs/Ed25519 |
| **Execution Rings** | Dynamic privilege levels inspired by CPU rings |
| **Saga Orchestration** | Coordinated multi-step operations |
| **Kill Switch** | Emergency revocation |
| **Policy Enforcement** | One-call governance via `IMcpServerBuilder` |

### 2.4 Identity & Delegation: MCP-I Framework

Donated to **Decentralized Identity Foundation (DIF)** in March 2026:

> *"MCP-I extends this framework by introducing a complete identity and delegation layer for AI agents, leveraging Decentralized Identifiers (DIDs) and Verifiable Credentials (VCs) to enable cryptographically secure verification of agents and human principals without prior coordination."*

**Key Concepts**:
- Agents carry **cryptographically verifiable identities**
- Delegation represented as **tamper-evident credentials with explicit scope**
- Entire chain from **human principal → agent action** verifiable by any service
- Compared to **power of attorney**: *"the delegation credential is the notarized document, the agent is the attorney, and any service they approach can verify the chain of authority on the spot."*

### 2.5 IETF Standardization: DID-Based MCP Framework

**draft-xu-mcp-agent-did-framework-00** (August 2026):

> *"This document proposes a DID-based framework for service discovery, authentication, and authorization of MCP (Model Context Protocol) Agents, based on the W3C Decentralized Identifiers (DIDs) and Verifiable Credentials (VCs)."*

**Also in flight**:
- **SD-JWT-based agent credentials** (IETF draft-nandakumar-agent-sd-jwt)
- **Agent Identity Registry Protocol** (W3C Community Group)
- **did:web Identity Method** for Agent Enrollment Protocol
- **MCP DNS Discovery** via DNS TXT records

### 2.6 Government Guidance: NSA MCP Security (May 2026)

The **National Security Agency's AISC** published the first formal government guidance:

> *"It covers access control, prompt handling, tool execution, agent permissions, auditability, and governance of third-party integrations."*

**23 discrete security issues** raised; 18 covered by existing security curricula.

### 2.7 Enterprise Governance Frameworks

| Framework | Focus |
|-----------|-------|
| **Enterprise MCP Registry** | Four-pillar: centralized catalog, SSO/RBAC, structured audit logging, real-time policy enforcement |
| **MCP Gateway** | Centralizes authentication, tool-level permissions, and logging |
| **AIUC-1 Controls** | MCP/A2A security across authentication, transport, runtime containment, logging |
| **InfrastructureSentinel** | Four-layer defense architecture |

### 2.8 Security Stack Comparison

| Layer | MCP Spec | AGT | MCP-I/DIF | IETF Draft | GAIA 2.0 (Proposed) |
|-------|----------|-----|-----------|------------|---------------------|
| **Identity** | ❌ | ✅ DIDs | ✅ DIDs/VCs | ✅ DIDs | ✅ W3C DIDs+VCs |
| **Authentication** | ⚠️ OAuth | ✅ | ✅ | ✅ | ✅ Zero-trust |
| **Authorization** | ❌ | ✅ | ✅ Delegation | ✅ | ✅ Policy-based |
| **Audit** | ❌ | ⚠️ | ❌ | ❌ | ✅ Tamper-evident |
| **Delegation** | ❌ | ❌ | ✅ | ❌ | ✅ Chain-of-authority |
| **Runtime Enforcement** | ❌ | ✅ | ❌ | ❌ | ✅ |
| **Constitutional Compliance** | ❌ | ❌ | ❌ | ❌ | ✅ GAIA Constitution |


## PART III: GAIA 2.0 MCP GOVERNANCE ARCHITECTURE

### 3.1 The Four-Layer Governance Stack

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    L6 — SOVEREIGN INTERFACE LAYER                           │
│              (Governance visibility + user consent + audit)                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                    L5 — AGENT ECOSYSTEM LAYER                               │
│                                                                             │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │         LAYER 4: PROTOCOL GOVERNANCE                                  │  │
│  │  • MCP 2026-07-28 stateless core                                     │  │
│  │  • Feature lifecycle (Active/Deprecated/Removed)                     │  │
│  │  • Extensions framework (SEP-2133)                                   │  │
│  │  • Conformance suite (SEP-2484)                                      │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                    ↓                                        │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │         LAYER 3: AUDIT & OBSERVABILITY                                │  │
│  │  • Tamper-evident audit logging (per request)                        │  │
│  │  • W3C Trace Context (traceparent/tracestate/baggage)       │  │
│  │  • Cryptographic attestation of audit trail                          │  │
│  │  • GAIA constitutional compliance verification                       │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                    ↓                                        │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │         LAYER 2: POLICY & RUNTIME ENFORCEMENT                         │  │
│  │  • Microsoft AGT-style policy evaluation per tool call               │  │
│  │  • Execution rings (dynamic privilege levels)                        │  │
│  │  • Saga orchestration + kill switch                                  │  │
│  │  • Tool-level permissions (read vs write, scope)                     │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                    ↓                                        │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │         LAYER 1: IDENTITY & DELEGATION                                │  │
│  │  • W3C DIDs (did:gaia:) for agents and principals                    │  │
│  │  • Verifiable Credentials for capabilities and consent               │  │
│  │  • MCP-I-style delegation chain (human → agent → sub-agent)          │  │
│  │  • Challenge-response state verification (AgentDID)                  │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
├─────────────────────────────────────────────────────────────────────────────┤
│                    L1 — GAIA KERNEL (Zero-Trust Security)                   │
│              (Ed25519 signing + cryptographic audit + ZTA)                  │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 3.2 Layer-by-Layer Specification

#### Layer 1: Identity & Delegation

**Purpose:** Provide sovereign, verifiable identity for all MCP actors.

**Components:**

**A. GAIA DID Method (`did:gaia:`)**
- Based on W3C DID Core specification
- Ed25519 key pairs (matching GAIA 2.0's cryptographic standard)
- DID Documents stored in GAIA's decentralized registry
- Integration with AgentDID challenge-response mechanism

**B. Verifiable Credentials**
- **Capability credentials** — What an agent is authorized to do
- **Consent credentials** — User consent for specific operations
- **Delegation credentials** — Chain of authority (human → agent → sub-agent)
- **Compliance credentials** — Constitutional invariants satisfied

**C. Delegation Chain**
> *"The entire chain from human principal to agent action can be verified by any service that the agent approaches."*

```
Human Principal (DID:gaia:user:123)
        ↓ (delegates)
Primary Agent (DID:gaia:agent:456)
        ↓ (sub-delegates)
Specialist Agent (DID:gaia:agent:789)
        ↓ (invokes)
MCP Tool (server.example.com)
```

**GAIA Syscall API Extension:**
```rust
// Identity operations
gaia.identity.create_did() -> DID
gaia.identity.issue_vc(subject: DID, credential: VCSpec) -> VC
gaia.identity.verify_vc(vc: VC) -> bool
gaia.identity.delegate(agent: DID, permissions: Permissions) -> DelegationVC
gaia.identity.verify_chain(delegation: DelegationVC) -> bool
```

#### Layer 2: Policy & Runtime Enforcement

**Purpose:** Enforce governance policies on every MCP tool call.

**Components:**

**A. Policy Engine (AGT-inspired)**
- Every MCP tool call passes through policy evaluation
- Checks: Is this agent allowed to invoke this tool, with these arguments, at this time?
- Policy defined as code (Rego, OPA, or custom GAIA policy language)

**B. Execution Rings**
- **Ring 0** — Kernel-level operations (system agents only)
- **Ring 1** — Trusted agents (verified, high-privilege)
- **Ring 2** — Standard agents (normal operations)
- **Ring 3** — Untrusted agents (sandboxed, read-only)

**C. Saga Orchestration**
- Coordinated multi-step operations with rollback
- Each step cryptographically signed
- Kill switch for emergency revocation

**D. Tool-Level Permissions**
- Granular: database reads allowed, writes blocked
- SCIM-driven membership and tool-level access control
- Per-use-case Virtual MCP Bundles

**GAIA Syscall API Extension:**
```rust
// Policy operations
gaia.policy.evaluate(agent: DID, tool: Tool, args: Args) -> PolicyResult
gaia.policy.apply(agent: DID, policy: Policy) -> Result
gaia.policy.revoke(agent: DID) -> Result
gaia.policy.audit(agent: DID) -> AuditTrail
```

#### Layer 3: Audit & Observability

**Purpose:** Provide tamper-evident, verifiable audit trails.

**Components:**

**A. Per-Request Audit Logging**
> *"Every request and response is logged against the calling agent's identity."*

| Field | Description |
|-------|-------------|
| `request_id` | Unique identifier (W3C Trace Context) |
| `agent_did` | Calling agent's DID |
| `delegation_chain` | Full chain of authority |
| `tool` | Tool name and server |
| `arguments` | Tool arguments (redacted as needed) |
| `result` | Tool result (redacted as needed) |
| `timestamp` | Time of request |
| `signature` | Ed25519 signature of entire log entry |
| `constitutional_check` | Which invariants were verified |

**B. Cryptographic Audit Trail**
- Each log entry signed with agent's Ed25519 key
- Hash-chain linking entries (tamper-evident)
- Verifiable by third parties without trusting the audit system
- Based on Mi-Memory D²ACCI governance model

**C. W3C Trace Context Integration**
MCP 2026-07-28 requires **traceparent, tracestate, baggage** headers:
- Enables distributed tracing across agent → MCP → downstream systems
- Correlation across organizational boundaries
- End-to-end observability

**GAIA Syscall API Extension:**
```rust
// Audit operations
gaia.audit.log(entry: AuditEntry) -> AuditReceipt
gaia.audit.verify(receipt: AuditReceipt) -> bool
gaia.audit.trace(request_id: ID) -> Trace
gaia.audit.export(query: AuditQuery) -> AuditReport
```

#### Layer 4: Protocol Governance

**Purpose:** Ensure GAIA 2.0's MCP implementation follows the evolving standard.

**Components:**

**A. MCP 2026-07-28 Compliance**
- Stateless core (no session dependency)
- Feature lifecycle tracking (Active/Deprecated/Removed)
- Extensions framework for GAIA-specific capabilities

**B. GAIA Extensions Framework**
GAIA 2.0-specific MCP extensions (as opt-in extensions per SEP-2133):
- `gaia-auth` — DID-based authentication
- `gaia-audit` — Cryptographic audit logging
- `gaia-delegation` — Chain-of-authority verification
- `gaia-constitution` — Constitutional invariant checking

**C. Conformance Testing**
- GAIA 2.0 MCP implementation must pass MCP conformance suite (SEP-2484)
- GAIA-specific conformance tests for extensions
- Continuous integration with upstream MCP conformance

**D. Version Management**
- Track feature lifecycle status of all MCP features used
- Plan migrations with 12-month minimum window
- Deprecated features flagged in audit logs


## PART IV: INTEGRATION WITH GAIA 2.0 LAYERS

| GAIA 2.0 Layer | MCP Governance Integration |
|----------------|---------------------------|
| **L0 (Hardware)** | Hardware-attested DIDs; TEE-based key protection |
| **L1 (Kernel)** | Ed25519 signing of all MCP requests; zero-trust syscalls |
| **L3 (MemOS)** | Audit logs stored as episodic memory; governance decisions as semantic memory |
| **L4 (Orchestration)** | Policy evaluation before task execution; constitutional compliance checking |
| **L5 (Agents)** | Agent DIDs; delegation chains; tool permissions |
| **L6 (Interface)** | User consent UI; audit visibility; governance dashboard |


## PART V: IMPLEMENTATION ROADMAP

### Phase 1 — Identity Foundation (Months 1-3)

- [ ] Define `did:gaia:` method specification
- [ ] Implement W3C DID/VC issuance and verification
- [ ] Integrate AgentDID challenge-response mechanism
- [ ] Implement MCP-I-style delegation chain
- [ ] Create GAIA-specific VC schemas (capability, consent, delegation, compliance)

### Phase 2 — MCP 2026-07-28 Compliance (Months 4-6)

- [ ] Upgrade to MCP 2026-07-28 stateless core
- [ ] Implement feature lifecycle tracking
- [ ] Create GAIA extensions framework
- [ ] Pass MCP conformance suite (SEP-2484)
- [ ] Implement W3C Trace Context for observability

### Phase 3 — Policy Enforcement (Months 7-9)

- [ ] Implement AGT-style policy engine
- [ ] Define execution rings (Ring 0-3)
- [ ] Implement saga orchestration + kill switch
- [ ] Create tool-level permission system
- [ ] Integrate with GAIA Constitution for compliance checking

### Phase 4 — Audit & Observability (Months 10-12)

- [ ] Implement cryptographic audit logging
- [ ] Create hash-chain audit trail
- [ ] Implement audit verification
- [ ] Build governance dashboard for L6 Interface
- [ ] Integrate with SIEM systems

### Phase 5 — Production Deployment (Months 13-15)

- [ ] Deploy GAIA MCP Gateway with full governance stack
- [ ] Implement SCIM group synchronization
- [ ] Deploy Shadow AI detection for off-gateway MCP usage
- [ ] Complete NSA MCP security guidance compliance


## PART VI: KEY DIFFERENTIATORS — GAIA 2.0 MCP GOVERNANCE

| Feature | MCP Spec | AGT | MCP-I | Enterprise Gateway | **GAIA 2.0** |
|---------|----------|-----|-------|-------------------|---------------|
| **DID/VC Identity** | ❌ | ✅ | ✅ | ❌ | ✅ |
| **Delegation Chain** | ❌ | ❌ | ✅ | ❌ | ✅ |
| **Runtime Policy** | ❌ | ✅ | ❌ | ✅ | ✅ |
| **Cryptographic Audit** | ❌ | ❌ | ❌ | ❌ | ✅ |
| **Constitutional Compliance** | ❌ | ❌ | ❌ | ❌ | ✅ |
| **Zero-Trust by Default** | ⚠️ | ⚠️ | ✅ | ⚠️ | ✅ |
| **Open Standard** | ✅ | ✅ | ✅ | ❌ | ✅ |
| **Sovereign by Design** | ❌ | ❌ | ⚠️ | ❌ | ✅ |
| **GAIA Integration** | ❌ | ❌ | ❌ | ❌ | ✅ |


## PART VII: RISKS & MITIGATIONS

| Risk | Severity | Mitigation |
|------|----------|------------|
| **MCP governance still evolving** | High | Build on stable MCP 2026-07-28; contribute to SEP process |
| **DID/VC standards fragmentation** | Medium | Align with W3C, DIF, and IETF efforts; use `did:gaia:` as bridge |
| **Policy enforcement overhead** | Medium | AGT shows feasible; optimize for <10ms latency |
| **Audit log storage growth** | Medium | Tiered storage (MemOS L3); retention policies |
| **Cross-organizational verification** | High | MCP-I delegation chain + IETF DID framework address this |
| **Governance capture** | Medium | GAIA Constitution + Apache governance model |


## CONCLUSION

**The gap is closing rapidly.** What was a "governance gap" in August 2026 has been addressed by multiple concurrent efforts:

| Effort | Status | GAIA 2.0 Action |
|--------|--------|-----------------|
| **MCP 2026-07-28** | Released July 2026 | Adopt stateless core + extensions framework |
| **Agentic AI Foundation** | 170+ members | Participate in governance |
| **Microsoft AGT** | Open-source | Adopt/integrate policy engine |
| **MCP-I/DIF** | Donated March 2026 | Adopt DID/VC identity layer |
| **IETF DID-MCP Draft** | August 2026 | Contribute to standardization |
| **NSA MCP Guidance** | May 2026 | Implement all 23 security recommendations |

**The solution exists, but integration is required.** No single effort solves the complete governance problem. GAIA 2.0 must integrate:

1. **MCP-I/DIF** — Identity and delegation (W3C DIDs/VCs)
2. **Microsoft AGT** — Runtime policy enforcement
3. **MCP 2026-07-28** — Protocol governance (stateless core, extensions)
4. **GAIA-specific** — Constitutional compliance, cryptographic audit, sovereignty

**The timing is right.** The pieces are coming together in real-time. GAIA 2.0 can be the **first sovereign MCP governance layer**—providing verifiable, auditable, constitutionally-compliant agent tool execution that works across organizational boundaries without sacrificing user sovereignty.

> *"The MCP 2026-07-28 revision... is the largest change since the protocol launched... The through-line: in a stateless world, governance is not paperwork produced alongside the system. It is load-bearing structure the system runs on."*


## QUICK REFERENCE

```
R#1.20 MCP GOVERNANCE & SECURITY MODEL — KEY FINDINGS

GAP: MCP has "no built-in governance" (as of August 2026)
SOLUTION: Four-layer governance architecture

Recommended Architecture:
- Layer 1: Identity & Delegation (W3C DIDs/VCs + MCP-I/DIF)
- Layer 2: Policy & Runtime (Microsoft AGT + execution rings)
- Layer 3: Audit & Observability (cryptographic audit + W3C Trace)
- Layer 4: Protocol Governance (MCP 2026-07-28 + extensions)

Key Standards & Frameworks:
- MCP 2026-07-28: Stateless core, feature lifecycle, extensions
- MCP-I/DIF: DID/VC-based identity and delegation
- Microsoft AGT: Runtime policy enforcement with DIDs/Ed25519
- IETF draft-xu-mcp-agent-did-framework: DID-based auth
- NSA MCP Security: 23 discrete security issues

GAIA-Specific Additions:
- Constitutional compliance checking
- Cryptographic audit trail (hash-chain)
- Sovereign by design (user-controlled identity)
- Zero-trust by default (every request verified)

Implementation Priority: CRITICAL — Required for L5 Agent Ecosystem security
Timeline: Phase 1 (Months 1-3): Identity foundation
```

---

*R#1.20 MCP Governance & Security Model Gap Analysis*
*GAIA 2.0 Research — September 9, 2026*
*License: Apache-2.0 | Open Source | Open Access*