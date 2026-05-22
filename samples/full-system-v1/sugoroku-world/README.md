# Full System V1 SugorokuWorld Samples

This root carries the `P-FSV1-01` source-first SugorokuWorld lane.

- `shared/src/world-core-base.mir`, `shared/src/membership-chat-base.mir`, and `shared/src/sugoroku-base.mir` provide the import chain from WorldCore through MembershipChat to SugorokuWorld.
- `expected/manifest.json` stores the generated package-manifest subset derived from source.
- `expected/run.json` stores the bounded runtime report subset derived from source execution.
- These rows prove bounded roll/publish/witness/handoff/quiescent-cut evidence only.

Non-claims:

- no final world/game runtime
- no distributed save/load
- no final portal/shard semantics here
