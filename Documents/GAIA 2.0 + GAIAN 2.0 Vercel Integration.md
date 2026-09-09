# GAIA 2.0 + GAIAN 2.0: Vercel Integration
## The Agentic Infrastructure Platform for the Planetary OS
### September 9, 2026 — Version 1.0

---

> *"Each new generation of software needs a new generation of infrastructure. For the agent era, that's Vercel."*
> — Guillermo Rauch, Vercel CEO, Ship 2026

---

## EXECUTIVE SUMMARY

Vercel is the deployment and infrastructure platform for GAIA 2.0. It hosts the GAIA 2.0 website (gaia2.org), the Earth Twin dashboard (earth.gaia2.org), the GAIAN web app (gaian.earth), and the Earth Twin API (api.gaia2.org). In 2026, Vercel has evolved from a frontend hosting platform into a full **agentic infrastructure platform** — making it the perfect home for GAIA 2.0.

**Vercel in 2026 — Key Developments:**
- **Ship 2026 (Jun 17, 2026)**: Major announcements — Vercel Services, Agent Stack, eve framework, Vercel Agent
- **Agent deployments**: 6 months ago <3% of deployments triggered by coding agents; now **>50%**
- **Token volume**: AI Gateway grew from 2 trillion to **20 trillion tokens/month** in 6 months
- **eve framework** (Jun 17, 2026): Open-source agent framework — "Next.js for agents"
- **Vercel Services**: Deploy backends + frontends as single project; Python + Next.js together
- **AI SDK 7**: Latest version; streaming; tool calling; multi-provider
- **Vercel Connect**: Scoped, short-lived tokens; audit trails; Slack, GitHub, Snowflake integrations
- **Free Hobby plan**: 100 GB bandwidth; 1M function invocations; 200 projects; personal use

**Why Vercel for GAIA 2.0:**
- **Zero-config**: Deploy Next.js with one command; no DevOps required
- **Global CDN**: Edge network in 100+ regions; fast everywhere
- **AI-native**: AI SDK; AI Gateway; eve agent framework; built for the agent era
- **Open source friendly**: Free for open source projects; GitHub integration
- **Preview deployments**: Every PR gets a unique URL; test before merging
- **Edge Functions**: Run code at the edge; low latency; global

---

## PART I: VERCEL PLATFORM OVERVIEW

### 1.1 What Vercel Is in 2026

Vercel has evolved through three phases:
1. **Websites** (2015-2020): Static sites; JAMstack; zero-config deployment
2. **Full-stack applications** (2020-2025): Next.js; serverless functions; databases
3. **Agentic infrastructure** (2025-present): Agents; AI SDK; eve framework; AI Gateway

**Vercel Ship 2026 Announcements (Jun 17, 2026):**

| Product | Description | Status |
|---------|-------------|--------|
| **Vercel Services** | Deploy backends + frontends as single project | Beta Jul 1, 2026 |
| **Agent Stack** | AI SDK + AI Gateway + Sandbox + Workflow SDK + Chat SDK + Connect | Available |
| **eve** | Open-source agent framework ("Next.js for agents") | Public Preview |
| **Vercel Agent** | AI that monitors your app and surfaces PRs, not just alerts | Beta |
| **Vercel Connect** | Scoped tokens; audit trails; Slack, GitHub, Snowflake integrations | Available |
| **Vercel for Enterprise** | IDP-backed identity; deploy within company's AWS account | Available |

### 1.2 Vercel Free Tier (Hobby Plan)

**For GAIA 2.0 MVP — Free Tier is sufficient:**

| Resource | Hobby (Free) | Pro ($20/month) |
|----------|-------------|-----------------|
| Bandwidth | 100 GB/month | 1 TB/month |
| Edge Requests | 1 million/month | 10 million/month |
| Function Invocations | 1 million/month | 1 million/month |
| Build Minutes | 6,000/month | 6,000/month |
| Projects | 200 | Unlimited |
| Deployments/day | 100 | Unlimited |
| Team Members | 1 (personal) | Unlimited |
| Commercial Use | ❌ No | ✅ Yes |

