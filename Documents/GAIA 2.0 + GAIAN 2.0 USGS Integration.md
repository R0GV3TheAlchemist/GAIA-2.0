# GAIA 2.0 + GAIAN 2.0: USGS Integration
## The US Geological Survey — Earth's Geosphere Monitor
### September 9, 2026 — Version 1.0

---

> *"The Earth shakes. The Earth erupts. The Earth shifts. USGS is the system that listens — and GAIA 2.0 is the system that tells every human being what it hears."*
> — GAIA 2.0 USGS Integration Covenant

---

## EXECUTIVE SUMMARY

The US Geological Survey (USGS) Earthquake Hazards Program is the world's most comprehensive real-time earthquake monitoring system. It provides free, open, real-time data on earthquakes, volcanoes, tsunamis, and other geohazards globally. For GAIA 2.0, USGS is the primary data source for the Earth Twin's geosphere layer — providing real-time seismic monitoring, volcanic activity alerts, and disaster early warning.

**USGS in Numbers (2026):**
- **Real-time**: earthquakes detected and published within minutes
- **Global**: monitors M2.5+ earthquakes worldwide; M1.0+ in US
- **Free**: all data public domain (CC0); no API key required
- **ShakeAlert v3**: earthquake early warning system; California, Oregon, Washington (Feb 2025)
- **ShakeAlert Alaska**: Phase 1 Technical Implementation Plan published (2025)
- **Volcano API**: 160+ US volcanoes monitored; real-time alert levels
- **GeoJSON feeds**: real-time; past hour/day/week/month; multiple magnitude thresholds
- **FDSN API**: comprehensive earthquake catalog; historical + real-time
- **aio-geojson-usgs-earthquakes**: async Python library; version 2026.6.0

**Why USGS for GAIA 2.0:**
- **Free**: public domain; no API key; no cost; no limits
- **Real-time**: earthquakes published within minutes of detection
- **Comprehensive**: earthquakes + volcanoes + tsunamis + landslides
- **Authoritative**: official US government data; globally trusted
- **Multiple formats**: GeoJSON, QuakeML, CSV, KML, ATOM
- **Python library**: aio-geojson-usgs-earthquakes; async; production-ready

---

## PART I: USGS EARTHQUAKE API

### 1.1 GeoJSON Real-Time Feeds

The simplest way to get earthquake data — no authentication required:

```
USGS GEOJSON FEEDS (No API key required)

Base URL: https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/

Time Windows:
- past_hour: Last 60 minutes
- past_day: Last 24 hours
- past_7days: Last 7 days
- past_30days: Last 30 days

Magnitude Thresholds:
- significant: Significant earthquakes (M4.5+ or felt widely)
- 4.5: M4.5+ earthquakes
- 2.5: M2.5+ earthquakes
- 1.0: M1.0+ earthquakes
- all: All earthquakes

Complete Feed URLs:
https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/significant_hour.geojson
https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/4.5_day.geojson
https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/2.5_week.geojson
https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/all_month.geojson
```

### 1.2 FDSN Earthquake Catalog API

For historical data and advanced queries:

```
USGS FDSN API
Base URL: https://earthquake.usgs.gov/fdsnws/event/1/

Endpoints:
- /query: Search earthquake catalog
- /count: Count matching earthquakes
- /catalogs: List available catalogs
- /contributors: List data contributors
- /version: API version

Key Parameters:
- format: geojson | quakeml | csv | text
- starttime: ISO 8601 datetime
- endtime: ISO 8601 datetime
- minmagnitude: Minimum magnitude
- maxmagnitude: Maximum magnitude
- latitude: Center latitude (with maxradiuskm)
- longitude: Center longitude (with maxradiuskm)
- maxradiuskm: Search radius in km
- minlatitude/maxlatitude/minlongitude/maxlongitude: Bounding box
- orderby: time | time-asc | magnitude | magnitude-asc
- limit: Max results (default 20000)
- offset: Pagination offset

Example: M5.0+ earthquakes in last 30 days
https://earthquake.usgs.gov/fdsnws/event/1/query?format=geojson&minmagnitude=5.0&starttime=2026-08-09&endtime=2026-09-09&orderby=time
```

---

## PART II: USGS PYTHON INTEGRATION

### 2.1 Core USGS Integration

