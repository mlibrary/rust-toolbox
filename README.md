# rust-toolbox
RustRover Agent Workspace

## OCFL Endpoint Core Feature Status (as of April 2026)

All core OCFL v1.1 repository and object management features are implemented and exposed via the HTTP API:
- Initialize repository
- Add object (v1)
- Add new version to object
- List objects
- Get object (extract content)
- Retrieve inventory.json as JSON
- List versions for an object
- Delete object
- Delete specific version of an object

Missing features are advanced/optional per the OCFL spec or relate to production-readiness (e.g., extensions, fixity, metadata preservation, authentication, advanced error reporting, concurrency, web UI). No further core features are required for a minimal OCFL v1.1 endpoint.
