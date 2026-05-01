# Report 1069 — active doc point-in-time wording repair

- Date: 2026-05-01 10:40 JST
- Author / agent: Codex
- Scope: reader-facing docs / research abstract wording maintenance
- Decision levels touched: none; documentation wording only

## Objective

Repair active reader-facing wording that could make dated family labels or dated phase snapshots look like the live repository macro phase or queue authority.

## Scope and assumptions

- Scope is limited to active `docs/hands_on/` and `docs/research_abstract/` wording plus `progress.md` logging.
- `README.md` and `Documentation.md` were audited and did not need edits in this package.
- Stop line: this package does not change specs, roadmap, sample status, validation results, implementation queue, public API / ABI status, or actual `U1` commitment.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/hands_on/avatar_fairy_follow_representative_slice_01.md`
- `docs/hands_on/network_transport_canaries_01.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/avatar_fairy_follow_plan_01.md`
- `docs/research_abstract/phase0-repository-memory-and-decision-boundary.md`
- `docs/research_abstract/phase1-current-l2-semantics-stabilization.md`
- `docs/research_abstract/phase2-parser-free-poc-and-detached-validation-loop.md`
- `docs/research_abstract/phase3-parser-boundary-and-first-checker-cut.md`
- `docs/research_abstract/phase4-shared-space-membership-and-practical-room-boundary.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- Docs-researcher sub-agent findings for active point-in-time wording drift

## Actions taken

- Reworded avatar follow hands-on and research abstract pages so `Macro 6 reserve` is a family-local historical lane label, not live macro phase authority.
- Reworded the network transport hands-on page from `2026-04-28 current closeout` to a dated first-cut closeout with a live-status pointer.
- Added a `docs/research_abstract/README.md` note that phase summaries contain dated repository-memory anchors and that live status / queue authority lives in `progress.md`, `tasks.md`, and `samples_progress.md`.
- Added the same dated-anchor note to phase0..phase6 research abstract summaries.
- Added a `progress.md` recent-log entry for the active-doc wording repair.

## Files changed

- `docs/hands_on/avatar_fairy_follow_representative_slice_01.md`
- `docs/hands_on/network_transport_canaries_01.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/avatar_fairy_follow_plan_01.md`
- `docs/research_abstract/phase0-repository-memory-and-decision-boundary.md`
- `docs/research_abstract/phase1-current-l2-semantics-stabilization.md`
- `docs/research_abstract/phase2-parser-free-poc-and-detached-validation-loop.md`
- `docs/research_abstract/phase3-parser-boundary-and-first-checker-cut.md`
- `docs/research_abstract/phase4-shared-space-membership-and-practical-room-boundary.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `progress.md`
- `docs/reports/1069-active-doc-point-in-time-wording-repair.md`

## Evidence / outputs / test results

Initial state:

```text
$ git status --short
<clean>

$ git log -1 --oneline
726da81 Cool samples validation wording
```

Targeted wording search after patch, before report:

```text
$ rg -n 'current macro-phase reading|macro-phase reading は `Macro 6 reserve`|2026-04-28 current closeout|2026-04-23 時点|dated repository-memory anchor|dated first-cut closeout|family-local historical lane label' docs/hands_on docs/research_abstract -g '*.md' -g '!docs/research_abstract/old/**'
docs/hands_on/avatar_fairy_follow_representative_slice_01.md:5:family-local historical lane label ...
docs/hands_on/network_transport_canaries_01.md:3:2026-04-28 dated first-cut closeout ...
docs/research_abstract/README.md:11:phase summary は、dated repository-memory anchor ...
docs/research_abstract/avatar_fairy_follow_plan_01.md:6:family-local historical lane label ...
phase0..phase6 each now mark the 2026-04-23 section as dated repository-memory anchor.
```

Docs-focused validation before report:

```text
$ python3 scripts/check_source_hierarchy.py
source hierarchy check
  repo_root: /home/yukatayu/dev/mir_poc_01
  required: 35
  present: 35
  missing: 0
  all required paths present

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 1066 numbered report(s).

$ git diff --check
<no output>
```

Post-report documentation validation:

```text
$ python3 scripts/check_source_hierarchy.py
source hierarchy check
  repo_root: /home/yukatayu/dev/mir_poc_01
  required: 35
  present: 35
  missing: 0
  all required paths present

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 1067 numbered report(s).

$ git diff --check
<no output>
```

## What changed in understanding

The affected active pages were not overclaiming final public completion, but several dated labels could be read as current live authority. The safer pattern is to mark family labels and dated phase snapshots explicitly, then point live status readers to `progress.md`, `tasks.md`, and `samples_progress.md`.

## Open questions

- Actual `U1` commitment remains open and user-facing.
- No sample status or validation result changed in this package.

## Suggested next prompt

Continue autonomous maintenance: re-run docs focused validation after this report, commit/push, then continue with active-doc stale wording audit or the next safe maintenance package.

## Plan update status

`plan/` 更新不要: this package changed reader-facing dated wording only and did not alter roadmap, boundary, sequencing, or long-lived repository memory.

## progress.md update status

`progress.md` 更新済み: timestamp and recent log were updated.

## tasks.md update status

`tasks.md` 更新不要: current task map, work ordering, and blockers did not change.

## samples_progress.md update status

`samples_progress.md` 更新不要: sample paths, validation commands, debug surfaces, blockers, percentages, and validation rows did not change.

## Skipped validations and reasons

- Full validation floor was not rerun because this package only changes documentation wording and package `1066` already recorded a fresh full validation checkpoint.
- Cargo tests and sample closeouts were not run because no code, samples, runner, or generated artifacts changed.

## Commit / push status

Pending at report write. Intended close command: `git commit --no-gpg-sign` followed by `git push`.

## Sub-agent session close status

Docs-researcher sub-agent `Gibbs` completed the active-doc point-in-time audit and was closed before patching.
