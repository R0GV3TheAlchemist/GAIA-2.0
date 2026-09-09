# GAIA 2.0 + GAIAN 2.0: Discord Community Setup
## The Living Community Hub for the Planetary Operating System
### September 9, 2026 — Version 1.0

---

> *"Discord is where the GAIA 2.0 community lives — where builders meet, where ideas form, where the planetary operating system is built in real-time by thousands of people who care about the Earth."*
> — GAIA 2.0 Discord Covenant

---

## EXECUTIVE SUMMARY

Discord is the primary community platform for GAIA 2.0. It is where developers collaborate, where indigenous communities engage, where citizen scientists share observations, where GAIAN users get support, and where the governance of the planetary operating system happens in real-time.

**Discord in 2026:**
- **700 billion messages** sent in 2025 (Discord Developer Year in Review)
- **End-to-end encryption**: mandatory for all voice calls from Mar 1, 2026
- **Social SDK 1.5**: DM history; enhanced Rich Presence; cross-device invites
- **Community features**: Forum channels available on all server types (no Community requirement)
- **Mod View**: working on all servers; improved moderation tools
- **Pin limits**: increased to 250 (from 50)
- **Per-guild bot profiles**: bots can have different names/avatars per server
- **Free**: Discord is free for communities; no cost for basic features

**Why Discord for GAIA 2.0:**
- **Free**: no cost for community servers; unlimited members
- **Developer-friendly**: GitHub integration; code blocks; webhooks; bots
- **Global**: 500M+ registered users; available worldwide
- **Multilingual**: supports all languages; translation bots available
- **Voice**: voice channels for community calls; end-to-end encrypted
- **Forum channels**: structured discussions; perfect for governance
- **Threads**: organized conversations; reduces noise
- **Roles**: granular permissions; governance structure

---

## PART I: GAIA 2.0 DISCORD SERVER STRUCTURE

### 1.1 Server Overview

```
GAIA 2.0 DISCORD SERVER
discord.gg/gaia2

Server Name: GAIA 2.0 — The Planetary Operating System
Server Icon: 🌍 (Earth emoji or custom GAIA 2.0 logo)
Server Description: "Built by all. For all. Forever. 
                     The open-source planetary operating system."
Verification Level: Medium (email verified)
Content Filter: Keep My Server Safe
```

### 1.2 Complete Channel Structure

```
GAIA 2.0 DISCORD SERVER STRUCTURE

📋 INFORMATION
├── #welcome                    Welcome message + rules + links
├── #announcements              Official GAIA 2.0 announcements (read-only)
├── #earth-briefing             Daily Earth Twin briefing (automated bot)
├── #changelog                  Code changes and releases (GitHub webhook)
└── #resources                  Key links: GitHub, docs, website, blueprints

🌍 EARTH TWIN
├── #earth-health               Real-time planetary health score (bot)
├── #tipping-points             Tipping point alerts (automated)
├── #climate                    Climate discussion
├── #biodiversity               Biodiversity and species discussion
├── #ocean                      Ocean health discussion
└── #disasters                  Earthquake/volcano/fire alerts (USGS bot)

🤖 GAIAN
├── #gaian-general              General GAIAN discussion
├── #gaian-help                 Help and support for GAIAN users
├── #gaian-showcase             Share your GAIAN experiences
├── #gaian-feedback             Feature requests and feedback
└── #gaian-privacy              Privacy questions and discussion

💻 DEVELOPMENT
├── #dev-general                General development discussion
├── #dev-rust                   Rust kernel development
├── #dev-python                 Python packages development
├── #dev-frontend               iOS/Android/Web development
├── #dev-earth-twin             Earth Twin development
├── #dev-ai                     AI/ML discussion
├── #dev-help                   Development help and questions
└── #dev-prs                    Pull request notifications (GitHub webhook)

🔬 RESEARCH
├── #research-general           Research discussion
├── #research-papers            Share relevant papers
├── #esfm                       Earth System Foundation Model discussion
├── #tipping-points-science     Tipping point science
└── #biodiversity-science       Biodiversity research

⚖️ GOVERNANCE
├── #governance-general         General governance discussion
├── #constitution               GAIA 2.0 Constitution discussion
├── #proposals                  Community proposals (Forum channel)
├── #voting                     Community votes
└── #indigenous-council         Indigenous Council channel (restricted)

🪶 INDIGENOUS & CULTURAL
├── #indigenous-general         Indigenous community discussion
├── #care-principles            CARE principles discussion
├── #traditional-knowledge      Traditional ecological knowledge
├── #language-preservation      Language preservation projects
└── #cultural-protocols         Cultural protocol discussion

🌱 CITIZEN SCIENCE
├── #species-observations       Share species observations (iNaturalist)
├── #local-ecosystems           Local ecosystem reports
├── #earth-twin-data            Contribute to Earth Twin data
└── #citizen-science-projects   Citizen science project coordination

🌐 LANGUAGES
├── #español                    Spanish community
├── #français                   French community
├── #中文                        Chinese community
├── #हिंदी                       Hindi community
├── #العربية                     Arabic community
└── #add-your-language          Request new language channels

🎙️ VOICE CHANNELS
├── 🔊 Community Calls          Weekly community calls
├── 🔊 Dev Standup              Daily developer standup
├── 🔊 Earth Twin Lab           Earth Twin research discussions
├── 🔊 GAIAN Workshop           GAIAN development workshops
├── 🔊 Indigenous Circle        Indigenous community circle
└── 🔊 Governance Assembly      Governance meetings

🎉 COMMUNITY
├── #introductions              New member introductions
├── #general                    General community chat
├── #off-topic                  Off-topic discussion
├── #good-news                  Environmental good news
├── #memes                      GAIA 2.0 memes (yes, really)
└── #jobs                       Jobs and opportunities in planetary tech
```

