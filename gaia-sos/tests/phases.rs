use gaia_sos::{live_containerd, live_mcp, live_slurm, live_whisper, sos_v1_tagged};

#[test]
fn twenty_one_to_thirty_are_fixtures() {
    assert!(!live_mcp());
    assert!(!live_containerd());
    assert!(!live_whisper());
    assert!(!live_slurm());
    assert!(!sos_v1_tagged());
}
