# Using constillo

Check whether a workflow is valid and turn it into a repeatable execution plan before handing it to an executor.

## Before you start

Planning is deterministic. Executing the plan belongs to the application that consumes it.

## First steps

Run from the repository root:

```sh
cargo test --locked
```

## How to assess the result

- Validate workflow declarations and input bindings.
- Inspect an ordered plan and stable validation results.

A passing source-level check establishes only what that check observes. Keep missing configuration, unavailable services and unverified deployment paths visible.

## Continue reading

[Repository overview](../README.md)
