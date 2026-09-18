-- Applied 2026-09-18 to Supabase project gaia-2-0 (yylqoiqobydrdsnnulip)
-- Covering index for gap_locks_gap_id_fkey; wrap auth.uid() as (SELECT auth.uid())
-- so RLS initplan is evaluated once per query. No data change. No live telemetry.

CREATE INDEX IF NOT EXISTS gap_locks_gap_id_idx ON public.gap_locks (gap_id);

DROP POLICY IF EXISTS memories_owner_all ON public.memories;
CREATE POLICY memories_owner_all ON public.memories
  FOR ALL TO authenticated
  USING ((owner_id IS NULL) OR (owner_id = (SELECT auth.uid())))
  WITH CHECK ((owner_id IS NULL) OR (owner_id = (SELECT auth.uid())));

DROP POLICY IF EXISTS consent_read_own ON public.consent_events;
CREATE POLICY consent_read_own ON public.consent_events
  FOR SELECT TO authenticated
  USING ((subject_id IS NULL) OR (subject_id = (SELECT auth.uid())) OR (actor_id = (SELECT auth.uid())));

DROP POLICY IF EXISTS consent_insert_own ON public.consent_events;
CREATE POLICY consent_insert_own ON public.consent_events
  FOR INSERT TO authenticated
  WITH CHECK ((actor_id IS NULL) OR (actor_id = (SELECT auth.uid())));

DROP POLICY IF EXISTS memory_access_events_owner_read ON public.memory_access_events;
CREATE POLICY memory_access_events_owner_read ON public.memory_access_events
  FOR SELECT TO authenticated
  USING (EXISTS (
    SELECT 1 FROM public.memories m
    WHERE m.memory_id = memory_access_events.memory_id
      AND ((m.owner_id IS NULL) OR (m.owner_id = (SELECT auth.uid())))
  ));

DROP POLICY IF EXISTS stewardship_events_own ON public.stewardship_events;
CREATE POLICY stewardship_events_own ON public.stewardship_events
  FOR ALL TO authenticated
  USING ((subject_id = (SELECT auth.uid())) OR (actor_id = (SELECT auth.uid())))
  WITH CHECK ((subject_id = (SELECT auth.uid())) OR (actor_id = (SELECT auth.uid())));

DROP POLICY IF EXISTS study_consents_own ON public.study_consents;
CREATE POLICY study_consents_own ON public.study_consents
  FOR ALL TO authenticated
  USING ((subject_id = (SELECT auth.uid())) OR (granted_by = (SELECT auth.uid())))
  WITH CHECK ((subject_id = (SELECT auth.uid())) OR (granted_by = (SELECT auth.uid())));

DROP POLICY IF EXISTS flourishing_scores_own ON public.flourishing_scores;
CREATE POLICY flourishing_scores_own ON public.flourishing_scores
  FOR ALL TO authenticated
  USING (subject_id = (SELECT auth.uid()))
  WITH CHECK (subject_id = (SELECT auth.uid()));
