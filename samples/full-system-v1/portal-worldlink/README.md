# Full System V1 PortalWorldLink Samples

This root carries the `P-FSV1-02` source-first PortalWorldLink lane.

- `shared/src/*.mir` carries the bounded source support modules for portal resolve, fallback, and admission rows.
- `expected/manifest.json` stores the generated package-manifest subset derived from source.
- `expected/run.json` stores the bounded runtime report subset derived from source execution.
- These rows prove bounded local portal resolve / handoff / admit / fallback evidence only.

Non-claims:

- no WAN federation
- no continuous spatial sync
- no final portal ABI
