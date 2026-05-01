# plan/31 — FAIRY-05 visibility-return carrier bundling

## 目的

この文書は、`R3` docs-first package として、
residual planned family `FAIRY-05` の visibility-return witness を
どの carrier shape で読むのが current repo-local line として最も honest かを整理する。

ここで固定するのは **comparison matrix と provisional recommendation** である。
`FAIRY-05` を active runnable widening へ昇格することでも、
helper closeout schema や final public avatar / visualization API を fix することでもない。

## current fixed facts

- active representative slice は `FAIRY-01/02/03/04/06`
- `FAIRY-05` は `samples/not_implemented/avatar-fairy-follow/` に残る planned family
- helper closeout `fairy05_reopen_gate` は current implementation inventory として
  `carrier_choice = UNRESOLVED` を返す
- planning-only candidate labels は `state_timeline` / `anchor_switch`
- these labels are not current debug modes and not final public API names
- planned sample wording requires that the visibility-return witness appear inside
  timeline witness refs before the switch occurs

## current evidence anchors

### active helper anchors

- `python3 scripts/avatar_follow_samples.py run 01_follow_remote_head_with_local_fallback --debug anchors --format json`
  - explicit follow / fallback lineage
- `python3 scripts/avatar_follow_samples.py run 02_remote_head_not_visible_falls_back_to_local --debug anchors --format json`
  - visibility-loss fallback remains local
- `python3 scripts/avatar_follow_samples.py run 03_remote_avatar_leaves_falls_back_to_local --debug membership --format json`
  - stale membership rejection before fallback
- `python3 scripts/avatar_follow_samples.py run 06_model_check_no_detached_anchor_observed --debug verification --format json`
  - detached-anchor safety verification canary

### closeout anchors

helper closeout returns:

- `planned_sample_ids`
- `planned_sample_paths`
- `current_focus`
- `planned_remaining`
- `fairy05_reopen_gate`

### planned sample anchor

`samples/not_implemented/avatar-fairy-follow/05_follow_target_reacquired_after_return.mir`
states:

- before promotion, helper-local evidence must show `state_timeline`
- before promotion, helper-local evidence must show `anchor_switch`
- visibility-return witness should appear inside timeline witness refs before the switch occurs

## carrier-choice matrix

| Candidate | What it preserves | What it loses or risks | Current docs-first judgment |
|---|---|---|---|
| timeline event | temporal ordering, witness-before-switch reading, pre/post visibility history | exact anchor switch may become implicit unless another carrier is added | necessary but not sufficient on its own |
| anchor-switch event | explicit local->remote reattachment boundary | reason/order/witness lineage can become implicit | necessary but not sufficient on its own |
| witness event | explicit visibility-return witness | switch order and anchor frontier become too implicit for helper-local evidence | too narrow as a standalone carrier |
| typed bundle | can carry `state_timeline` + `anchor_switch` together, with visibility-return witness nested in timeline witness refs, while keeping naming provisional | bundle shape could be overread as final public API if the stop line is omitted | current provisional recommendation |

## provisional recommendation

the provisional docs-first reading recorded here is:

- keep helper closeout implementation inventory at `carrier_choice = UNRESOLVED`
- narrow repository memory to a provisional reading in which `FAIRY-05`
  should be carried as a **typed bundle over**:
  - `state_timeline`
  - `anchor_switch`
- carry visibility-return witness as a timeline witness ref inside that bundle,
  rather than as a standalone top-level carrier

### why this is the recorded narrow line

- the planned sample explicitly requires both timeline and switch evidence
- the planned sample wording already points to witness refs inside timeline
- this keeps ordering and switch frontier explicit together
- this avoids treating `state_timeline` or `anchor_switch` as settled debug mode names
- this avoids freezing a final public avatar / visualization API

## implementation boundary

`R3` closeout does **not** require:

- changing `scripts/avatar_follow_samples.py`
- changing helper closeout `carrier_choice`
- adding new debug modes
- promoting `FAIRY-05` to active runnable status

Those belong to a later runnable widening package if the reopen gate is chosen.

## reopen gate remains unchanged

Do not reopen `FAIRY-05` as runnable widening unless the same package carries:

- one positive reacquire-after-return sample
- one negative companion for missing return witness or stale membership
- explicit helper-local evidence for ordering and switch boundary
- docs / report / snapshot sync

## reopen-criteria inventory

- must be present in the same reopen package:
  - one positive reacquire-after-return sample
  - at least one negative companion covering missing return witness or stale membership
  - explicit helper-local `state_timeline` evidence
  - explicit helper-local `anchor_switch` evidence
  - docs / report / snapshot sync for the widened sample family
- still unresolved before any reopen:
  - `carrier_choice` remains `UNRESOLVED`
  - exact helper-local CLI/debug naming remains `UNRESOLVED`
  - whether the negative companion set should include only one of missing-return-witness / stale-membership or both
  - whether the widened evidence should remain in the same helper or move behind a separate helper boundary
- stop line:
  - reopen criteria do not by themselves fix a final public avatar runtime API
  - reopen criteria do not by themselves fix a final public visualization protocol
  - reopen criteria do not by themselves justify hot-plug / projection / transport coupling

## validation floor

- `python3 -m unittest scripts.tests.test_avatar_follow_samples`
- `python3 scripts/avatar_follow_samples.py check-all --format json`
- `python3 scripts/avatar_follow_samples.py closeout --format json`
- `python3 scripts/avatar_follow_samples.py run 01_follow_remote_head_with_local_fallback --debug anchors --format json`
- `python3 scripts/avatar_follow_samples.py run 02_remote_head_not_visible_falls_back_to_local --debug anchors --format json`
- `python3 scripts/avatar_follow_samples.py run 03_remote_avatar_leaves_falls_back_to_local --debug membership --format json`
- `python3 scripts/avatar_follow_samples.py run 06_model_check_no_detached_anchor_observed --debug verification --format json`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## stop line

Do not claim:

- `FAIRY-05` is active runnable evidence
- `state_timeline` / `anchor_switch` are current public debug mode names
- final public avatar runtime API
- final public visualization protocol
- real transport / session / auth semantics
- hot-plug / `AttachPoint` implementation

## historical follow-on focus

historical follow-on focus after `R3` closeout was kept-later hot-plug
work around real migration / rollback boundary rather than looping on
`FAIRY-05` wording again.
