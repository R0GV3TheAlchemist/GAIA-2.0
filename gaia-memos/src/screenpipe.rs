//! Screenpipe adapter stub (Blueprint 59).
//! Does not talk to screenpipe. Returns a fake accessibility line only if `consent.screen`.
//! License: Apache-2.0

use crate::continuity::{CaptureConsent, Episode};

pub struct ScreenpipeStub;

impl ScreenpipeStub {
    pub fn capture(&self, consent: &CaptureConsent) -> Result<Episode, &'static str> {
        if !consent.screen {
            return Err("capture denied: screen consent false");
        }
        Ok(Episode {
            t_unix_ms: 0,
            text: "[stub accessibility] focused window text".into(),
            modality: "screen".into(),
            snapshot_id: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nops_when_screen_off() {
        let stub = ScreenpipeStub;
        assert!(stub.capture(&CaptureConsent::all_off()).is_err());
    }

    #[test]
    fn stub_line_when_screen_on() {
        let mut c = CaptureConsent::all_off();
        c.screen = true;
        let ep = ScreenpipeStub.capture(&c).unwrap();
        assert_eq!(ep.modality, "screen");
        assert!(ep.text.contains("stub accessibility"));
    }
}
