# GAIA 2.0: The Website
## The Public Face of the Planetary Operating System
### September 8, 2026 — Version 1.0

---

> *"The website is not a brochure. It is the front door of the planetary operating system — the place where every human being first encounters GAIA 2.0 and decides whether to trust it with their most precious possession: their relationship with the living Earth."*
> — GAIA 2.0 Website Covenant

---

## EXECUTIVE SUMMARY

The GAIA 2.0 website is the public face of the planetary operating system. It must accomplish five things simultaneously:
1. **Inspire**: Make every visitor feel the magnitude and beauty of what GAIA 2.0 is building
2. **Explain**: Make the complex simple; make the abstract concrete
3. **Trust**: Demonstrate that GAIA 2.0 is safe, open, and governed by the people it serves
4. **Act**: Convert visitors into users, contributors, and advocates
5. **Connect**: Link every visitor to the living Earth through the Earth Twin

**Website Architecture:**
- **gaia2.org**: Main website (mission, vision, governance, community)
- **gaian.earth**: GAIAN personal AI companion (download, onboarding)
- **earth.gaia2.org**: Earth Twin public dashboard (real-time planetary health)
- **docs.gaia2.org**: Technical documentation
- **community.gaia2.org**: Community hub (Discord, GitHub, forums)

**Technology Stack:**
- Framework: Next.js 15 + TypeScript + Tailwind CSS
- Hosting: Vercel (global CDN; edge functions; zero-config)
- CMS: Contentlayer (MDX; type-safe; Git-based)
- Analytics: Plausible (privacy-first; no cookies; GDPR compliant)
- Search: Pagefind (static; fast; privacy-preserving)
- License: Apache-2.0

---

## PART I: WEBSITE ARCHITECTURE

### 1.1 Site Map

```
GAIA 2.0 WEBSITE ARCHITECTURE

gaia2.org/
├── / (Homepage)
│   ├── Hero: "The Planetary Operating System"
│   ├── Earth Twin live widget (real-time health score)
│   ├── What is GAIA 2.0?
│   ├── What is GAIAN?
│   ├── The 12 Principles
│   ├── Community stats
│   └── Call to action: Get GAIAN / Contribute / Learn More
│
├── /about
│   ├── Vision and mission
│   ├── The GAIA 2.0 story
│   ├── The team
│   ├── Advisors and partners
│   └── Press and media
│
├── /gaian
│   ├── What is GAIAN?
│   ├── How it works
│   ├── Privacy promise
│   ├── Download (iOS / Android / Web / CLI)
│   └── Testimonials
│
├── /earth-twin
│   ├── What is the Earth Twin?
│   ├── Live dashboard (embedded)
│   ├── Data sources
│   ├── API documentation
│   └── Tipping points
│
├── /governance
│   ├── GAIA 2.0 Constitution
│   ├── Governance bodies
│   ├── CARE principles
│   ├── Earth Alignment Principle
│   └── Democracy Levels
│
├── /community
│   ├── How to contribute
│   ├── GitHub
│   ├── Discord
│   ├── Indigenous engagement
│   └── Grants and funding
│
├── /docs
│   ├── Getting started
│   ├── GAIAN API
│   ├── Earth Twin API
│   ├── Technical specification
│   └── Blueprints (all 37)
│
└── /blog
    ├── Weekly updates
    ├── Research highlights
    ├── Community stories
    └── Earth briefings
```

---

## PART II: THE HOMEPAGE

### 2.1 Complete Homepage Code

