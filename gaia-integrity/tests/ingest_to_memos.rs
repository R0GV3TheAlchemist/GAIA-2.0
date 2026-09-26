//! Contract: gaia-ingest → gaia-memos (#935).
//! A DocumentChunk text can be stored and recalled as a MemCube.

use gaia_ingest::embed::PassthroughEmbedder;
use gaia_ingest::IngestPipeline;
use gaia_memos::{CubeType, MemCube, MemOs};

#[test]
fn ingest_chunk_is_recallable_from_memos() {
    let path = std::env::temp_dir().join("gaia-integrity-memos-fixture.md");
    std::fs::write(&path, "# Title\n\nThe river is blue.\n").expect("write fixture");
    let pipeline = IngestPipeline {
        embedder: Some(Box::new(PassthroughEmbedder)),
        ..Default::default()
    };
    let chunks = pipeline.from_path(&path).expect("ingest");
    assert!(!chunks.is_empty());
    let mut mem = MemOs::new();
    let cube = MemCube::new(CubeType::Semantic, chunks[0].text.clone(), chunks[0].source.clone());
    mem.put(cube);
    let hits = mem.recall("river blue", 3);
    assert!(!hits.is_empty(), "memos must recall ingested chunk text");
    assert!(hits.iter().any(|(_, c)| c.content.contains("river")));
}
