# OCFL Library Design

This document describes the design and implementation details of the OCFL v1.1 core library. It will be updated as the implementation progresses.

## Overview
- Implements the OCFL v1.1 specification (https://ocfl.io/1.1/spec/)
- Functional, trait-based API
- Error handling via anyhow
- No community extensions (core only)

## Structure
- `OcflRepo` trait: defines repository operations
- `OcflRepoImpl`: default implementation
- All operations return `anyhow::Result`

## Implementation Plan
- [ ] Repository initialization
- [ ] Add object
- [ ] Get object
- [ ] List objects

## Decisions
- All file system operations use std::fs and std::path
- All errors are wrapped with anyhow for context
- No 'unsafe' code unless absolutely necessary

---

Further sections will be added as implementation proceeds.

