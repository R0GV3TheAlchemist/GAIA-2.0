//! Contract: gaia-memos → gaia-aikd (#935).
//! A recalled cube is valid input to GenerationContext.

use gaia_aikd::GenerationContext;
use gaia_ingest::auth::{AgentId, ChunkMetadata};
use gaia_memos::{CubeType, MemCube, MemOs};

fn metadata() -> ChunkMetadata {
    ChunkMetadata {
        source: "memos".into(),
        date: "2026-09-26".into(),
        author: None,
        domain: "test".into(),
        confidence: 1.0,
        version: "1".into(),
        authorized_for: vec![],
    }
}

#[test]
fn recalled_cube_builds_generation_context() {
    let mut mem = MemOs::new();
    mem.put(MemCube::new(CubeType::Semantic, "The river is blue.", "memos"));
    let hits = mem.recall("river", 1);
    assert!(!hits.is_empty());
    let caller = AgentId::new("agent-test");
    let candidates = hits
        .into_iter()
        .map(|(_, cube)| (cube.content, None, 0, 0, metadata()))
        .collect();
    let ctx = GenerationContext::build(caller, candidates);
    assert!(!ctx.chunks.is_empty());
}
