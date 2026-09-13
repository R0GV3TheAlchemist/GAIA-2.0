use gaia_earth::{GridScale, Observation, SimJob, SimMode, SourceKind, SystemTwin};

#[test]
fn what_next_writes_ensemble_members() {
    let job = SimJob {
        mode: SimMode::WhatNext,
        region: "Texas".into(),
        horizon_days: 10,
        ensemble_size: 3,
        scale: GridScale::National1km,
    };
    let run = job.run(&[]).unwrap();
    assert_eq!(run.members.len(), 3);
}

#[test]
fn what_if_does_not_overwrite_observations() {
    let observed = Observation::admit(
        SystemTwin::Atmosphere,
        SourceKind::Measured,
        21.0,
        Some(0.2),
        "degC",
    )
    .unwrap();
    let before = observed.clone();
    let job = SimJob {
        mode: SimMode::WhatIf,
        region: "Texas".into(),
        horizon_days: 365,
        ensemble_size: 1,
        scale: GridScale::Global25km,
    };
    let _ = job.run(&[observed.clone()]).unwrap();
    assert_eq!(observed, before);
    assert_eq!(observed.source, SourceKind::Measured);
}
