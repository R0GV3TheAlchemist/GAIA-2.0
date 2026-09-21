/**
 * gateway.ts — typed fetch helpers for gaia-gateway (localhost:7700).
 * All paths are relative to /api which Vite proxies to the gateway.
 */

const BASE = '/api';

// ── Types ──────────────────────────────────────────────────────────────

export interface IntentRequest {
  text: string;
  profile?: string;
}

export interface IntentResponse {
  id: string;
  status: string;
}

export interface Agent {
  id: string;
  name: string;
  status: string;
}

export interface AgentCreateRequest {
  name: string;
  manifest?: unknown;
}

// ── Intent ─────────────────────────────────────────────────────────────

export async function submitIntent(req: IntentRequest): Promise<IntentResponse> {
  const res = await fetch(`${BASE}/intent`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(req),
  });
  if (!res.ok) throw new Error(`POST /intent failed: ${res.status}`);
  return res.json() as Promise<IntentResponse>;
}

/**
 * openIntentStream — opens a Server-Sent Events connection to
 * GET /intent/stream and calls onChunk for each data line received.
 * Returns a cleanup function that closes the EventSource.
 */
export function openIntentStream(
  onChunk: (chunk: string) => void,
  onError?: (err: Event) => void,
): () => void {
  const es = new EventSource(`${BASE}/intent/stream`);

  es.onmessage = (e: MessageEvent) => {
    onChunk(e.data as string);
  };

  es.onerror = (e) => {
    onError?.(e);
    es.close();
  };

  return () => es.close();
}

// ── Agents ─────────────────────────────────────────────────────────────

export async function listAgents(): Promise<Agent[]> {
  const res = await fetch(`${BASE}/agents`);
  // Gateway stub returns 404 for GET /agents until wired — handle gracefully
  if (res.status === 404) return [];
  if (!res.ok) throw new Error(`GET /agents failed: ${res.status}`);
  return res.json() as Promise<Agent[]>;
}

export async function revokeAgent(id: string): Promise<void> {
  const res = await fetch(`${BASE}/agents/${encodeURIComponent(id)}/revoke`, {
    method: 'DELETE',
  });
  if (res.status === 204) return;
  if (res.status === 404) throw new Error(`Agent '${id}' not found`);
  throw new Error(`DELETE /agents/${id}/revoke failed: ${res.status}`);
}

export async function createAgent(req: AgentCreateRequest): Promise<Agent> {
  const res = await fetch(`${BASE}/agents`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(req),
  });
  if (!res.ok) throw new Error(`POST /agents failed: ${res.status}`);
  return res.json() as Promise<Agent>;
}
