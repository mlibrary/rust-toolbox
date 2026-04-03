# OCFL Add Object Design

This document describes the design and implementation of the `add_object` operation for the OCFL v1.1 core library.

## OCFL 1.1 Requirements
- Each object must have a unique object root directory under the repository root.
- The object root must contain a file named `0=ocfl_object_1.1`.
- The object directory must contain a `content` directory for the object's files.
- The implementation must copy the source file(s) into the object's content directory.
- The implementation must not overwrite existing objects.
- All errors must be handled with context.

## Implementation Plan
- Validate that the repository root exists and is a valid OCFL repo.
- Validate that the object does not already exist.
- Create the object root and required files.
- Copy the source file(s) into the content directory.
- Return errors with context using anyhow.

## Next Steps
- Implement the `add_object` method in the library.
- Add tests for object addition.
- Document edge cases and error handling.

