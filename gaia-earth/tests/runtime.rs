use gaia_earth::*;

#[test]
fn what_now_demo_has_published_cadence() {
    let mut job = SimJob::demo_now("austin", 100);
    assert_eq!(job.mode, SimMode::WhatNow);
    assert_eq!(job.cadence_secs, 3600);
    job.tick(200);
    assert_eq!(job.last_tick, 100);
    job.tick(3800);
    assert_eq!(job.last_tick, 3800);
}

#[test]
fn what_next_writes_ensemble_members() {
    let job = SimJob {
        mode: SimMode::WhatNext,
        region: "austin".into(),
        horizon_days: 10,
        ensemble_size: 3,
        scale: GridScale::Global25km,
        cadence_secs: 0,
        last_tick: 0,
    };
    let run = job.run(&[]).unwrap();
    assert_eq!(run.members.len(), 3);
    assert!(run.members.iter().all(|m| m.source == SourceKind::Synthetic));
}

#[test]
fn what_if_does_not_overwrite_observations() {
    let obs = Observation::admit(
        SystemTwin::Atmosphere,
        SourceKind::Measured,
        21.0,
        Some(0.2),
        "degC",
    )
    .unwrap();
    let before = obs.clone();
    let job = SimJob {
        mode: SimMode::WhatIf,
        region: "austin".into(),
        horizon_days: 365,
        ensemble_size: 2,
        scale: GridScale::Global25km,
        cadence_secs: 0,
        last_tick: 0,
    };
    let _ = job.run(std::slice::from_ref(&obs)).unwrap();
    assert_eq!(obs, before);
}
