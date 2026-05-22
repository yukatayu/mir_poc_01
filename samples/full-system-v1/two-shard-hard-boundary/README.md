# Full System V1 TwoShardHardBoundary Samples

This root carries the `P-FSV1-02` source-first TwoShardHardBoundary lane.

- `shared/src/*.mir` carries the bounded source support modules for shard offer, commit, and rejection rows.
- `expected/manifest.json` stores the generated package-manifest subset derived from source.
- `expected/run.json` stores the bounded runtime report subset derived from source execution.
- These rows prove bounded local two-shard hard-boundary evidence only.

Non-claims:

- no WAN federation
- no continuous infinite shard sync
- no distributed durable save/load