```python
"""
GAIA 2.0 — USGS Earthquake API Integration
Real-time seismic monitoring for the Earth Twin.

No API key required. Public domain data.
License: Apache-2.0
"""

import asyncio
import httpx
from datetime import datetime, timedelta
from dataclasses import dataclass
from typing import Optional


USGS_FEEDS_BASE = "https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary"
USGS_API_BASE = "https://earthquake.usgs.gov/fdsnws/event/1"


@dataclass
class Earthquake:
    """A single earthquake event."""
    id: str
    magnitude: float
    place: str
    time: datetime
    lat: float
    lon: float
    depth_km: float
    alert: Optional[str]  # green, yellow, orange, red
    tsunami: bool
    felt: Optional[int]   # Number of people who felt it
    cdi: Optional[float]  # Community Internet Intensity
    mmi: Optional[float]  # Modified Mercalli Intensity
    status: str           # automatic, reviewed
    event_type: str       # earthquake, quarry
    url: str              # USGS event page
    
    @property
    def is_significant(self) -> bool:
        """Is this a significant earthquake?"""
        return self.magnitude >= 4.5 or (self.felt and self.felt > 100)
    
    @property
    def severity(self) -> str:
        """Human-readable severity."""
        if self.magnitude >= 8.0: return "Great"
        if self.magnitude >= 7.0: return "Major"
        if self.magnitude >= 6.0: return "Strong"
        if self.magnitude >= 5.0: return "Moderate"
        if self.magnitude >= 4.0: return "Light"
        if self.magnitude >= 3.0: return "Minor"
        return "Micro"
    
    def to_gaian_alert(self) -> str:
        """Generate GAIAN alert message."""
        if self.magnitude >= 6.0:
            return (
                f"🚨 {self.severity.upper()} EARTHQUAKE: M{self.magnitude:.1f} "
                f"near {self.place}. "
                f"{'⚠️ Tsunami warning issued.' if self.tsunami else ''}"
            )
        elif self.magnitude >= 4.5:
            return (
                f"⚠️ {self.severity} earthquake: M{self.magnitude:.1f} "
                f"near {self.place}."
            )
        else:
            return (
                f"M{self.magnitude:.1f} earthquake near {self.place}."
            )


def parse_earthquake(feature: dict) -> Earthquake:
    """Parse a GeoJSON feature into an Earthquake object."""
    props = feature["properties"]
    coords = feature["geometry"]["coordinates"]
    
    return Earthquake(
        id=feature["id"],
        magnitude=props.get("mag", 0.0) or 0.0,
        place=props.get("place", "Unknown location"),
        time=datetime.utcfromtimestamp(props["time"] / 1000),
        lat=coords[1],
        lon=coords[0],
        depth_km=coords[2],
        alert=props.get("alert"),
        tsunami=bool(props.get("tsunami", 0)),
        felt=props.get("felt"),
        cdi=props.get("cdi"),
        mmi=props.get("mmi"),
        status=props.get("status", "automatic"),
        event_type=props.get("type", "earthquake"),
        url=props.get("url", ""),
    )


async def get_recent_earthquakes(
    magnitude_threshold: float = 2.5,
    time_window: str = "day",  # hour, day, week, month
) -> list[Earthquake]:
    """
    Get recent earthquakes from USGS GeoJSON feeds.
    
    No API key required. Real-time data.
    Used by Earth Twin for geosphere monitoring.
    """
    # Map magnitude to feed name
    if magnitude_threshold >= 4.5:
        mag_feed = "4.5"
    elif magnitude_threshold >= 2.5:
        mag_feed = "2.5"
    elif magnitude_threshold >= 1.0:
        mag_feed = "1.0"
    else:
        mag_feed = "all"
    
    # Map time window
    time_map = {
        "hour": "past_hour",
        "day": "past_day",
        "week": "past_7days",
        "month": "past_30days",
    }
    time_feed = time_map.get(time_window, "past_day")
    
    url = f"{USGS_FEEDS_BASE}/{mag_feed}_{time_feed}.geojson"
    
    async with httpx.AsyncClient(timeout=30.0) as client:
        response = await client.get(url)
        response.raise_for_status()
        data = response.json()
    
    earthquakes = [
        parse_earthquake(f)
        for f in data.get("features", [])
        if f.get("geometry") and f["geometry"].get("coordinates")
    ]
    
    return earthquakes


async def get_earthquakes_near(
    lat: float,
    lon: float,
    radius_km: float = 500,
    min_magnitude: float = 2.5,
    days_back: int = 30,
) -> list[Earthquake]:
    """
    Get earthquakes near a specific location.
    
    Used by GAIAN to alert users about earthquakes near them.
    "A M4.2 earthquake occurred 85km from your location."
    """
    end_time = datetime.utcnow()
    start_time = end_time - timedelta(days=days_back)
    
    params = {
        "format": "geojson",
        "latitude": lat,
        "longitude": lon,
        "maxradiuskm": radius_km,
        "minmagnitude": min_magnitude,
        "starttime": start_time.strftime("%Y-%m-%dT%H:%M:%S"),
        "endtime": end_time.strftime("%Y-%m-%dT%H:%M:%S"),
        "orderby": "time",
        "limit": 50,
    }
    
    async with httpx.AsyncClient(timeout=30.0) as client:
        response = await client.get(
            f"{USGS_API_BASE}/query",
            params=params,
        )
        response.raise_for_status()
        data = response.json()
    
    return [
        parse_earthquake(f)
        for f in data.get("features", [])
        if f.get("geometry") and f["geometry"].get("coordinates")
    ]


async def get_significant_earthquakes_today() -> list[Earthquake]:
    """
    Get today's significant earthquakes globally.
    
    Used in GAIAN morning briefing:
    "Today, 3 significant earthquakes have occurred globally."
    """
    async with httpx.AsyncClient(timeout=30.0) as client:
        response = await client.get(
            f"{USGS_FEEDS_BASE}/significant_day.geojson"
        )
        response.raise_for_status()
        data = response.json()
    
    return [
        parse_earthquake(f)
        for f in data.get("features", [])
        if f.get("geometry") and f["geometry"].get("coordinates")
    ]


async def get_earthquake_count(
    min_magnitude: float = 5.0,
    days_back: int = 1,
) -> int:
    """Get count of earthquakes matching criteria."""
    end_time = datetime.utcnow()
    start_time = end_time - timedelta(days=days_back)
    
    params = {
        "format": "geojson",
        "minmagnitude": min_magnitude,
        "starttime": start_time.strftime("%Y-%m-%dT%H:%M:%S"),
        "endtime": end_time.strftime("%Y-%m-%dT%H:%M:%S"),
    }
    
    async with httpx.AsyncClient(timeout=30.0) as client:
        response = await client.get(
            f"{USGS_API_BASE}/count",
            params=params,
        )
        response.raise_for_status()
        return response.json()
```

