use gaia_earth::*;

#[test]
fn drill_exercises_simulated_only() {
    let lake = Lake::in_memory();
    let id = MemCubeId::from_bytes([7u8; 32]);
    let run = run_bleaching_drill(&lake, id);
    assert_eq!(run.steps.len(), 5);
    assert!(run.steps.iter().all(|s| s.status == "simulated"));
    assert!(!twin_v1_tagged());
    assert!(!live_ews_network());
}
