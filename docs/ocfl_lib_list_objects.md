# OCFL List Objects Design

This document describes the design and implementation of the `list_objects` operation for the OCFL v1.1 core library.

## OCFL 1.1 Requirements
- List all OCFL object IDs present in the repository.
- An object is identified by a directory under the repo root containing a `0=ocfl_object_1.1` file.
- The implementation must return a list of all such object IDs (directory names).
- All errors must be handled with context.

## Implementation Plan
- Validate that the repository root exists and is a valid OCFL repo.
- Iterate over all directories in the repo root.
- For each directory, check for the presence of `0=ocfl_object_1.1`.
- Collect and return the directory names as object IDs.
- Return errors with context using anyhow.

## Next Steps
- Implement the `list_objects` method in the library.
- Add tests for object listing.
- Document edge cases and error handling.

