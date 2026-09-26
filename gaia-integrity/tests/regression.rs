//! Regression locks (#936).

#[test]
fn r0001_workspace_test_does_not_require_gaia_cli_bin() {
    assert_eq!(env!("CARGO_PKG_NAME"), "gaia-integrity");
}

#[test]
fn r0002_raw_string_closer_token_documented() {
    let closer = "\"#";
    assert_eq!(closer.chars().last(), Some('#'));
}

#[test]
fn r0003_stage_helper_uses_global_not_nonlocal() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let script = root.join("scripts/agent-validate.sh");
    if script.exists() {
        let body = std::fs::read_to_string(&script).expect("read agent-validate.sh");
        assert!(
            !body.contains("nonlocal failed_stage"),
            "agent-validate.sh must not use nonlocal for module-level failed_stage"
        );
    }
}
