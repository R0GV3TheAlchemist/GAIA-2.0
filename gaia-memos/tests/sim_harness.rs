//! Simulation harness: prove Continuity contracts (Blueprint 59).
//! Mirrors the 6/6 in-process proof. No Screenpipe crate.

use gaia_memos::{
    CaptureConsent, Continuity, CubeType, Episode, EpisodeStore, FileRef, MemCube, MemOs,
    ScreenpipeStub,
};

fn tmp(name: &str) -> String {
    let p = std::env::temp_dir().join(name);
    let _ = std::fs::remove_file(&p);
    p.to_str().unwrap().to_string()
}

#[test]
fn sim_six_proofs() {
    // P1 consent-off denies capture
    let mut c = Continuity::new();
    let p1 = tmp("gaia_sim_p1.sqlite");
    c.attach_store(EpisodeStore::open(&p1).unwrap());
    let denied = c.remember_life(Episode {
        t_unix_ms: 0,
        text: "secret".into(),
        modality: "files".into(),
        snapshot_id: None,
    });
    assert!(denied.is_err(), "P1");
    assert!(c.ask_history("secret").is_empty(), "P1 ask");

    // P2 ingest + FTS CARE + working cube
    c.consent.files = true;
    let mut mem = MemOs::new();
    c.ingest_episode(
        Episode {
            t_unix_ms: 1,
            text: "open CARE.md DestinE draft".into(),
            modality: "files".into(),
            snapshot_id: None,
        },
        &mut mem,
    )
    .unwrap();
    assert!(
        c.ask_history("CARE").iter().any(|e| e.text.contains("CARE")),
        "P2 fts"
    );
    assert!(!mem.recall("CARE", 3).is_empty(), "P2 cube");

    // P3 forget_in hides FTS and cubes
    c.forget_in("open CARE.md DestinE draft", &mut mem).unwrap();
    assert!(c.ask_history("CARE").is_empty(), "P3 fts");
    assert!(mem.recall("CARE", 3).is_empty(), "P3 cube");

    // P4 A→B same UUID
    let mut a = MemOs::new();
    let id = a.put(MemCube::new(CubeType::Plaintext, "alice identity", "fixture"));
    let mut b = MemOs::new();
    assert_eq!(b.import(a.export_all()), 1, "P4 import");
    let restored = b.get(id).unwrap();
    assert_eq!(restored.id, id, "P4 uuid");
    assert_eq!(restored.content, "alice identity", "P4 content");

    // P5 hydrate files+step after restart
    let p5 = tmp("gaia_sim_p5.sqlite");
    {
        let mut c5 = Continuity::new();
        c5.attach_store(EpisodeStore::open(&p5).unwrap());
        c5.pause_world_with_files(
            "destinE memo",
            "DRAFTING_CARE",
            vec![FileRef {
                path: "Documents/CARE.md".into(),
                hash: "abc123".into(),
            }],
        );
    }
    let mut c5b = Continuity::new();
    c5b.attach_store(EpisodeStore::open(&p5).unwrap());
    let snap = c5b.restore_world("destinE memo").expect("P5 snap");
    assert_eq!(snap.current_step, "DRAFTING_CARE", "P5 step");
    assert_eq!(snap.open_files[0].path, "Documents/CARE.md", "P5 path");
    assert_eq!(snap.open_files[0].hash, "abc123", "P5 hash");

    // P6 screen stub nops unless consent.screen
    let stub = ScreenpipeStub;
    assert!(stub.capture(&CaptureConsent::all_off()).is_err(), "P6 off");
    let mut sc = CaptureConsent::all_off();
    sc.screen = true;
    let ep = stub.capture(&sc).unwrap();
    assert_eq!(ep.modality, "screen", "P6 on");

    let _ = std::fs::remove_file(&p1);
    let _ = std::fs::remove_file(&p5);
}
