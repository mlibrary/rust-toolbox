
# OCFL Rust Toolbox: Project Plan (Updated June 2024)

## Overview

This project implements a minimal but functional OCFL (Oxford Common File Layout) v1.1 repository in Rust, with a focus on correctness, testability, and extensibility. The project consists of:

- **ocfl_lib**: Core OCFL v1.1 library (object management, inventory, versioning)
- **ocfl_endpoint**: HTTP API server for OCFL operations (Axum-based)
- **ocfl_cli**: Command-line client for interacting with the endpoint
- **ocfl_bdd**: BDD (Cucumber) integration tests for end-to-end validation

---

## Current State

### 1. ocfl_lib

- Implements the OCFL v1.1 core repository operations:
  - Initialize repository
  - Add object (v1)
  - Get object (extract content)
  - List objects
  - Add new version to object
  - Retrieve inventory.json as JSON
  - List versions for an object
  - Delete object
  - Delete specific version of an object
- Uses `anyhow` for error handling, `serde`/`serde_json` for inventory, and `sha2` for digests.
- Test coverage: Core trait and implementation are covered by unit tests and BDD tests.

### 2. ocfl_endpoint

- HTTP API exposing OCFL operations:
  - `/init` (POST): Initialize repository
  - `/add` (POST): Add object
  - `/add_version` (POST): Add new version to object
  - `/list` (GET): List objects
  - `/get` (GET): Extract object content
  - `/inventory` (GET): Get inventory.json for object
  - `/versions` (GET): List versions for object
  - `/delete_object` (POST): Delete object
  - `/delete_version` (POST): Delete version of object
- Uses Axum, async, and shares state via Arc.
- All endpoints are exercised by BDD tests.

### 3. ocfl_cli

- Command-line client for basic OCFL endpoint operations:
  - `init`
  - `add <object_id> <src_path>`
  - `list`
- Uses `clap` for argument parsing and `reqwest` for HTTP.
- Can be extended to support more endpoints.

### 4. ocfl_bdd

- Cucumber-based BDD test suite.
- Covers all major OCFL operations via HTTP API.
- Steps implemented for setup, actions, and assertions (including file system checks).
- Validates OCFL 1.1 compliance for inventory structure and versioning.

---

## Features Implemented

- OCFL v1.1 repository and object structure
- Inventory management (JSON, digests, versioning)
- Versioned content storage and retrieval
- HTTP API for all major OCFL operations
- CLI for basic operations
- End-to-end BDD tests (integration + file system checks)
- Error handling and atomic writes for inventory

---

## Features NOT Implemented / Limitations

- No support for advanced OCFL features (storage roots, extensions, fixity, sidecar files, etc.)
- No user authentication or authorization
- No concurrent/multi-user safety (single-process, no locking)
- No preservation of file metadata (mtime, permissions, etc.)
- No pluggable digest algorithms (sha512 only)
- No object-level fixity or sidecar files
- No advanced CLI features (e.g., version management, inventory validation)
- No web UI

---

## Next Steps / TODO

- Add support for more OCFL features (fixity, extensions, etc.)
- Improve error reporting and API responses
- Add more CLI commands (delete, get, version management)
- Add more comprehensive unit and property-based tests
- Consider concurrency/multi-user safety
- Documentation and usage examples

---

## How to Build & Test

```sh
# Build all tools
cargo build --workspace

# Run unit tests
cargo test -p ocfl_lib

# Run BDD/integration tests
cargo test -p ocfl_bdd

# Run endpoint server
cargo run -p ocfl_endpoint

# Use CLI
cargo run -p ocfl_cli -- add obj1 /path/to/file
```

---

## Status

- **Core library**: Complete for basic OCFL 1.1 operations
- **HTTP API**: Complete for basic operations, covered by BDD tests
- **CLI**: Basic, functional, extensible
- **Tests**: Good coverage (unit + BDD)
- **Ready for experimentation, not for production**

---

*This plan reflects the current state as of June 2024. See code and tests for details.*