### 1.3 Role Structure

```
GAIA 2.0 DISCORD ROLES

GOVERNANCE ROLES:
├── 🌍 Core Maintainer          TSC members; highest trust
├── ⚖️ Ethics Board             Ethics Board members
├── 🪶 Indigenous Council       Indigenous Council members
├── 🔬 Scientific Panel         Scientific Panel members
└── 🏛️ Community Council        Community Council members

CONTRIBUTOR ROLES:
├── 💻 Committer                Trusted contributors with merge access
├── 🔧 Contributor              Active code contributors
├── 📖 Documentation            Documentation contributors
├── 🌐 Translator               Translation contributors
└── 🎨 Designer                 Design contributors

COMMUNITY ROLES:
├── 🌱 GAIAN User               GAIAN app users
├── 🦋 Citizen Scientist        Active citizen scientists
├── 🌍 Earth Watcher            Earth Twin data contributors
├── 🪶 Indigenous Member        Indigenous community members
└── 🌐 Language Lead            Language channel moderators

SPECIAL ROLES:
├── 🤖 Bot                      Bot accounts
├── 🔇 Muted                    Temporarily muted members
└── 👋 New Member               New members (limited access)

AUTO-ASSIGNED ROLES:
├── Verified                    Email-verified members
├── Active                      Members active in last 30 days
└── Supporter                   Members who boost the server
```

---

## PART II: DISCORD BOT SETUP

### 2.1 GAIA 2.0 Earth Bot