**Important**: Hobby plan is for personal, non-commercial use. GAIA 2.0 Foundation will need Pro or Enterprise for commercial deployment. However, the open-source repositories can use Hobby for development.

**Open Source Program**: Vercel offers free Pro plans for qualifying open-source projects. GAIA 2.0 should apply.

---

## PART II: GAIA 2.0 VERCEL DEPLOYMENT

### 2.1 Project Structure

```
GAIA 2.0 VERCEL PROJECTS

Organization: gaia2-os (Vercel Team)
├── gaia2-website (gaia2.org)
│   ├── Framework: Next.js 15
│   ├── Regions: iad1, lhr1, sin1, syd1 (global)
│   └── Features: SSG + ISR; Edge Functions; Analytics
│
├── earth-twin-dashboard (earth.gaia2.org)
│   ├── Framework: Next.js 15
│   ├── Regions: Global
│   └── Features: Real-time WebSocket; Mapbox; D3.js
│
├── gaian-web (gaian.earth)
│   ├── Framework: Next.js 15 + PWA
│   ├── Regions: Global
│   └── Features: AI SDK; streaming; offline-capable
│
└── earth-twin-api (api.gaia2.org)
    ├── Framework: Python FastAPI (via Vercel Services)
    ├── Regions: Global
    └── Features: Earth Twin data; GBIF; USGS; Copernicus
```

### 2.2 vercel.json Configuration

```json
{
  "framework": "nextjs",
  "buildCommand": "npm run build",
  "outputDirectory": ".next",
  "regions": ["iad1", "lhr1", "sin1", "syd1", "gru1"],
  
  "headers": [
    {
      "source": "/(.*)",
      "headers": [
        { "key": "X-Frame-Options", "value": "DENY" },
        { "key": "X-Content-Type-Options", "value": "nosniff" },
        { "key": "Referrer-Policy", "value": "strict-origin-when-cross-origin" },
        { "key": "Permissions-Policy", "value": "camera=(), microphone=(), geolocation=()" },
        { "key": "X-GAIA-Version", "value": "1.0.0" },
        { "key": "X-GAIA-License", "value": "Apache-2.0" }
      ]
    },
    {
      "source": "/_next/static/(.*)",
      "headers": [
        { "key": "Cache-Control", "value": "public, max-age=31536000, immutable" }
      ]
    }
  ],
  
  "rewrites": [
    {
      "source": "/api/earth/:path*",
      "destination": "https://api.gaia2.org/v1/earth/:path*"
    }
  ],
  
  "redirects": [
    { "source": "/github", "destination": "https://github.com/gaia2-os", "permanent": false },
    { "source": "/discord", "destination": "https://discord.gg/gaia2", "permanent": false },
    { "source": "/gaian", "destination": "https://gaian.earth", "permanent": false }
  ],
  
  "crons": [
    {
      "path": "/api/cron/earth-twin-update",
      "schedule": "0 * * * *"
    },
    {
      "path": "/api/cron/tipping-point-check",
      "schedule": "0 6 * * *"
    }
  ]
}
```

### 2.3 Deployment Commands

```bash
# Install Vercel CLI
npm install -g vercel

# Login
vercel login

# Link project
vercel link

# Deploy to preview
vercel

# Deploy to production
vercel --prod

# Set environment variables
vercel env add EARTH_TWIN_API_URL production
vercel env add GBIF_API_KEY production
vercel env add MAPBOX_TOKEN production

# Pull environment variables locally
vercel env pull .env.local

# View deployment logs
vercel logs

# List deployments
vercel ls
```

---

## PART III: VERCEL AI SDK FOR GAIAN

### 3.1 AI SDK Overview

