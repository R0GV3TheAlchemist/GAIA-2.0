//! #503 listed shelf. Existing APIs only. No PQC crate.

use gaia_acp::{from_invoke, refuse_live_supabase, ClaimClass, ReasonCode, TraceKind};

#[test]
fn live_sink_stays_refused() {
    assert!(refuse_live_supabase().is_err());
}

#[test]
fn prohibited_claim_cannot_allow() {
    let ev = from_invoke(
        TraceKind::Allow,
        1,
        "agent-a",
        "intent-1",
        "corr-1",
        ReasonCode::UntrustedAuthority,
        "hash",
        ClaimClass::Prohibited,
    );
    assert_eq!(ev.kind, TraceKind::Deny);
    assert_eq!(ev.claim_class, ClaimClass::Prohibited);
}