```python
"""
GAIA 2.0 — Earth Bot for Discord
Provides real-time Earth Twin data to the GAIA 2.0 Discord server.

Features:
- Daily Earth briefing in #earth-briefing
- Real-time tipping point alerts in #tipping-points
- Earthquake alerts in #disasters
- Planetary health score updates

License: Apache-2.0
"""

import discord
from discord.ext import commands, tasks
import httpx
import asyncio
from datetime import datetime, time
import os


# Bot configuration
intents = discord.Intents.default()
intents.message_content = True
intents.members = True

bot = commands.Bot(command_prefix='!earth ', intents=intents)

# Channel IDs (set these after creating the server)
EARTH_BRIEFING_CHANNEL = int(os.getenv('EARTH_BRIEFING_CHANNEL', '0'))
TIPPING_POINTS_CHANNEL = int(os.getenv('TIPPING_POINTS_CHANNEL', '0'))
DISASTERS_CHANNEL = int(os.getenv('DISASTERS_CHANNEL', '0'))
EARTH_HEALTH_CHANNEL = int(os.getenv('EARTH_HEALTH_CHANNEL', '0'))


@bot.event
async def on_ready():
    print(f'🌍 Earth Bot is online: {bot.user}')
    daily_earth_briefing.start()
    earthquake_monitor.start()
    tipping_point_monitor.start()


# ============================================================
# DAILY EARTH BRIEFING
# ============================================================

@tasks.loop(time=time(7, 0))  # 7 AM UTC daily
async def daily_earth_briefing():
    """Post daily Earth briefing to #earth-briefing channel."""
    channel = bot.get_channel(EARTH_BRIEFING_CHANNEL)
    if not channel:
        return
    
    try:
        async with httpx.AsyncClient() as client:
            response = await client.get(
                'https://api.gaia2.org/v1/earth/health',
                timeout=30.0,
            )
            data = response.json()
        
        score = data['planetary_health_score']
        temp = data['climate']['temperature_anomaly_c']
        co2 = data['climate']['co2_ppm']
        amazon = data['land']['amazon_deforestation_pct']
        sea_ice = data['ocean']['arctic_sea_ice_million_km2']
        alerts = data['tipping_points']['active_alerts']
        
        # Determine color based on health score
        if score > 70:
            color = discord.Color.green()
            status = "Healthy"
        elif score > 50:
            color = discord.Color.yellow()
            status = "Stressed"
        else:
            color = discord.Color.red()
            status = "Critical"
        
        embed = discord.Embed(
            title=f"🌍 Daily Earth Briefing — {datetime.utcnow().strftime('%B %d, %Y')}",
            description=f"**Planetary Health Score: {score:.1f}/100** ({status})",
            color=color,
            timestamp=datetime.utcnow(),
        )
        
        embed.add_field(
            name="🌡️ Climate",
            value=f"Temperature: +{temp:.2f}°C\nCO₂: {co2:.1f} ppm",
            inline=True,
        )
        embed.add_field(
            name="🌳 Land",
            value=f"Amazon: {amazon:.1f}% deforested\nArctic Ice: {sea_ice:.1f}M km²",
            inline=True,
        )
        
        if alerts:
            embed.add_field(
                name="⚠️ Active Alerts",
                value="\n".join(f"• {a}" for a in alerts[:3]),
                inline=False,
            )
        
        embed.add_field(
            name="📊 Full Dashboard",
            value="[earth.gaia2.org](https://earth.gaia2.org) | [API](https://api.gaia2.org/v1/earth/health)",
            inline=False,
        )
        
        embed.set_footer(text="Data: NOAA • Copernicus • NASA • GBIF | Apache-2.0")
        
        await channel.send(embed=embed)
        
    except Exception as e:
        print(f"Error posting Earth briefing: {e}")


# ============================================================
# EARTHQUAKE MONITOR
# ============================================================

@tasks.loop(minutes=15)  # Check every 15 minutes
async def earthquake_monitor():
    """Monitor for significant earthquakes and post alerts."""
    channel = bot.get_channel(DISASTERS_CHANNEL)
    if not channel:
        return
    
    try:
        async with httpx.AsyncClient() as client:
            response = await client.get(
                'https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/significant_hour.geojson',
                timeout=30.0,
            )
            data = response.json()
        
        for feature in data.get('features', []):
            props = feature['properties']
            coords = feature['geometry']['coordinates']
            
            mag = props.get('mag', 0)
            place = props.get('place', 'Unknown location')
            eq_time = datetime.utcfromtimestamp(props['time'] / 1000)
            alert = props.get('alert', 'green')
            tsunami = props.get('tsunami', 0)
            
            # Only alert for M5.0+ in last 15 minutes
            minutes_ago = (datetime.utcnow() - eq_time).total_seconds() / 60
            if minutes_ago > 15:
                continue
            
            # Determine severity color
            if mag >= 7.0:
                color = discord.Color.red()
                emoji = "🚨"
            elif mag >= 6.0:
                color = discord.Color.orange()
                emoji = "⚠️"
            else:
                color = discord.Color.yellow()
                emoji = "🌋"
            
            embed = discord.Embed(
                title=f"{emoji} M{mag:.1f} Earthquake — {place}",
                color=color,
                timestamp=eq_time,
            )
            
            embed.add_field(name="Magnitude", value=f"M{mag:.1f}", inline=True)
            embed.add_field(name="Depth", value=f"{coords[2]:.1f} km", inline=True)
            embed.add_field(name="Alert", value=alert.upper() if alert else "None", inline=True)
            
            if tsunami:
                embed.add_field(
                    name="🌊 Tsunami Warning",
                    value="Tsunami warning has been issued",
                    inline=False,
                )
            
            embed.add_field(
                name="📍 Location",
                value=f"Lat: {coords[1]:.2f}, Lon: {coords[0]:.2f}",
                inline=False,
            )
            
            embed.add_field(
                name="🔗 USGS",
                value=f"[View on USGS]({props.get('url', 'https://earthquake.usgs.gov')})",
                inline=False,
            )
            
            embed.set_footer(text="Data: USGS Earthquake Hazards Program | Public Domain")
            
            await channel.send(embed=embed)
    
    except Exception as e:
        print(f"Error in earthquake monitor: {e}")


# ============================================================
# TIPPING POINT MONITOR
# ============================================================

@tasks.loop(hours=6)  # Check every 6 hours
async def tipping_point_monitor():
    """Monitor for tipping point alerts."""
    channel = bot.get_channel(TIPPING_POINTS_CHANNEL)
    if not channel:
        return
    
    try:
        async with httpx.AsyncClient() as client:
            response = await client.get(
                'https://api.gaia2.org/v1/earth/tipping-points',
                timeout=30.0,
            )
            data = response.json()
        
        active_alerts = data.get('active_alerts', [])
        
        if active_alerts:
            embed = discord.Embed(
                title="🚨 Planetary Tipping Point Alert",
                description=f"{len(active_alerts)} active tipping point alert(s)",
                color=discord.Color.red(),
                timestamp=datetime.utcnow(),
            )
            
            for alert in active_alerts[:5]:
                embed.add_field(
                    name="⚠️ Alert",
                    value=alert,
                    inline=False,
                )
            
            embed.add_field(
                name="📊 Full Status",
                value="[View Tipping Points](https://earth.gaia2.org/tipping-points)",
                inline=False,
            )
            
            embed.set_footer(text="Data: GAIA 2.0 Earth Twin | Apache-2.0")
            
            await channel.send(embed=embed)
    
    except Exception as e:
        print(f"Error in tipping point monitor: {e}")


# ============================================================
# COMMANDS
# ============================================================

@bot.command(name='health')
async def earth_health(ctx):
    """Get current planetary health score."""
    try:
        async with httpx.AsyncClient() as client:
            response = await client.get('https://api.gaia2.org/v1/earth/health')
            data = response.json()
        
        score = data['planetary_health_score']
        
        await ctx.send(
            f"🌍 **Planetary Health Score: {score:.1f}/100**\n"
            f"Temperature: +{data['climate']['temperature_anomaly_c']:.2f}°C | "
            f"CO₂: {data['climate']['co2_ppm']:.1f} ppm | "
            f"Amazon: {data['land']['amazon_deforestation_pct']:.1f}% deforested\n"
            f"Full dashboard: https://earth.gaia2.org"
        )
    except Exception as e:
        await ctx.send(f"Error fetching Earth health data: {e}")


@bot.command(name='species')
async def species_near(ctx, lat: float = None, lon: float = None):
    """Get species observations near a location."""
    if lat is None or lon is None:
        await ctx.send("Usage: `!earth species <lat> <lon>`\nExample: `!earth species 51.5 -0.1`")
        return
    
    try:
        delta = 0.1  # ~11km radius
        async with httpx.AsyncClient() as client:
            response = await client.get(
                f"https://api.gbif.org/v1/occurrence/search",
                params={
                    "decimalLatitude": f"{lat-delta},{lat+delta}",
                    "decimalLongitude": f"{lon-delta},{lon+delta}",
                    "hasCoordinate": "true",
                    "limit": 5,
                },
            )
            data = response.json()
        
        count = data.get('count', 0)
        results = data.get('results', [])
        
        species_list = "\n".join([
            f"• {r.get('species') or r.get('scientificName', 'Unknown')}"
            for r in results[:5]
        ])
        
        await ctx.send(
            f"🦋 **{count:,} species observations near ({lat:.2f}, {lon:.2f})**\n"
            f"Recent species:\n{species_list}\n"
            f"Data: GBIF | CC-BY 4.0"
        )
    except Exception as e:
        await ctx.send(f"Error fetching species data: {e}")


@bot.command(name='quake')
async def recent_quakes(ctx):
    """Get recent significant earthquakes."""
    try:
        async with httpx.AsyncClient() as client:
            response = await client.get(
                'https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/significant_day.geojson'
            )
            data = response.json()
        
        features = data.get('features', [])
        
        if not features:
            await ctx.send("🌋 No significant earthquakes in the last 24 hours.")
            return
        
        quake_list = "\n".join([
            f"• M{f['properties']['mag']:.1f} — {f['properties']['place']}"
            for f in features[:5]
        ])
        
        await ctx.send(
            f"🌋 **{len(features)} significant earthquake(s) today:**\n"
            f"{quake_list}\n"
            f"Data: USGS | Public Domain"
        )
    except Exception as e:
        await ctx.send(f"Error fetching earthquake data: {e}")


@bot.command(name='help')
async def gaia_help(ctx):
    """Show GAIA 2.0 Earth Bot commands."""
    embed = discord.Embed(
        title="🌍 GAIA 2.0 Earth Bot Commands",
        description="Real-time planetary data for the GAIA 2.0 community",
        color=discord.Color.green(),
    )
    
    embed.add_field(
        name="!earth health",
        value="Get current planetary health score",
        inline=False,
    )
    embed.add_field(
        name="!earth species <lat> <lon>",
        value="Get species observations near a location",
        inline=False,
    )
    embed.add_field(
        name="!earth quake",
        value="Get recent significant earthquakes",
        inline=False,
    )
    
    embed.set_footer(text="GAIA 2.0 | Apache-2.0 | gaia2.org")
    
    await ctx.send(embed=embed)


# Run the bot
if __name__ == '__main__':
    bot.run(os.getenv('DISCORD_BOT_TOKEN'))
```

