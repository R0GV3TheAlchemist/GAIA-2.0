//! #27 in-process API gateway in front of the L4 orchestrator.
//! No Axum socket, WebSocket, or gRPC.

use gaia_memos::MemOs;
use gaia_orchestrator::{IntentEngine, IntentSigner};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::session::{IntentEvent, IntentRecord, Session, SessionError};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ForwardedIntent {
    pub session: IntentRecord,
    pub graph_id: Uuid,
    pub stored: bool,
}

/// Session plus local IntentEngine. Same text goes through CLI and this gateway.
pub struct OrchestratorGateway {
    session: Session,
    engine: IntentEngine,
    mem: MemOs,
    signer: IntentSigner,
}

impl Default for OrchestratorGateway {
    fn default() -> Self {
        Self::local()
    }
}

impl OrchestratorGateway {
    pub fn local() -> Self {
        Self {
            session: Session::new(),
            engine: IntentEngine::local_stub(),
            mem: MemOs::new(),
            signer: IntentSigner::generate(),
        }
    }

    pub fn session(&self) -> &Session {
        &self.session
    }

    pub fn session_mut(&mut self) -> &mut Session {
        &mut self.session
    }

    pub fn boot_developer(&mut self) -> Result<(), SessionError> {
        self.session.init("developer")?;
        self.session.start()?;
        Ok(())
    }

    /// CLI and HTTP both call this. Parses and stores on L4, records the session intent.
    pub fn forward_intent(&mut self, text: &str) -> Result<ForwardedIntent, SessionError> {
        let graph = self
            .engine
            .parse(text, &mut self.mem)
            .map_err(SessionError::Usage)?;
        let graph_id = self
            .engine
            .store(graph, &self.signer)
            .map_err(SessionError::Usage)?;
        self.engine
            .verify_stored(graph_id)
            .map_err(SessionError::Usage)?;
        let session = self.session.declare_intent(text)?;
        Ok(ForwardedIntent {
            session,
            graph_id,
            stored: true,
        })
    }

    /// Stream is the recorded events, in order. Not a WebSocket.
    pub fn stream(&self, intent_id: u64) -> Result<Vec<IntentEvent>, SessionError> {
        let record = self
            .session
            .intents()
            .iter()
            .find(|i| i.id == intent_id)
            .ok_or_else(|| SessionError::Usage(format!("unknown intent {intent_id}")))?;
        Ok(record.events.clone())
    }

    pub fn exec(&mut self, args: &[&str]) -> Result<String, SessionError> {
        match args.first().copied() {
            Some("intent") => {
                let text = args.get(1..).unwrap_or(&[]).join(" ");
                let forwarded = self.forward_intent(&text)?;
                Ok(format!(
                    "intent_id={} graph={} stored={} events={}",
                    forwarded.session.id,
                    forwarded.graph_id,
                    forwarded.stored,
                    forwarded.session.events.len()
                ))
            }
            _ => self.session.exec(args),
        }
    }
}
