//! #72 GAIAN-to-GAIAN channel. Signed text only. No raw biometrics.

use crate::GaianError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelCap {
    Calendar,
    Text,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Envelope {
    pub cap: ChannelCap,
    pub body: String,
}

#[derive(Debug, Default)]
pub struct Channel {
    mutual: bool,
    open: bool,
    mailbox: Vec<Envelope>,
    log: Vec<String>,
}

impl Channel {
    pub fn consent_both(&mut self, a: bool, b: bool) {
        self.mutual = a && b;
        self.open = self.mutual;
        self.log.push(format!("mutual={}", self.mutual));
    }

    pub fn send(&mut self, signed: bool, env: Envelope) -> Result<(), GaianError> {
        if !self.mutual || !self.open {
            return Err(GaianError::NoConsent);
        }
        if !signed {
            return Err(GaianError::Unsigned);
        }
        let lower = env.body.to_ascii_lowercase();
        if lower.contains("face mesh")
            || lower.contains("health store")
            || lower.contains("voice raw")
        {
            self.log.push("biometric refused".into());
            return Err(GaianError::ThirdPartyLikeness);
        }
        self.mailbox.push(env);
        self.log.push("sent".into());
        Ok(())
    }

    pub fn inbox(&self) -> &[Envelope] {
        &self.mailbox
    }

    pub fn audit(&self) -> &[String] {
        &self.log
    }

    pub fn revoke(&mut self) {
        self.open = false;
        self.log.push("revoked".into());
    }
}