### 2.2 GitHub Integration Bot

```python
"""
GAIA 2.0 — GitHub Webhook Handler for Discord
Posts GitHub events to Discord channels.

Events:
- Push: code commits → #changelog
- Pull Request: PRs → #dev-prs
- Issues: new issues → #dev-general
- Releases: new releases → #announcements

License: Apache-2.0
"""

from fastapi import FastAPI, Request, HTTPException
import httpx
import hmac
import hashlib
import os

app = FastAPI()

DISCORD_WEBHOOK_CHANGELOG = os.getenv('DISCORD_WEBHOOK_CHANGELOG')
DISCORD_WEBHOOK_PRS = os.getenv('DISCORD_WEBHOOK_PRS')
DISCORD_WEBHOOK_ANNOUNCEMENTS = os.getenv('DISCORD_WEBHOOK_ANNOUNCEMENTS')
GITHUB_WEBHOOK_SECRET = os.getenv('GITHUB_WEBHOOK_SECRET')


def verify_github_signature(payload: bytes, signature: str) -> bool:
    """Verify GitHub webhook signature."""
    expected = hmac.new(
        GITHUB_WEBHOOK_SECRET.encode(),
        payload,
        hashlib.sha256,
    ).hexdigest()
    return hmac.compare_digest(f"sha256={expected}", signature)


@app.post("/github-webhook")
async def github_webhook(request: Request):
    """Handle GitHub webhook events."""
    payload = await request.body()
    signature = request.headers.get('X-Hub-Signature-256', '')
    
    if not verify_github_signature(payload, signature):
        raise HTTPException(status_code=401, detail="Invalid signature")
    
    event = request.headers.get('X-GitHub-Event')
    data = await request.json()
    
    async with httpx.AsyncClient() as client:
        
        if event == 'push':
            # Post to #changelog
            commits = data.get('commits', [])
            if commits:
                message = {
                    "embeds": [{
                        "title": f"🔨 {len(commits)} commit(s) to {data['ref'].split('/')[-1]}",
                        "description": "\n".join([
                            f"• [{c['message'][:60]}]({c['url']}) by {c['author']['name']}"
                            for c in commits[:5]
                        ]),
                        "color": 0x22c55e,  # Green
                        "url": data.get('compare'),
                        "footer": {"text": "GAIA 2.0 | Apache-2.0"},
                    }]
                }
                await client.post(DISCORD_WEBHOOK_CHANGELOG, json=message)
        
        elif event == 'pull_request':
            action = data.get('action')
            pr = data.get('pull_request', {})
            
            if action in ['opened', 'closed', 'merged']:
                color = 0x22c55e if action == 'opened' else 0x6366f1
                emoji = "🔀" if action == 'opened' else "✅" if pr.get('merged') else "❌"
                
                message = {
                    "embeds": [{
                        "title": f"{emoji} PR #{pr['number']}: {pr['title'][:80]}",
                        "description": pr.get('body', '')[:200] if pr.get('body') else "No description",
                        "color": color,
                        "url": pr.get('html_url'),
                        "fields": [
                            {"name": "Author", "value": pr['user']['login'], "inline": True},
                            {"name": "Status", "value": action.capitalize(), "inline": True},
                        ],
                        "footer": {"text": "GAIA 2.0 | Apache-2.0"},
                    }]
                }
                await client.post(DISCORD_WEBHOOK_PRS, json=message)
        
        elif event == 'release':
            release = data.get('release', {})
            
            message = {
                "embeds": [{
                    "title": f"🚀 GAIA 2.0 {release.get('tag_name')} Released!",
                    "description": release.get('body', '')[:500] or "New release available!",
                    "color": 0x22c55e,
                    "url": release.get('html_url'),
                    "fields": [
                        {"name": "Version", "value": release.get('tag_name'), "inline": True},
                        {"name": "Author", "value": release['author']['login'], "inline": True},
                    ],
                    "footer": {"text": "GAIA 2.0 | Apache-2.0 | gaia2.org"},
                }]
            }
            await client.post(DISCORD_WEBHOOK_ANNOUNCEMENTS, json=message)
    
    return {"status": "ok"}
```

