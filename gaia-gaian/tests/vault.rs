use gaia_gaian::{constitution_principles, ConsentScope, GaianError, Vault};

#[test]
fn unsigned_actions_are_rejected_and_wipe_clears_embeddings() {
    let mut vault = Vault::new();
    assert_eq!(vault.act(false, "embed").unwrap_err(), GaianError::Unsigned);
    vault.act(true, "embed").unwrap();
    assert_eq!(vault.embeddings().len(), 1);
    let receipt = vault.wipe();
    assert!(receipt.contains("deletion-receipt"));
    assert!(vault.embeddings().is_empty());
    assert!(vault
        .inspect_audit()
        .iter()
        .any(|e| e.event.contains("wiped")));
}

#[test]
fn cloud_sync_opt_in_is_audited() {
    let mut vault = Vault::new();
    vault.enable_cloud_sync(true);
    assert!(vault
        .inspect_audit()
        .iter()
        .any(|e| e.event.contains("cloud-sync=true")));
}

#[test]
fn consent_scopes_and_constitution_are_machine_readable() {
    assert_eq!(ConsentScope::all().len(), 7);
    assert_eq!(constitution_principles().len(), 10);
    assert!(constitution_principles().contains(&"child protection"));
}