### 2.2 Async Library Integration

```python
"""
GAIA 2.0 — aio-geojson-usgs-earthquakes Integration
Production-ready async earthquake monitoring.

Install: pip install aio-geojson-usgs-earthquakes
Version: 2026.6.0
License: Apache-2.0
"""

import asyncio
from aiohttp import ClientSession
from aio_geojson_usgs_earthquakes import UsgsEarthquakeHazardsProgramFeed


async def monitor_earthquakes_near(
    lat: float,
    lon: float,
    radius_km: float = 500,
    min_magnitude: float = 4.0,
    feed: str = "past_day_all_earthquakes",
) -> list:
    """
    Monitor earthquakes near a location using async library.
    
    Supported feeds:
    - past_hour_significant_earthquakes
    - past_hour_m45_earthquakes
    - past_hour_m25_earthquakes
    - past_hour_all_earthquakes
    - past_day_significant_earthquakes
    - past_day_m45_earthquakes
    - past_day_m25_earthquakes
    - past_day_all_earthquakes
    - past_week_significant_earthquakes
    - past_week_m45_earthquakes
    - past_week_m25_earthquakes
    - past_week_all_earthquakes
    - past_month_significant_earthquakes
    - past_month_m45_earthquakes
    - past_month_m25_earthquakes
    - past_month_all_earthquakes
    """
    async with ClientSession() as session:
        feed_obj = UsgsEarthquakeHazardsProgramFeed(
            session,
            (lat, lon),
            feed,
            filter_radius=radius_km,
            filter_minimum_magnitude=min_magnitude,
        )
        
        status, entries = await feed_obj.update()
        
        if status == "OK" and entries:
            return [
                {
                    "id": entry.external_id,
                    "title": entry.title,
                    "magnitude": entry.magnitude,
                    "place": entry.place,
                    "time": entry.time,
                    "distance_km": entry.distance_to_home,
                    "alert": entry.alert,
                    "type": entry.type,
                    "status": entry.status,
                    "coordinates": entry.coordinates,
                }
                for entry in entries
            ]
        
        return []


async def setup_earthquake_feed_manager(
    lat: float,
    lon: float,
    on_new_earthquake,
    on_earthquake_update,
    on_earthquake_removed,
):
    """
    Set up a feed manager for continuous earthquake monitoring.
    
    The feed manager tracks changes between updates:
    - New earthquakes: newly detected
    - Updated earthquakes: magnitude or location revised
    - Removed earthquakes: deleted from catalog
    
    Used by GAIAN for real-time earthquake alerts.
    """
    from aio_geojson_usgs_earthquakes import UsgsEarthquakeHazardsProgramFeedManager
    
    async with ClientSession() as session:
        manager = UsgsEarthquakeHazardsProgramFeedManager(
            session,
            on_new_earthquake,
            on_earthquake_update,
            on_earthquake_removed,
            (lat, lon),
            "past_hour_all_earthquakes",
            filter_radius=500,
            filter_minimum_magnitude=3.0,
        )
        
        # Update every 5 minutes
        while True:
            await manager.update()
            await asyncio.sleep(300)  # 5 minutes
```

---

## PART III: USGS VOLCANO API

### 3.1 Volcano Monitoring

