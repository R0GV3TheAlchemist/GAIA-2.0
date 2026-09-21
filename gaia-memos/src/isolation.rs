//! [`UserScope`] — per-user MemCube namespace.
//!
//! Every `MemOs` operation in production flows through a `UserScope`.
//! A scope binds a `user_did` string to a `MemOs` instance so that:
//!
//! - User A's cubes are stored under `user_did = did:key:gaia:...:A`
//! - User B cannot address, read, or overwrite User A's cubes
//! - The underlying `MemOs` HashMap is completely separate per scope
//!
//! When the persistent `MemOs::open(path, user_did)` constructor is used
//! the SQLite store enforces the `user_did` column as an additional WHERE
//! predicate on every query, providing defence-in-depth even if scopes are
//! accidentally shared.
//!
//! ## Thread safety
//! `UserScope` is `Send` but not `Sync` — wrap in `Arc<Mutex<UserScope>>`
//! for multi-threaded access (same pattern as `KernelHost`).

use crate::{CubeType, MemCube, MemOs, Result};
use uuid::Uuid;

/// A `MemOs` instance bound to a single user DID.
pub struct UserScope {
    pub user_did: String,
    pub mem:      MemOs,
}

impl UserScope {
    /// Create an isolated in-process scope for `user_did`.
    pub fn new(user_did: impl Into<String>) -> Self {
        Self {
            user_did: user_did.into(),
            mem:      MemOs::new(),
        }
    }

    /// Write a cube into this user's scope.
    pub fn put(&mut self, cube: MemCube) -> Uuid {
        self.mem.put(cube)
    }

    /// Read a cube — returns `Err` if the cube does not belong to this scope.
    /// Because each `UserScope` owns its own `MemOs`, cross-scope reads are
    /// structurally impossible; this method exists for explicit auditing.
    pub fn get(&self, id: Uuid) -> Result<MemCube> {
        self.mem.get(id)
    }

    /// Recall from this user's memory only.
    pub fn recall(&mut self, query: &str, k: usize) -> Vec<(f32, MemCube)> {
        self.mem.recall(query, k)
    }

    /// Convenience: put a plaintext cube with a given source tag.
    pub fn remember(&mut self, content: impl Into<String>, source: impl Into<String>) -> Uuid {
        self.put(MemCube::new(CubeType::Plaintext, content, source))
    }

    pub fn cube_count(&self) -> usize {
        self.mem.export_all().len()
    }
}
