use gaia_ukd::{publish_tek, GaianKnowledge, TekStore, UkdError};

#[test]
fn tek_stays_empty_without_agreement() {
    let store = TekStore::new();
    assert!(store.is_empty());
    assert_eq!(publish_tek(false, "story").unwrap_err(), UkdError::NoAgreement);
}

#[test]
fn gaian_can_store_local_knowledge_state() {
    let mut state = GaianKnowledge::local_only();
    state.known.push("linear-algebra".into());
    state.learning.push("probability".into());
    state.frontier.push("quantum-computing".into());
    state.gaps.push("error-correction".into());
    assert_eq!(state.known.len(), 1);
}
