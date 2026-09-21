//! Canonical ingestion schema — slice 1 of #729.
//!
//! `NormalizedObservation` is the single record type shared by every
//! adapter (Copernicus, NOAA, GBIF, SensorThings, community nodes).
//! It is intentionally broad: optional `value`/`unit` fields mean a
//! record can represent a point measurement *or* a metadata envelope
//! for a raster scene stored as a `RawArtifactRef`.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use gaia_earth::{SourceKind, SystemTwin};

use crate::{
    artifact::RawArtifactRef,
    provenance::ProvenanceReceipt,
};

// ── Data source ─────────────────────────────────────────────────────────────

/// Every external data source named in #729.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataSource {
    /// Copernicus Data Space (EU — Sentinel-1/2/3/5P, ERA5).
    Copernicus,
    /// NOAA (weather, ocean, atmosphere, climate).
    Noaa,
    /// NASA Earth Observing System (MODIS, VIIRS, GEDI, …).
    Nasa,
    /// USGS (geology, hydrology, land use, seismic).
    Usgs,
    /// GBIF (Global Biodiversity Information Facility — 2.5B occurrence records).
    Gbif,
    /// iNaturalist (citizen-science biodiversity observations).
    INaturalist,
    /// OGC SensorThings API IoT sensor network.
    SensorThings,
    /// Home Node — personal environmental sensor.
    HomeNode,
    /// Community Node — neighbourhood / municipal sensor cluster.
    CommunityNode,
    /// Ocean Node — marine instrument buoy / Argo float.
    OceanNode,
    /// Commercial satellite feed (Planet, Maxar, Airbus, …).
    SatelliteCommercial,
}

impl DataSource {
    /// Default SPDX license for this source's open data tier.
    pub fn default_license(self) -> &'static str {
        match self {
            Self::Copernicus              => "CC-BY-4.0",
            Self::Noaa                    => "CC0-1.0",
            Self::Nasa                    => "CC0-1.0",
            Self::Usgs                    => "CC0-1.0",
            Self::Gbif                    => "CC-BY-4.0",
            Self::INaturalist             => "CC-BY-NC-4.0",
            Self::SensorThings            => "CC-BY-4.0",
            Self::HomeNode                => "CC-BY-4.0",
            Self::CommunityNode           => "CC-BY-4.0",
            Self::OceanNode               => "CC0-1.0",
            Self::SatelliteCommercial     => "proprietary",
        }
    }

    /// Human-readable name for logging and UI.
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Copernicus          => "Copernicus Data Space",
            Self::Noaa                => "NOAA",
            Self::Nasa                => "NASA EOS",
            Self::Usgs                => "USGS",
            Self::Gbif                => "GBIF",
            Self::INaturalist         => "iNaturalist",
            Self::SensorThings        => "OGC SensorThings",
            Self::HomeNode            => "Home Node",
            Self::CommunityNode       => "Community Node",
            Self::OceanNode           => "Ocean Node",
            Self::SatelliteCommercial => "Commercial Satellite",
        }
    }
}

// ── Observation kind ─────────────────────────────────────────────────────────

/// The structural kind of a normalized observation record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservationKind {
    /// A scalar measurement at a point in space and time (e.g. temperature, CO₂ ppm).
    PointMeasurement,
    /// A remote-sensing raster scene (Sentinel-2 tile, MODIS granule, …).
    RasterScene,
    /// A biodiversity occurrence record (GBIF / iNaturalist taxon + location).
    OccurrenceRecord,
    /// A moving platform track (ship AIS, aircraft, Argo float trajectory).
    Track,
    /// A spatially gridded field (ERA5 reanalysis, CMIP6 output, …).
    GriddedField,
    /// An alert or threshold-crossing event (AdvanTip EWI, sensor fault, …).
    Alert,
}

// ── Geometry ─────────────────────────────────────────────────────────────────

/// A WGS-84 point.  Latitude ∈ [−90, 90], longitude ∈ [−180, 180].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoPoint {
    pub lat: f64,
    pub lon: f64,
}