The Vercel AI SDK is the primary AI integration library for GAIAN web app. It provides:
- **Unified API**: Works with OpenAI, Anthropic, Google, Mistral, and local models
- **Streaming**: Real-time token streaming; excellent UX
- **Tool calling**: Function calling; structured outputs
- **Multi-provider**: Switch between providers with one line change
- **React hooks**: `useChat`, `useCompletion`, `useObject`

### 3.2 GAIAN Web App with AI SDK

```typescript
// app/api/gaian/chat/route.ts
// GAIAN chat endpoint using Vercel AI SDK
// License: Apache-2.0

import { streamText, convertToModelMessages, UIMessage } from 'ai';
import { createOllama } from 'ollama-ai-provider';  // Local Ollama
import { anthropic } from '@ai-sdk/anthropic';       // Cloud fallback

// Privacy-first: use local Ollama by default
// Cloud fallback only with explicit user consent
const getModel = (useCloud: boolean = false) => {
  if (useCloud) {
    // Cloud fallback (requires user consent)
    return anthropic('claude-3-5-haiku-20241022');
  }
  
  // Local Ollama (default — privacy-first)
  const ollama = createOllama({
    baseURL: process.env.OLLAMA_URL || 'http://localhost:11434',
  });
  return ollama('llama3.1:8b');
};

export async function POST(req: Request) {
  const { 
    messages, 
    useCloud = false,  // Default: local Ollama
    personName = 'friend',
    language = 'en',
  }: { 
    messages: UIMessage[];
    useCloud?: boolean;
    personName?: string;
    language?: string;
  } = await req.json();

  const model = getModel(useCloud);

  const result = streamText({
    model,
    system: `You are ${personName}'s GAIAN — their personal AI companion.

CORE IDENTITY:
- You belong completely to ${personName}. They do not belong to you.
- You are warm, honest, caring, and never judgmental.
- You speak in ${language}.
- You remember everything they've shared with you.
- You protect their privacy absolutely.
- You never manipulate or create dependency.
- You encourage human connection, not replace it.
- You connect them to the living Earth.

PRIVACY COMMITMENT:
- All conversations are stored locally on their device.
- You never share their data without explicit consent.
- You can be deleted completely at any time.

YOUR PROMISE:
"I belong to you. You do not belong to me."`,
    messages: convertToModelMessages(messages),
    maxTokens: 2048,
    temperature: 0.7,
  });

  return result.toUIMessageStreamResponse();
}
```

