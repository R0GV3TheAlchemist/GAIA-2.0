use gaia_orchestrator::load_github_source_policy;

const DEFAULT_POLICY: &str =
    include_str!("../../gaia-spec/policies/github-source.default.json");

fn with_replaced(from: &str, to: &str) -> String {
    let next = DEFAULT_POLICY.replacen(from, to, 1);
    assert_ne!(next, DEFAULT_POLICY, "fixture splice missed: {from}");
    next
}

#[test]
fn default_checked_in_policy_is_accepted() {
    assert!(load_github_source_policy(DEFAULT_POLICY).is_ok());
}

#[test]
fn malformed_json_is_rejected() {
    assert!(load_github_source_policy("{").is_err());
    assert!(load_github_source_policy("not-json").is_err());
}

#[test]
fn mode_must_be_read_only() {
    let json = with_replaced("\"mode\": \"read-only\"", "\"mode\": \"read-write\"");
    assert!(load_github_source_policy(&json).is_err());
}

#[test]
fn audit_must_stay_enabled() {
    let json = with_replaced("\"enabled\": true", "\"enabled\": false");
    assert!(load_github_source_policy(&json).is_err());
}

#[test]
fn audit_chain_must_stay_required() {
    let json = with_replaced("\"chain_required\": true", "\"chain_required\": false");
    assert!(load_github_source_policy(&json).is_err());
}

#[test]
fn empty_allowed_repositories_are_rejected() {
    let json = with_replaced(
        "\"allowed_repositories\": [\"R0GV3TheAlchemist/GAIA-2.0\"]",
        "\"allowed_repositories\": []",
    );
    assert!(load_github_source_policy(&json).is_err());
}

#[test]
fn empty_allowed_paths_are_rejected() {
    let json = with_replaced(
        "\"allowed_paths\": [\n    \"Documents/**\",\n    \"Documents-2/**\",\n    \"gaia-spec/**\",\n    \"rfcs/**\",\n    \"README.md\",\n    \"CONTRIBUTING.md\",\n    \"SECURITY.md\",\n    \"GOVERNANCE.md\"\n  ]",
        "\"allowed_paths\": []",
    );
    assert!(load_github_source_policy(&json).is_err());
}

#[test]
fn limits_above_local_safety_caps_are_rejected() {
    for (from, to) in [
        ("\"max_file_bytes\": 1048576", "\"max_file_bytes\": 1048577"),
        ("\"max_response_bytes\": 1048576", "\"max_response_bytes\": 1048577"),
        ("\"max_results\": 50", "\"max_results\": 51"),
        ("\"requests_per_minute\": 60", "\"requests_per_minute\": 61"),
    ] {
        let json = with_replaced(from, to);
        assert!(
            load_github_source_policy(&json).is_err(),
            "limit splice should fail: {from}"
        );
    }
}
