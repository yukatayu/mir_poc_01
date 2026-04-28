# avatar-follow representative slice

This directory is the active clean near-end representative slice for legacy phase 8 sample-family label `avatar fairy follow / fallback anchor`, current `Macro 6 reserve`.

It demonstrates:

- remote-head follow with explicit local fallback lineage
- visibility-loss-only fallback without claiming transport recovery
- leave-triggered stale-anchor rejection
- invalid cross-anchor lineage rejection without hidden repair
- detached-anchor safety as a model-check-oriented canary

This is a helper-local runnable slice. It is not final public runtime API, final public visualization protocol, or a production avatar stack.

## How to run

```bash
python3 scripts/avatar_follow_samples.py list
python3 scripts/avatar_follow_samples.py check-all --format json
python3 scripts/avatar_follow_samples.py closeout --format json
```

Useful focused runs:

```bash
python3 scripts/avatar_follow_samples.py run 01_follow_remote_head_with_local_fallback --debug anchors --format json
python3 scripts/avatar_follow_samples.py run 02_remote_head_not_visible_falls_back_to_local --debug anchors --format json
python3 scripts/avatar_follow_samples.py run 03_remote_avatar_leaves_falls_back_to_local --debug membership --format json
python3 scripts/avatar_follow_samples.py run 04_invalid_cross_anchor_chain_rejected --format json
python3 scripts/avatar_follow_samples.py run 06_model_check_no_detached_anchor_observed --debug verification --format json
```

## Sample matrix

| ID | Sample | Main point | Preferred debug | Expected outcome |
|---|---|---|---|---|
| `FAIRY-01` | `01_follow_remote_head_with_local_fallback.mir` | visible remote head follow with explicit fallback lineage | `anchors` | success |
| `FAIRY-02` | `02_remote_head_not_visible_falls_back_to_local.mir` | visibility loss falls back locally without claiming transport recovery | `anchors` | fallback on visibility loss |
| `FAIRY-03` | `03_remote_avatar_leaves_falls_back_to_local.mir` | leave invalidates stale anchor and falls back locally | `membership` | fallback after reject |
| `FAIRY-04` | `04_invalid_cross_anchor_chain_rejected.mir` | invalid cross-anchor chain is rejected without hidden repair | `summary` | rejection |
| `FAIRY-06` | `06_model_check_no_detached_anchor_observed.mir` | detached anchor safety canary | `verification` | model-check pass |

`FAIRY-05` remains planned under `samples/not_implemented/avatar-fairy-follow/`.
Current helper closeout only exposes the planned path inventory and `fairy05_reopen_gate`; it does not promote `FAIRY-05` to runnable status.

## Debug surfaces

- `summary`
  - short human-readable result summary.
- `anchors`
  - attached/fallback/rejected anchor inventory and lineage edges.
- `membership`
  - membership-epoch and leave/fallback timeline.
- `verification`
  - model-check property and verification log.

These outputs are helper-local evidence surfaces. They are not the final public visualization or transport contract.
