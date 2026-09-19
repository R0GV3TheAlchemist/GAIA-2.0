//! #453 scale-layer bind. Calls APIs already on main.
use gaia_sos::{
    live_mcp, live_slurm, refuse_consciousness_runtime, second_kernel, sentience, sos_v1_tagged,
};

#[test]
fn scale_sos_refuses_planet_os_mind() {
    assert!(!sos_v1_tagged());
    assert!(!second_kernel());
    assert!(!sentience());
    assert!(!live_mcp());
    assert!(!live_slurm());
    assert!(refuse_consciousness_runtime().is_err());
}
