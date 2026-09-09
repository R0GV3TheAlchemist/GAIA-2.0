import { InvalidArgument, NotImplementedCapability } from "./errors.ts";
import type { AgentSpec, MemCube, ResourceSpec, Signature, TaskHandle } from "./types.ts";

export class GaiaClient {
  intent(goal: string): TaskHandle {
    if (!goal || !goal.trim()) {
      throw new InvalidArgument("goal must not be empty");
    }
    return { intentId: crypto.randomUUID(), state: "admitted" };
  }

  context(query: string): MemCube {
    if (!query || !query.trim()) {
      throw new InvalidArgument("query must not be empty");
    }
    return { id: crypto.randomUUID(), type: "plaintext", lifecycle: "active", content: query };
  }

  invoke(agent: AgentSpec): string {
    if (!agent.agentId) {
      throw new InvalidArgument("agent_id required");
    }
    return `invoked:${agent.name}`;
  }

  observe(sensor: string): string {
    if (!sensor) {
      throw new InvalidArgument("sensor required");
    }
    return `observe:${sensor}`;
  }

  sign(payload: Uint8Array): Signature {
    if (!payload.length) {
      throw new InvalidArgument("payload required");
    }
    throw new NotImplementedCapability("Ed25519 signing lands with Phase 1 (#14/#19)");
  }

  verify(payload: Uint8Array, _signature: Signature): boolean {
    if (!payload.length) {
      throw new InvalidArgument("payload required");
    }
    throw new NotImplementedCapability("Ed25519 verify lands with Phase 1 (#14/#19)");
  }

  declare(resource: ResourceSpec): string {
    if (!resource.name) {
      throw new InvalidArgument("resource name required");
    }
    return crypto.randomUUID();
  }
}