impl GeoPoint {
    /// Validates coordinate ranges and returns the point.
    pub fn new(lat: f64, lon: f64) -> Result<Self, GeoError> {
        if !(-90.0..=90.0).contains(&lat) {
            return Err(GeoError::LatOutOfRange(lat));
        }
        if !(-180.0..=180.0).contains(&lon) {
            return Err(GeoError::LonOutOfRange(lon));
        }
        Ok(Self { lat, lon })
    }

    /// Returns `true` if both coordinates are finite.
    pub fn is_finite(&self) -> bool {
        self.lat.is_finite() && self.lon.is_finite()
    }
}

/// A WGS-84 axis-aligned bounding box.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoBBox {
    pub min_lon: f64,
    pub min_lat: f64,
    pub max_lon: f64,
    pub max_lat: f64,
}

impl GeoBBox {
    pub fn new(
        min_lon: f64,
        min_lat: f64,
        max_lon: f64,
        max_lat: f64,
    ) -> Result<Self, GeoError> {
        if min_lon >= max_lon {
            return Err(GeoError::BBoxLonInverted { min_lon, max_lon });
        }
        if min_lat >= max_lat {
            return Err(GeoError::BBoxLatInverted { min_lat, max_lat });
        }
        if !(-180.0..=180.0).contains(&min_lon) || !(-180.0..=180.0).contains(&max_lon) {
            return Err(GeoError::LonOutOfRange(min_lon));
        }
        if !(-90.0..=90.0).contains(&min_lat) || !(-90.0..=90.0).contains(&max_lat) {
            return Err(GeoError::LatOutOfRange(min_lat));
        }
        Ok(Self { min_lon, min_lat, max_lon, max_lat })
    }

