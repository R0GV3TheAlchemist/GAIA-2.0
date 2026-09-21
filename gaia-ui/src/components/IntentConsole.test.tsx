import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import IntentConsole from './IntentConsole';

// Mock gateway module so tests don't hit the network
vi.mock('../api/gateway', () => ({
  submitIntent: vi.fn().mockResolvedValue({ id: 'test-id', status: 'queued' }),
  openIntentStream: vi.fn().mockImplementation((onChunk: (c: string) => void) => {
    onChunk('[stub] intent stream not yet wired to orchestrator');
    return () => {};
  }),
}));

beforeEach(() => { vi.clearAllMocks(); });

describe('IntentConsole', () => {
  it('renders the heading and empty trace state', () => {
    render(<IntentConsole />);
    expect(screen.getByRole('heading', { name: /intent console/i })).toBeTruthy();
    expect(screen.getByText(/submit an intent/i)).toBeTruthy();
  });

  it('submit button is disabled when input is empty', () => {
    render(<IntentConsole />);
    const btn = screen.getByRole('button', { name: /submit intent/i }) as HTMLButtonElement;
    expect(btn.disabled).toBe(true);
  });

  it('shows a trace entry after submitting an intent', async () => {
    const user = userEvent.setup();
    render(<IntentConsole />);
    const input = screen.getByPlaceholderText(/describe your intent/i);
    await user.type(input, 'test intent');
    await user.click(screen.getByRole('button', { name: /submit intent/i }));
    expect(await screen.findByText(/▶ test intent/)).toBeTruthy();
    expect(await screen.findByText(/queued.*test-id/i)).toBeTruthy();
  });
});
