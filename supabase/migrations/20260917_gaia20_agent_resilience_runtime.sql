-- GAIA 2.0 CT-003: circuit breaker, governance failover, and execution gate
-- Live migration: gaia20_agent_resilience_runtime
-- Proof: proofs/PROOF-GAIA20-AGENT-001.md

CREATE TABLE IF NOT EXISTS public.agent_incidents (
  incident_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  observed_at timestamptz NOT NULL DEFAULT now(),
  agent_role agent_role NOT NULL,
  instance_id text NOT NULL,
  event_type text NOT NULL CHECK (event_type IN ('failure','circuit_opened','circuit_half_open','circuit_closed','failover','execution_suspended')),
  failure_count integer,
  previous_cb_state circuit_breaker_state,
  current_cb_state circuit_breaker_state,
  detail jsonb NOT NULL DEFAULT '{}'::jsonb
);
CREATE INDEX IF NOT EXISTS idx_agent_incidents_role_time ON public.agent_incidents (agent_role, observed_at DESC);
ALTER TABLE public.agent_incidents ENABLE ROW LEVEL SECURITY;
CREATE POLICY agent_incidents_read ON public.agent_incidents FOR SELECT TO authenticated USING (true);

CREATE OR REPLACE FUNCTION public.execution_permitted()
RETURNS boolean LANGUAGE sql STABLE SECURITY INVOKER SET search_path = public AS $$
  SELECT EXISTS (
    SELECT 1 FROM public.agent_health
    WHERE agent_role = 'safety' AND is_primary = true AND cb_state = 'CLOSED' AND COALESCE(uptime_ratio,0) >= 0.999
  ) AND EXISTS (
    SELECT 1 FROM public.agent_health
    WHERE agent_role = 'consent' AND is_primary = true AND cb_state = 'CLOSED' AND COALESCE(uptime_ratio,0) >= 0.999
  );
$$;

CREATE OR REPLACE FUNCTION public.report_agent_failure(target_role agent_role, target_instance_id text, observed_at_input timestamptz DEFAULT now())
RETURNS TABLE (agent_role agent_role, instance_id text, consecutive_failures integer, cb_state circuit_breaker_state, execution_allowed boolean)
LANGUAGE plpgsql SECURITY INVOKER SET search_path = public AS $$
DECLARE a public.agent_health%ROWTYPE; next_count integer; next_state circuit_breaker_state;
BEGIN
  SELECT * INTO a FROM public.agent_health
  WHERE agent_health.agent_role = target_role AND agent_health.instance_id = target_instance_id FOR UPDATE;
  IF NOT FOUND THEN RAISE EXCEPTION 'agent %/% not found', target_role, target_instance_id USING ERRCODE='P0002'; END IF;
  next_count := CASE WHEN a.last_failure_at IS NOT NULL AND observed_at_input-a.last_failure_at <= interval '30 seconds' THEN a.failure_count+1 ELSE 1 END;
  next_state := CASE WHEN next_count >= 3 THEN 'OPEN'::circuit_breaker_state ELSE a.cb_state END;
  UPDATE public.agent_health SET failure_count=next_count,last_failure_at=observed_at_input,cb_state=next_state,observed_at=observed_at_input WHERE id=a.id;
  INSERT INTO public.agent_incidents(observed_at,agent_role,instance_id,event_type,failure_count,previous_cb_state,current_cb_state,detail)
  VALUES(observed_at_input,target_role,target_instance_id,'failure',next_count,a.cb_state,next_state,jsonb_build_object('window_seconds',30));
  IF a.cb_state <> 'OPEN' AND next_state = 'OPEN' THEN
    INSERT INTO public.agent_incidents(observed_at,agent_role,instance_id,event_type,failure_count,previous_cb_state,current_cb_state,detail)
    VALUES(observed_at_input,target_role,target_instance_id,'circuit_opened',next_count,a.cb_state,next_state,jsonb_build_object('trip_threshold',3));
    IF target_role IN ('safety','consent') THEN
      INSERT INTO public.consent_events(action,scope,cause,shard_key,payload)
      VALUES('suspend','execution','governance circuit breaker opened',target_role::text,jsonb_build_object('agent_role',target_role,'instance_id',target_instance_id,'failure_count',next_count));
    END IF;
  END IF;
  RETURN QUERY SELECT target_role,target_instance_id,next_count,next_state,public.execution_permitted();
END; $$;