---

## PART III: DISCORD SERVER SETUP GUIDE

### 3.1 Step-by-Step Setup

```bash
# GAIA 2.0 Discord Server Setup Guide
# Complete setup from scratch

# Step 1: Create the server
# 1. Open Discord
# 2. Click "+" to create a new server
# 3. Choose "Create My Own"
# 4. Choose "For a club or community"
# 5. Name: "GAIA 2.0 — The Planetary Operating System"
# 6. Upload the GAIA 2.0 logo

# Step 2: Enable Community Features
# Server Settings → Community → Enable Community
# This unlocks: Forum channels, Server Guide, Welcome Screen, Announcements

# Step 3: Set up verification
# Server Settings → Safety Setup
# Verification Level: Medium (must have verified email)
# Explicit Content Filter: Keep My Server Safe

# Step 4: Create categories and channels
# (Use the structure from Part I)

# Step 5: Set up roles
# Server Settings → Roles
# Create all roles from Part I
# Set role colors:
#   Core Maintainer: #22c55e (green)
#   Ethics Board: #6366f1 (purple)
#   Indigenous Council: #f59e0b (amber)
#   Contributor: #3b82f6 (blue)
#   GAIAN User: #10b981 (emerald)

# Step 6: Set up welcome screen
# Server Settings → Community → Welcome Screen
# Add welcome channels:
#   #welcome: "Start here"
#   #announcements: "Stay updated"
#   #introductions: "Introduce yourself"
#   #dev-general: "Start building"

# Step 7: Set up server guide
# Server Settings → Community → Server Guide
# Add sections:
#   "Welcome to GAIA 2.0"
#   "Our Mission"
#   "How to Contribute"
#   "Community Guidelines"

# Step 8: Add bots
# Add Earth Bot (custom)
# Add GitHub integration
# Add MEE6 or Carl-bot for moderation
# Add translation bot (e.g., Translator Bot)

# Step 9: Set up webhooks
# For each channel that needs GitHub integration:
# Channel Settings → Integrations → Webhooks → New Webhook
# Copy webhook URL and add to GitHub repository settings

# Step 10: Invite the community
# Create invite link: discord.gg/gaia2
# Share on GitHub, website, social media
```