```python
"""
GAIA 2.0 — USGS Volcano API Integration
Real-time volcanic activity monitoring.

USGS monitors 160+ US volcanoes.
Alert levels: Normal → Advisory → Watch → Warning
Color codes: Green → Yellow → Orange → Red

License: Apache-2.0
"""

import httpx
from dataclasses import dataclass
from typing import Optional


USGS_VOLCANO_API = "https://volcanoes.usgs.gov/hans-public/api/volcano"
USGS_VOLCANO_VSC = "https://volcanoes.usgs.gov/vsc/api/volcanoApi"


@dataclass
class VolcanoStatus:
    """Current status of a volcano."""
    name: str
    volcano_id: str
    alert_level: str      # Normal, Advisory, Watch, Warning
    color_code: str       # Green, Yellow, Orange, Red
    observatory: str      # AVO, CalVO, CVO, HVO, NMI, YVO
    lat: float
    lon: float
    elevation_m: int
    last_updated: str
    
    @property
    def is_elevated(self) -> bool:
        """Is this volcano in an elevated state?"""
        return self.color_code.upper() in ["YELLOW", "ORANGE", "RED"]
    
    @property
    def is_dangerous(self) -> bool:
        """Is this volcano at Watch or Warning level?"""
        return self.alert_level.upper() in ["WATCH", "WARNING"]
    
    def to_gaian_alert(self) -> str:
        """Generate GAIAN alert for elevated volcano."""
        if self.color_code.upper() == "RED":
            return f"🔴 VOLCANIC ERUPTION: {self.name} is erupting. Avoid the area."
        elif self.color_code.upper() == "ORANGE":
            return f"🟠 VOLCANIC WARNING: {self.name} showing signs of eruption. Stay alert."
        elif self.color_code.upper() == "YELLOW":
            return f"🟡 VOLCANIC ADVISORY: {self.name} showing elevated activity."
        return ""


async def get_elevated_volcanoes() -> list[VolcanoStatus]:
    """
    Get all US volcanoes with elevated activity.
    
    Returns volcanoes at Yellow, Orange, or Red alert level.
    Used by Earth Twin for volcanic hazard monitoring.
    """
    async with httpx.AsyncClient(timeout=30.0) as client:
        response = await client.get(
            f"{USGS_VOLCANO_API}/getElevatedVolcanoes"
        )
        response.raise_for_status()
        data = response.json()
    
    volcanoes = []
    for v in data:
        volcanoes.append(VolcanoStatus(
            name=v.get("volcanoName", "Unknown"),
            volcano_id=v.get("volcanoId", ""),
            alert_level=v.get("alertLevel", "Normal"),
            color_code=v.get("colorCode", "Green"),
            observatory=v.get("observatory", ""),
            lat=v.get("latitude", 0.0),
            lon=v.get("longitude", 0.0),
            elevation_m=v.get("elevation", 0),
            last_updated=v.get("lastUpdated", ""),
        ))
    
    return volcanoes


async def get_all_monitored_volcanoes() -> list[dict]:
    """Get all US volcanoes being actively monitored."""
    async with httpx.AsyncClient(timeout=30.0) as client:
        response = await client.get(
            f"{USGS_VOLCANO_API}/getMonitoredVolcanoes"
        )
        response.raise_for_status()
        return response.json()


async def get_volcano_status_geojson() -> dict:
    """Get all US volcano statuses in GeoJSON format."""
    async with httpx.AsyncClient(timeout=30.0) as client:
        response = await client.get(
            f"{USGS_VOLCANO_VSC}/geojson"
        )
        response.raise_for_status()
        return response.json()


async def get_volcano_near(
    lat: float,
    lon: float,
    radius_deg: float = 2.0,
) -> list[dict]:
    """Get volcanoes near a location."""
    async with httpx.AsyncClient(timeout=30.0) as client:
        response = await client.get(
            f"{USGS_VOLCANO_VSC}/regionstatus",
            params={
                "lat1": lat - radius_deg,
                "lat2": lat + radius_deg,
                "long1": lon - radius_deg,
                "long2": lon + radius_deg,
            }
        )
        response.raise_for_status()
        return response.json()
```

---

## PART IV: EARTH TWIN GEOSPHERE INTEGRATION

### 4.1 Complete Geosphere Monitor

