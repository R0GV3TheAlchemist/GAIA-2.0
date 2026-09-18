use crate::types::ProposedAction;

/// Narrow fake adapter. Denied calls must never reach execute.
pub trait FakeAdapter {
    fn execute(&mut self, action: &ProposedAction) -> Result<(), &'static str>;
}

#[derive(Debug, Default)]
pub struct RecordingAdapter {
    pub executed: Vec<String>,
}

impl RecordingAdapter {
    pub fn count(&self) -> usize {
        self.executed.len()
    }
}

impl FakeAdapter for RecordingAdapter {
    fn execute(&mut self, action: &ProposedAction) -> Result<(), &'static str> {
        self.executed.push(action.request_hash());
        Ok(())
    }
}
