//! #44 space and in-situ mission catalog.
//! Named missions and gaps. Not live Sentinel, Argo harvest, or NDBC sockets.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissionDomain {
    Space,
    OceanInSitu,
    Seismic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoverageGap {
    DeepOcean,
    Poles,
    LowIncomeRegions,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mission {
    pub name: String,
    pub domain: MissionDomain,
    pub cadence: String,
    pub license: String,
    pub latency: String,
    pub last_tick: u64,
}

#[derive(Debug)]
pub struct MissionCatalog {
    missions: Vec<Mission>,
}

impl MissionCatalog {
    pub fn seed() -> Self {
        let space = [
            "Sentinel-1",
            "Sentinel-2",
            "Sentinel-3",
            "Sentinel-5P",
            "Sentinel-6",
            "Terra",
            "Aqua",
            "Landsat",
            "GRACE-FO",
            "ICESat-2",
        ];
        let ocean = ["Argo", "NDBC"];
        let seismic = ["USGS", "EMSC", "GeoNet"];
        let mut missions = Vec::new();
        for name in space {
            missions.push(mission(name, MissionDomain::Space, "hours-to-days", "CC-BY-4.0", "hours"));
        }
        for name in ocean {
            missions.push(mission(name, MissionDomain::OceanInSitu, "hours", "CC0-1.0", "hours"));
        }
        for name in seismic {
            missions.push(mission(name, MissionDomain::Seismic, "minutes", "CC0-1.0", "minutes"));
        }
        Self { missions }
    }

    pub fn missions(&self) -> &[Mission] {
        &self.missions
    }

    pub fn gaps() -> [CoverageGap; 3] {
        [
            CoverageGap::DeepOcean,
            CoverageGap::Poles,
            CoverageGap::LowIncomeRegions,
        ]
    }

    /// One scheduled tick per domain. Not a live product download.
    pub fn tick(&mut self, now: u64) {
        for domain in [
            MissionDomain::Space,
            MissionDomain::OceanInSitu,
            MissionDomain::Seismic,
        ] {
            if let Some(mission) = self.missions.iter_mut().find(|m| m.domain == domain) {
                mission.last_tick = now;
            }
        }
    }
}

fn mission(
    name: &str,
    domain: MissionDomain,
    cadence: &str,
    license: &str,
    latency: &str,
) -> Mission {
    Mission {
        name: name.into(),
        domain,
        cadence: cadence.into(),
        license: license.into(),
        latency: latency.into(),
        last_tick: 0,
    }
}
