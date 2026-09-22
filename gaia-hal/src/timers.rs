//! Timer subsystem — monotonic nanosecond-resolution clock.

use std::time::{Duration, Instant};

/// A monotonic clock backed by the OS's highest-resolution timer.
///
/// Uses `std::time::Instant` under the hood, which maps to
/// `clock_gettime(CLOCK_MONOTONIC)` on Linux and `mach_absolute_time`
/// on macOS — both nanosecond-resolution on modern hardware.
#[derive(Debug, Clone)]
pub struct MonotonicClock {
    origin: Instant,
}

impl MonotonicClock {
    /// Create a new clock, anchoring the zero point at the current instant.
    pub fn new() -> Self {
        Self { origin: Instant::now() }
    }

    /// Nanoseconds elapsed since this clock was created.
    pub fn elapsed_ns(&self) -> u64 {
        let d: Duration = self.origin.elapsed();
        d.as_nanos() as u64
    }

    /// Current timestamp as nanoseconds since the clock's origin.
    pub fn now_ns(&self) -> u64 {
        self.elapsed_ns()
    }

    /// Best-effort clock resolution in nanoseconds.
    ///
    /// On all Tier 1+ targets this is 1 ns (the `Instant` API guarantees
    /// at least nanosecond representation). Actual hardware resolution
    /// varies; 1–100 ns is typical on modern x86_64/aarch64.
    pub fn resolution_ns() -> u64 {
        1 // std::time::Instant has nanosecond precision
    }
}

impl Default for MonotonicClock {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clock_resolution_is_nanosecond() {
        assert_eq!(MonotonicClock::resolution_ns(), 1);
    }

    #[test]
    fn elapsed_ns_is_non_negative_and_advances() {
        let clk = MonotonicClock::new();
        let t0 = clk.elapsed_ns();
        // Spin briefly to let time pass.
        std::thread::sleep(std::time::Duration::from_millis(1));
        let t1 = clk.elapsed_ns();
        assert!(t1 >= t0, "clock must be monotone");
        // At least 1 µs should have elapsed in 1 ms sleep.
        assert!(t1 - t0 >= 1_000, "expected at least 1 µs elapsed");
    }
}
