## Plan: Add OCFL Object Versioning & Inventory Support

This plan introduces OCFL object versioning and inventory (inventory.json) support to rust-toolbox, using BDD-driven development. It covers scenario design, feature file updates, step definition changes, core implementation in `ocfl_lib`, endpoint/API adjustments, and test/validation steps, all in line with project conventions (functional Rust, anyhow error handling).

### Steps

1. Draft a BDD scenario outline for OCFL object versioning and inventory in [`ocfl_bdd/tests/`](tools/ocfl_bdd/tests/) feature files.
2. Update the relevant feature file (e.g., `object_versioning.feature`) to include scenarios for creating, updating, and retrieving OCFL object versions and inventory.
3. Identify and update/add step definitions in [`ocfl_bdd/`](tools/ocfl_bdd/) to support new BDD steps, ensuring coverage for versioning and inventory operations.
4. Implement core logic in [`ocfl_lib/src/`](tools/ocfl_lib/src/) to:
    - Create new object versions and update `inventory.json`
    - Retrieve object version history and inventory details
    - Validate OCFL versioning rules and inventory structure
5. Update or add endpoints/APIs in [`ocfl_endpoint/`](tools/ocfl_endpoint/) and CLI commands in [`ocfl_cli/`](tools/ocfl_cli/) if external access to versioning/inventory is required.
6. Write and run BDD and unit tests to validate versioning and inventory features, using `cargo check` and BDD test runner.

### Further Considerations

1. Should versioning support include both mutable and immutable object states, or only immutable (per OCFL spec)?
2. Should endpoints expose full inventory.json or only selected fields? Option A: Full, Option B: Partial, Option C: Configurable.
3. Recommend reviewing OCFL spec compliance for inventory structure and versioning edge cases.

