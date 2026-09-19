use gaia_acp::ReasonCode;

fn all_codes() -> [ReasonCode; 35] {
    [
        ReasonCode::Allow,
        ReasonCode::DefaultDeny,
        ReasonCode::ToolNotListed,
        ReasonCode::PathDenied,
        ReasonCode::HostDenied,
        ReasonCode::Expired,
        ReasonCode::BudgetExceeded,
        ReasonCode::Revoked,
        ReasonCode::EmergencyStop,
        ReasonCode::ApprovalRequired,
        ReasonCode::ApprovalMissing,
        ReasonCode::ApprovalMismatch,
        ReasonCode::ApprovalReplay,
        ReasonCode::ApprovalExpired,
        ReasonCode::EgressDenied,
        ReasonCode::SsrfDenied,
        ReasonCode::TraversalDenied,
        ReasonCode::ProtectedPath,
        ReasonCode::IdentityCreateDenied,
        ReasonCode::SecretDenied,
        ReasonCode::TierForbidden,
        ReasonCode::UntrustedAuthority,
        ReasonCode::Malformed,
        ReasonCode::StateInvalid,
        ReasonCode::ConfigRejected,
        ReasonCode::CrossAgent,
        ReasonCode::ContextMismatch,
        ReasonCode::NotYetValid,
        ReasonCode::DelegationDenied,
        ReasonCode::HashTampered,
        ReasonCode::NonceMismatch,
        ReasonCode::AutonomyCap,
        ReasonCode::ConfirmRequired,
        ReasonCode::VaultDumpDenied,
        ReasonCode::LifeSafetyDenied,
    ]
}

#[test]
fn audit_reason_strings_are_stable_codes_not_doctrine() {
    let banned = [
        "discernment",
        "coexistence",
        "schumann",
        "consciousness",
        "harmonic",
        "dimensional",
        "sentience",
    ];
    for code in all_codes() {
        let wire = code.as_str();
        assert!(
            wire.starts_with("GAIA_ACP_"),
            "{wire} is not a stable ACP code"
        );
        let low = wire.to_ascii_lowercase();
        for word in banned {
            assert!(!low.contains(word), "{wire} contains doctrine word {word}");
        }
    }
}