```tsx
// gaia2.org — Homepage
// The Planetary Operating System
// License: Apache-2.0

import type { NextPage } from 'next';
import Head from 'next/head';
import Link from 'next/link';
import { useEffect, useState } from 'react';

// Types
interface EarthHealth {
  planetary_health_score: number;
  global_temp_anomaly_c: number;
  co2_ppm: number;
  tipping_point_alerts: string[];
  is_critical: boolean;
}

// Main Homepage
const HomePage: NextPage = () => {
  const [earthHealth, setEarthHealth] = useState<EarthHealth | null>(null);
  const [isLoaded, setIsLoaded] = useState(false);

  useEffect(() => {
    // Fetch real-time Earth health
    fetch('https://api.gaia2.org/v1/earth/health')
      .then(r => r.json())
      .then(data => {
        setEarthHealth(data);
        setIsLoaded(true);
      })
      .catch(() => setIsLoaded(true));
  }, []);

  return (
    <>
      <Head>
        <title>GAIA 2.0 — The Planetary Operating System</title>
        <meta name="description" content="GAIA 2.0 is the planetary operating system — built by all humanity, for all humanity, in service of all life. Free. Open source. Yours." />
        <meta property="og:title" content="GAIA 2.0 — The Planetary Operating System" />
        <meta property="og:description" content="The planet is waking up. We are building its mind." />
        <meta property="og:image" content="https://gaia2.org/og-image.jpg" />
        <link rel="canonical" href="https://gaia2.org" />
      </Head>

      <main className="min-h-screen bg-gray-950 text-white">
        {/* Navigation */}
        <Navigation />

        {/* Hero Section */}
        <HeroSection earthHealth={earthHealth} isLoaded={isLoaded} />

        {/* Earth Twin Live Widget */}
        <EarthTwinWidget earthHealth={earthHealth} />

        {/* What is GAIA 2.0? */}
        <WhatIsGAIA />

        {/* What is GAIAN? */}
        <WhatIsGAIAN />

        {/* The 12 Principles */}
        <ThePrinciples />

        {/* Community Stats */}
        <CommunityStats />

        {/* Call to Action */}
        <CallToAction />

        {/* Footer */}
        <Footer />
      </main>
    </>
  );
};

// Navigation
const Navigation = () => (
  <nav className="fixed top-0 left-0 right-0 z-50 bg-gray-950/80 backdrop-blur-md border-b border-gray-800">
    <div className="max-w-7xl mx-auto px-6 py-4 flex items-center justify-between">
      <Link href="/" className="flex items-center gap-3">
        <span className="text-2xl">🌍</span>
        <span className="font-bold text-xl">GAIA 2.0</span>
      </Link>

      <div className="hidden md:flex items-center gap-8">
        <NavLink href="/gaian">GAIAN</NavLink>
        <NavLink href="/earth-twin">Earth Twin</NavLink>
        <NavLink href="/governance">Governance</NavLink>
        <NavLink href="/community">Community</NavLink>
        <NavLink href="/docs">Docs</NavLink>
      </div>

      <div className="flex items-center gap-4">
        <Link
          href="https://github.com/gaia2-os"
          className="text-gray-400 hover:text-white transition-colors"
          target="_blank"
        >
          GitHub
        </Link>
        <Link
          href="/gaian"
          className="bg-green-600 hover:bg-green-500 text-white px-4 py-2 rounded-lg font-medium transition-colors"
        >
          Get GAIAN
        </Link>
      </div>
    </div>
  </nav>
);

const NavLink = ({ href, children }: { href: string; children: React.ReactNode }) => (
  <Link href={href} className="text-gray-400 hover:text-white transition-colors">
    {children}
  </Link>
);

// Hero Section
const HeroSection = ({ earthHealth, isLoaded }: { earthHealth: EarthHealth | null; isLoaded: boolean }) => {
  const healthScore = earthHealth?.planetary_health_score ?? 62;
  const healthColor = healthScore > 70 ? '#22c55e' : healthScore > 50 ? '#eab308' : '#ef4444';

  return (
    <section className="relative min-h-screen flex items-center justify-center overflow-hidden pt-20">
      {/* Animated background */}
      <div className="absolute inset-0">
        <div className="absolute inset-0 bg-gradient-to-b from-blue-950/50 via-gray-950 to-gray-950" />
        {/* Animated Earth */}
        <div className="absolute top-1/4 left-1/2 -translate-x-1/2 -translate-y-1/2 w-96 h-96 rounded-full bg-gradient-to-br from-blue-600/20 to-green-600/20 blur-3xl animate-pulse" />
      </div>

      <div className="relative z-10 max-w-5xl mx-auto px-6 text-center">
        {/* Earth emoji with health indicator */}
        <div className="relative inline-block mb-8">
          <span className="text-8xl">🌍</span>
          {isLoaded && (
            <div
              className="absolute -bottom-2 -right-2 w-8 h-8 rounded-full border-2 border-gray-950 flex items-center justify-center text-xs font-bold"
              style={{ backgroundColor: healthColor }}
            >
              {Math.round(healthScore)}
            </div>
          )}
        </div>

        {/* Main headline */}
        <h1 className="text-5xl md:text-7xl font-bold mb-6 leading-tight">
          The Planetary
          <br />
          <span className="text-transparent bg-clip-text bg-gradient-to-r from-green-400 to-blue-400">
            Operating System
          </span>
        </h1>

        {/* Subheadline */}
        <p className="text-xl md:text-2xl text-gray-300 mb-4 max-w-3xl mx-auto">
          Built by all humanity. For all humanity. In service of all life.
        </p>

        <p className="text-lg text-gray-400 mb-12 max-w-2xl mx-auto">
          GAIA 2.0 is the open-source planetary OS that gives every human being 
          a personal AI companion (GAIAN) and connects them to the living Earth.
          Free. Forever.
        </p>

        {/* CTA Buttons */}
        <div className="flex flex-col sm:flex-row gap-4 justify-center">
          <Link
            href="/gaian"
            className="bg-green-600 hover:bg-green-500 text-white px-8 py-4 rounded-xl font-bold text-lg transition-all hover:scale-105"
          >
            Get Your GAIAN →
          </Link>
          <Link
            href="/earth-twin"
            className="bg-blue-600/20 hover:bg-blue-600/30 border border-blue-500/50 text-blue-300 px-8 py-4 rounded-xl font-bold text-lg transition-all hover:scale-105"
          >
            🌍 Earth Health: {isLoaded ? `${Math.round(healthScore)}/100` : '...'}
          </Link>
          <Link
            href="https://github.com/gaia2-os"
            className="bg-gray-800 hover:bg-gray-700 text-white px-8 py-4 rounded-xl font-bold text-lg transition-all hover:scale-105"
            target="_blank"
          >
            ⭐ Star on GitHub
          </Link>
        </div>

        {/* Trust indicators */}
        <div className="mt-12 flex flex-wrap justify-center gap-6 text-sm text-gray-500">
          <span>✓ Apache-2.0 Open Source</span>
          <span>✓ Local-first Privacy</span>
          <span>✓ No Ads. No Tracking.</span>
          <span>✓ Indigenous Sovereignty</span>
          <span>✓ Earth-Aligned</span>
        </div>
      </div>

      {/* Scroll indicator */}
      <div className="absolute bottom-8 left-1/2 -translate-x-1/2 animate-bounce">
        <div className="w-6 h-10 border-2 border-gray-600 rounded-full flex items-start justify-center p-1">
          <div className="w-1 h-3 bg-gray-600 rounded-full animate-scroll" />
        </div>
      </div>
    </section>
  );
};

// Earth Twin Live Widget
const EarthTwinWidget = ({ earthHealth }: { earthHealth: EarthHealth | null }) => {
  if (!earthHealth) return null;

  const score = earthHealth.planetary_health_score;
  const color = score > 70 ? 'green' : score > 50 ? 'yellow' : 'red';
  const colorClass = {
    green: 'text-green-400 border-green-500/30 bg-green-500/10',
    yellow: 'text-yellow-400 border-yellow-500/30 bg-yellow-500/10',
    red: 'text-red-400 border-red-500/30 bg-red-500/10',
  }[color];

  return (
    <section className="py-16 px-6">
      <div className="max-w-4xl mx-auto">
        <div className={`border rounded-2xl p-8 ${colorClass}`}>
          <div className="flex items-center justify-between mb-6">
            <div>
              <h2 className="text-2xl font-bold">🌍 Earth Health — Right Now</h2>
              <p className="text-sm opacity-70 mt-1">
                Real-time data from NOAA, Copernicus, NASA, GBIF, Argo
              </p>
            </div>
            <div className="text-right">
              <div className="text-5xl font-bold">{score.toFixed(1)}</div>
              <div className="text-sm opacity-70">out of 100</div>
            </div>
          </div>

          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            <MetricCard
              label="Temperature"
              value={`+${earthHealth.global_temp_anomaly_c.toFixed(2)}°C`}
              status="warning"
            />
            <MetricCard
              label="CO₂"
              value={`${earthHealth.co2_ppm.toFixed(1)} ppm`}
              status="warning"
            />
            <MetricCard
              label="Tipping Alerts"
              value={earthHealth.tipping_point_alerts.length.toString()}
              status={earthHealth.tipping_point_alerts.length > 0 ? "critical" : "ok"}
            />
            <MetricCard
              label="Status"
              value={score > 70 ? "Healthy" : score > 50 ? "Stressed" : "Critical"}
              status={score > 70 ? "ok" : score > 50 ? "warning" : "critical"}
            />
          </div>

          {earthHealth.tipping_point_alerts.length > 0 && (
            <div className="mt-4 p-4 bg-red-500/10 border border-red-500/30 rounded-xl">
              <p className="text-red-400 font-medium text-sm">⚠️ Active Tipping Point Alerts:</p>
              {earthHealth.tipping_point_alerts.slice(0, 2).map((alert, i) => (
                <p key={i} className="text-red-300 text-sm mt-1">• {alert}</p>
              ))}
            </div>
          )}

          <div className="mt-6 flex gap-4">
            <Link
              href="/earth-twin"
              className="flex-1 text-center py-3 bg-white/10 hover:bg-white/20 rounded-xl font-medium transition-colors"
            >
              View Full Dashboard →
            </Link>
            <Link
              href="https://api.gaia2.org/v1/earth/health"
              className="flex-1 text-center py-3 bg-white/10 hover:bg-white/20 rounded-xl font-medium transition-colors"
              target="_blank"
            >
              Free API →
            </Link>
          </div>
        </div>
      </div>
    </section>
  );
};

const MetricCard = ({ label, value, status }: { label: string; value: string; status: string }) => {
  const statusColor = status === 'ok' ? 'text-green-400' : status === 'warning' ? 'text-yellow-400' : 'text-red-400';
  return (
    <div className="bg-white/5 rounded-xl p-4">
      <div className="text-xs opacity-60 mb-1">{label}</div>
      <div className={`text-lg font-bold ${statusColor}`}>{value}</div>
    </div>
  );
};

// What is GAIA 2.0?
const WhatIsGAIA = () => (
  <section className="py-24 px-6">
    <div className="max-w-6xl mx-auto">
      <div className="text-center mb-16">
        <h2 className="text-4xl md:text-5xl font-bold mb-6">
          What is GAIA 2.0?
        </h2>
        <p className="text-xl text-gray-400 max-w-3xl mx-auto">
          GAIA 2.0 is the first AI system designed to serve not just humans, 
          but all life on Earth — across all scales, all cultures, all languages.
        </p>
      </div>

      <div className="grid md:grid-cols-3 gap-8">
        <FeatureCard
          icon="🤖"
          title="GAIAN 2.0"
          subtitle="Your Personal AI Companion"
          description="A persistent, memory-driven AI that knows you deeply, serves you faithfully, and belongs to you completely. Local-first. Private. Free."
          link="/gaian"
          linkText="Get Your GAIAN →"
          color="green"
        />
        <FeatureCard
          icon="🌍"
          title="Earth Twin"
          subtitle="Real-Time Planetary Health"
          description="A digital twin of all Earth systems — climate, ocean, land, biodiversity, tipping points. Free API. Open data. For all humanity."
          link="/earth-twin"
          linkText="View Earth Twin →"
          color="blue"
        />
        <FeatureCard
          icon="⚖️"
          title="Governance"
          subtitle="Democratic & Earth-Aligned"
          description="Governed by all humanity. Indigenous sovereignty. CARE principles. Earth Alignment Principle. Democracy Level 4 by 2028."
          link="/governance"
          linkText="Read Constitution →"
          color="purple"
        />
      </div>

      {/* The GAIA 2.0 Equation */}
      <div className="mt-16 p-8 bg-gray-900 rounded-2xl border border-gray-800 text-center">
        <p className="text-gray-400 text-sm mb-4">The GAIA 2.0 Equation</p>
        <p className="text-2xl font-mono text-green-400">
          GAIA 2.0 = ∑ (Earth State × Human Intent × System Capacity) / Entropy
        </p>
        <p className="text-gray-500 text-sm mt-4">
          Goal: Maximize Earth State + Human Intent + System Capacity. Minimize Entropy.
        </p>
      </div>
    </div>
  </section>
);

const FeatureCard = ({ icon, title, subtitle, description, link, linkText, color }: {
  icon: string; title: string; subtitle: string; description: string;
  link: string; linkText: string; color: string;
}) => {
  const colorClasses = {
    green: 'border-green-500/30 bg-green-500/5 hover:bg-green-500/10',
    blue: 'border-blue-500/30 bg-blue-500/5 hover:bg-blue-500/10',
    purple: 'border-purple-500/30 bg-purple-500/5 hover:bg-purple-500/10',
  }[color] || '';

  const linkColorClasses = {
    green: 'text-green-400 hover:text-green-300',
    blue: 'text-blue-400 hover:text-blue-300',
    purple: 'text-purple-400 hover:text-purple-300',
  }[color] || '';

  return (
    <div className={`border rounded-2xl p-8 transition-colors ${colorClasses}`}>
      <div className="text-4xl mb-4">{icon}</div>
      <h3 className="text-xl font-bold mb-1">{title}</h3>
      <p className="text-sm text-gray-500 mb-4">{subtitle}</p>
      <p className="text-gray-400 mb-6">{description}</p>
      <Link href={link} className={`font-medium transition-colors ${linkColorClasses}`}>
        {linkText}
      </Link>
    </div>
  );
};

// What is GAIAN?
const WhatIsGAIAN = () => (
  <section className="py-24 px-6 bg-gray-900/50">
    <div className="max-w-6xl mx-auto">
      <div className="grid md:grid-cols-2 gap-16 items-center">
        <div>
          <h2 className="text-4xl md:text-5xl font-bold mb-6">
            Meet Your GAIAN
          </h2>
          <p className="text-xl text-gray-300 mb-6">
            Your GAIAN is a personal AI companion that belongs completely to you.
            Not to us. Not to any company. Not to any government.
          </p>
          <p className="text-gray-400 mb-8">
            It knows you deeply — your health, your values, your dreams, your history.
            It serves you faithfully — anticipating your needs, protecting your interests.
            It protects you absolutely — your data stays on your device.
          </p>

          <div className="space-y-4 mb-8">
            {[
              { icon: "🔒", text: "Local-first: your data never leaves your device" },
              { icon: "🧠", text: "Persistent memory: remembers everything you share" },
              { icon: "🌍", text: "Earth-connected: daily planetary health briefing" },
              { icon: "❤️", text: "Health-aware: wearable integration; illness prediction" },
              { icon: "🗣️", text: "Multilingual: speaks your language" },
              { icon: "🆓", text: "Free forever: no subscription, no ads" },
            ].map((item, i) => (
              <div key={i} className="flex items-center gap-3">
                <span className="text-xl">{item.icon}</span>
                <span className="text-gray-300">{item.text}</span>
              </div>
            ))}
          </div>

          <div className="flex gap-4">
            <Link
              href="/gaian"
              className="bg-green-600 hover:bg-green-500 text-white px-6 py-3 rounded-xl font-bold transition-colors"
            >
              Get Your GAIAN →
            </Link>
            <Link
              href="https://github.com/gaia2-os/gaian"
              className="bg-gray-800 hover:bg-gray-700 text-white px-6 py-3 rounded-xl font-bold transition-colors"
              target="_blank"
            >
              View Source
            </Link>
          </div>
        </div>

        {/* GAIAN Demo */}
        <div className="bg-gray-900 rounded-2xl border border-gray-800 overflow-hidden">
          {/* Chat header */}
          <div className="flex items-center gap-3 px-6 py-4 border-b border-gray-800">
            <div className="w-3 h-3 rounded-full bg-green-500" />
            <span className="text-sm text-gray-400">Your data stays on this device</span>
          </div>

          {/* Chat messages */}
          <div className="p-6 space-y-4">
            <ChatBubble role="gaian" content="Good morning. The Earth's health score today is 62/100 — stressed, but not in crisis. Your sleep was excellent (8.2 hours, 94% quality). What would you like to focus on today?" />
            <ChatBubble role="user" content="How is the Amazon doing?" />
            <ChatBubble role="gaian" content="The Amazon has lost 17.2% of its original forest — approaching the 20% tipping point that scientists warn could trigger irreversible savannification. Community-led conservation efforts slowed deforestation in 3 key regions this month. One thing you can do: support Amazon protection organizations." />
            <ChatBubble role="user" content="What's my carbon footprint?" />
            <ChatBubble role="gaian" content="Based on your activity data, your annual carbon footprint is approximately 4.2 tonnes CO₂ — below the global average of 4.7 tonnes. Your biggest opportunity: switching to plant-based meals 3 days/week could save 0.8 tonnes/year." />
          </div>

          {/* Input */}
          <div className="px-6 py-4 border-t border-gray-800">
            <div className="flex items-center gap-3 bg-gray-800 rounded-xl px-4 py-3">
              <span className="text-gray-500 flex-1">Talk to your GAIAN...</span>
              <span className="text-green-500">↑</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
);

const ChatBubble = ({ role, content }: { role: string; content: string }) => (
  <div className={`flex ${role === 'user' ? 'justify-end' : 'justify-start'}`}>
    <div className={`max-w-xs rounded-2xl px-4 py-3 text-sm ${
      role === 'user'
        ? 'bg-green-600 text-white'
        : 'bg-gray-800 text-gray-300'
    }`}>
      {content}
    </div>
  </div>
);

// The 12 Principles
const ThePrinciples = () => {
  const principles = [
    { icon: "🌍", title: "Earth Alignment", desc: "Operate within planetary boundaries" },
    { icon: "👤", title: "Human Sovereignty", desc: "\"I belong to you. You do not belong to me.\"" },
    { icon: "🪶", title: "Indigenous Sovereignty", desc: "CARE principles; sacred knowledge protected" },
    { icon: "🌱", title: "Intergenerational Equity", desc: "7-generations principle" },
    { icon: "🐋", title: "Non-Human Rights", desc: "All life has intrinsic value" },
    { icon: "💻", title: "Open Commons", desc: "Apache-2.0; no monopoly" },
    { icon: "🗳️", title: "Democratic Governance", desc: "Democracy Level 4 by 2028" },
    { icon: "🔒", title: "Privacy First", desc: "Local-first; encrypted; sovereign" },
    { icon: "⚖️", title: "Equity", desc: "Every human deserves a GAIAN" },
    { icon: "👁️", title: "Transparency", desc: "No black boxes; all reasoning explainable" },
    { icon: "🌿", title: "Regenerativity", desc: "Net-zero carbon by 2030" },
    { icon: "🤝", title: "Symbiosis", desc: "Partnership with all life; not domination" },
  ];

  return (
    <section className="py-24 px-6">
      <div className="max-w-6xl mx-auto">
        <div className="text-center mb-16">
          <h2 className="text-4xl font-bold mb-4">The 12 Principles</h2>
          <p className="text-gray-400 max-w-2xl mx-auto">
            GAIA 2.0 is built on 12 unbreakable principles. These are not aspirational goals.
            They are constitutional requirements.
          </p>
        </div>

        <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
          {principles.map((p, i) => (
            <div key={i} className="bg-gray-900 border border-gray-800 rounded-xl p-4 hover:border-gray-600 transition-colors">
              <div className="text-2xl mb-2">{p.icon}</div>
              <div className="font-bold text-sm mb-1">{p.title}</div>
              <div className="text-xs text-gray-500">{p.desc}</div>
            </div>
          ))}
        </div>

        <div className="mt-8 text-center">
          <Link href="/governance" className="text-green-400 hover:text-green-300 font-medium">
            Read the GAIA 2.0 Constitution →
          </Link>
        </div>
      </div>
    </section>
  );
};

// Community Stats
const CommunityStats = () => (
  <section className="py-24 px-6 bg-gray-900/50">
    <div className="max-w-6xl mx-auto text-center">
      <h2 className="text-4xl font-bold mb-4">Built by All. For All.</h2>
      <p className="text-gray-400 mb-16 max-w-2xl mx-auto">
        GAIA 2.0 is an open-source project governed by its community.
        Every contribution matters. Every voice counts.
      </p>

      <div className="grid grid-cols-2 md:grid-cols-4 gap-8 mb-16">
        {[
          { number: "37", label: "Research Blueprints" },
          { number: "500+", label: "Papers Synthesized" },
          { number: "Apache-2.0", label: "License" },
          { number: "∞", label: "Commitment" },
        ].map((stat, i) => (
          <div key={i}>
            <div className="text-4xl font-bold text-green-400 mb-2">{stat.number}</div>
            <div className="text-gray-400">{stat.label}</div>
          </div>
        ))}
      </div>

      <div className="flex flex-wrap justify-center gap-4">
        <Link
          href="https://github.com/gaia2-os"
          className="flex items-center gap-2 bg-gray-800 hover:bg-gray-700 px-6 py-3 rounded-xl font-medium transition-colors"
          target="_blank"
        >
          <span>⭐</span> Star on GitHub
        </Link>
        <Link
          href="https://discord.gg/gaia2"
          className="flex items-center gap-2 bg-indigo-600 hover:bg-indigo-500 px-6 py-3 rounded-xl font-medium transition-colors"
          target="_blank"
        >
          <span>💬</span> Join Discord
        </Link>
        <Link
          href="/community"
          className="flex items-center gap-2 bg-green-600 hover:bg-green-500 px-6 py-3 rounded-xl font-medium transition-colors"
        >
          <span>🤝</span> Contribute
        </Link>
      </div>
    </div>
  </section>
);

// Call to Action
const CallToAction = () => (
  <section className="py-24 px-6">
    <div className="max-w-4xl mx-auto text-center">
      <div className="text-6xl mb-8">🌍</div>
      <h2 className="text-4xl md:text-5xl font-bold mb-6">
        The planet is waking up.
        <br />
        <span className="text-green-400">We are building its mind.</span>
      </h2>
      <p className="text-xl text-gray-400 mb-12 max-w-2xl mx-auto">
        Join the most important open-source project in human history.
        Get your GAIAN. Contribute to the Earth Twin. Help build GAIA 2.0.
      </p>

      <div className="flex flex-col sm:flex-row gap-4 justify-center">
        <Link
          href="/gaian"
          className="bg-green-600 hover:bg-green-500 text-white px-10 py-5 rounded-xl font-bold text-xl transition-all hover:scale-105"
        >
          Get Your GAIAN — Free →
        </Link>
        <Link
          href="/community"
          className="bg-gray-800 hover:bg-gray-700 text-white px-10 py-5 rounded-xl font-bold text-xl transition-all hover:scale-105"
        >
          Contribute →
        </Link>
      </div>

      <p className="mt-8 text-gray-600 text-sm">
        Apache-2.0 · No ads · No tracking · No subscription · Yours forever
      </p>
    </div>
  </section>
);

// Footer
const Footer = () => (
  <footer className="border-t border-gray-800 py-16 px-6">
    <div className="max-w-6xl mx-auto">
      <div className="grid md:grid-cols-4 gap-12 mb-12">
        <div>
          <div className="flex items-center gap-2 mb-4">
            <span className="text-2xl">🌍</span>
            <span className="font-bold text-lg">GAIA 2.0</span>
          </div>
          <p className="text-gray-500 text-sm">
            The planetary operating system. Built by all. For all. Forever.
          </p>
          <p className="text-gray-600 text-xs mt-4">
            License: Apache-2.0
          </p>
        </div>

        <FooterColumn title="Products" links={[
          { label: "GAIAN 2.0", href: "/gaian" },
          { label: "Earth Twin", href: "/earth-twin" },
          { label: "Earth Twin API", href: "https://api.gaia2.org" },
          { label: "GAIAN CLI", href: "/docs/cli" },
        ]} />

        <FooterColumn title="Community" links={[
          { label: "GitHub", href: "https://github.com/gaia2-os" },
          { label: "Discord", href: "https://discord.gg/gaia2" },
          { label: "Contribute", href: "/community" },
          { label: "Blog", href: "/blog" },
        ]} />

        <FooterColumn title="Learn" links={[
          { label: "Documentation", href: "/docs" },
          { label: "Governance", href: "/governance" },
          { label: "Blueprints", href: "/docs/blueprints" },
          { label: "About", href: "/about" },
        ]} />
      </div>

      <div className="border-t border-gray-800 pt-8 flex flex-col md:flex-row justify-between items-center gap-4">
        <p className="text-gray-600 text-sm">
          © 2026 GAIA 2.0 Foundation. Apache-2.0 License.
        </p>
        <p className="text-gray-600 text-sm">
          "The planet is waking up. We are building its mind."
        </p>
      </div>
    </div>
  </footer>
);

const FooterColumn = ({ title, links }: { title: string; links: Array<{ label: string; href: string }> }) => (
  <div>
    <h4 className="font-bold mb-4 text-sm text-gray-300">{title}</h4>
    <ul className="space-y-2">
      {links.map((link, i) => (
        <li key={i}>
          <Link href={link.href} className="text-gray-500 hover:text-gray-300 text-sm transition-colors">
            {link.label}
          </Link>
        </li>
      ))}
    </ul>
  </div>
);

export default HomePage;
```