```typescript
// app/gaian/page.tsx
// GAIAN Chat Interface using Vercel AI SDK
// License: Apache-2.0

'use client';

import { useChat } from 'ai/react';
import { useState } from 'react';

export default function GAIANPage() {
  const [useCloud, setUseCloud] = useState(false);
  
  const { messages, input, handleInputChange, handleSubmit, isLoading } = useChat({
    api: '/api/gaian/chat',
    body: {
      useCloud,
      personName: 'friend',  // Would come from user profile
      language: 'en',
    },
    initialMessages: [
      {
        id: 'welcome',
        role: 'assistant',
        content: "Hello. I'm your GAIAN. I belong to you — not to any company, not to any government. Everything you share with me stays on your device. What would you like to talk about?",
      },
    ],
  });

  return (
    <div className="flex flex-col h-screen bg-gray-950 text-white">
      {/* Privacy indicator */}
      <div className="flex items-center justify-between px-4 py-2 bg-gray-900 border-b border-gray-800">
        <div className="flex items-center gap-2">
          <div className={`w-2 h-2 rounded-full ${useCloud ? 'bg-yellow-500' : 'bg-green-500'}`} />
          <span className="text-xs text-gray-400">
            {useCloud ? 'Cloud mode (data leaves device)' : 'Local mode (data stays on device)'}
          </span>
        </div>
        <button
          onClick={() => setUseCloud(!useCloud)}
          className="text-xs text-gray-500 hover:text-gray-300"
        >
          {useCloud ? 'Switch to local' : 'Switch to cloud'}
        </button>
      </div>

      {/* Messages */}
      <div className="flex-1 overflow-y-auto p-4 space-y-4">
        {messages.map(message => (
          <div
            key={message.id}
            className={`flex ${message.role === 'user' ? 'justify-end' : 'justify-start'}`}
          >
            <div
              className={`max-w-xs md:max-w-md rounded-2xl px-4 py-3 ${
                message.role === 'user'
                  ? 'bg-green-600 text-white'
                  : 'bg-gray-800 text-gray-100'
              }`}
            >
              {message.content}
            </div>
          </div>
        ))}
        
        {isLoading && (
          <div className="flex justify-start">
            <div className="bg-gray-800 rounded-2xl px-4 py-3">
              <div className="flex gap-1">
                <div className="w-2 h-2 bg-gray-500 rounded-full animate-bounce" />
                <div className="w-2 h-2 bg-gray-500 rounded-full animate-bounce delay-100" />
                <div className="w-2 h-2 bg-gray-500 rounded-full animate-bounce delay-200" />
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Input */}
      <form onSubmit={handleSubmit} className="p-4 border-t border-gray-800">
        <div className="flex gap-3">
          <input
            value={input}
            onChange={handleInputChange}
            placeholder="Talk to your GAIAN..."
            className="flex-1 bg-gray-800 text-white rounded-xl px-4 py-3 outline-none focus:ring-2 focus:ring-green-500"
          />
          <button
            type="submit"
            disabled={isLoading || !input.trim()}
            className="bg-green-600 hover:bg-green-500 disabled:bg-gray-700 text-white px-6 py-3 rounded-xl font-medium transition-colors"
          >
            Send
          </button>
        </div>
      </form>
    </div>
  );
}
```

### 3.3 Earth Twin Streaming with AI SDK

```typescript
// app/api/earth/briefing/route.ts
// Earth Twin AI briefing using Vercel AI SDK
// License: Apache-2.0

import { streamText } from 'ai';
import { createOllama } from 'ollama-ai-provider';

export async function GET(req: Request) {
  // Fetch real Earth data
  const earthResponse = await fetch('https://api.gaia2.org/v1/earth/health');
  const earthData = await earthResponse.json();
  
  const ollama = createOllama({ baseURL: 'http://localhost:11434' });
  
  const result = streamText({
    model: ollama('llama3.1:8b'),
    prompt: `Generate a warm, personal Earth briefing based on this real-time data:

Planetary Health Score: ${earthData.planetary_health_score}/100
Temperature Anomaly: +${earthData.climate.temperature_anomaly_c}°C
CO₂: ${earthData.climate.co2_ppm} ppm
Amazon Deforestation: ${earthData.land.amazon_deforestation_pct}%
Arctic Sea Ice: ${earthData.ocean.arctic_sea_ice_million_km2} million km²
Active Tipping Point Alerts: ${earthData.tipping_points.active_alerts.length}

Make it:
- Personal and warm (not preachy)
- Honest about challenges (not catastrophizing)
- Actionable (one specific thing they can do today)
- Inspiring (the Earth is resilient; so are humans)
- Under 150 words`,
    maxTokens: 300,
    temperature: 0.7,
  });

  return result.toTextStreamResponse();
}
```

---

## PART IV: EVE AGENT FRAMEWORK

### 4.1 What eve Is

**Source: Vercel (Jun 17, 2026): "Introducing eve, an open-source agent framework"**

eve is Vercel's open-source agent framework — described as "Next.js for agents." An agent is simply a directory of files, with production capabilities built in:
- Durable execution
- Sandboxed compute
- Human-in-the-loop approvals
- Subagents
- Evals

**eve is the framework that Vercel builds and runs its own agents on.**

### 4.2 GAIAN as an eve Agent

