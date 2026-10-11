import { planCondition } from './condition.mjs';

const limit = Number(process.env.CONSTILLO_CRM_MAX_INPUT_BYTES ?? 1048576);
if (!Number.isSafeInteger(limit) || limit <= 0) throw new Error('Invalid input limit');
let size = 0;
const chunks = [];
try {
  for await (const chunk of process.stdin) {
    size += chunk.length;
    if (size > limit) throw new Error('Input exceeds configured limit');
    chunks.push(chunk);
  }
  process.stdout.write(JSON.stringify(planCondition(JSON.parse(Buffer.concat(chunks).toString('utf8')))) + '\n');
} catch {
  // Invalid input can include private CRM data. Do not echo it or its parser error.
  process.stderr.write('CRM condition input rejected\n');
  process.exitCode = 1;
}
