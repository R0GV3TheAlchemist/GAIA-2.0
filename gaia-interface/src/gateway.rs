use serde_json::{json, Value};

use crate::session::{Session, SessionError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpResponse {
    pub status: u16,
    pub body: String,
}

/// In-process HTTP adapter over Session. No socket is bound.
pub struct HttpGateway<'a> {
    session: &'a mut Session,
}

impl<'a> HttpGateway<'a> {
    pub fn new(session: &'a mut Session) -> Self {
        Self { session }
    }

    pub fn handle(&mut self, req: HttpRequest) -> HttpResponse {
        match self.dispatch(&req) {
            Ok(value) => HttpResponse {
                status: 200,
                body: value.to_string(),
            },
            Err(error) => HttpResponse {
                status: status_for(&error),
                body: json!({"error": error.to_string()}).to_string(),
            },
        }
    }

    fn dispatch(&mut self, req: &HttpRequest) -> Result<Value, SessionError> {
        match (req.method.as_str(), req.path.as_str()) {
            ("POST", "/init") => {
                let profile = json_string(&req.body, "profile").unwrap_or_else(|| "developer".into());
                let profile = self.session.init(&profile)?;
                Ok(json!(profile))
            }
            ("POST", "/start") => {
                self.session.start()?;
                Ok(json!({"started": true}))
            }
            ("POST", "/intent") => {
                let text = json_string(&req.body, "text").unwrap_or_default();
                let record = self.session.declare_intent(&text)?;
                Ok(json!(record))
            }
            ("POST", path) if path.starts_with("/agents/") && path.ends_with("/revoke") => {
                let id = path
                    .trim_start_matches("/agents/")
                    .trim_end_matches("/revoke")
                    .trim_matches('/');
                let agent = self.session.revoke(id)?;
                Ok(json!(agent))
            }
            ("GET", "/agents") => Ok(json!(self.session.agents())),
            ("GET", "/status") => Ok(json!({
                "started": self.session.started(),
                "profile": self.session.profile(),
                "agents": self.session.agents().len(),
                "intents": self.session.intents().len(),
            })),
            _ => Err(SessionError::Usage("unknown route".into())),
        }
    }
}

fn json_string(body: &str, key: &str) -> Option<String> {
    let value: Value = serde_json::from_str(body).ok()?;
    value.get(key)?.as_str().map(str::to_owned)
}

fn status_for(error: &SessionError) -> u16 {
    match error {
        SessionError::NotInitialized | SessionError::NotStarted => 409,
        SessionError::UnknownProfile(_) | SessionError::UnknownAgent(_) => 404,
        SessionError::CloudDenied | SessionError::AlreadyRevoked(_) => 403,
        SessionError::Usage(_) => 400,
    }
}