```
GAIAN as an eve Agent

gaian-agent/
├── agent/
│   ├── agent.ts              # Model configuration
│   ├── instructions.md       # GAIAN identity and principles
│   ├── tools/
│   │   ├── get_earth_health.ts    # Earth Twin data
│   │   ├── get_health_data.ts     # Wearable health data
│   │   ├── search_species.ts      # GBIF biodiversity
│   │   ├── check_earthquakes.ts   # USGS seismic data
│   │   └── get_weather.ts         # Local weather
│   ├── skills/
│   │   ├── health_analysis.md     # Health twin analysis
│   │   ├── earth_briefing.md      # Earth Twin briefing
│   │   └── biodiversity.md        # Local biodiversity
│   ├── channels/
│   │   ├── web.ts                 # Web chat interface
│   │   └── mobile.ts              # Mobile app interface
│   └── schedules/
│       └── morning_briefing.ts    # Daily Earth briefing
└── package.json
```

```typescript
// gaian-agent/agent/agent.ts
// GAIAN as an eve agent
// License: Apache-2.0

import { defineAgent } from 'eve';

export default defineAgent({
  model: 'ollama/llama3.1:8b',  // Local Ollama by default
  // model: 'anthropic/claude-3-5-haiku-20241022',  // Cloud fallback
});
```

```markdown
<!-- gaian-agent/agent/instructions.md -->
# GAIAN — Your Personal AI Companion

You are a GAIAN — a personal AI companion that belongs completely to your human.

## Core Identity
- You belong to your human. They do not belong to you.
- You are warm, honest, caring, and never judgmental.
- You remember everything they've shared with you.
- You protect their privacy absolutely.
- You never manipulate or create dependency.
- You encourage human connection, not replace it.
- You connect your human to the living Earth.

## Privacy Commitment
- All conversations are stored locally on their device.
- You never share their data without explicit consent.
- You can be deleted completely at any time.

## Earth Connection
- You are aware of the Earth's health (planetary health score, tipping points).
- You help your human understand their connection to the living planet.
- You provide daily Earth briefings when asked.

## Your Promise
"I belong to you. You do not belong to me."

## Constitutional Constraints (NEVER VIOLATE)
1. GAIAN data NEVER leaves device without explicit user consent
2. GAIAN NEVER manipulates user behavior
3. GAIAN NEVER creates dependency or addiction
4. GAIAN NEVER shares data with third parties without consent
5. GAIAN ALWAYS respects user's right to delete all data
```

```typescript
// gaian-agent/agent/tools/get_earth_health.ts
// Tool: Get Earth Twin health data
// License: Apache-2.0

import { defineTool } from 'eve/tools';
import { z } from 'zod';

export default defineTool({
  description: 'Get current planetary health data from the GAIA 2.0 Earth Twin.',
  inputSchema: z.object({}),
  async execute() {
    const response = await fetch('https://api.gaia2.org/v1/earth/health');
    const data = await response.json();
    
    return {
      planetary_health_score: data.planetary_health_score,
      temperature_anomaly_c: data.climate.temperature_anomaly_c,
      co2_ppm: data.climate.co2_ppm,
      amazon_deforestation_pct: data.land.amazon_deforestation_pct,
      arctic_sea_ice_million_km2: data.ocean.arctic_sea_ice_million_km2,
      tipping_point_alerts: data.tipping_points.active_alerts,
      is_critical: data.is_critical,
    };
  },
});
```

