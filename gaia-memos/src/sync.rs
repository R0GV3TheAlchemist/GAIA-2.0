//! Cross-device memory synchronisation stub.
//!
//! Implements the export/import half of the cross-device sync spec from #724.
//! Full push/pull over the GAIA federation transport is tracked in #731
//! (GAIAN mobile integration).
//!
//! ## Version vector
//! Each `SyncBundle` carries a `version_vector: HashMap<String, u64>` where
//! the key is a node DID and the value is the last-seen sequence number from
//! that node.  The receiver merges by taking the higher version for each
//! cube (last-write-wins per cube UUID).
//!
//! ## Conflict resolution
//! Current policy: last-write-wins by `version` field on `MemCube`.
//! A CRDT approach (e.g. LWW-Element-Set) is noted for #731.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{MemCube, MemOs};

/// A portable bundle of MemCubes for cross-device transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncBundle {
    /// DID of the node that produced this bundle.
    pub origin_did:     String,
    /// Lamport-style version vector: node_did → sequence.
    pub version_vector: HashMap<String, u64>,
    /// Cubes being transferred.
    pub cubes:          Vec<MemCube>,
}

impl SyncBundle {
    pub fn new(origin_did: impl Into<String>, cubes: Vec<MemCube>) -> Self {
        let mut vv = HashMap::new();
        vv.insert(origin_did.to_owned().clone(), cubes.len() as u64);
        Self {
            origin_did: origin_did.into(),
            version_vector: vv,
            cubes,
        }
    }
}

/// Export all cubes from `mem` into a [`SyncBundle`].
pub fn export(mem: &MemOs, origin_did: impl Into<String>) -> SyncBundle {
    SyncBundle::new(origin_did, mem.export_all())
}

/// Merge a [`SyncBundle`] into `mem`, applying last-write-wins per cube UUID.
/// Returns the number of cubes actually written (new or updated).
pub fn merge(mem: &mut MemOs, bundle: SyncBundle) -> usize {
    let mut written = 0;
    for incoming in bundle.cubes {
        match mem.get(incoming.id) {
            Ok(existing) if existing.version >= incoming.version => {
                // Local is newer or equal — skip.
            }
            _ => {
                mem.put(incoming);
                written += 1;
            }
        }
    }
    written
}