    /// Returns the centre point of the bounding box.
    pub fn centre(&self) -> GeoPoint {
        GeoPoint {
            lat: (self.min_lat + self.max_lat) / 2.0,
            lon: (self.min_lon + self.max_lon) / 2.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum GeoError {
    #[error("latitude {0} out of range [-90, 90]")]
    LatOutOfRange(f64),
    #[error("longitude {0} out of range [-180, 180]")]
    LonOutOfRange(f64),
    #[error("bbox lon inverted: min_lon={min_lon} >= max_lon={max_lon}")]
    BBoxLonInverted { min_lon: f64, max_lon: f64 },
    #[error("bbox lat inverted: min_lat={min_lat} >= max_lat={max_lat}")]
    BBoxLatInverted { min_lat: f64, max_lat: f64 },
}

// ── Canonical observation record ─────────────────────────────────────────────

/// The canonical record shared by every ingestion adapter.
///
/// Fields are intentionally broad so that a point measurement (NOAA
/// temperature), a raster scene (Copernicus Sentinel-2 tile), and a
/// biodiversity occurrence record (GBIF taxon + location) all fit the
/// same type without forcing fake uniformity.
///
/// - `value` / `unit` / `uncertainty` are `None` for raster scenes and
///   occurrence records where the payload lives in `artifact`.
/// - `point` and `bbox` are both optional; a gridded field has `bbox`,
///   a station measurement has `point`, a track has neither (the
///   geometry lives in `artifact`).
/// - `attributes` is an open BTreeMap for source-native metadata
///   (e.g. Sentinel-2 scene cloud cover, GBIF taxon rank, Argo float ID)
///   that does not fit a fixed schema column.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedObservation {
    /// UUID v4 assigned at ingestion time.
    pub id: String,
    /// Which Earth system component this observation describes.
    pub system: SystemTwin,
    /// Measured or synthetic.
    pub source_kind: SourceKind,
    /// The external data source.
    pub source: DataSource,
    /// Structural kind of this record.
    pub kind: ObservationKind,
    /// Unix timestamp (seconds) of the observation.
    pub measured_at_unix: u64,
    /// Point geometry (present for station / occurrence records).
    pub point: Option<GeoPoint>,
    /// Bounding box (present for scenes and gridded fields).
    pub bbox: Option<GeoBBox>,
    /// Physical unit string (SI preferred, e.g. "K", "ppm", "m/s").
    pub unit: Option<String>,
    /// Scalar measurement value.
    pub value: Option<f64>,
    /// Measurement uncertainty (1-sigma, same unit as `value`).
    pub uncertainty: Option<f64>,
    /// Cryptographic provenance receipt — always present.
    pub provenance: ProvenanceReceipt,
    /// Reference to raw artifact stored in SFS (present for raster / track / occurrence).
    pub artifact: Option<RawArtifactRef>,
    /// Source-native metadata that does not fit a fixed schema column.
    pub attributes: BTreeMap<String, String>,
}

impl NormalizedObservation {
    /// Returns `true` if the record has at least one geometry.
    pub fn has_geometry(&self) -> bool {
        self.point.is_some() || self.bbox.is_some()
    }

    /// Returns `true` if the record carries a scalar measurement.
    pub fn is_scalar(&self) -> bool {
        self.value.is_some() && self.unit.is_some()
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── GeoPoint ──

    #[test]
    fn geopoint_valid() {
        let p = GeoPoint::new(51.5074, -0.1278).unwrap();
        assert!(p.is_finite());
    }

    #[test]
    fn geopoint_lat_out_of_range() {
        assert!(matches!(
            GeoPoint::new(91.0, 0.0),
            Err(GeoError::LatOutOfRange(_))
        ));
    }

    #[test]
    fn geopoint_lon_out_of_range() {
        assert!(matches!(
            GeoPoint::new(0.0, 181.0),
            Err(GeoError::LonOutOfRange(_))
        ));
    }

    // ── GeoBBox ──

    #[test]
    fn geobbox_valid_and_centre() {
        let bbox = GeoBBox::new(-10.0, -5.0, 10.0, 5.0).unwrap();
        let c = bbox.centre();
        assert!((c.lat - 0.0).abs() < 1e-9);
        assert!((c.lon - 0.0).abs() < 1e-9);
    }

    #[test]
    fn geobbox_inverted_lon_rejected() {
        assert!(matches!(
            GeoBBox::new(10.0, -5.0, -10.0, 5.0),
            Err(GeoError::BBoxLonInverted { .. })
        ));
    }

    #[test]
    fn geobbox_inverted_lat_rejected() {
        assert!(matches!(
            GeoBBox::new(-10.0, 5.0, 10.0, -5.0),
            Err(GeoError::BBoxLatInverted { .. })
        ));
    }

    // ── DataSource ──

    #[test]
    fn data_source_licenses_are_non_empty() {
        let sources = [
            DataSource::Copernicus,
            DataSource::Noaa,
            DataSource::Nasa,
            DataSource::Usgs,
            DataSource::Gbif,
            DataSource::INaturalist,
            DataSource::SensorThings,
            DataSource::HomeNode,
            DataSource::CommunityNode,
            DataSource::OceanNode,
            DataSource::SatelliteCommercial,
        ];
        for s in sources {
            assert!(!s.default_license().is_empty(), "{s:?} has empty license");
            assert!(!s.display_name().is_empty(), "{s:?} has empty display_name");
        }
    }

    // ── NormalizedObservation round-trip ──

    #[test]
    fn normalized_observation_serde_roundtrip() {
        use crate::provenance::ProvenanceReceipt;
        use std::collections::BTreeMap;

        let obs = NormalizedObservation {
            id: "00000000-0000-0000-0000-000000000001".into(),
            system: gaia_earth::SystemTwin::Atmosphere,
            source_kind: gaia_earth::SourceKind::Measured,
            source: DataSource::Noaa,
            kind: ObservationKind::PointMeasurement,
            measured_at_unix: 1_700_000_000,
            point: Some(GeoPoint::new(35.0, -97.0).unwrap()),
            bbox: None,
            unit: Some("K".into()),
            value: Some(294.15),
            uncertainty: Some(0.5),
            provenance: ProvenanceReceipt {
                source: DataSource::Noaa,
                source_url: "https://api.noaa.gov/obs/test".into(),
                external_id: "OBS-001".into(),
                fetched_at_unix: 1_700_000_001,
                observed_at_unix: 1_700_000_000,
                sha256: "a".repeat(64),
                license: "CC0-1.0".into(),
            },
            artifact: None,
            attributes: BTreeMap::new(),
        };
        let json = serde_json::to_string(&obs).unwrap();
        let back: NormalizedObservation = serde_json::from_str(&json).unwrap();
        assert_eq!(obs.id, back.id);
        assert_eq!(obs.value, back.value);
        assert!(back.has_geometry());
        assert!(back.is_scalar());
    }
}
