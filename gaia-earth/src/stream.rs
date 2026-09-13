//! #43 streaming skeleton. In-process topics. Not Kafka, Flink, or Iceberg.

use crate::{Observation, SourceKind, SystemTwin, TwinError};

#[derive(Debug, Clone, PartialEq)]
pub struct StreamEvent {
    pub topic: String,
    pub observation: Observation,
    pub quality: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadLetter {
    pub topic: String,
    pub reason: String,
}

#[derive(Debug, Clone, Default)]
pub struct StreamMetrics {
    pub published: u64,
    pub consumed: u64,
    pub dead_letters: u64,
}

impl StreamMetrics {
    pub fn lag(&self) -> u64 {
        self.published.saturating_sub(self.consumed)
    }
}

#[derive(Debug, Clone, Default)]
pub struct StreamBus {
    events: Vec<StreamEvent>,
    dead: Vec<DeadLetter>,
    offset: usize,
    metrics: StreamMetrics,
}

impl StreamBus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn topic(system: SystemTwin) -> &'static str {
        match system {
            SystemTwin::Atmosphere => "stream.atmosphere",
            SystemTwin::Ocean => "stream.ocean",
            SystemTwin::Land => "stream.land",
            SystemTwin::Biosphere => "stream.biosphere",
            SystemTwin::Cryosphere => "stream.cryosphere",
            SystemTwin::Lithosphere => "stream.lithosphere",
            SystemTwin::Anthroposphere => "stream.anthroposphere",
            SystemTwin::Technosphere => "stream.technosphere",
            SystemTwin::Magnetosphere => "stream.magnetosphere",
        }
    }

    pub fn publish(
        &mut self,
        system: SystemTwin,
        source: SourceKind,
        value: f64,
        uncertainty: Option<f64>,
        unit: &str,
    ) -> Result<(), TwinError> {
        let topic = Self::topic(system).to_string();
        match Observation::admit(system, source, value, uncertainty, unit) {
            Ok(observation) => {
                self.events.push(StreamEvent {
                    topic,
                    observation,
                    quality: "L1Screened".into(),
                });
                self.metrics.published += 1;
                Ok(())
            }
            Err(err) => {
                self.dead.push(DeadLetter {
                    topic,
                    reason: format!("{err}"),
                });
                self.metrics.dead_letters += 1;
                Err(err)
            }
        }
    }

    pub fn consume(&mut self) -> Option<StreamEvent> {
        if self.offset >= self.events.len() {
            return None;
        }
        let event = self.events[self.offset].clone();
        self.offset += 1;
        self.metrics.consumed += 1;
        Some(event)
    }

    pub fn checkpoint(&self) -> StreamBus {
        self.clone()
    }

    pub fn restore(saved: StreamBus) -> StreamBus {
        saved
    }

    pub fn metrics(&self) -> &StreamMetrics {
        &self.metrics
    }

    pub fn dead_letters(&self) -> &[DeadLetter] {
        &self.dead
    }
}
