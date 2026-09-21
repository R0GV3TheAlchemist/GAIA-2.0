import { useState, useEffect, useRef, useCallback } from 'react';
import { submitIntent, openIntentStream } from '../api/gateway';

interface TraceEntry {
  id: string;
  text: string;
  ts: number;
}

export default function IntentConsole() {
  const [input, setInput]         = useState('');
  const [traces, setTraces]       = useState<TraceEntry[]>([]);
  const [streaming, setStreaming] = useState(false);
  const [error, setError]         = useState<string | null>(null);
  const traceEndRef               = useRef<HTMLDivElement>(null);
  const cleanupRef                = useRef<(() => void) | null>(null);

  // Auto-scroll to latest trace entry
  useEffect(() => {
    traceEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [traces]);

  // Close SSE on unmount
  useEffect(() => () => { cleanupRef.current?.(); }, []);

  const addTrace = useCallback((text: string) => {
    setTraces((prev) => [
      ...prev,
      { id: `${Date.now()}-${Math.random()}`, text, ts: Date.now() },
    ]);
  }, []);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const text = input.trim();
    if (!text) return;

    setError(null);
    setInput('');
    addTrace(`▶ ${text}`);

    try {
      const resp = await submitIntent({ text });
      addTrace(`  ↳ queued  id=${resp.id}  status=${resp.status}`);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Submit failed');
      return;
    }

    // Open SSE stream for live results
    cleanupRef.current?.();
    setStreaming(true);
    cleanupRef.current = openIntentStream(
      (chunk) => addTrace(`  ${chunk}`),
      (err) => {
        setStreaming(false);
        if (err instanceof ErrorEvent && err.message) {
          setError(err.message);
        }
      },
    );
  };

  const handleStopStream = () => {
    cleanupRef.current?.();
    cleanupRef.current = null;
    setStreaming(false);
    addTrace('  ↳ stream closed');
  };

  return (
    <section aria-labelledby="console-heading">
      <h1 id="console-heading" className="section-title">Intent Console</h1>

      {/* Trace output */}
      <div
        className="card trace-output"
        role="log"
        aria-live="polite"
        aria-label="Intent trace output"
      >
        {traces.length === 0 ? (
          <p className="trace-empty">Submit an intent to see live traces here.</p>
        ) : (
          traces.map((t) => (
            <div key={t.id} className="trace-line">
              <span className="trace-ts" aria-hidden="true">
                {new Date(t.ts).toLocaleTimeString()}
              </span>
              <span className="trace-text">{t.text}</span>
            </div>
          ))
        )}
        <div ref={traceEndRef} aria-hidden="true" />
      </div>

      {/* Error banner */}
      {error && (
        <p className="error-banner" role="alert">{error}</p>
      )}

      {/* Input form */}
      <form className="intent-form" onSubmit={handleSubmit}>
        <label htmlFor="intent-input" className="sr-only">Intent text</label>
        <input
          id="intent-input"
          type="text"
          className="intent-input"
          placeholder="Describe your intent…"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          disabled={streaming}
          autoComplete="off"
          spellCheck={false}
        />
        <button
          type="submit"
          className="btn btn-primary"
          disabled={streaming || !input.trim()}
          aria-label="Submit intent"
        >
          Submit
        </button>
        {streaming && (
          <button
            type="button"
            className="btn btn-ghost"
            onClick={handleStopStream}
            aria-label="Stop intent stream"
          >
            Stop
          </button>
        )}
      </form>

      <style>{`
        .trace-output {
          font-family: 'Menlo', 'Consolas', 'Cascadia Code', monospace;
          font-size: var(--text-xs);
          min-height: 280px;
          max-height: 480px;
          overflow-y: auto;
          margin-bottom: var(--space-4);
          display: flex;
          flex-direction: column;
          gap: var(--space-1);
        }
        .trace-empty { color: var(--color-text-faint); font-style: italic; }
        .trace-line  { display: flex; gap: var(--space-3); }
        .trace-ts    { color: var(--color-text-faint); flex-shrink: 0; width: 7ch; }
        .trace-text  { color: var(--color-text); word-break: break-all; }
        .error-banner {
          color: var(--color-error);
          font-size: var(--text-sm);
          margin-bottom: var(--space-3);
        }
        .intent-form {
          display: flex;
          gap: var(--space-3);
          align-items: center;
        }
        .intent-input {
          flex: 1;
          padding: var(--space-2) var(--space-3);
          background: var(--color-surface-2);
          border: 1px solid var(--color-border);
          border-radius: var(--radius-md);
          color: var(--color-text);
          font-size: var(--text-sm);
          font-family: inherit;
          transition: border-color var(--transition);
        }
        .intent-input:focus {
          outline: none;
          border-color: var(--color-primary);
        }
        .intent-input:disabled { opacity: 0.5; }
        .sr-only {
          position: absolute; width: 1px; height: 1px; padding: 0;
          margin: -1px; overflow: hidden; clip: rect(0,0,0,0);
          white-space: nowrap; border-width: 0;
        }
      `}</style>
    </section>
  );
}
