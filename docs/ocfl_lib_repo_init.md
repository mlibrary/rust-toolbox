# OCFL Repository Initialization

This document describes the implementation of OCFL v1.1 repository initialization in the core library.

## OCFL 1.1 Requirements
- The repository root must contain a file named `0=ocfl_1.1` with the OCFL version and spec URL.
- The root directory must not already exist.
- The implementation must create the directory and write the required file.

## Implementation Notes
- Uses `std::fs::create_dir_all` to create the directory.
- Uses `std::fs::write` to create the `0=ocfl_1.1` file.
- All errors are wrapped with `anyhow::Context` for clarity.
- No 'unsafe' code is used.

## Example
```rust
let repo = OcflRepoImpl::new("/tmp/ocfl");
repo.init_repo("/tmp/ocfl")?;
```

## Next Steps
- Implement add_object, get_object, and list_objects.
- Add more validation and OCFL structure as required by the spec.

