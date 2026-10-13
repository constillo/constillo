# constillo

Check whether a workflow is valid and turn it into a repeatable execution plan before handing it to an executor.

## What you can do

- Validate workflow declarations and input bindings.
- Inspect an ordered plan and stable validation results.

## Current scope

Planning is deterministic. Executing the plan belongs to the application that consumes it.

Package distribution is not activated by this documentation. Use the checked-in source and the declared dependency versions; published availability must be verified separately.

## Getting started

Install Rust 1.97 or newer and make the declared dependencies available. Resolve declared third-party dependencies from public registries and source-only managed components from their fixed public Git revision. No private registry or adjacent checkout is part of the supported OSS source path. Run from this repository:

```sh
cargo test --locked
```

## Documentation and source

[Interface reference](docs/interface-reference.md)

[Usage guide](docs/getting-started.md)

[Examples](examples) · [Schemas](schemas) · [Implementation and public interfaces](src) · [Verification cases](tests) · [Contributing](CONTRIBUTING.md) · [Security reporting](SECURITY.md) · [License](LICENSE) · [Attribution notices](NOTICE)

## CRM condition adapter

[adapters/crm](adapters/crm) owns the migrated scalar condition planner.
`node adapters/crm/cli.mjs` accepts `constillo.crm-condition/v1` on stdin and emits
a non-executable decision receipt. NERP executes business effects. The migrated
component retains its MIT attribution in `adapters/crm/LICENSE.haytai`.
Run `node --test adapters/crm/condition.test.mjs` for its boundary tests.

[OSS responsibility and source completion](docs/oss-source-contract.md) defines the supported package boundary and required acceptance. It does not declare unexecuted source, distribution or production checks complete.
