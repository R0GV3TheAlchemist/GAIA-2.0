//! Contract tests for module boundaries that exist in this scaffold (#935).

use gaia_acp::tool_auth::{authorize, AgentId, ToolCall, ToolId, ToolPermissionTier};
use gaia_acp::{ToolAuditLog, ToolOutcome};
use gaia_agents::tool_registry::{ToolId as RegToolId, ToolPermissionTier as RegTier, ToolRegistration, ToolRegistry};
use gaia_aikd::GenerationContext;
use gaia_ingest::auth::{AgentId as IngestAgent, ChunkMetadata};
use gaia_ingest::embed::PassthroughEmbedder;
use gaia_ingest::IngestPipeline;
use gaia_runtime::{enforce_grounding, ChunkId, GroundedResponse};

fn metadata() -> ChunkMetadata {
    ChunkMetadata {
        source: "fixture.md".into(),
        date: "2026-09-25".into(),
        author: None,
        domain: "test".into(),
        confidence: 1.0,
        version: "1".into(),
        authorized_for: vec![],
    }
}

#[test]
fn ingest_to_aikd_markdown_reaches_generation_context() {
    let path = std::env::temp_dir().join("gaia-integrity-fixture.md");
    std::fs::write(&path, "# Title\n\nThe river is blue.\n").expect("write fixture");
    let pipeline = IngestPipeline {
        embedder: Some(Box::new(PassthroughEmbedder)),
        ..Default::default()
    };
    let chunks = pipeline.from_path(&path).expect("ingest");
    assert!(!chunks.is_empty());
    assert!(chunks[0].embedding.is_some());
    assert!(!chunks[0].text.is_empty());
    let caller = IngestAgent::new("agent-test");
    let candidates = chunks
        .into_iter()
        .map(|c| (c.text, None, 0, 0, metadata()))
        .collect();
    let ctx = GenerationContext::build(caller, candidates);
    assert!(!ctx.chunks.is_empty());
}

#[test]
fn aikd_to_runtime_grounded_response_shape() {
    let caller = IngestAgent::new("agent-test");
    let ctx = GenerationContext::build(
        caller,
        vec![("The river is blue.".into(), None, 0, 0, metadata())],
    );
    let source = ChunkId("chunk-1".into());
    let response = GroundedResponse::grounded(ctx.chunks[0].text.clone(), vec![source], 0.8)
        .expect("grounded response");
    enforce_grounding(true, &response.source_ids).expect("sources present");
    assert!(!response.content.is_empty());
}

#[test]
fn agents_to_acp_tool_call_writes_audit() {
    let mut registry = ToolRegistry::default();
    registry.register(ToolRegistration {
        tool_id: RegToolId("echo".into()),
        name: "echo".into(),
        permission_tier: RegTier::Unrestricted,
        description: "echo".into(),
    });
    assert!(registry.get(&RegToolId("echo".into())).is_some());
    let mut log = ToolAuditLog::default();
    let call = ToolCall {
        agent_id: AgentId("a1".into()),
        tool_id: ToolId("echo".into()),
        allowed_agents: None,
        tier: ToolPermissionTier::Unrestricted,
        explicit_approval: false,
        human_approval_token: None,
        params: "{}".into(),
    };
    let entry = authorize(&call, &mut log).expect("unrestricted permitted");
    assert_eq!(entry.outcome, ToolOutcome::Permitted);
    assert_eq!(log.len(), 1);
}

#[test]
fn invented_citation_fails_lookup() {
    let store = gaia_ingest::ChunkStore::new();
    let err = gaia_aikd::lookup_citations(&store, &["not-a-chunk".into()]).unwrap_err();
    assert!(matches!(err, gaia_aikd::CitationError::UnknownChunk(_)));
}

