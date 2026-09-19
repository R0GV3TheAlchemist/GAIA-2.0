use gaia_ukd::path_to_quantum_computing;

#[test]
fn quantum_path_is_ordered_with_open_resources() {
    let path = path_to_quantum_computing();
    assert_eq!(path.goal, "quantum computing");
    assert!(path.steps.len() >= 3);
    assert!(path
        .steps
        .windows(2)
        .all(|w| w[0].difficulty <= w[1].difficulty));
    assert!(path
        .steps
        .iter()
        .all(|s| s.resource.starts_with("https://")));
}
