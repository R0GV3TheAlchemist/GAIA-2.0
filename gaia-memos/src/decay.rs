//! Ebbinghaus decay + Hebbian reinforcement for MemCube importance scores.
//!
//! ## Forgetting curve (Ebbinghaus)
//! Importance decays multiplicatively each scheduler tick for cubes that
//! have not been accessed since the last tick:
//!
//! ```text
//! importance_new = importance_old × factor   (factor ∈ (0, 1))
//! ```
//!
//! The default `factor` is **0.95** (≈ 5 % decay per tick).
//!
//! ## Hebbian reinforcement
//! Each `recall()` access increments `access_count` and adds +0.05 to
//! importance (capped at 1.0).  "Neurons that fire together wire together."
//! This is already implemented in `MemOs::recall()` and is preserved here
//! as documentation of the combined mechanism.
//!
//! ## Consolidation threshold
//! When a Working/Activation cube's importance exceeds **0.85** it is
//! promoted to the Episodic tier via `MemOs::migrate()`.

use crate::MemCube;

/// Default per-tick decay multiplier (Ebbinghaus).
pub const DEFAULT_DECAY_FACTOR: f32 = 0.95;

/// Minimum importance floor — cubes never decay below this.
pub const IMPORTANCE_FLOOR: f32 = 0.01;

/// Importance threshold above which a cube is promoted to a higher tier.
pub const PROMOTION_THRESHOLD: f32 = 0.85;

/// Apply one decay tick to a slice of cubes in-place.
/// Only cubes with `access_count == 0` since last tick are decayed;
/// cubes that were accessed are reinforced instead (+0.05, capped at 1.0).
/// Returns the number of cubes whose importance changed.
pub fn tick(cubes: &mut [MemCube], factor: f32) -> usize {
    let mut n = 0;
    for cube in cubes.iter_mut() {
        if cube.access_count == 0 {
            let before = cube.importance;
            cube.importance = (cube.importance * factor).max(IMPORTANCE_FLOOR);
            if (cube.importance - before).abs() > f32::EPSILON {
                n += 1;
            }
        } else {
            // Hebbian: already accessed this tick — reinforce and reset counter.
            cube.importance = (cube.importance + 0.05).min(1.0);
            cube.access_count = 0;
            n += 1;
        }
    }
    n
}

/// Return the ids of cubes whose importance has fallen below `threshold`
/// and should be archived or deleted.
pub fn below_threshold(cubes: &[MemCube], threshold: f32) -> Vec<uuid::Uuid> {
    cubes
        .iter()
        .filter(|c| c.importance < threshold)
        .map(|c| c.id)
        .collect()
}
