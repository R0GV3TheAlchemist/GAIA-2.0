//! #42 first-wave connectors. Bounded fixtures. Not live Copernicus, Kafka, or GBIF HTTP.

use crate::{Lake, Observation, SourceKind, SystemTwin, TwinError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Connector {
    Copernicus,
    OpenWeather,
    Gbif,
    Usgs,
    Erddap,
}

impl Connector {
    pub fn all() -> [Connector; 5] {
        [Self::Copernicus, Self::OpenWeather, Self::Gbif, Self::Usgs, Self::Erddap]
    }
    pub fn topic(self) -> &'static str {
        match self {
            Self::Copernicus => "feeds.satellite",
            Self::OpenWeather => "feeds.weather",
            Self::Gbif => "feeds.biodiversity",
            Self::Usgs => "feeds.seismic",
            Self::Erddap => "feeds.ocean",
        }
    }
    pub fn license(self) -> &'static str {
        match self {
            Self::Copernicus | Self::Gbif => "CC-BY-4.0",
            Self::OpenWeather | Self::Usgs | Self::Erddap => "CC0-1.0",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FeedRecord {
    pub connector: Connector,
    pub topic: String,
    pub license: String,
    pub provenance: String,
    pub observation: Observation,
}

#[derive(Debug)]
pub struct RateLimit {
    pub max: u32,
    used: u32,
}

impl RateLimit {
    pub fn new(max: u32) -> Self {
        Self { max, used: 0 }
    }
    pub fn take(&mut self) -> Result<(), TwinError> {
        if self.used >= self.max {
            return Err(TwinError::FeedUnavailable);
        }
        self.used += 1;
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct FeedBus {
    live: bool,
}

impl FeedBus {
    pub fn fixtures() -> Self {
        Self { live: false }
    }
    pub fn pull_live(&self, _connector: Connector) -> Result<FeedRecord, TwinError> {
        let _ = self.live;
        Err(TwinError::FeedUnavailable)
    }
    pub fn pull_sample(&self, connector: Connector) -> Result<FeedRecord, TwinError> {
        let (system, value, unit) = match connector {
            Connector::Copernicus => (SystemTwin::Land, 0.42, "ndvi"),
            Connector::OpenWeather => (SystemTwin::Atmosphere, 21.4, "degC"),
            Connector::Gbif => (SystemTwin::Biosphere, 1.0, "count"),
            Connector::Usgs => (SystemTwin::Lithosphere, 4.1, "Mw"),
            Connector::Erddap => (SystemTwin::Ocean, 18.2, "degC"),
        };
        let observation = Observation::admit(system, SourceKind::Synthetic, value, Some(0.2), unit)?;
        Ok(FeedRecord {
            connector,
            topic: connector.topic().into(),
            license: connector.license().into(),
            provenance: format!("fixture sample; not a live {connector:?} pull"),
            observation,
        })
    }
    /// Bounded sample → lake. Missing host is not invented.
    pub fn ingest_sample(&self, lake: &mut Lake, connector: Connector) -> Result<(), TwinError> {
        let rec = self.pull_sample(connector)?;
        lake.write_raw(connector.topic(), rec.observation)?;
        lake.promote(connector.topic())?;
        Ok(())
    }
}