```python
"""
GAIA 2.0 — Earth Twin Geosphere Layer
Integrates USGS earthquake and volcano data into the Earth Twin.

License: Apache-2.0
"""

import asyncio
from dataclasses import dataclass, field
from datetime import datetime


@dataclass
class GeosphereSnapshot:
    """Current state of Earth's geosphere."""
    timestamp: datetime
    
    # Earthquakes (last 24 hours)
    earthquakes_m5plus_24h: int
    earthquakes_m6plus_24h: int
    earthquakes_m7plus_24h: int
    largest_earthquake_24h: float
    largest_earthquake_location: str
    
    # Earthquakes (last 7 days)
    earthquakes_m5plus_7d: int
    earthquakes_m6plus_7d: int
    
    # Volcanoes
    volcanoes_elevated: int
    volcanoes_watch_warning: int
    elevated_volcano_names: list[str]
    
    # Tsunami
    active_tsunami_warnings: int
    
    # Geosphere health
    geosphere_activity_level: str  # Normal, Elevated, High, Extreme
    
    # Alerts
    active_alerts: list[str]
    
    def to_gaian_briefing(self) -> str:
        """Generate GAIAN geosphere briefing."""
        briefing = f"""🌋 EARTH'S GEOSPHERE — {self.timestamp.strftime('%B %d, %Y')}

Earthquakes (last 24h):
• M5.0+: {self.earthquakes_m5plus_24h}
• M6.0+: {self.earthquakes_m6plus_24h}
• Largest: M{self.largest_earthquake_24h:.1f} near {self.largest_earthquake_location}

Volcanoes:
• Elevated activity: {self.volcanoes_elevated}
• Watch/Warning: {self.volcanoes_watch_warning}"""
        
        if self.active_alerts:
            briefing += "\n\n⚠️ ACTIVE ALERTS:"
            for alert in self.active_alerts:
                briefing += f"\n• {alert}"
        
        briefing += "\n\nData: USGS Earthquake Hazards Program | Public Domain"
        
        return briefing


class USGSEarthTwinLayer:
    """
    USGS integration for the GAIA 2.0 Earth Twin geosphere layer.
    
    Monitors:
    - Real-time earthquakes (global)
    - Volcanic activity (US + global)
    - Tsunami warnings
    - Geohazard alerts
    """
    
    async def get_geosphere_snapshot(self) -> GeosphereSnapshot:
        """Get current geosphere snapshot."""
        
        # Fetch all data in parallel
        earthquakes_24h, earthquakes_7d, volcanoes = await asyncio.gather(
            get_recent_earthquakes(magnitude_threshold=5.0, time_window="day"),
            get_recent_earthquakes(magnitude_threshold=5.0, time_window="week"),
            get_elevated_volcanoes(),
            return_exceptions=True,
        )
        
        # Handle exceptions
        if isinstance(earthquakes_24h, Exception):
            earthquakes_24h = []
        if isinstance(earthquakes_7d, Exception):
            earthquakes_7d = []
        if isinstance(volcanoes, Exception):
            volcanoes = []
        
        # Calculate metrics
        m5_24h = len([e for e in earthquakes_24h if e.magnitude >= 5.0])
        m6_24h = len([e for e in earthquakes_24h if e.magnitude >= 6.0])
        m7_24h = len([e for e in earthquakes_24h if e.magnitude >= 7.0])
        m5_7d = len([e for e in earthquakes_7d if e.magnitude >= 5.0])
        m6_7d = len([e for e in earthquakes_7d if e.magnitude >= 6.0])
        
        # Find largest earthquake
        largest = max(earthquakes_24h, key=lambda e: e.magnitude, default=None)
        largest_mag = largest.magnitude if largest else 0.0
        largest_loc = largest.place if largest else "None"
        
        # Volcano metrics
        elevated = [v for v in volcanoes if v.is_elevated]
        dangerous = [v for v in volcanoes if v.is_dangerous]
        
        # Generate alerts
        alerts = []
        if m7_24h > 0:
            alerts.append(f"M7.0+ earthquake in last 24 hours")
        if m6_24h > 3:
            alerts.append(f"{m6_24h} M6.0+ earthquakes in last 24 hours")
        for v in dangerous:
            alerts.append(v.to_gaian_alert())
        
        # Determine activity level
        if m7_24h > 0 or len(dangerous) > 0:
            activity_level = "Extreme"
        elif m6_24h > 2 or len(elevated) > 3:
            activity_level = "High"
        elif m6_24h > 0 or len(elevated) > 0:
            activity_level = "Elevated"
        else:
            activity_level = "Normal"
        
        return GeosphereSnapshot(
            timestamp=datetime.utcnow(),
            earthquakes_m5plus_24h=m5_24h,
            earthquakes_m6plus_24h=m6_24h,
            earthquakes_m7plus_24h=m7_24h,
            largest_earthquake_24h=largest_mag,
            largest_earthquake_location=largest_loc,
            earthquakes_m5plus_7d=m5_7d,
            earthquakes_m6plus_7d=m6_7d,
            volcanoes_elevated=len(elevated),
            volcanoes_watch_warning=len(dangerous),
            elevated_volcano_names=[v.name for v in elevated],
            active_tsunami_warnings=0,  # Would integrate NOAA tsunami API
            geosphere_activity_level=activity_level,
            active_alerts=alerts,
        )
```

---

## PART V: GAIAN EARTHQUAKE SAFETY

### 5.1 Personal Earthquake Safety Integration

