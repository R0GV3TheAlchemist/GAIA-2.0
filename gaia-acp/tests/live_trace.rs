use gaia_acp::*;

fn ev(kind: TraceKind, reason: ReasonCode) -> TraceEvent {
    from_invoke(
        kind,
        1,
        "agent-a",
        "intent-a",
        "corr-a",
        reason,
        "hash-a",
        ClaimClass::Established,
    )
}

#[test]
fn default_mode_does_not_forward() {
    let mut t = RecordingLiveTransport::default();
    let err = try_forward(&LiveTraceConfig::off(), &mut t, &ev(TraceKind::Allow, ReasonCode::Allow));
    assert_eq!(err, Err(LiveSendError::Disabled));
    assert!(t.rows.is_empty());
    assert!(refuse_live_supabase().is_err());
}

#[test]
fn mapped_row_matches_deployed_columns() {
    let row = map_row(&ev(TraceKind::Deny, ReasonCode::ConfirmRequired));
    assert_eq!(row.event, "deny");
    assert_eq!(row.gaian_id.as_deref(), Some("agent-a"));
    assert_eq!(row.correlation_id.as_deref(), Some("corr-a"));
    assert!(row.canon_refs.is_empty());
    assert_eq!(row.inputs, serde_json::json!({}));
    assert_eq!(row.outputs, serde_json::json!({}));
    assert_eq!(row.error.as_deref(), Some(ReasonCode::ConfirmRequired.as_str()));
    assert_eq!(row.meta["schema"], "gaia.trace_events.v1");
    assert_eq!(row.meta["reason_code"], ReasonCode::ConfirmRequired.as_str());
}

#[test]
fn allow_row_has_empty_error() {
    let row = map_row(&ev(TraceKind::Allow, ReasonCode::Allow));
    assert_eq!(row.event, "allow");
    assert!(row.error.is_none());
    assert_eq!(row.inputs, serde_json::json!({}));
    assert_eq!(row.outputs, serde_json::json!({}));
}

#[test]
fn test_boundary_records_without_touching_local_on_failure() {
    let mut local = MemoryTraceSink::default();
    let event = ev(TraceKind::Kill, ReasonCode::EmergencyStop);
    local.emit(event.clone());
    let mut t = RecordingLiveTransport {
        fail_next: true,
        ..Default::default()
    };
    let live = try_forward(&LiveTraceConfig::mapped_test(), &mut t, &event);
    assert_eq!(live, Err(LiveSendError::Transport));
    assert_eq!(local.events.len(), 1);
    assert!(t.rows.is_empty());
}

#[test]
fn test_boundary_accepts_mapped_row() {
    let mut t = RecordingLiveTransport::default();
    try_forward(
        &LiveTraceConfig::mapped_test(),
        &mut t,
        &ev(TraceKind::Allow, ReasonCode::Allow),
    )
    .unwrap();
    assert_eq!(t.rows.len(), 1);
    assert_eq!(t.rows[0].event, "allow");
}

#[test]
fn debug_blob_has_no_credential_shape() {
    let cfg = LiveTraceConfig::mapped_test();
    assert!(credential_is_absent(&format!("{cfg:?}")));
    let row = map_row(&ev(TraceKind::Allow, ReasonCode::Allow));
    assert!(credential_is_absent(&format!("{row:?}")));
}