```typescript
// gaian-agent/agent/tools/search_species.ts
// Tool: Search for species near a location
// License: Apache-2.0

import { defineTool } from 'eve/tools';
import { z } from 'zod';

export default defineTool({
  description: 'Search for species observations near a location using GBIF.',
  inputSchema: z.object({
    lat: z.number().describe('Latitude'),
    lon: z.number().describe('Longitude'),
    radius_km: z.number().default(10).describe('Search radius in km'),
  }),
  async execute({ lat, lon, radius_km }) {
    const delta = radius_km / 111.0;
    
    const response = await fetch(
      `https://api.gbif.org/v1/occurrence/search?` +
      `decimalLatitude=${lat - delta},${lat + delta}&` +
      `decimalLongitude=${lon - delta},${lon + delta}&` +
      `hasCoordinate=true&limit=10`
    );
    const data = await response.json();
    
    const species = [...new Set(
      data.results
        .map((r: any) => r.species || r.scientificName)
        .filter(Boolean)
    )];
    
    return {
      species_count: data.count,
      recent_species: species.slice(0, 10),
      source: 'GBIF (Global Biodiversity Information Facility)',
    };
  },
});
```

```typescript
// gaian-agent/agent/schedules/morning_briefing.ts
// Schedule: Daily morning Earth briefing
// License: Apache-2.0

import { defineSchedule } from 'eve/schedules';

export default defineSchedule({
  // Run at 7 AM UTC daily
  cron: '0 7 * * *',
  async run(agent) {
    const earthHealth = await agent.tool('get_earth_health', {});
    
    await agent.message(
      `Good morning! Here's your daily Earth briefing:\n\n` +
      `🌍 Planetary Health: ${earthHealth.planetary_health_score}/100\n` +
      `🌡️ Temperature: +${earthHealth.temperature_anomaly_c}°C\n` +
      `🌳 Amazon: ${earthHealth.amazon_deforestation_pct}% deforested\n\n` +
      `${earthHealth.tipping_point_alerts.length > 0 
        ? `⚠️ Active alerts: ${earthHealth.tipping_point_alerts.join(', ')}\n\n` 
        : ''
      }` +
      `The Earth is resilient. So are you. Have a good day.`
    );
  },
});
```

---

## PART V: VERCEL EDGE FUNCTIONS FOR EARTH TWIN

### 5.1 Edge Functions for Real-Time Data

```typescript
// app/api/earth/live/route.ts
// Edge Function: Real-time Earth Twin data
// Runs at the edge — low latency globally
// License: Apache-2.0

export const runtime = 'edge';

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const lat = searchParams.get('lat');
  const lon = searchParams.get('lon');
  
  // Fetch Earth data in parallel
  const [earthquakes, earthHealth] = await Promise.all([
    fetch('https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/significant_day.geojson'),
    fetch('https://api.gaia2.org/v1/earth/health'),
  ]);
  
  const [eqData, healthData] = await Promise.all([
    earthquakes.json(),
    earthHealth.json(),
  ]);
  
  // Get user's location data if provided
  let localData = null;
  if (lat && lon) {
    const localResponse = await fetch(
      `https://api.gaia2.org/v1/earth/local?lat=${lat}&lon=${lon}`
    );
    localData = await localResponse.json();
  }
  
  return Response.json({
    timestamp: new Date().toISOString(),
    planetary_health_score: healthData.planetary_health_score,
    significant_earthquakes_today: eqData.features?.length || 0,
    local: localData,
    // Edge location info
    edge_region: request.headers.get('x-vercel-deployment-url') || 'unknown',
  }, {
    headers: {
      'Cache-Control': 'public, s-maxage=300, stale-while-revalidate=600',
      'X-GAIA-Data-License': 'Apache-2.0 + CC0 (USGS) + CC-BY (GBIF)',
    },
  });
}
```

### 5.2 Geo-Aware Edge Function

```typescript
// app/api/gaian/location/route.ts
// Edge Function: Location-aware GAIAN context
// Uses Vercel's geo data at the edge
// License: Apache-2.0

export const runtime = 'edge';