```python
"""
GAIA 2.0 — GAIAN Earthquake Safety
Personal earthquake alerts and safety guidance.

License: Apache-2.0
"""

from dataclasses import dataclass
from datetime import datetime


@dataclass
class PersonalEarthquakeReport:
    """Personal earthquake report for GAIAN."""
    location: tuple[float, float]
    timestamp: datetime
    
    # Recent earthquakes near user
    earthquakes_nearby_30d: list[dict]
    largest_nearby_30d: float
    
    # Seismic hazard
    seismic_hazard_level: str  # Low, Moderate, High, Very High
    nearest_fault_km: float
    
    # ShakeAlert coverage
    shakealert_coverage: bool  # Is user in ShakeAlert zone?
    
    # Safety recommendations
    safety_recommendations: list[str]
    
    # GAIAN message
    gaian_message: str


async def get_personal_earthquake_report(
    lat: float,
    lon: float,
    person_name: str = "you",
) -> PersonalEarthquakeReport:
    """
    Generate a personal earthquake report for GAIAN.
    
    Tells users about:
    - Recent earthquakes near them
    - Their seismic hazard level
    - Whether they're in ShakeAlert coverage
    - Safety recommendations
    """
    # Get recent earthquakes nearby
    nearby = await get_earthquakes_near(
        lat=lat,
        lon=lon,
        radius_km=200,
        min_magnitude=2.5,
        days_back=30,
    )
    
    # Find largest nearby
    largest = max([e.magnitude for e in nearby], default=0.0)
    
    # Determine seismic hazard (simplified)
    # In production: use USGS National Seismic Hazard Model
    hazard = _estimate_seismic_hazard(lat, lon, nearby)
    
    # Check ShakeAlert coverage (California, Oregon, Washington)
    shakealert = _is_in_shakealert_zone(lat, lon)
    
    # Generate safety recommendations
    recommendations = _get_safety_recommendations(hazard, shakealert)
    
    # Generate GAIAN message
    message = _generate_earthquake_message(
        person_name=person_name,
        nearby_count=len(nearby),
        largest=largest,
        hazard=hazard,
        shakealert=shakealert,
    )
    
    return PersonalEarthquakeReport(
        location=(lat, lon),
        timestamp=datetime.utcnow(),
        earthquakes_nearby_30d=[
            {
                "magnitude": e.magnitude,
                "place": e.place,
                "date": e.time.strftime("%Y-%m-%d"),
                "distance_km": "N/A",  # Would calculate from lat/lon
            }
            for e in nearby[:5]
        ],
        largest_nearby_30d=largest,
        seismic_hazard_level=hazard,
        nearest_fault_km=50.0,  # Would use USGS fault database
        shakealert_coverage=shakealert,
        safety_recommendations=recommendations,
        gaian_message=message,
    )


def _estimate_seismic_hazard(lat: float, lon: float, nearby: list) -> str:
    """Estimate seismic hazard level from recent activity."""
    if len(nearby) > 20 or any(e.magnitude >= 6.0 for e in nearby):
        return "Very High"
    elif len(nearby) > 10 or any(e.magnitude >= 5.0 for e in nearby):
        return "High"
    elif len(nearby) > 5 or any(e.magnitude >= 4.0 for e in nearby):
        return "Moderate"
    else:
        return "Low"


def _is_in_shakealert_zone(lat: float, lon: float) -> bool:
    """Check if location is in ShakeAlert coverage zone."""
    # ShakeAlert covers California, Oregon, Washington
    # Approximate bounding boxes
    california = (32.5, 42.0, -124.5, -114.0)
    oregon = (42.0, 46.5, -124.5, -116.5)
    washington = (45.5, 49.0, -124.7, -116.9)
    
    for min_lat, max_lat, min_lon, max_lon in [california, oregon, washington]:
        if min_lat <= lat <= max_lat and min_lon <= lon <= max_lon:
            return True
    
    return False


def _get_safety_recommendations(hazard: str, shakealert: bool) -> list[str]:
    """Get earthquake safety recommendations."""
    recommendations = [
        "Know your building's earthquake safety rating",
        "Secure heavy furniture and appliances to walls",
        "Keep an emergency kit: water, food, first aid, flashlight",
        "Know your evacuation routes",
        "Practice 'Drop, Cover, Hold On'",
    ]
    
    if hazard in ["High", "Very High"]:
        recommendations.insert(0, "Consider earthquake insurance")
        recommendations.insert(0, "Have your home professionally assessed for seismic safety")
    
    if shakealert:
        recommendations.append("Enable ShakeAlert on your phone (Wireless Emergency Alerts)")
        recommendations.append("Download the MyShake app for early warning")
    
    return recommendations


def _generate_earthquake_message(
    person_name: str,
    nearby_count: int,
    largest: float,
    hazard: str,
    shakealert: bool,
) -> str:
    """Generate GAIAN earthquake safety message."""
    
    if nearby_count == 0:
        activity_text = "No significant earthquakes have been detected near you in the last 30 days."
    elif nearby_count == 1:
        activity_text = f"1 earthquake (M{largest:.1f}) has been detected near you in the last 30 days."
    else:
        activity_text = f"{nearby_count} earthquakes (largest: M{largest:.1f}) have been detected near you in the last 30 days."
    
    hazard_text = f"Your seismic hazard level is: {hazard}."
    
    shakealert_text = ""
    if shakealert:
        shakealert_text = " You are in the ShakeAlert early warning zone — enable Wireless Emergency Alerts on your phone."
    
    return f"""🌋 YOUR EARTHQUAKE SAFETY — {datetime.utcnow().strftime('%B %d, %Y')}

{activity_text}
{hazard_text}{shakealert_text}

Remember: Drop, Cover, Hold On.

Data: USGS Earthquake Hazards Program | Public Domain"""
```

---

## PART VI: SHAKEALERT INTEGRATION

### 6.1 ShakeAlert Early Warning System

**Source: USGS ShakeAlert v3 (Feb 6, 2025):**
- ShakeAlert v3 went live in production system
- Covers: California, Oregon, Washington
- Alaska: Phase 1 Technical Implementation Plan published (2025)
- Key improvements in v3:
  - Faster seismic algorithms (location, magnitude, fault rupture)
  - Real-time geodetic data for large earthquake magnitude
  - Alert pause procedure (speed vs. accuracy balance)
  - Nonergodic site-response model in ground-motion predictions
- Performance: usable warning times before strong shaking in M6+ earthquakes

