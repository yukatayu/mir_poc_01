# avatar-fairy-follow planned samples

This directory is a planned phase 8 sample family.

- It is not an active runnable sample family.
- It is not parsed by the current clean near-end runner.
- It exists to keep the future representative slice explicit without over-claiming implementation.

## Historical anchor

The historical prototype anchor is:

- `samples/prototype/current-l2-dynamic-attach-detach/p03-avatar-controller-attach-detach.txt`

That prototype is useful as planning evidence, but it is not the current active sample surface.

## Planned sample IDs

| ID | File | Goal | Expected future view |
|---|---|---|---|
| `FAIRY-01` | `01_follow_remote_head_with_local_fallback.mir` | follow remote head when visible | follow summary / anchor graph |
| `FAIRY-02` | `02_remote_head_not_visible_falls_back_to_local.mir` | visibility loss causes fallback | fallback reason / visibility |
| `FAIRY-03` | `03_remote_avatar_leaves_falls_back_to_local.mir` | leave causes stale-anchor reject and fallback | membership / fallback |
| `FAIRY-04` | `04_invalid_cross_anchor_chain_rejected.mir` | reject invalid cross-anchor chain | static rejection / lineage mismatch |
| `FAIRY-05` | `05_follow_target_reacquired_after_return.mir` | reacquire remote target after return | state timeline / anchor switch |
| `FAIRY-06` | `06_model_check_no_detached_anchor_observed.mir` | verification sample for detached-anchor safety | verification view |

## Promotion rule

This family should not be marked active until all of the following exist:

- a chosen helper command surface
- at least one positive runnable sample
- at least one negative/rejection runnable sample
- a named debug/visualization output surface
- report-backed validation evidence

Until then, this directory stays under `samples/not_implemented/`.
