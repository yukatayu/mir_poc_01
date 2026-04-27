# avatar-fairy-follow residual planned sample

This directory keeps the residual planned phase 8 sample family.

- It is not the active runnable sample family.
- It is not parsed by the current clean near-end runner.
- It exists to keep the remaining future widening explicit without over-claiming implementation.

The current active representative slice is:

- `samples/clean-near-end/avatar-follow/`
- `python3 scripts/avatar_follow_samples.py ...`

## Historical anchor

The historical prototype anchor is:

- `samples/prototype/current-l2-dynamic-attach-detach/p03-avatar-controller-attach-detach.txt`

That prototype is useful as planning evidence, but it is not the current active sample surface.

## Residual planned sample IDs

| ID | File | Goal | Expected future view |
|---|---|---|---|
| `FAIRY-05` | `05_follow_target_reacquired_after_return.mir` | reacquire remote target after return | state timeline / anchor switch |

Promoted active canaries already live under `samples/clean-near-end/avatar-follow/`:

- `FAIRY-01`
- `FAIRY-02`
- `FAIRY-03`
- `FAIRY-04`
- `FAIRY-06`

## Residual promotion rule

This residual family should not be marked active until all of the following exist:

- a chosen helper command surface
- at least one positive runnable sample
- at least one negative/rejection runnable sample
- a named debug/visualization output surface
- report-backed validation evidence for the widened scope

Until then, these residual samples stay under `samples/not_implemented/`.
