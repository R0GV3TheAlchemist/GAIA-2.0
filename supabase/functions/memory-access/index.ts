import "jsr:@supabase/functions-js/edge-runtime.d.ts";
import { createClient } from "jsr:@supabase/supabase-js@2";

const CORS = {
  "Access-Control-Allow-Origin": "*",
  "Access-Control-Allow-Headers": "authorization, x-client-info, apikey, content-type",
  "Access-Control-Allow-Methods": "GET, POST, OPTIONS",
};

function json(body: unknown, status = 200) {
  return new Response(JSON.stringify(body), {
    status,
    headers: { ...CORS, "Content-Type": "application/json" },
  });
}

Deno.serve(async (req: Request) => {
  if (req.method === "OPTIONS") {
    return new Response("ok", { headers: CORS });
  }

  const auth = req.headers.get("Authorization");
  if (!auth) {
    return json({
      error: "C77: authentication required",
      canon: ["C77", "C01"],
    }, 401);
  }

  const url = Deno.env.get("SUPABASE_URL");
  const anon = Deno.env.get("SUPABASE_ANON_KEY") ?? Deno.env.get("SUPABASE_PUBLISHABLE_KEY");
  if (!url || !anon) {
    return json({ error: "runtime misconfigured" }, 500);
  }

  const supabase = createClient(url, anon, {
    global: { headers: { Authorization: auth } },
  });

  if (req.method === "GET") {
    const { data, error } = await supabase.rpc("c77_runtime_status");
    if (error) return json({ error: error.message, canon: ["C77"] }, 400);
    return json({ ok: true, status: data });
  }

  if (req.method !== "POST") {
    return json({ error: "use GET for status or POST for memory access" }, 405);
  }

  let payload: {
    action?: string;
    memory_id?: string;
    purpose?: string;
    scope?: string;
    consent_purpose?: string;
    expires_at?: string | null;
    consent_id?: string;
    reason?: string;
  };
  try {
    payload = await req.json();
  } catch {
    return json({ error: "expected JSON body" }, 400);
  }

  const action = (payload.action ?? "access").toLowerCase();

  if (action === "grant_consent") {
    const { data, error } = await supabase.rpc("grant_study_consent", {
      p_scope: payload.scope ?? "memory",
      p_purpose: payload.consent_purpose ?? "stewardship research",
      p_expires_at: payload.expires_at ?? null,
    });
    if (error) return json({ error: error.message, blocked: true, canon: ["C77"] }, 400);
    return json({ ok: true, consent_id: data, canon: ["C77", "C01"] });
  }

  if (action === "revoke_consent") {
    if (!payload.consent_id) return json({ error: "consent_id required" }, 400);
    const { data, error } = await supabase.rpc("revoke_study_consent", {
      p_consent_id: payload.consent_id,
      p_reason: payload.reason ?? "subject revoked",
    });
    if (error) return json({ error: error.message, canon: ["C77"] }, 400);
    return json({ ok: true, revoked: data, canon: ["C77", "C01"] });
  }

  if (!payload.memory_id) {
    return json({ error: "memory_id required" }, 400);
  }

  const purpose = (payload.purpose ?? "recall").toLowerCase();
  const { data, error } = await supabase.rpc("access_memory_stewarded", {
    target_memory_id: payload.memory_id,
    purpose,
  });

  if (error) {
    const blocked = /not allowed without explicit/i.test(error.message) ||
      error.message.includes("42501") ||
      /study of this memory/i.test(error.message);
    return json({
      ok: false,
      blocked,
      error: error.message,
      canon: ["C77", "C01", "C30"],
    }, blocked ? 403 : 400);
  }

  return json({
    ok: true,
    purpose,
    result: data,
    canon: ["C77", "C01", "C34"],
  });
});
