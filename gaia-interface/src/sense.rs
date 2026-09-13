//! #29 local voice, vision, and mobile surfaces.
//! Default policy: audio and video do not leave the device.
//! Whisper.cpp, Piper, LLaVA, and Flutter are not wired.

use std::path::Path;

use crate::gateway::{HttpGateway, HttpRequest, HttpResponse};
use crate::orch::{ForwardedIntent, OrchestratorGateway};
use crate::session::{Session, SessionError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SensePolicy {
    pub audio_leaves_device: bool,
    pub video_leaves_device: bool,
}

impl Default for SensePolicy {
    fn default() -> Self {
        Self {
            audio_leaves_device: false,
            video_leaves_device: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SenseError {
    CloudDenied,
    MissingLocalFile(String),
    EmptyTranscript,
    Session(SessionError),
}

impl std::fmt::Display for SenseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CloudDenied => write!(f, "audio/video must stay on device unless opted in"),
            Self::MissingLocalFile(path) => write!(f, "local file missing: {path}"),
            Self::EmptyTranscript => write!(f, "spoken transcript is empty"),
            Self::Session(error) => write!(f, "{error}"),
        }
    }
}

impl From<SessionError> for SenseError {
    fn from(error: SessionError) -> Self {
        Self::Session(error)
    }
}

/// Local ASR stand-in. Does not load whisper.cpp.
pub struct LocalAsr {
    policy: SensePolicy,
}

impl LocalAsr {
    pub fn new(policy: SensePolicy) -> Self {
        Self { policy }
    }

    pub fn transcribe_local(&self, transcript: &str) -> Result<String, SenseError> {
        if self.policy.audio_leaves_device {
            return Err(SenseError::CloudDenied);
        }
        let text = transcript.trim();
        if text.is_empty() {
            return Err(SenseError::EmptyTranscript);
        }
        Ok(text.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalCaption {
    pub path: String,
    pub bytes: u64,
    pub text: String,
    pub left_device: bool,
}

/// Local vision stand-in. Does not load LLaVA or Moondream.
pub struct LocalVision {
    policy: SensePolicy,
}

impl LocalVision {
    pub fn new(policy: SensePolicy) -> Self {
        Self { policy }
    }

    pub fn describe_local(&self, path: &Path) -> Result<LocalCaption, SenseError> {
        if self.policy.video_leaves_device {
            return Err(SenseError::CloudDenied);
        }
        let meta = std::fs::metadata(path).map_err(|_| {
            SenseError::MissingLocalFile(path.display().to_string())
        })?;
        if !meta.is_file() {
            return Err(SenseError::MissingLocalFile(path.display().to_string()));
        }
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("image");
        Ok(LocalCaption {
            path: path.display().to_string(),
            bytes: meta.len(),
            text: format!("local image {name} bytes={} (on-device, no cloud)", meta.len()),
            left_device: false,
        })
    }
}

/// Mobile client against a self-hosted in-process node. Not Flutter.
pub struct MobileClient<'a> {
    session: &'a mut Session,
}

impl<'a> MobileClient<'a> {
    pub fn self_hosted(session: &'a mut Session) -> Self {
        Self { session }
    }

    pub fn submit_intent(&mut self, text: &str) -> HttpResponse {
        HttpGateway::new(self.session).handle(HttpRequest {
            method: "POST".into(),
            path: "/intent".into(),
            body: format!(r#"{{"text":{}}}"#, serde_json::to_string(text).unwrap_or_else(|_| "\"\".into())),
        })
    }
}

/// Spoken transcript + local image + mobile adapter on one developer box.
pub struct SenseSurface {
    policy: SensePolicy,
    gateway: OrchestratorGateway,
}

impl SenseSurface {
    pub fn developer() -> Result<Self, SenseError> {
        let mut gateway = OrchestratorGateway::local();
        gateway.boot_developer()?;
        Ok(Self {
            policy: SensePolicy::default(),
            gateway,
        })
    }

    pub fn policy(&self) -> SensePolicy {
        self.policy
    }

    pub fn speak(&mut self, transcript: &str) -> Result<ForwardedIntent, SenseError> {
        let text = LocalAsr::new(self.policy).transcribe_local(transcript)?;
        Ok(self.gateway.forward_intent(&text)?)
    }

    pub fn see(&self, path: &Path) -> Result<LocalCaption, SenseError> {
        LocalVision::new(self.policy).describe_local(path)
    }

    pub fn mobile_intent(&mut self, text: &str) -> HttpResponse {
        MobileClient::self_hosted(self.gateway.session_mut()).submit_intent(text)
    }
}