#[test]
fn ingested_citation_grounds_acp_invoke() {
    let text = "The treaty was signed in 1992.";
    let mut store = gaia_ingest::ChunkStore::new();
    store.record(gaia_ingest::ChunkId::from_text(text));
    let hex = gaia_aikd::id_for_text(text);
    let ids = gaia_aikd::lookup_citations(&store, &[hex.clone()]).expect("live lookup");
    let mut plane = gaia_acp::ControlPlane::start(1_700_000_000, "agent-a").unwrap();
    let mut manifest = gaia_acp::CapabilityManifest::local_reader("agent-a", 1_700_000_000);
    let action = gaia_acp::ProposedAction {
        agent_id: "agent-a".into(),
        tool: "local_read".into(),
        method: "call".into(),
        target: "docs/a.md".into(),
        action_class: gaia_acp::ActionClass::LocalRead,
        payload: text.into(),
        nonce: "nonce-agent-a".into(),
        gateway_id: "gateway-local".into(),
        server_id: "server-local".into(),
        resource_id: "repo-local".into(),
        wants_delegation: false,
    };
    let r = plane.invoke_grounded(
        &mut manifest,
        &action,
        None,
        None,
        gaia_acp::GroundingClaim::required(ids),
    );
    assert!(r.executed);
    assert_eq!(hex.len(), 64);
}

#[test]
fn persisted_store_reload_accepts_lookup() {
    use gaia_ingest::{embed::EmbeddingModel, FileChunkStore, HashingEmbedder};
    let text = "Persistent treaty clause.";
    let path = std::env::temp_dir().join("gaia-integrity-chunks.jsonl");
    let _ = std::fs::remove_file(&path);
    let embedder = HashingEmbedder::new();
    let vecs = embedder.embed(&[text]).unwrap();
    assert_eq!(vecs[0].dim(), 384);
    {
        let mut store = FileChunkStore::open(&path).unwrap();
        store.record(text, embedder.model_id(), Some(&vecs[0])).unwrap();
    }
    let store = FileChunkStore::open(&path).unwrap();
    let hex = gaia_aikd::id_for_text(text);
    let ids = gaia_aikd::lookup_citations(store.store(), &[hex]).expect("reloaded lookup");
    assert_eq!(ids.len(), 1);
}

#[test]
fn rank_hits_ground_acp_invoke() {
    use gaia_ingest::{embed::EmbeddingModel, FileChunkStore, HashingEmbedder};
    let climate = "the earth twin observes climate";
    let piano = "purple piano recipes";
    let path = std::env::temp_dir().join("gaia-integrity-rank-ground.jsonl");
    let _ = std::fs::remove_file(&path);
    let embedder = HashingEmbedder::new();
    let vecs = embedder.embed(&[climate, piano]).unwrap();
    let mut store = FileChunkStore::open(&path).unwrap();
    store.record(climate, embedder.model_id(), Some(&vecs[0])).unwrap();
    store.record(piano, embedder.model_id(), Some(&vecs[1])).unwrap();
    let hits = gaia_aikd::rank_persisted("earth twin climate", &store, &embedder, 1).unwrap();
    assert_eq!(hits[0].text, climate);
    let ids = gaia_aikd::citations_from_hits(store.store(), &hits).expect("ranked ids exist");
    let mut plane = gaia_acp::ControlPlane::start(1_700_000_000, "agent-a").unwrap();
    let mut manifest = gaia_acp::CapabilityManifest::local_reader("agent-a", 1_700_000_000);
    let action = gaia_acp::ProposedAction {
        agent_id: "agent-a".into(),
        tool: "local_read".into(),
        method: "call".into(),
        target: "docs/a.md".into(),
        action_class: gaia_acp::ActionClass::LocalRead,
        payload: climate.into(),
        nonce: "nonce-agent-a".into(),
        gateway_id: "gateway-local".into(),
        server_id: "server-local".into(),
        resource_id: "repo-local".into(),
        wants_delegation: false,
    };
    let r = plane.invoke_grounded(
        &mut manifest,
        &action,
        None,
        None,
        gaia_acp::GroundingClaim::required(ids),
    );
    assert!(r.executed);
}