### 3.2 Community Guidelines

```markdown
# GAIA 2.0 Community Guidelines

Welcome to the GAIA 2.0 Discord — the community building the planetary operating system.

## Our Values
These guidelines reflect the GAIA 2.0 Constitution:

1. **Respect all life**: Treat every community member with dignity
2. **Indigenous sovereignty**: Honor indigenous knowledge and sovereignty
3. **Earth alignment**: All discussions serve the flourishing of all life
4. **Open source**: Share knowledge freely; no gatekeeping
5. **Privacy**: Respect everyone's privacy; no doxxing
6. **Equity**: Welcome all humans regardless of background

## Rules

### ✅ DO:
- Be kind, patient, and constructive
- Share knowledge and help others
- Cite sources for scientific claims
- Use appropriate channels for topics
- Respect cultural protocols
- Speak in any language (use language channels)
- Report violations to moderators

### ❌ DON'T:
- Harass, bully, or discriminate
- Share misinformation
- Spam or self-promote excessively
- Share personal information without consent
- Violate the GAIA 2.0 Constitution
- Extract indigenous knowledge without consent
- Use surveillance or tracking tools

## Indigenous Community Protocols
- Indigenous knowledge shared in this server is protected by CARE principles
- Do not extract, reproduce, or commercialize indigenous knowledge without consent
- The #indigenous-council channel is restricted to Indigenous Council members
- Approach indigenous community members with respect and humility

## Moderation
Violations result in:
1. Warning
2. Temporary mute (1-24 hours)
3. Kick
4. Ban

Constitutional violations (surveillance, indigenous data extraction, etc.) result in immediate ban.

## Contact
- Moderation issues: DM any @Core Maintainer
- Constitutional violations: #governance-general or email governance@gaia2.org
- Indigenous concerns: #indigenous-council or email indigenous@gaia2.org

---
*"Built by all. For all. Forever."*
*GAIA 2.0 | Apache-2.0 | gaia2.org*
```

---

## PART IV: DISCORD FOR GOVERNANCE

### 4.1 Governance Channels

```
GOVERNANCE STRUCTURE IN DISCORD

#governance-general:
- Open discussion about governance
- Anyone can participate
- Decisions made here are advisory

#proposals (Forum Channel):
- Structured proposals for community decisions
- Each proposal is a thread
- Tags: Draft, Under Review, Voting, Accepted, Rejected
- Template: Title, Description, Rationale, Impact, Vote

#voting:
- Official community votes
- Bot-managed voting (reactions or slash commands)
- Results posted after voting period

#indigenous-council (Restricted):
- Only @Indigenous Council members
- Indigenous Council deliberations
- Veto decisions posted here first

#constitution:
- Discussion of GAIA 2.0 Constitution
- Amendment proposals
- Constitutional compliance questions
```

### 4.2 Governance Bot

