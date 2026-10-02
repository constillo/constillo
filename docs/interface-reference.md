# constillo interface reference

Use the [usage guide](getting-started.md) for the first steps. This reference preserves the current interface details and operational limits. Run command examples from the repository root, after preparing the exact declared dependencies and registered configuration.

## Engine-neutral input

`validate-input` accepts a local, closed Serde implementation of NERP's
`estate://contracts/workflow-input/v1` wire contract. Constillo has no Cargo
path dependency on NERP. The local schema is kept at
[`schemas/constillo.workflow-input.v1.schema.json`](../schemas/constillo.workflow-input.v1.schema.json).

Semantic validation is fail closed. It checks workflow identity and version,
the department daily report schema and JSON media type, artifact SHA-256 and
size, namespaced opaque references, classification/customer-data consistency,
the SHA-256 idempotency key, and `external_actions=false`. Runtime mode is
rejected because this build has no executor. Opaque references reject local
paths, HTTP URLs, secret locators, and credential-shaped values.

`plan-input` additionally binds the input to the selected versioned workflow,
rejects every external-action step, reuses the deterministic DAG planner, and
emits a `constillo.input-receipt/v1` document. The receipt contains only opaque
identifiers, SHA-256 digests, acceptance flags, and a non-executable plan; it
does not copy report bodies, paths, credentials, or provider data. Its
`receipt_id` and `input_digest` are deterministic for the same semantic input
and workflow plan.

The output contracts are:

- [`schemas/constillo.input-validation.v1.schema.json`](../schemas/constillo.input-validation.v1.schema.json)
- [`schemas/constillo.input-receipt.v1.schema.json`](../schemas/constillo.input-receipt.v1.schema.json)

Constillo remains plan-only: it has no execution, network, credential-storage,
provider-client, or daemon capability.

## Decision-bound planning

`plan-authorized` accepts a closed TPAR decision receipt and independently
recomputes its digest before planning. The workflow must contain the action
that records the receipt's recommendation. The emitted
[`constillo.workflow-authorization-receipt.v1`](../schemas/constillo.workflow-authorization-receipt.v1.schema.json)
binds the HAT, decision, and workflow plan digests and remains non-executable.
Changing one bound character causes planning to fail closed.

## Library use

Each boundary is usable without the CLI:

```rust
use constillo::{parse_workflow_json, plan};

let workflow = parse_workflow_json(include_str!("examples/workflow.valid.json"))?;
let proposal = plan(&workflow).map_err(|report| format!("{:?}", report.diagnostics))?;
assert!(!proposal.executable);
# Ok::<(), String>(())
```

`parse_workflow_json` and `parse_workflow_input_json` enforce a 1 MiB input
boundary and closed nested objects. Workflow and dependency collections have
explicit limits. The CLI additionally refuses symlinks and special files.
This local package is not published while execution remains a separate
boundary.
