//! Sensor subsystem — typed sensor bus and mock event source.

use std::sync::{Arc, Mutex};

/// Classification of a sensor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SensorKind {
    /// Microphone / audio input.
    Audio,
    /// Camera / video input.
    Video,
    /// Inertial measurement unit (accelerometer + gyroscope).
    Imu,
    /// LiDAR / depth camera.
    Lidar,
    /// Environmental sensor (temperature, humidity, CO₂, etc.).
    Environmental,
    /// Generic / unknown sensor type.
    Generic,
}

/// A single typed event from a sensor.
#[derive(Debug, Clone)]
pub struct SensorEvent {
    /// What kind of sensor produced this event.
    pub kind: SensorKind,
    /// Sensor identifier (e.g. device path or serial number).
    pub source_id: String,
    /// Timestamp in nanoseconds (monotonic).
    pub timestamp_ns: u64,
    /// Raw payload bytes. Format is sensor-specific.
    pub payload: Vec<u8>,
}

/// A bus that aggregates events from one or more sensor sources.
///
/// At Tier 0 a mock source is used. Real drivers inject events via
/// `SensorBus::push`.
#[derive(Debug, Default, Clone)]
pub struct SensorBus {
    queue: Arc<Mutex<Vec<SensorEvent>>>,
}

impl SensorBus {
    /// Create a new empty sensor bus.
    pub fn new() -> Self {
        Self { queue: Arc::new(Mutex::new(Vec::new())) }
    }

    /// Push a new event onto the bus (called by drivers or mock sources).
    pub fn push(&self, event: SensorEvent) {
        self.queue.lock().unwrap().push(event);
    }

    /// Drain all queued events and return them.
    pub fn drain(&self) -> Vec<SensorEvent> {
        let mut q = self.queue.lock().unwrap();
        std::mem::take(&mut *q)
    }

    /// Return the number of pending events without consuming them.
    pub fn pending(&self) -> usize {
        self.queue.lock().unwrap().len()
    }
}

/// Inject a batch of synthetic events into a bus for testing.
pub fn inject_mock_events(bus: &SensorBus, kind: SensorKind, count: usize) {
    for i in 0..count {
        bus.push(SensorEvent {
            kind,
            source_id:    format!("mock-{kind:?}-{i}"),
            timestamp_ns: i as u64 * 1_000_000,
            payload:      vec![i as u8; 4],
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bus_push_and_drain() {
        let bus = SensorBus::new();
        inject_mock_events(&bus, SensorKind::Imu, 3);
        assert_eq!(bus.pending(), 3);
        let events = bus.drain();
        assert_eq!(events.len(), 3);
        assert_eq!(bus.pending(), 0);
        for (i, ev) in events.iter().enumerate() {
            assert_eq!(ev.kind, SensorKind::Imu);
            assert_eq!(ev.timestamp_ns, i as u64 * 1_000_000);
        }
    }

    #[test]
    fn bus_accepts_typed_events_from_mock_source() {
        let bus = SensorBus::new();
        inject_mock_events(&bus, SensorKind::Environmental, 5);
        let drained = bus.drain();
        assert!(drained.iter().all(|e| e.kind == SensorKind::Environmental));
    }
}