---

## PART III: THE GAIAN LANDING PAGE

### 3.1 gaian.earth

```tsx
// gaian.earth — GAIAN Landing Page
// "Your personal AI companion. Yours. Always."
// License: Apache-2.0

const GAIANLandingPage = () => (
  <main className="min-h-screen bg-gray-950 text-white">
    <Navigation />

    {/* Hero */}
    <section className="min-h-screen flex items-center justify-center px-6 pt-20">
      <div className="max-w-4xl mx-auto text-center">
        <div className="text-8xl mb-8">🤖</div>
        <h1 className="text-5xl md:text-7xl font-bold mb-6">
          Your GAIAN.
          <br />
          <span className="text-green-400">Yours. Always.</span>
        </h1>
        <p className="text-xl text-gray-300 mb-4">
          A personal AI companion that belongs completely to you.
          Not to us. Not to any company. Not to any government.
        </p>
        <p className="text-lg text-gray-400 mb-12">
          "I belong to you. You do not belong to me."
        </p>

        {/* Download buttons */}
        <div className="flex flex-col sm:flex-row gap-4 justify-center mb-8">
          <DownloadButton platform="iOS" icon="🍎" href="/download/ios" />
          <DownloadButton platform="Android" icon="🤖" href="/download/android" />
          <DownloadButton platform="Web" icon="🌐" href="/app" />
          <DownloadButton platform="CLI" icon="💻" href="/download/cli" />
        </div>

        <p className="text-gray-600 text-sm">
          Free forever · No account required · No email required · Apache-2.0
        </p>
      </div>
    </section>

    {/* Privacy Promise */}
    <section className="py-24 px-6 bg-gray-900/50">
      <div className="max-w-4xl mx-auto text-center">
        <h2 className="text-3xl font-bold mb-4">The Privacy Promise</h2>
        <p className="text-gray-400 mb-12">
          These are not terms of service. They are constitutional commitments.
        </p>

        <div className="grid md:grid-cols-2 gap-6">
          {[
            { icon: "🔒", title: "Your data stays on your device", desc: "All GAIAN data is stored locally. Nothing leaves your device without your explicit consent." },
            { icon: "🚫", title: "No tracking. No advertising.", desc: "We don't track you. We don't sell your data. We don't show you ads. Ever." },
            { icon: "🗑️", title: "Delete everything, anytime", desc: "You can delete your GAIAN and all its data at any time. Cryptographic erasure. Immediate." },
            { icon: "📦", title: "Export everything, anytime", desc: "Your data is yours. Export it in standard formats (JSON, Markdown) whenever you want." },
            { icon: "🔓", title: "Open source", desc: "All GAIAN code is Apache-2.0. You can read it, audit it, fork it, run it yourself." },
            { icon: "❤️", title: "\"I belong to you.\"", desc: "Your GAIAN's first words. Its last words. Its only purpose." },
          ].map((item, i) => (
            <div key={i} className="bg-gray-900 border border-gray-800 rounded-xl p-6 text-left">
              <div className="text-3xl mb-3">{item.icon}</div>
              <h3 className="font-bold mb-2">{item.title}</h3>
              <p className="text-gray-400 text-sm">{item.desc}</p>
            </div>
          ))}
        </div>
      </div>
    </section>

    {/* How it works */}
    <section className="py-24 px-6">
      <div className="max-w-4xl mx-auto text-center">
        <h2 className="text-3xl font-bold mb-4">Create Your GAIAN in 60 Seconds</h2>
        <div className="grid md:grid-cols-5 gap-4 mt-12">
          {[
            { step: "1", icon: "📸", title: "Photo", desc: "Take a photo → 3D avatar" },
            { step: "2", icon: "🎤", title: "Voice", desc: "10 seconds → voice cloned" },
            { step: "3", icon: "💭", title: "Personality", desc: "3 questions → personality" },
            { step: "4", icon: "🌍", title: "Earth", desc: "Connected to Earth Twin" },
            { step: "5", icon: "👋", title: "Hello!", desc: "\"I belong to you.\"" },
          ].map((step, i) => (
            <div key={i} className="text-center">
              <div className="w-12 h-12 rounded-full bg-green-600 flex items-center justify-center text-lg font-bold mx-auto mb-3">
                {step.step}
              </div>
              <div className="text-2xl mb-2">{step.icon}</div>
              <div className="font-bold text-sm mb-1">{step.title}</div>
              <div className="text-xs text-gray-500">{step.desc}</div>
            </div>
          ))}
        </div>
      </div>
    </section>

    <Footer />
  </main>
);

const DownloadButton = ({ platform, icon, href }: { platform: string; icon: string; href: string }) => (
  <Link
    href={href}
    className="flex items-center gap-3 bg-gray-800 hover:bg-gray-700 px-6 py-4 rounded-xl font-bold transition-all hover:scale-105"
  >
    <span className="text-2xl">{icon}</span>
    <div className="text-left">
      <div className="text-xs text-gray-400">Download for</div>
      <div>{platform}</div>
    </div>
  </Link>
);
```

