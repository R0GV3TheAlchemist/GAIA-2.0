//! [`ExecutionEngine`] — the 12-stage GAIA intent execution pipeline.
//!
//! Stages 4–12 run once per task tier in topological order; tasks within a
//! tier are dispatched through [`crate::execution::fanout::run_fanout`] (#937).