CREATE OR REPLACE FUNCTION public.report_agent_success(target_role agent_role, target_instance_id text, observed_at_input timestamptz DEFAULT now())
RETURNS TABLE (agent_role agent_role, instance_id text, cb_state circuit_breaker_state)
LANGUAGE plpgsql SECURITY INVOKER SET search_path = public AS $$
DECLARE a public.agent_health%ROWTYPE; next_state circuit_breaker_state;
BEGIN
  SELECT * INTO a FROM public.agent_health
  WHERE agent_health.agent_role=target_role AND agent_health.instance_id=target_instance_id FOR UPDATE;
  IF NOT FOUND THEN RAISE EXCEPTION 'agent %/% not found',target_role,target_instance_id USING ERRCODE='P0002'; END IF;
  next_state := CASE a.cb_state WHEN 'OPEN' THEN 'HALF_OPEN'::circuit_breaker_state WHEN 'HALF_OPEN' THEN 'CLOSED'::circuit_breaker_state ELSE 'CLOSED'::circuit_breaker_state END;
  UPDATE public.agent_health SET cb_state=next_state,failure_count=CASE WHEN next_state='CLOSED' THEN 0 ELSE failure_count END,observed_at=observed_at_input WHERE id=a.id;
  IF next_state <> a.cb_state THEN
    INSERT INTO public.agent_incidents(observed_at,agent_role,instance_id,event_type,failure_count,previous_cb_state,current_cb_state,detail)
    VALUES(observed_at_input,target_role,target_instance_id,CASE WHEN next_state='HALF_OPEN' THEN 'circuit_half_open' ELSE 'circuit_closed' END,CASE WHEN next_state='CLOSED' THEN 0 ELSE a.failure_count END,a.cb_state,next_state,'{}'::jsonb);
  END IF;
  RETURN QUERY SELECT target_role,target_instance_id,next_state;
END; $$;

CREATE OR REPLACE FUNCTION public.failover_governance_agent(target_role agent_role, failed_instance_id text, observed_at_input timestamptz DEFAULT now())
RETURNS TABLE (agent_role agent_role, failed_instance text, promoted_instance text, failover_ms integer)
LANGUAGE plpgsql SECURITY INVOKER SET search_path = public AS $$
DECLARE failed public.agent_health%ROWTYPE; standby public.agent_health%ROWTYPE;
BEGIN
  IF target_role NOT IN ('safety','consent') THEN RAISE EXCEPTION 'failover is restricted to safety and consent agents' USING ERRCODE='22023'; END IF;
  SELECT * INTO failed FROM public.agent_health WHERE agent_health.agent_role=target_role AND agent_health.instance_id=failed_instance_id AND agent_health.is_primary=true FOR UPDATE;
  IF NOT FOUND THEN RAISE EXCEPTION 'primary agent %/% not found',target_role,failed_instance_id USING ERRCODE='P0002'; END IF;
  SELECT * INTO standby FROM public.agent_health WHERE agent_health.agent_role=target_role AND agent_health.is_primary=false FOR UPDATE;
  IF NOT FOUND THEN RAISE EXCEPTION 'standby agent for % not found',target_role USING ERRCODE='P0002'; END IF;
  UPDATE public.agent_health SET is_primary=false,cb_state='OPEN',last_failover_at=observed_at_input,observed_at=observed_at_input WHERE id=failed.id;
  UPDATE public.agent_health SET is_primary=true,cb_state='CLOSED',failure_count=0,last_failover_at=observed_at_input,observed_at=observed_at_input WHERE id=standby.id;
  INSERT INTO public.agent_incidents(observed_at,agent_role,instance_id,event_type,previous_cb_state,current_cb_state,detail)
  VALUES(observed_at_input,target_role,failed_instance_id,'failover',failed.cb_state,'OPEN',jsonb_build_object('promoted_instance',standby.instance_id,'failover_ms',0));
  INSERT INTO public.consent_events(action,scope,cause,duration_ms,shard_key,payload)
  VALUES('failover','governance','automatic governance standby promotion',0,target_role::text,jsonb_build_object('failed_instance',failed_instance_id,'promoted_instance',standby.instance_id));
  RETURN QUERY SELECT target_role,failed_instance_id,standby.instance_id,0;
END; $$;

REVOKE ALL ON FUNCTION public.report_agent_failure(agent_role,text,timestamptz) FROM PUBLIC;
REVOKE ALL ON FUNCTION public.report_agent_success(agent_role,text,timestamptz) FROM PUBLIC;
REVOKE ALL ON FUNCTION public.failover_governance_agent(agent_role,text,timestamptz) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION public.report_agent_failure(agent_role,text,timestamptz) TO authenticated;
GRANT EXECUTE ON FUNCTION public.report_agent_success(agent_role,text,timestamptz) TO authenticated;
GRANT EXECUTE ON FUNCTION public.failover_governance_agent(agent_role,text,timestamptz) TO authenticated;
GRANT EXECUTE ON FUNCTION public.execution_permitted() TO authenticated;