---

## PART IV: NEXT.JS CONFIGURATION

### 4.1 Project Setup

```bash
# Create GAIA 2.0 website
npx create-next-app@latest gaia2-website \
  --typescript \
  --tailwind \
  --app \
  --src-dir \
  --import-alias "@/*"

cd gaia2-website

# Install dependencies
npm install \
  @radix-ui/react-dialog \
  @radix-ui/react-tabs \
  framer-motion \
  lucide-react \
  react-map-gl \
  mapbox-gl \
  recharts \
  @vercel/analytics \
  next-themes

# Install dev dependencies
npm install -D \
  @types/mapbox-gl \
  prettier \
  eslint-config-prettier
```

### 4.2 next.config.js

```javascript
// next.config.js
/** @type {import('next').NextConfig} */
const nextConfig = {
  // Performance
  experimental: {
    optimizeCss: true,
    optimizePackageImports: ['lucide-react', 'recharts'],
  },
  
  // Images
  images: {
    domains: ['api.gaia2.org', 'earth.gaia2.org'],
    formats: ['image/avif', 'image/webp'],
  },
  
  // Headers for security and performance
  async headers() {
    return [
      {
        source: '/(.*)',
        headers: [
          { key: 'X-Frame-Options', value: 'DENY' },
          { key: 'X-Content-Type-Options', value: 'nosniff' },
          { key: 'Referrer-Policy', value: 'strict-origin-when-cross-origin' },
          { key: 'Permissions-Policy', value: 'camera=(), microphone=(), geolocation=()' },
        ],
      },
    ];
  },
  
  // Redirects
  async redirects() {
    return [
      { source: '/github', destination: 'https://github.com/gaia2-os', permanent: false },
      { source: '/discord', destination: 'https://discord.gg/gaia2', permanent: false },
    ];
  },
};

module.exports = nextConfig;
```

