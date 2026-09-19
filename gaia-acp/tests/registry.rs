use gaia_acp::*;

#[test]
fn unsigned_mcp_config_is_rejected() {
    let mut cfg = McpServerConfig::example_safe();
    assert!(lint_mcp_config(&cfg).is_ok());
    cfg.signed = false;
    assert_eq!(lint_mcp_config(&cfg), Err(ReasonCode::ConfigRejected));
}

#[test]
fn unpinned_still_rejected_when_signed() {
    let mut cfg = McpServerConfig::example_safe();
    cfg.version = "latest".into();
    assert_eq!(lint_mcp_config(&cfg), Err(ReasonCode::ConfigRejected));
}

#[test]
fn registry_lists_local_aip_names_only() {
    let mut reg = LocalRegistry::new();
    assert!(reg
        .register_local("reader", "./gaia-spec/examples/reader.aip.json")
        .is_ok());
    assert_eq!(reg.list_names(), vec!["reader".to_string()]);
    assert_eq!(
        reg.register_local("remote", "https://example.invalid/agent.json"),
        Err(ReasonCode::ConfigRejected)
    );
    assert_eq!(
        reg.register_local("mdns", "mdns://_mcp._tcp.local/agent"),
        Err(ReasonCode::ConfigRejected)
    );
    assert_eq!(reg.list_names(), vec!["reader".to_string()]);
}
