# plan/24 — avatar follow representative slice roadmap

## purpose

legacy phase 8 sample-family label `avatar fairy follow / fallback anchor` を、historical prototype と docs-first skeleton だけでなく、
repo-local helper canary として追える current repository memory を置く。

current macro-phase reading は `Macro 6 reserve` である。

## current status

- active representative slice は `samples/clean-near-end/avatar-follow/`
- active helper surface は `scripts/avatar_follow_samples.py`
- active sample IDs は `FAIRY-01`, `FAIRY-02`, `FAIRY-03`, `FAIRY-04`, `FAIRY-06`
- residual planned IDs は `FAIRY-05`
- historical prototype anchor は `samples/prototype/current-l2-dynamic-attach-detach/p03-avatar-controller-attach-detach.*`

## active representative slice

| ID | role | current meaning |
|---|---|---|
| `FAIRY-01` | positive | visible remote head follow with explicit local fallback lineage |
| `FAIRY-02` | positive-with-fallback | visibility-loss-only fallback stays local without transport recovery claim |
| `FAIRY-03` | negative-with-fallback | leave invalidates stale anchor, rejects it, and falls back locally |
| `FAIRY-04` | rejection | invalid cross-anchor lineage is rejected without hidden repair |
| `FAIRY-06` | verification | detached anchor must not remain observable |

## helper surface

- `list`
- `run <sample>`
- `check-all`
- `closeout`

debug modes are helper-local evidence views only:

- `summary`
- `anchors`
- `membership`
- `verification`

These are not the final public visualization or runtime API.

## planned residual family

The following remain planned under `samples/not_implemented/avatar-fairy-follow/`:

- `FAIRY-05`
  reacquire-after-return

They stay planned until the reopen gate below is justified for the active helper.

## unresolved carrier choice for `FAIRY-05`

If `FAIRY-05` is ever promoted into `scripts/avatar_follow_samples.py`, the
current repo-local gate is only that explicit state timeline / anchor switch
evidence must exist.

- `UNRESOLVED`: whether visibility-return witness is carried as a timeline
  event, anchor-switch event, witness event, or typed bundle
- `UNRESOLVED`: helper-local CLI/debug surface exact naming
- current working assumption:
  candidate labels `state_timeline` / `anchor_switch` are acceptable as
  planning-only names

These names are planning-only helper-local carrier labels. They are not final
public visualization or runtime API names.

## reopen gate for runnable widening

Do not reopen `FAIRY-05` as runnable widening unless all of the following are
worth carrying in the same package:

- one positive reacquire-after-return sample
- one negative/rejection companion for missing return witness or stale
  membership evidence
- helper-local evidence that shows the switch order explicitly
- docs / report / snapshot sync that keeps active / planned / mixed-gate
  boundaries readable

## validation floor

- `python3 -m unittest scripts.tests.test_avatar_follow_samples`
- `python3 scripts/avatar_follow_samples.py check-all --format json`
- `python3 scripts/avatar_follow_samples.py run 01_follow_remote_head_with_local_fallback --debug anchors --format json`
- `python3 scripts/avatar_follow_samples.py run 02_remote_head_not_visible_falls_back_to_local --debug anchors --format json`
- `python3 scripts/avatar_follow_samples.py run 03_remote_avatar_leaves_falls_back_to_local --debug membership --format json`
- `python3 scripts/avatar_follow_samples.py run 06_model_check_no_detached_anchor_observed --debug verification --format json`

## current stop line

Do not claim:

- final public avatar runtime API
- final visualization protocol
- real transport / session / auth surface
- hot-plug / `AttachPoint` implementation
- production game / world / engine adapter

This package is a repo-local representative slice only.
`FAIRY-05` design fixation does not by itself promote the sample to active
helper status.