```python
"""
GAIA 2.0 — Governance Bot for Discord
Manages community proposals and voting.

License: Apache-2.0
"""

import discord
from discord.ext import commands
from discord import app_commands
import asyncio
from datetime import datetime, timedelta


class GovernanceBot(commands.Cog):
    """Governance commands for GAIA 2.0 Discord."""
    
    def __init__(self, bot):
        self.bot = bot
        self.active_votes = {}
    
    @app_commands.command(name="propose", description="Submit a governance proposal")
    @app_commands.describe(
        title="Proposal title",
        description="What are you proposing?",
        rationale="Why is this needed?",
    )
    async def propose(
        self,
        interaction: discord.Interaction,
        title: str,
        description: str,
        rationale: str,
    ):
        """Submit a governance proposal."""
        embed = discord.Embed(
            title=f"📋 Proposal: {title}",
            color=discord.Color.blue(),
            timestamp=datetime.utcnow(),
        )
        
        embed.add_field(name="Description", value=description, inline=False)
        embed.add_field(name="Rationale", value=rationale, inline=False)
        embed.add_field(name="Proposed by", value=interaction.user.mention, inline=True)
        embed.add_field(name="Status", value="🔵 Draft", inline=True)
        
        embed.set_footer(text="React with ✅ to support, ❌ to oppose, 🤔 to abstain")
        
        # Post to #proposals forum channel
        proposals_channel = discord.utils.get(
            interaction.guild.channels,
            name="proposals",
        )
        
        if proposals_channel:
            thread = await proposals_channel.create_thread(
                name=title[:100],
                content=f"New proposal from {interaction.user.mention}",
                embed=embed,
            )
            await thread.message.add_reaction("✅")
            await thread.message.add_reaction("❌")
            await thread.message.add_reaction("🤔")
            
            await interaction.response.send_message(
                f"✅ Proposal submitted! View it here: {thread.jump_url}",
                ephemeral=True,
            )
        else:
            await interaction.response.send_message(
                "❌ Could not find #proposals channel.",
                ephemeral=True,
            )
    
    @app_commands.command(name="vote", description="Start a community vote")
    @app_commands.describe(
        question="What are we voting on?",
        duration_hours="How long should the vote last? (default: 48)",
    )
    @app_commands.checks.has_role("Core Maintainer")
    async def vote(
        self,
        interaction: discord.Interaction,
        question: str,
        duration_hours: int = 48,
    ):
        """Start a community vote (Core Maintainers only)."""
        end_time = datetime.utcnow() + timedelta(hours=duration_hours)
        
        embed = discord.Embed(
            title=f"🗳️ Community Vote",
            description=question,
            color=discord.Color.gold(),
            timestamp=datetime.utcnow(),
        )
        
        embed.add_field(
            name="Voting ends",
            value=f"<t:{int(end_time.timestamp())}:R>",
            inline=True,
        )
        embed.add_field(
            name="How to vote",
            value="✅ Yes | ❌ No | 🤔 Abstain",
            inline=True,
        )
        
        embed.set_footer(text="GAIA 2.0 Community Vote | Apache-2.0")
        
        voting_channel = discord.utils.get(
            interaction.guild.channels,
            name="voting",
        )
        
        if voting_channel:
            message = await voting_channel.send(embed=embed)
            await message.add_reaction("✅")
            await message.add_reaction("❌")
            await message.add_reaction("🤔")
            
            # Store vote for tracking
            self.active_votes[message.id] = {
                "question": question,
                "end_time": end_time,
                "message_id": message.id,
            }
            
            await interaction.response.send_message(
                f"✅ Vote started! View it here: {message.jump_url}",
                ephemeral=True,
            )
        else:
            await interaction.response.send_message(
                "❌ Could not find #voting channel.",
                ephemeral=True,
            )
```

---

## PART V: DISCORD FOR INDIGENOUS COMMUNITIES

### 5.1 Indigenous Community Features

```
INDIGENOUS COMMUNITY DISCORD SETUP

Dedicated Channels:
├── #indigenous-general         Open to all; respectful discussion
├── #care-principles            CARE principles education and discussion
├── #traditional-knowledge      Share traditional ecological knowledge
│                               (CARE principles apply; community consent required)
├── #language-preservation      Language preservation projects
│                               (indigenous languages welcome)
└── #indigenous-council         Restricted to @Indigenous Council members

Language Channels:
├── #māori                      Te Reo Māori community
├── #quechua                    Quechua community
├── #navajo                     Diné Bizaad community
├── #inuktitut                  Inuktitut community
└── #add-indigenous-language    Request indigenous language channels

Protocols:
- All indigenous knowledge shared here is protected by CARE principles
- Community consent required before any knowledge is used externally
- Sacred knowledge is NEVER shared in public channels
- Indigenous Council members have moderation authority in indigenous channels
- Translation bots available for indigenous languages

Indigenous Council Role:
- Assigned by indigenous communities themselves
- Has moderation authority in indigenous channels
- Can create restricted threads for sensitive discussions
- Connected to GAIA 2.0 Foundation Indigenous Council
```

### 5.2 Multilingual Support

