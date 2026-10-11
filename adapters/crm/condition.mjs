// CRM condition evaluation migrated from Haytai (MIT); see LICENSE.haytai.
const operators = new Set(['equals', 'not_equals', 'contains']);
const forbiddenKeys = new Set(['__proto__', 'prototype', 'constructor']);

export function evaluateCondition(context, config) {
  if (!context || typeof context !== 'object' || Array.isArray(context)) {
    throw new Error('context must be an object');
  }
  if (!config || typeof config !== 'object' || Array.isArray(config)) {
    throw new Error('condition must be an object');
  }
  const operator = config.operator ?? 'equals';
  if (!operators.has(operator)) throw new Error('Unsupported condition operator');
  if (typeof config.field !== 'string' || !config.field.length) {
    throw new Error('condition.field is required');
  }
  const keys = config.field.split('.');
  if (keys.some(key => !key || forbiddenKeys.has(key))) throw new Error('Invalid field path');
  const actual = keys.reduce((value, key) => (
    value && typeof value === 'object' && Object.hasOwn(value, key) ? value[key] : undefined
  ), context);
  if (operator === 'contains') {
    return typeof actual === 'string' && typeof config.value === 'string' && actual.includes(config.value);
  }
  // Wire equality is defined for scalars; object identity cannot cross a JSON boundary.
  if (actual !== null && typeof actual === 'object' || config.value !== null && typeof config.value === 'object') {
    throw new Error('equals/not_equals require scalar values');
  }
  return operator === 'not_equals' ? actual !== config.value : actual === config.value;
}

export function planCondition(input) {
  if (input.schema_version !== 'constillo.crm-condition/v1') throw new Error('Unsupported schema');
  return {
    schema_version: 'constillo.crm-condition-result/v1',
    mode: 'planning-only',
    executable: false,
    matches: evaluateCondition(input.context, input.condition),
  };
}