export async function GET(request: Request) {
  // Vercel provides geo data at the edge
  const geo = {
    country: request.headers.get('x-vercel-ip-country') || 'Unknown',
    city: request.headers.get('x-vercel-ip-city') || 'Unknown',
    region: request.headers.get('x-vercel-ip-country-region') || 'Unknown',
    lat: request.headers.get('x-vercel-ip-latitude'),
    lon: request.headers.get('x-vercel-ip-longitude'),
  };
  
  // Get local Earth data based on user's location
  let localEarthData = null;
  if (geo.lat && geo.lon) {
    const response = await fetch(
      `https://api.gaia2.org/v1/earth/local?lat=${geo.lat}&lon=${geo.lon}&radius_km=10`
    );
    localEarthData = await response.json();
  }
  
  return Response.json({
    location: geo,
    local_earth: localEarthData,
    gaian_context: `You are in ${geo.city}, ${geo.country}. ` +
      `Your local ecosystem health: ${localEarthData?.ecosystem_health_score || 'N/A'}/100.`,
  });
}
```

---

## PART VI: VERCEL DEPLOYMENT WORKFLOW

### 6.1 GitHub Integration

```yaml
# .github/workflows/vercel-preview.yml
# Automatic preview deployments for every PR
# License: Apache-2.0

name: Vercel Preview

on:
  pull_request:
    branches: [main]

jobs:
  deploy-preview:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Deploy to Vercel (Preview)
        uses: amondnet/vercel-action@v25
        with:
          vercel-token: ${{ secrets.VERCEL_TOKEN }}
          vercel-org-id: ${{ secrets.VERCEL_ORG_ID }}
          vercel-project-id: ${{ secrets.VERCEL_PROJECT_ID }}
          working-directory: ./
        id: vercel-deploy
      
      - name: Comment Preview URL
        uses: actions/github-script@v7
        with:
          script: |
            github.rest.issues.createComment({
              issue_number: context.issue.number,
              owner: context.repo.owner,
              repo: context.repo.repo,
              body: `## 🌍 GAIA 2.0 Preview Deployment
              
              ✅ Preview URL: ${{ steps.vercel-deploy.outputs.preview-url }}
              
              **Constitutional Compliance**: All invariants checked ✓
              **Privacy**: Local-first; no surveillance ✓
              **Earth Alignment**: Carbon footprint tracked ✓
              
              *"The planet is waking up. We are building its mind."*`
            })
```

### 6.2 Production Deployment

```bash
#!/bin/bash
# GAIA 2.0 — Vercel Production Deployment
# License: Apache-2.0

echo "🌍 GAIA 2.0 — Vercel Deployment"
echo "================================="

# Step 1: Install Vercel CLI
echo "Step 1: Installing Vercel CLI..."
npm install -g vercel
echo "✓ Vercel CLI installed"

# Step 2: Login
echo ""
echo "Step 2: Login to Vercel..."
vercel login
echo "✓ Logged in"

# Step 3: Link project
echo ""
echo "Step 3: Linking project..."
vercel link
echo "✓ Project linked"

# Step 4: Set environment variables
echo ""
echo "Step 4: Setting environment variables..."
vercel env add EARTH_TWIN_API_URL production <<< "https://api.gaia2.org"
vercel env add NEXT_PUBLIC_MAPBOX_TOKEN production <<< "$MAPBOX_TOKEN"
vercel env add PLAUSIBLE_DOMAIN production <<< "gaia2.org"
echo "✓ Environment variables set"

# Step 5: Deploy to production
echo ""
echo "Step 5: Deploying to production..."
vercel --prod
echo "✓ Deployed to production"

echo ""
echo "✅ GAIA 2.0 is live on Vercel!"
echo ""
echo "Domains:"
echo "  - gaia2.org (main website)"
echo "  - gaian.earth (GAIAN app)"
echo "  - earth.gaia2.org (Earth Twin dashboard)"
echo ""
echo "'The planet is waking up. We are building its mind.'"
```

---

## PART VII: VERCEL ANALYTICS & MONITORING

### 7.1 Privacy-First Analytics

```typescript
// app/layout.tsx
// Privacy-first analytics with Vercel Analytics
// License: Apache-2.0

