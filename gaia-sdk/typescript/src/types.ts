export interface Intent {
  goal: string;
}

export interface TaskHandle {
  intentId: string;
  state: string;
}

export interface MemCube {
  id: string;
  type: string;
  lifecycle: string;
  content?: string;
}

export interface AgentSpec {
  agentId: string;
  name: string;
}

export interface ResourceSpec {
  name: string;
  kind?: string;
}

export interface Signature {
  algorithm: string;
  bytes: Uint8Array;
}
