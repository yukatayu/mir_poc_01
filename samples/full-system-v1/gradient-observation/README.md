# Full System V1 GradientObservation Samples

This root carries the `P-FSV1-02` source-first GradientObservation lane.

- `shared/src/*.mir` carries the bounded source support modules for overlap view, handoff hint, and observer-visible reject-event rows.
- `expected/manifest.json` stores the generated package-manifest subset derived from source.
- `expected/run.json` stores the bounded runtime report subset derived from source execution.
- These rows prove bounded local observer-only gradient evidence plus freshness-contract rejection only.

Non-claims:

- no write authority in gradient zones
- no runtime-enforced write-authority admission proof in this package
- no continuous spatial sync
- no WAN federation
