//! #56 system-twin profiles. Catalog only.

use crate::SystemTwin;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileGap {
    Noosphere,
    CommercialImagery,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemProfile {
    pub system: SystemTwin,
    pub variables: &'static str,
    pub sources: &'static str,
    pub models: &'static str,
    pub cadence: &'static str,
    pub phase: u8,
}

impl SystemProfile {
    pub fn all() -> [SystemProfile; 9] {
        [
            profile(
                SystemTwin::Atmosphere,
                "temp, wind, precip",
                "stations",
                "GraphCast/AIFS",
                "6-hourly",
                2,
            ),
            profile(
                SystemTwin::Ocean,
                "SST, AMOC",
                "Argo/NDBC",
                "NEMO-stub",
                "hours",
                2,
            ),
            profile(
                SystemTwin::Land,
                "cover, fire",
                "Landsat/Sentinel",
                "TerraMind-stub",
                "30m-1m",
                2,
            ),
            profile(
                SystemTwin::Biosphere,
                "species, biome",
                "GBIF",
                "biodiversity-stub",
                "days",
                2,
            ),
            profile(
                SystemTwin::Cryosphere,
                "ice, snow",
                "ICESat-2",
                "mass-stub",
                "days",
                1,
            ),
            profile(
                SystemTwin::Lithosphere,
                "quake",
                "USGS",
                "catalog",
                "minutes",
                1,
            ),
            profile(
                SystemTwin::Anthroposphere,
                "cities, emissions",
                "OSM",
                "inventory-stub",
                "days",
                2,
            ),
            profile(
                SystemTwin::Technosphere,
                "energy, transport",
                "public stats",
                "inventory-stub",
                "hours",
                2,
            ),
            profile(
                SystemTwin::Magnetosphere,
                "storm, aurora",
                "SWPC",
                "space-stub",
                "minutes",
                1,
            ),
        ]
    }

    pub fn gaps() -> [ProfileGap; 2] {
        [ProfileGap::Noosphere, ProfileGap::CommercialImagery]
    }
}

fn profile(
    system: SystemTwin,
    variables: &'static str,
    sources: &'static str,
    models: &'static str,
    cadence: &'static str,
    phase: u8,
) -> SystemProfile {
    SystemProfile {
        system,
        variables,
        sources,
        models,
        cadence,
        phase,
    }
}