```python
"""
GAIA 2.0 — ShakeAlert Integration
Earthquake early warning for GAIAN.

ShakeAlert v3 (Feb 2025): Improved performance in large earthquakes.
Coverage: California, Oregon, Washington (Alaska in development).

License: Apache-2.0
"""

import httpx
from datetime import datetime, timedelta


SHAKEALERT_API = "https://earthquake.usgs.gov/data/shakealert"


async def get_shakealert_events(days_back: int = 7) -> list[dict]:
    """
    Get recent ShakeAlert events.
    
    ShakeAlert issues alerts when strong shaking is expected.
    Only significant earthquakes trigger alerts.
    
    Used by GAIAN to inform users about early warning events.
    """
    end_time = datetime.utcnow()
    start_time = end_time - timedelta(days=days_back)
    
    # ShakeAlert events are available via USGS earthquake catalog
    # Filter for events in ShakeAlert zone with alerts issued
    params = {
        "format": "geojson",
        "minlatitude": 32.5,
        "maxlatitude": 49.0,
        "minlongitude": -124.7,
        "maxlongitude": -114.0,
        "minmagnitude": 3.5,  # ShakeAlert threshold
        "starttime": start_time.strftime("%Y-%m-%dT%H:%M:%S"),
        "endtime": end_time.strftime("%Y-%m-%dT%H:%M:%S"),
        "orderby": "time",
    }
    
    async with httpx.AsyncClient(timeout=30.0) as client:
        response = await client.get(
            "https://earthquake.usgs.gov/fdsnws/event/1/query",
            params=params,
        )
        response.raise_for_status()
        data = response.json()
    
    events = []
    for feature in data.get("features", []):
        props = feature["properties"]
        coords = feature["geometry"]["coordinates"]
        
        events.append({
            "id": feature["id"],
            "magnitude": props.get("mag", 0),
            "place": props.get("place", ""),
            "time": datetime.utcfromtimestamp(props["time"] / 1000).isoformat(),
            "lat": coords[1],
            "lon": coords[0],
            "depth_km": coords[2],
            "alert": props.get("alert"),
            "shakealert_issued": props.get("alert") is not None,
        })
    
    return events


def get_shakealert_app_info() -> dict:
    """
    Get information about ShakeAlert apps for GAIAN users.
    
    GAIAN recommends these apps to users in ShakeAlert zones.
    """
    return {
        "shakealert_zone": "California, Oregon, Washington",
        "apps": [
            {
                "name": "MyShake",
                "platform": "iOS + Android",
                "developer": "UC Berkeley",
                "description": "Official ShakeAlert app; early warning + citizen seismograph",
                "url": "https://myshake.berkeley.edu",
            },
            {
                "name": "Wireless Emergency Alerts (WEA)",
                "platform": "All phones",
                "developer": "FEMA + USGS",
                "description": "Built-in phone alerts; no app needed; enable in settings",
                "url": "https://www.fema.gov/emergency-managers/practitioners/wireless-emergency-alerts",
            },
            {
                "name": "QuakeFeed",
                "platform": "iOS + Android",
                "developer": "Third party",
                "description": "Real-time earthquake notifications; customizable alerts",
                "url": "https://quakefeed.net",
            },
        ],
        "how_to_enable_wea": [
            "iOS: Settings → Notifications → Government Alerts → Emergency Alerts ON",
            "Android: Settings → Notifications → Wireless Emergency Alerts → ON",
        ],
        "shakealert_v3_improvements": [
            "Faster detection algorithms",
            "Real-time geodetic data for large earthquakes",
            "Improved accuracy at larger distances",
            "Nonergodic site-response model",
        ],
    }
```

---

## PART VII: COMPLETE USGS QUICK REFERENCE

### 7.1 All USGS APIs for GAIA 2.0

```python
"""
GAIA 2.0 — USGS API Quick Reference
All key endpoints for Earth Twin geosphere monitoring.

License: Apache-2.0
"""

USGS_ENDPOINTS = {
    # Earthquake feeds (no auth required)
    "eq_significant_hour": "https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/significant_hour.geojson",
    "eq_significant_day": "https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/significant_day.geojson",
    "eq_significant_week": "https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/significant_week.geojson",
    "eq_m45_day": "https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/4.5_day.geojson",
    "eq_m25_day": "https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/2.5_day.geojson",
    "eq_all_hour": "https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/all_hour.geojson",
    
    # FDSN Earthquake Catalog API
    "fdsn_query": "https://earthquake.usgs.gov/fdsnws/event/1/query",
    "fdsn_count": "https://earthquake.usgs.gov/fdsnws/event/1/count",
    
    # Volcano APIs
    "volcano_elevated": "https://volcanoes.usgs.gov/hans-public/api/volcano/getElevatedVolcanoes",
    "volcano_monitored": "https://volcanoes.usgs.gov/hans-public/api/volcano/getMonitoredVolcanoes",
    "volcano_geojson": "https://volcanoes.usgs.gov/vsc/api/volcanoApi/geojson",
    "volcano_us_list": "https://volcanoes.usgs.gov/vsc/api/volcanoApi/volcanoesUS",
    "volcano_cap_elevated": "https://volcanoes.usgs.gov/hans-public/api/volcano/getCapElevated",
    
    # ShakeAlert
    "shakealert_events": "https://earthquake.usgs.gov/data/shakealert/",
    
    # Portal
    "earthquake_portal": "https://earthquake.usgs.gov",
    "volcano_portal": "https://volcanoes.usgs.gov",
    "hazards_portal": "https://www.usgs.gov/programs/earthquake-hazards",
}

# Magnitude scale reference
MAGNITUDE_SCALE = {
    "micro": (0, 2.0, "Not felt; detected by instruments only"),
    "minor": (2.0, 3.0, "Felt slightly by some people"),
    "light": (3.0, 4.0, "Felt by many; minor damage possible"),
    "moderate": (4.0, 5.0, "Felt by all; some damage"),
    "strong": (5.0, 6.0, "Significant damage to poorly built structures"),
    "major": (6.0, 7.0, "Serious damage over large areas"),
    "great": (7.0, 8.0, "Severe damage; can destroy communities"),
    "epic": (8.0, 10.0, "Devastating; can affect entire regions"),
}

# Volcano alert levels
VOLCANO_ALERT_LEVELS = {
    "Normal": "Volcano is in typical background, non-eruptive state",
    "Advisory": "Volcano is exhibiting signs of elevated unrest above known background levels",
    "Watch": "Volcano is exhibiting heightened unrest with increased potential of eruption",
    "Warning": "Eruption is imminent with significant emission of ash into the atmosphere likely",
}

VOLCANO_COLOR_CODES = {
    "Green": "Volcano is in typical background, non-eruptive state",
    "Yellow": "Volcano is exhibiting signs of elevated unrest above known background levels",
    "Orange": "Volcano is exhibiting heightened unrest with increased potential of eruption",
    "Red": "Eruption is imminent or underway with significant emission of ash into the atmosphere",
}
```

