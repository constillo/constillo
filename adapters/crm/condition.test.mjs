import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import test from 'node:test';
import { evaluateCondition, planCondition } from './condition.mjs';

test('CRM scalar conditions retain equals, not_equals and contains semantics', () => {
  const context = { message: { subject: 'invoice ready', count: 2 } };
  assert.equal(evaluateCondition(context, { field: 'message.count', value: 2 }), true);
  assert.equal(evaluateCondition(context, { field: 'message.count', operator: 'not_equals', value: 3 }), true);
  assert.equal(evaluateCondition(context, { field: 'message.subject', operator: 'contains', value: 'invoice' }), true);
  assert.equal(evaluateCondition(context, { field: 'message.count', operator: 'contains', value: '2' }), false);
});

test('prototype traversal and unknown operators fail closed', () => {
  for (const field of ['__proto__.x', 'constructor.name', 'message..subject']) {
    assert.throws(() => evaluateCondition({}, { field, value: 'x' }));
  }
  assert.throws(() => evaluateCondition({}, { field: 'x', operator: 'execute', value: 'x' }));
  assert.throws(() => evaluateCondition({ x: {} }, { field: 'x', value: {} }));
});

test('wire receipt is non-executable and rejects other schema versions', () => {
  const receipt = planCondition({ schema_version: 'constillo.crm-condition/v1', context: { x: false }, condition: { field: 'x', value: false } });
  assert.equal(receipt.matches, true);
  assert.equal(receipt.executable, false);
  assert.throws(() => planCondition({ schema_version: 'v0' }));
});

test('CLI uses stdin and does not echo rejected private input', () => {
  const cli = fileURLToPath(new URL('./cli.mjs', import.meta.url));
  const input = { schema_version: 'constillo.crm-condition/v1', context: { x: 1 }, condition: { field: 'x', value: 1 } };
  const valid = spawnSync(process.execPath, [cli], { input: JSON.stringify(input), encoding: 'utf8' });
  assert.equal(valid.status, 0);
  assert.equal(JSON.parse(valid.stdout).matches, true);
  const invalid = spawnSync(process.execPath, [cli], { input: 'private-marker-not-json', encoding: 'utf8' });
  assert.notEqual(invalid.status, 0);
  assert.equal(invalid.stdout, '');
  assert.equal(invalid.stderr.includes('private-marker'), false);
  const oversized = spawnSync(process.execPath, [cli], { input: JSON.stringify(input), encoding: 'utf8', env: { ...process.env, CONSTILLO_CRM_MAX_INPUT_BYTES: '2' } });
  assert.notEqual(oversized.status, 0);
});
