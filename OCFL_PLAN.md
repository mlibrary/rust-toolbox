# OCFL Implementation Plan

This file tracks the step-by-step plan and progress for implementing an Oxford Common File Layout ([OCFL](https://ocfl.io/)) library, endpoint, and CLI tool in this project.

## Steps
1. Design OCFL library API (functional, trait-based, anyhow error handling)
2. Implement OCFL library core logic and unit tests
3. Create HTTP endpoint crate using the OCFL library (RESTful, anyhow for errors)
4. Create CLI tool crate that interacts with the endpoint (clap, HTTP client)
5. Integrate error handling and user-friendly messages
6. Ensure functional style, minimal 'unsafe', and aarch64-apple-darwin compatibility
7. Run `cargo check` after every file change
8. Run `cargo test` (unit tests) after every change — fast, no network or I/O
9. Run `cargo test -p ocfl_bdd` (BDD integration tests) after a feature is complete
10. Maintain test coverage above 80% for all crates (unit + integration)

---

Progress and notes will be updated as work proceeds.
