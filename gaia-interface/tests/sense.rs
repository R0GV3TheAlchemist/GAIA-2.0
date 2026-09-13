//! #29: spoken intent, local vision, self-hosted mobile. No cloud media.

use gaia_interface::{LocalAsr, LocalVision, SensePolicy, SenseSurface};
use std::fs;
use std::io::Write;

#[test]
fn spoken_intent_reaches_the_orchestrator() {
    let mut surface = SenseSurface::developer().unwrap();
    let forwarded = surface.speak("research and summarize CARE").unwrap();
    assert!(forwarded.stored);
    assert_eq!(forwarded.session.text, "research and summarize CARE");
    assert_eq!(forwarded.session.events[0].kind, "admitted");
}

#[test]
fn default_policy_keeps_audio_and_video_on_device() {
    let surface = SenseSurface::developer().unwrap();
    assert!(!surface.policy().audio_leaves_device);
    assert!(!surface.policy().video_leaves_device);
    let cloud = SensePolicy {
        audio_leaves_device: true,
        video_leaves_device: true,
    };
    assert!(LocalAsr::new(cloud).transcribe_local("hello").is_err());
}

#[test]
fn vision_describes_a_local_image_without_upload() {
    let dir = std::env::temp_dir().join("gaia-sense-29");
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("frame.bin");
    let mut file = fs::File::create(&path).unwrap();
    file.write_all(&[0u8; 32]).unwrap();
    let caption = LocalVision::new(SensePolicy::default())
        .describe_local(&path)
        .unwrap();
    assert_eq!(caption.bytes, 32);
    assert!(!caption.left_device);
    assert!(caption.text.contains("on-device"));
    assert!(caption.text.contains("no cloud"));
}

#[test]
fn missing_local_image_is_refused() {
    let err = LocalVision::new(SensePolicy::default())
        .describe_local(std::path::Path::new("/no/such/gaia-frame.png"))
        .unwrap_err();
    assert!(format!("{err}").contains("missing"));
}

#[test]
fn mobile_client_submits_intent_to_self_hosted_node() {
    let mut surface = SenseSurface::developer().unwrap();
    let response = surface.mobile_intent("research CARE from phone");
    assert_eq!(response.status, 200);
    assert!(response.body.contains("research CARE from phone"));
}
