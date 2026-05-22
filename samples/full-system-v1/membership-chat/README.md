# Full System V1 MembershipChat Samples

This root carries the `P-FSV1-01` source-first MembershipChat lane.

- `shared/src/world-core-base.mir` and `shared/src/membership-chat-base.mir` provide the import chain from WorldCore to MembershipChat.
- `expected/manifest.json` stores the generated package-manifest subset derived from source.
- `expected/run.json` stores the bounded runtime report subset derived from source execution.
- These rows prove bounded room message and stale-membership rejection evidence only.

Non-claims:

- no final room/auth grammar
- no transport-coupled chat runtime
- no final rate-limit execution policy