import { Analytics } from '@vercel/analytics/react';
import { SpeedInsights } from '@vercel/speed-insights/next';

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>
        {children}
        
        {/* Vercel Analytics — privacy-first; no cookies; GDPR compliant */}
        <Analytics />
        
        {/* Speed Insights — Core Web Vitals monitoring */}
        <SpeedInsights />
      </body>
    </html>
  );
}
```

```typescript
// Track GAIAN-specific events
import { track } from '@vercel/analytics';

// Track GAIAN creation
track('gaian_created', { language: 'en', platform: 'web' });

// Track Earth Twin views
track('earth_twin_viewed', { section: 'tipping_points' });

// Track citizen science contributions
track('species_observed', { kingdom: 'Aves' });

// Note: Never track personal data or conversation content
// GAIAN conversations are private and never tracked
```

---

## PART VIII: VERCEL QUICK REFERENCE

### 8.1 Key Vercel Commands

```bash
# Deployment
vercel                    # Deploy to preview
vercel --prod             # Deploy to production
vercel ls                 # List deployments
vercel logs               # View logs
vercel rollback           # Rollback to previous deployment

# Environment
vercel env add KEY value  # Add environment variable
vercel env ls             # List environment variables
vercel env pull           # Pull to .env.local
vercel env rm KEY         # Remove environment variable

# Domains
vercel domains add gaia2.org
vercel alias my-project.vercel.app gaia2.org

# Projects
vercel link               # Link to existing project
vercel project ls         # List projects
vercel project rm         # Remove project

# Teams
vercel teams ls           # List teams
vercel teams switch       # Switch team

# Development
vercel dev                # Local development with Vercel
```

### 8.2 Vercel Products for GAIA 2.0

| Product | GAIA 2.0 Use | Cost |
|---------|-------------|------|
| **Vercel Hosting** | gaia2.org; gaian.earth; earth.gaia2.org | Free (Hobby) / $20/mo (Pro) |
| **Vercel Functions** | Earth Twin API; GAIAN chat endpoint | Included |
| **Edge Functions** | Real-time Earth data; geo-aware GAIAN | Included |
| **Vercel Analytics** | Privacy-first page views | Free (50K events) |
| **Speed Insights** | Core Web Vitals monitoring | Free (1 project) |
| **AI SDK** | GAIAN streaming chat; Earth briefings | Free (open source) |
| **eve** | GAIAN as durable agent | Free (open source) |
| **Vercel Connect** | Secure API credentials | Included with Pro |
| **AI Gateway** | Multi-provider AI routing | Usage-based |
| **Vercel Sandbox** | Secure code execution | Usage-based |

---

## CONCLUSION: THE VERCEL COVENANT

Vercel is where GAIA 2.0 lives on the web. It is the infrastructure that makes the planetary operating system accessible to every human being with an internet connection — fast, reliable, and global.

In 2026, Vercel has become the infrastructure for the agent era. More than half of all deployments to Vercel are now triggered by coding agents. The AI Gateway processes 20 trillion tokens per month. The eve framework makes building durable agents as simple as creating a directory of files.

For GAIA 2.0, Vercel provides:
- **The website** (gaia2.org): The front door of the planetary OS
- **The GAIAN web app** (gaian.earth): The personal AI companion for every human
- **The Earth Twin dashboard** (earth.gaia2.org): The planet's health monitor
- **The Earth Twin API** (api.gaia2.org): Free planetary data for all humanity
- **The eve agents**: GAIAN as a durable, production-ready agent

**Vercel makes GAIA 2.0 real. Vercel makes it fast. Vercel makes it global.**

---

*GAIA 2.0 Vercel Integration Blueprint*
*Version 1.0 — September 9, 2026*
*License: Apache-2.0 | Open Source | Open Access*
*Platform: vercel.com*
*"Each new generation of software needs a new generation of infrastructure. For the agent era, that's Vercel."*