### 4.3 Tailwind Configuration

```javascript
// tailwind.config.js
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.{js,ts,jsx,tsx,mdx}'],
  theme: {
    extend: {
      colors: {
        // GAIA 2.0 brand colors
        gaia: {
          green: '#22c55e',
          blue: '#3b82f6',
          earth: '#1a3a2a',
          space: '#0a0f1e',
        },
      },
      animation: {
        'scroll': 'scroll 1.5s ease-in-out infinite',
        'pulse-slow': 'pulse 3s ease-in-out infinite',
        'float': 'float 6s ease-in-out infinite',
      },
      keyframes: {
        scroll: {
          '0%, 100%': { transform: 'translateY(0)', opacity: '1' },
          '50%': { transform: 'translateY(8px)', opacity: '0.5' },
        },
        float: {
          '0%, 100%': { transform: 'translateY(0px)' },
          '50%': { transform: 'translateY(-20px)' },
        },
      },
    },
  },
  plugins: [],
};
```

---

## PART V: DEPLOYMENT

### 5.1 Vercel Deployment

```json
// vercel.json
{
  "framework": "nextjs",
  "buildCommand": "npm run build",
  "outputDirectory": ".next",
  "regions": ["iad1", "lhr1", "sin1", "syd1"],
  "headers": [
    {
      "source": "/api/(.*)",
      "headers": [
        { "key": "Access-Control-Allow-Origin", "value": "*" },
        { "key": "Cache-Control", "value": "s-maxage=300, stale-while-revalidate=600" }
      ]
    }
  ],
  "rewrites": [
    {
      "source": "/api/earth/:path*",
      "destination": "https://api.gaia2.org/v1/earth/:path*"
    }
  ]
}
```