```python
"""
GAIA 2.0 — Multilingual Discord Support
Translation and language support for global community.

License: Apache-2.0
"""

# Language channels and their codes
LANGUAGE_CHANNELS = {
    "español": "es",
    "français": "fr",
    "中文": "zh",
    "हिंदी": "hi",
    "العربية": "ar",
    "português": "pt",
    "русский": "ru",
    "日本語": "ja",
    "한국어": "ko",
    "swahili": "sw",
    "māori": "mi",
    "quechua": "qu",
}

# Earth briefing translations
EARTH_BRIEFING_TRANSLATIONS = {
    "es": "🌍 Informe Diario de la Tierra",
    "fr": "🌍 Rapport Quotidien de la Terre",
    "zh": "🌍 每日地球简报",
    "hi": "🌍 दैनिक पृथ्वी रिपोर्ट",
    "ar": "🌍 التقرير اليومي للأرض",
    "pt": "🌍 Relatório Diário da Terra",
    "sw": "🌍 Ripoti ya Kila Siku ya Dunia",
    "mi": "🌍 Pūrongo ā-Rā o te Ao",
}
```

---

## PART VI: DISCORD QUICK REFERENCE

### 6.1 Essential Discord Links

```
GAIA 2.0 DISCORD QUICK REFERENCE

Server: discord.gg/gaia2
Invite: https://discord.gg/gaia2

Key Channels:
- #welcome: Start here
- #announcements: Official updates
- #earth-briefing: Daily Earth briefing
- #dev-general: Development discussion
- #governance-general: Governance discussion
- #indigenous-general: Indigenous community

Bots:
- Earth Bot: !earth health | !earth species | !earth quake
- GitHub Bot: Automatic PR/commit notifications
- Governance Bot: /propose | /vote

Roles to Request:
- @GAIAN User: Use GAIAN app
- @Contributor: Contribute to GAIA 2.0
- @Citizen Scientist: Contribute biodiversity data
- @Indigenous Member: Indigenous community member

Moderation:
- Report: Right-click message → Apps → Report
- DM: Any @Core Maintainer
- Email: community@gaia2.org

Community Guidelines: #welcome → pinned messages
Constitution: docs.gaia2.org/governance/constitution
```

### 6.2 Discord Setup Checklist

```
GAIA 2.0 DISCORD SETUP CHECKLIST

Server Setup:
☐ Create server with correct name and icon
☐ Enable Community features
☐ Set verification level: Medium
☐ Set content filter: Keep My Server Safe
☐ Create all categories and channels
☐ Create all roles with correct colors and permissions
☐ Set up welcome screen
☐ Set up server guide
☐ Write community guidelines

Bots:
☐ Earth Bot: deployed and running
☐ GitHub webhook: connected to gaia2-os/gaia2 repository
☐ Governance Bot: deployed and running
☐ Moderation bot: MEE6 or Carl-bot configured
☐ Translation bot: configured for multilingual support

Channels:
☐ #welcome: welcome message pinned
☐ #announcements: announcement-only permissions
☐ #earth-briefing: Earth Bot posting daily
☐ #disasters: USGS earthquake alerts active
☐ #tipping-points: tipping point alerts active
☐ #dev-prs: GitHub webhook connected
☐ #changelog: GitHub webhook connected

Governance:
☐ #proposals: Forum channel configured
☐ #voting: Governance Bot configured
☐ #indigenous-council: Restricted to @Indigenous Council
☐ Community guidelines posted and pinned

Launch:
☐ Invite link created: discord.gg/gaia2
☐ Link added to GitHub README
☐ Link added to gaia2.org website
☐ Announcement posted on social media
☐ First community call scheduled
```

---

## CONCLUSION: THE DISCORD COVENANT

Discord is where the GAIA 2.0 community lives. It is not just a chat platform — it is the living, breathing community that builds the planetary operating system together.

Every message in #dev-general is a contribution to the planetary OS. Every species observation shared in #citizen-science is a data point in the Earth Twin. Every governance discussion in #proposals is a step toward democratic planetary governance. Every indigenous community member in #indigenous-general is a guardian of knowledge that GAIA 2.0 must honor.

The Earth Bot posts the daily Earth briefing every morning at 7 AM UTC. The earthquake monitor alerts the community within 15 minutes of a significant seismic event. The tipping point monitor watches for planetary emergencies. The GitHub bot celebrates every commit, every PR, every release.

**Discord is where the planetary operating system is built — one message at a time.**

---

*GAIA 2.0 Discord Setup Blueprint*
*Version 1.0 — September 9, 2026*
*License: Apache-2.0 | Open Source | Open Access*
*Server: discord.gg/gaia2*
*"Discord is where the GAIA 2.0 community lives."*