import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import AgentMatrix from './AgentMatrix';

vi.mock('../api/gateway', () => ({
  listAgents: vi.fn().mockResolvedValue([]),
  revokeAgent: vi.fn().mockResolvedValue(undefined),
}));

beforeEach(() => { vi.clearAllMocks(); });

describe('AgentMatrix', () => {
  it('renders the heading', async () => {
    render(<AgentMatrix />);
    expect(screen.getByRole('heading', { name: /agents/i })).toBeTruthy();
  });

  it('shows empty state when no agents are running', async () => {
    render(<AgentMatrix />);
    expect(await screen.findByText(/no running agents/i)).toBeTruthy();
  });
});