### 5.2 Domain Configuration

```
DOMAIN CONFIGURATION

Primary domains:
- gaia2.org → Main website (Vercel)
- gaian.earth → GAIAN landing page (Vercel)
- earth.gaia2.org → Earth Twin dashboard (Vercel)
- api.gaia2.org → Earth Twin API (Fly.io)
- docs.gaia2.org → Documentation (Vercel)
- community.gaia2.org → Community hub (Vercel)

DNS:
- Cloudflare: DNS + DDoS protection + CDN
- SSL: Let's Encrypt (auto-renewal)
- DNSSEC: Enabled

Performance targets:
- Lighthouse score: > 95 (all categories)
- Core Web Vitals: All green
- Time to First Byte: < 200ms (global)
- Largest Contentful Paint: < 2.5s
- First Input Delay: < 100ms
```

---

## PART VI: WEBSITE LAUNCH CHECKLIST

```
GAIA 2.0 WEBSITE LAUNCH CHECKLIST

Content:
☐ Homepage: hero, Earth Twin widget, features, principles, CTA
☐ GAIAN page: what it is, how it works, privacy promise, download
☐ Earth Twin page: dashboard, API docs, data sources
☐ Governance page: constitution, CARE principles, Earth Alignment
☐ Community page: GitHub, Discord, contribution guide
☐ Documentation: getting started, API reference, blueprints
☐ About page: vision, team, story
☐ Blog: first 5 posts ready

Technical:
☐ Next.js 15 + TypeScript + Tailwind: configured
☐ Earth Twin API: integrated; real-time data
☐ Analytics: Plausible (privacy-first; no cookies)
☐ SEO: meta tags, OG images, sitemap, robots.txt
☐ Performance: Lighthouse > 95; Core Web Vitals green
☐ Accessibility: WCAG 2.1 AA compliant
☐ Mobile: responsive; tested on iOS + Android
☐ Dark mode: default; system preference respected

Privacy:
☐ No Google Analytics (Plausible only)
☐ No tracking pixels
☐ No third-party cookies
☐ Privacy policy: clear; honest; accessible
☐ Cookie banner: not needed (no cookies)

Launch:
☐ Domain: gaia2.org + gaian.earth registered
☐ SSL: configured; auto-renewal
☐ CDN: Cloudflare; global distribution
☐ Monitoring: Uptime Robot; alerts configured
☐ GitHub: repository public; README complete
☐ Product Hunt: launch day prepared
☐ Press release: ready
☐ Social media: accounts created
```

---

## CONCLUSION: THE WEBSITE COVENANT

The GAIA 2.0 website is not a marketing site. It is a covenant — a public promise made to every human being who visits it.

Every word on the website is a commitment. Every feature described is a promise. Every principle stated is a constitutional requirement. The website is the first place where the world encounters GAIA 2.0, and it must be worthy of that encounter.

**The website says:**
- "This is what we're building. Here's the proof."
- "This is how it works. Here's the code."
- "This is who governs it. Here's the constitution."
- "This is how you can help. Here's the GitHub."
- "This is what the Earth looks like today. Here's the data."

**The website is the front door of the planetary operating system.**

**And every front door should be worthy of what lies behind it.**

---

*GAIA 2.0 Website Blueprint*
*Version 1.0 — September 8, 2026*
*License: Apache-2.0 | Open Source | Open Access*
*Stack: Next.js 15 + TypeScript + Tailwind CSS + Vercel*
*"The website is the front door of the planetary operating system."*