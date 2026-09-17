-- GAIA 2.0 backbone: memory, consent, agents, lithic KB, proofs
-- Applied 2026-09-17 to Supabase project gaia-2-0 (yylqoiqobydrdsnnulip)

CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TYPE memory_tier AS ENUM ('HOT', 'WARM', 'COLD');
CREATE TYPE agent_role AS ENUM ('orchestrator', 'execution', 'safety', 'consent', 'memory', 'knowledge', 'interface', 'monitor');
CREATE TYPE circuit_breaker_state AS ENUM ('CLOSED', 'OPEN', 'HALF_OPEN');
CREATE TYPE consent_action AS ENUM ('grant', 'revoke', 'failover', 'suspend', 'resume');
CREATE TYPE proof_type AS ENUM ('simulation', 'formal', 'empirical', 'emergent_confirmation');
CREATE TYPE proof_status AS ENUM ('proven', 'partial', 'in_progress');

CREATE TABLE IF NOT EXISTS memories (
  memory_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  created_at timestamptz NOT NULL DEFAULT now(),
  content_hash text NOT NULL,
  content jsonb NOT NULL DEFAULT '{}'::jsonb,
  tier memory_tier NOT NULL DEFAULT 'WARM',
  last_accessed timestamptz,
  access_count integer NOT NULL DEFAULT 0,
  relevance_score double precision NOT NULL DEFAULT 0.5 CHECK (relevance_score >= 0 AND relevance_score <= 1),
  decay_rate double precision NOT NULL DEFAULT 0.03,
  tier_upgraded_at timestamptz,
  owner_id uuid,
  updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS consent_events (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  created_at timestamptz NOT NULL DEFAULT now(),
  subject_id uuid,
  actor_id uuid,
  action consent_action NOT NULL,
  scope text NOT NULL,
  cause text,
  duration_ms integer,
  shard_key text,
  payload jsonb NOT NULL DEFAULT '{}'::jsonb
);

CREATE TABLE IF NOT EXISTS agent_health (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  agent_role agent_role NOT NULL,
  instance_id text NOT NULL,
  is_primary boolean NOT NULL DEFAULT true,
  cb_state circuit_breaker_state NOT NULL DEFAULT 'CLOSED',
  failure_count integer NOT NULL DEFAULT 0,
  last_failure_at timestamptz,
  last_failover_at timestamptz,
  uptime_ratio double precision,
  observed_at timestamptz NOT NULL DEFAULT now(),
  UNIQUE (agent_role, instance_id)
);

CREATE TABLE IF NOT EXISTS elements (
  z integer PRIMARY KEY CHECK (z >= 1 AND z <= 118),
  symbol text NOT NULL,
  name text NOT NULL,
  data jsonb NOT NULL DEFAULT '{}'::jsonb,
  updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS minerals (
  id text PRIMARY KEY,
  name text NOT NULL,
  formula text NOT NULL,
  element_ids integer[] NOT NULL,
  mineral_class text NOT NULL,
  crystal_system text,
  hardness_mohs text,
  gaia_layer_alignment text[],
  crystal_link text,
  notes text,
  data jsonb NOT NULL DEFAULT '{}'::jsonb,
  updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS crystals (
  crystal_id text PRIMARY KEY,
  name text NOT NULL,
  mineral_class text,
  crystal_system text,
  septagram_nodes text[],
  data jsonb NOT NULL DEFAULT '{}'::jsonb,
  version text,
  last_updated date,
  updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS proofs (
  proof_id text PRIMARY KEY,
  proof_type proof_type NOT NULL,
  status proof_status NOT NULL DEFAULT 'in_progress',
  method text NOT NULL,
  results text,
  canon_path text,
  github_sha text,
  committed_at timestamptz,
  created_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_memories_tier ON memories (tier);
CREATE INDEX IF NOT EXISTS idx_memories_owner ON memories (owner_id);
CREATE INDEX IF NOT EXISTS idx_memories_last_accessed ON memories (last_accessed);
CREATE INDEX IF NOT EXISTS idx_consent_subject ON consent_events (subject_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_consent_shard ON consent_events (shard_key);
CREATE INDEX IF NOT EXISTS idx_agent_health_role ON agent_health (agent_role);

CREATE OR REPLACE VIEW memory_retention_metric AS
SELECT
  count(*) FILTER (
    WHERE tier IN ('HOT','WARM')
      AND relevance_score >= 0.50
      AND created_at >= now() - interval '30 days'
  )::double precision
  / NULLIF(count(*) FILTER (
    WHERE tier IN ('HOT','WARM')
      AND created_at >= now() - interval '30 days'
  ), 0) AS retention_rate_30d
FROM memories;

ALTER TABLE memories ENABLE ROW LEVEL SECURITY;
ALTER TABLE consent_events ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_health ENABLE ROW LEVEL SECURITY;
ALTER TABLE elements ENABLE ROW LEVEL SECURITY;
ALTER TABLE minerals ENABLE ROW LEVEL SECURITY;
ALTER TABLE crystals ENABLE ROW LEVEL SECURITY;
ALTER TABLE proofs ENABLE ROW LEVEL SECURITY;

CREATE POLICY memories_owner_all ON memories
  FOR ALL TO authenticated
  USING (owner_id IS NULL OR owner_id = auth.uid())
  WITH CHECK (owner_id IS NULL OR owner_id = auth.uid());

CREATE POLICY consent_read_own ON consent_events
  FOR SELECT TO authenticated
  USING (subject_id IS NULL OR subject_id = auth.uid() OR actor_id = auth.uid());

CREATE POLICY consent_insert_own ON consent_events
  FOR INSERT TO authenticated
  WITH CHECK (actor_id IS NULL OR actor_id = auth.uid());

CREATE POLICY agent_health_read ON agent_health
  FOR SELECT TO authenticated USING (true);
CREATE POLICY elements_read ON elements FOR SELECT TO authenticated USING (true);
CREATE POLICY minerals_read ON minerals FOR SELECT TO authenticated USING (true);
CREATE POLICY crystals_read ON crystals FOR SELECT TO authenticated USING (true);
CREATE POLICY proofs_read ON proofs FOR SELECT TO authenticated USING (true);
