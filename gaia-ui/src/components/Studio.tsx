import { useState } from 'react';
import { createAgent } from '../api/gateway';

const DEFAULT_MANIFEST = `# Agent manifest (TOML)
[agent]
name        = "my-agent"
description = "What this agent does"

[permissions]
network  = false
fs_read  = ["~/.gaia/data"]
fs_write = []

[runtime]
max_memory_mb = 256
timeout_s     = 30
`;

export default function Studio() {
  const [manifest, setManifest] = useState(DEFAULT_MANIFEST);
  const [name, setName]         = useState('');
  const [status, setStatus]     = useState<string | null>(null);
  const [error, setError]       = useState<string | null>(null);
  const [busy, setBusy]         = useState(false);

  const handleDeploy = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim()) { setError('Agent name is required'); return; }
    setError(null);
    setStatus(null);
    setBusy(true);
    try {
      const agent = await createAgent({ name: name.trim(), manifest });
      setStatus(`✓ Agent '${agent.name}' created — id=${agent.id}`);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Deploy failed');
    } finally {
      setBusy(false);
    }
  };

  return (
    <section aria-labelledby="studio-heading">
      <h1 id="studio-heading" className="section-title">Studio</h1>
      <p className="studio-desc">
        Compose an agent manifest and deploy it to the runtime.
      </p>

      <form className="studio-form" onSubmit={handleDeploy}>
        <div className="form-field">
          <label htmlFor="agent-name" className="form-label">Agent name</label>
          <input
            id="agent-name"
            type="text"
            className="form-input"
            placeholder="my-agent"
            value={name}
            onChange={(e) => setName(e.target.value)}
            disabled={busy}
            required
          />
        </div>

        <div className="form-field">
          <label htmlFor="agent-manifest" className="form-label">
            Manifest <span className="form-label-hint">(TOML)</span>
          </label>
          <textarea
            id="agent-manifest"
            className="form-textarea"
            value={manifest}
            onChange={(e) => setManifest(e.target.value)}
            disabled={busy}
            rows={14}
            spellCheck={false}
            aria-label="Agent manifest TOML"
          />
        </div>

        {error  && <p className="error-banner" role="alert">{error}</p>}
        {status && <p className="status-msg"  role="status">{status}</p>}

        <button
          type="submit"
          className="btn btn-primary"
          disabled={busy}
          aria-label="Deploy agent to runtime"
        >
          {busy ? 'Deploying…' : 'Deploy agent'}
        </button>
      </form>

      <style>{`
        .studio-desc {
          color: var(--color-text-muted);
          font-size: var(--text-sm);
          margin-bottom: var(--space-6);
        }
        .studio-form {
          display: flex;
          flex-direction: column;
          gap: var(--space-4);
          max-width: 640px;
        }
        .form-field { display: flex; flex-direction: column; gap: var(--space-2); }
        .form-label {
          font-size: var(--text-sm);
          font-weight: 500;
          color: var(--color-text);
        }
        .form-label-hint { color: var(--color-text-faint); font-weight: 400; }
        .form-input,
        .form-textarea {
          padding: var(--space-2) var(--space-3);
          background: var(--color-surface-2);
          border: 1px solid var(--color-border);
          border-radius: var(--radius-md);
          color: var(--color-text);
          font-size: var(--text-sm);
          font-family: inherit;
          transition: border-color var(--transition);
        }
        .form-textarea {
          font-family: 'Menlo', 'Consolas', monospace;
          font-size: var(--text-xs);
          resize: vertical;
        }
        .form-input:focus,
        .form-textarea:focus {
          outline: none;
          border-color: var(--color-primary);
        }
        .form-input:disabled,
        .form-textarea:disabled { opacity: 0.5; }
        .error-banner {
          color: var(--color-error);
          font-size: var(--text-sm);
        }
        .status-msg {
          color: var(--color-success);
          font-size: var(--text-sm);
        }
      `}</style>
    </section>
  );
}
