import { useState, useEffect, useCallback } from 'react';
import { listAgents, revokeAgent, type Agent } from '../api/gateway';

type AgentRow = Agent & { revoking?: boolean };

export default function AgentMatrix() {
  const [agents, setAgents]   = useState<AgentRow[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError]     = useState<string | null>(null);

  const fetchAgents = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await listAgents();
      setAgents(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load agents');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => { void fetchAgents(); }, [fetchAgents]);

  const handleRevoke = async (id: string) => {
    setAgents((prev) =>
      prev.map((a) => (a.id === id ? { ...a, revoking: true } : a)),
    );
    try {
      await revokeAgent(id);
      setAgents((prev) => prev.filter((a) => a.id !== id));
    } catch (err) {
      setError(err instanceof Error ? err.message : `Failed to revoke ${id}`);
      setAgents((prev) =>
        prev.map((a) => (a.id === id ? { ...a, revoking: false } : a)),
      );
    }
  };

  return (
    <section aria-labelledby="agents-heading">
      <div className="agents-header">
        <h1 id="agents-heading" className="section-title">Agents</h1>
        <button
          className="btn btn-ghost"
          onClick={fetchAgents}
          aria-label="Refresh agent list"
        >
          Refresh
        </button>
      </div>

      {error && <p className="error-banner" role="alert">{error}</p>}

      {loading ? (
        <p aria-live="polite" className="loading-msg">Loading agents…</p>
      ) : agents.length === 0 ? (
        <div className="card empty-state" role="status">
          <p>No running agents.</p>
          <p className="empty-hint">Start an agent with <code>gaia agent deploy</code> or via the Studio.</p>
        </div>
      ) : (
        <div className="card" style={{ padding: 0, overflow: 'hidden' }}>
          <table className="agent-table" aria-label="Running agents">
            <thead>
              <tr>
                <th scope="col">ID</th>
                <th scope="col">Name</th>
                <th scope="col">Status</th>
                <th scope="col"><span className="sr-only">Actions</span></th>
              </tr>
            </thead>
            <tbody>
              {agents.map((agent) => (
                <tr key={agent.id}>
                  <td className="cell-mono">{agent.id.slice(0, 8)}…</td>
                  <td>{agent.name}</td>
                  <td>
                    <span
                      className={`status-badge status-${agent.status}`}
                      aria-label={`Status: ${agent.status}`}
                    >
                      {agent.status}
                    </span>
                  </td>
                  <td className="cell-actions">
                    <button
                      className="btn btn-danger"
                      disabled={agent.revoking}
                      onClick={() => handleRevoke(agent.id)}
                      aria-label={`Revoke agent ${agent.name}`}
                    >
                      {agent.revoking ? 'Revoking…' : 'Revoke'}
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      <style>{`
        .agents-header {
          display: flex;
          align-items: center;
          justify-content: space-between;
          margin-bottom: var(--space-4);
        }
        .loading-msg { color: var(--color-text-muted); }
        .error-banner {
          color: var(--color-error);
          font-size: var(--text-sm);
          margin-bottom: var(--space-3);
        }
        .empty-state {
          color: var(--color-text-muted);
          text-align: center;
          padding: var(--space-12) var(--space-8);
        }
        .empty-hint { font-size: var(--text-sm); margin-top: var(--space-2); }
        .empty-hint code {
          background: var(--color-surface-2);
          padding: 0.1em 0.4em;
          border-radius: var(--radius-sm);
          font-size: 0.9em;
        }
        .agent-table {
          width: 100%;
          border-collapse: collapse;
          font-size: var(--text-sm);
        }
        .agent-table th,
        .agent-table td {
          padding: var(--space-3) var(--space-4);
          text-align: left;
          border-bottom: 1px solid var(--color-border);
        }
        .agent-table th {
          font-weight: 600;
          color: var(--color-text-muted);
          font-size: var(--text-xs);
          text-transform: uppercase;
          letter-spacing: 0.04em;
        }
        .agent-table tbody tr:last-child td { border-bottom: none; }
        .agent-table tbody tr:hover { background: var(--color-surface-2); }
        .cell-mono { font-family: monospace; color: var(--color-text-faint); }
        .cell-actions { text-align: right; }
        .status-badge {
          display: inline-block;
          padding: 0.15em 0.6em;
          border-radius: var(--radius-sm);
          font-size: var(--text-xs);
          font-weight: 500;
          text-transform: capitalize;
          background: var(--color-surface-offset);
          color: var(--color-text-muted);
        }
        .status-running  { background: oklch(from #6daa45 l c h / 0.15); color: #6daa45; }
        .status-deploying{ background: oklch(from #fdab43 l c h / 0.15); color: #fdab43; }
        .status-error    { background: oklch(from #dd6974 l c h / 0.15); color: #dd6974; }
        .sr-only {
          position: absolute; width: 1px; height: 1px; padding: 0;
          margin: -1px; overflow: hidden; clip: rect(0,0,0,0);
          white-space: nowrap; border-width: 0;
        }
      `}</style>
    </section>
  );
}