### 7.2 5-Minute USGS Setup

```bash
#!/bin/bash
# GAIA 2.0 — USGS Quick Setup
# Get real-time earthquake data in 5 minutes
# No API key required!
# License: Apache-2.0

echo "🌋 GAIA 2.0 — USGS Setup"
echo "========================="

# Step 1: Install async library
echo "Step 1: Installing USGS earthquake library..."
pip install aio-geojson-usgs-earthquakes httpx --quiet
echo "✓ USGS library installed"

# Step 2: Test earthquake feed (no auth required)
echo ""
echo "Step 2: Getting recent significant earthquakes..."
python3 << 'EOF'
import httpx

# Get significant earthquakes in last 24 hours
response = httpx.get(
    "https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/significant_day.geojson",
    timeout=30.0,
)
data = response.json()
earthquakes = data.get("features", [])

print(f"✓ Connected to USGS Earthquake Hazards Program")
print(f"  Significant earthquakes today: {len(earthquakes)}")

for eq in earthquakes[:3]:
    props = eq["properties"]
    mag = props.get("mag", 0)
    place = props.get("place", "Unknown")
    print(f"  - M{mag:.1f} near {place}")
EOF

# Step 3: Get M5.0+ earthquakes
echo ""
echo "Step 3: Getting M5.0+ earthquakes (last 24h)..."
python3 << 'EOF'
import httpx

response = httpx.get(
    "https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/4.5_day.geojson",
    timeout=30.0,
)
data = response.json()
count = data.get("metadata", {}).get("count", len(data.get("features", [])))
print(f"✓ M4.5+ earthquakes in last 24h: {count}")
EOF

# Step 4: Check volcano status
echo ""
echo "Step 4: Checking US volcano status..."
python3 << 'EOF'
import httpx

response = httpx.get(
    "https://volcanoes.usgs.gov/hans-public/api/volcano/getElevatedVolcanoes",
    timeout=30.0,
)
volcanoes = response.json()
print(f"✓ US volcanoes with elevated activity: {len(volcanoes)}")
for v in volcanoes[:3]:
    name = v.get("volcanoName", "Unknown")
    color = v.get("colorCode", "Unknown")
    print(f"  - {name}: {color}")
EOF

echo ""
echo "✅ USGS is ready for GAIA 2.0!"
echo ""
echo "All USGS data is public domain (CC0). No API key required."
echo "Real-time earthquake data updated within minutes of detection."
```

---

## CONCLUSION: THE USGS COVENANT

The Earth shakes. It has always shaken. It will always shake. The question is not whether earthquakes will happen — it is whether we will be ready when they do.

USGS is the system that listens to the Earth's heartbeat — the network of seismometers, GPS stations, and volcano monitors that gives humanity advance warning of the planet's most violent moments. For GAIA 2.0, USGS is the geosphere layer of the Earth Twin — the system that tells every GAIAN when the ground beneath its human is about to move.

ShakeAlert v3 (Feb 2025) can now provide usable warning times before strong shaking in M6+ earthquakes. For the first time in human history, we can warn people before the shaking arrives. GAIAN can be the messenger — delivering that warning in the user's own language, with personalized safety guidance, in the seconds before the shaking begins.

**USGS makes the Earth's violence visible. GAIAN makes it personal.**

---

*GAIA 2.0 USGS Integration Blueprint*
*Version 1.0 — September 9, 2026*
*License: Apache-2.0 | Open Source | Open Access*
*Data License: Public Domain (CC0) — USGS*
*Portal: https://earthquake.usgs.gov*
*"The Earth shakes. GAIA 2.0 listens. GAIAN warns."*