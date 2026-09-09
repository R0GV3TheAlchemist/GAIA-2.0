import assert from "node:assert/strict";
import { test } from "node:test";
import { GaiaClient } from "./client.ts";
import { InvalidArgument, NotImplementedCapability } from "./errors.ts";

test("intent admits", () => {
  const handle = new GaiaClient().intent("hello");
  assert.equal(handle.state, "admitted");
  assert.ok(handle.intentId);
});

test("intent rejects empty", () => {
  assert.throws(() => new GaiaClient().intent("  "), InvalidArgument);
});

test("sign is explicitly unimplemented", () => {
  assert.throws(() => new GaiaClient().sign(new Uint8Array([1])), NotImplementedCapability);
});
