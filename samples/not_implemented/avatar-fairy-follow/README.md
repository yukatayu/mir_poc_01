# avatar-fairy-follow residual planned sample

This directory keeps the residual planned phase 8 sample family.

- `phase 8` は legacy sample-family label です。
- current macro-phase reading は `Macro 6 reserve` です。

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
| `FAIRY-05` | `05_follow_target_reacquired_after_return.mir` | reacquire remote target after return | explicit state timeline / anchor switch evidence; witness bundling unresolved |

Promoted active canaries already live under `samples/clean-near-end/avatar-follow/`:

- `FAIRY-01`
- `FAIRY-02`
- `FAIRY-03`
- `FAIRY-04`
- `FAIRY-06`

## Planned helper-local gate before promotion

If `FAIRY-05` is ever promoted into the active helper, the current repo-local
minimum is only that explicit state timeline / anchor switch evidence exists.

- `UNRESOLVED`: whether visibility-return witness is carried as a timeline
  event, anchor-switch event, witness event, or typed bundle
- `UNRESOLVED`: helper-local CLI/debug surface exact naming
- current working assumption:
  candidate labels `state_timeline` / `anchor_switch` are acceptable as
  planning-only names

This is a planning-only helper-local carrier. It is not the final public
visualization protocol or avatar runtime API.

## Residual promotion rule

This residual family should not be marked active until all of the following exist:

- a chosen helper command surface
- at least one positive runnable sample
- at least one negative/rejection runnable sample
  for missing return witness or stale membership evidence
- a named debug/visualization output surface
  that makes state timeline / anchor-switch evidence readable
- report-backed validation evidence for the widened scope

Until then, these residual samples stay under `samples/not_implemented/`.
