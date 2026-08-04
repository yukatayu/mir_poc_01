# Report 2583 — Mir Theory v0 / I1+ Milestone 2 T0/G0 semantic assertions

- Date: 2026-08-04 10:12 JST
- Author / agent: Codex orchestrator
- Scope: replace only the current T0/G0 profile's mutable whole-file control
  mechanism; preserve v1/v2 artifacts; create, validate, and accept one v3
  semantic-assertion artifact before applying G0 exit/T1 entry.
- Decision levels touched: L1 phase-governance profile and its acceptance.
  North Star, Constitution guarantees, SCN/proof/runtime/public-contract state
  remain outside this milestone.

## Objective

Close T0/G0 with a deterministic, revision-bound semantic-assertion profile so
ordinary reader-facing document maintenance is not misclassified as a Mir
semantic failure. The same milestone must record profile adoption, fresh pass
artifact/digest acceptance, G0-D3 acceptance, G0 exit, and T1 entry without
rewriting v1/v2 history or claiming implementation readiness.

## Scope and assumptions

- M2 starts clean from `bf0f2bf75173ca7984f2119f24cbed66936ea627 ==
  origin/main` after M1 close.
- ADR-0015 and the owner direction authorize this lifecycle operation; the
  normal ADR-0013 human-acceptance reservation is superseded only for this
  bounded M2 sequence by ADR-0017.
- Version 1/2 artifacts remain byte-preserved history. V3 binds its source
  commit and semantic selectors, never a mutable reader-facing whole-file hash.
- M2 does not change SCN, OBL/proof, parser/checker/runtime, I1 authorization,
  public API/ABI/wire, or deployment.

## Start state / dirty state

- Official lifecycle: `T0`; G0-D1 historical accepted cut, G0-D2 v2 adoption,
  G0-D4 historical waiver; G0-D3/G0 exit/T1 entry absent.
- v1 artifact is nonconforming historical evidence; v2 artifact is valid
  historical `fail` from fixed-control hash drift. Both are preserved.
- Worktree was clean and no submodule, external worktree, or user change was
  present at M2 start.

## Documents consulted

- Canon: Constitution, ADR-0013/0015, Plan 00/01, source hierarchy, current
  T0 proposal/artifacts, and M1-aligned entry/MAP/ADR routes.
- LAB: Plan 247, current snapshots, Plan 198 artifact, and directly referenced
  Report 2581. Historical report corpus was not read.
- Independent planner: M2 Canon-first pre-edit review, read-only.

## Actions taken

1. Compared v2 fixed whole-file controls with the smallest alternative, v3
   revision-bound semantic assertions; rejected v2 re-pin/reuse and adopted v3.
2. Added a focused test-first producer contract for semantic drift, clean-suite
   and agent-hierarchy negative controls, deterministic artifact canonicalization,
   and digest tampering rejection.
3. Added the v3 profile/ADR/proposal route and the profile producer. Fresh
   artifact generation and lifecycle acceptance are recorded below after their
   evidence is available.

## Files changed

- Profile/governance: `mirrorea_canon/plan/04-t0-g0-semantic-assertion-profile.md`,
  ADR-0017, PROPOSAL-020, ADR-0013/README, Plan 01/README, Canon README/MAP.
- Producer/test/taxonomy: `scripts/evaluate_t0_semantic_assertions.py`,
  `scripts/tests/test_t0_semantic_assertions.py`, and `scripts/README.md`.
- Report: this file.

## Commands run

- Pre-edit planner review and source/artifact inventory.
- TDD RED/GREEN test commands for the producer.
- The first GREEN command was accidentally launched from `mirrorea_canon/` and
  failed to import the repository `scripts` package; rerunning it from the
  repository root passed all focused tests.
- Artifact reproduction, negative controls, acceptance validation, Canon index,
  docs/hierarchy, diff, review, commit/push/parity commands are recorded when
  they complete.

## Evidence / outputs / test results

- RED: the new producer test initially failed with missing producer file.
- GREEN: all three focused producer tests pass after implementation from the
  repository root.
- Profile-adoption validation passed: Canon index has `142` files; `make docs`
  passed its agent configuration, source hierarchy (`798/798`), and complete
  documentation scaffold (`1737` numbered reports) checks.
- Fresh v3 artifact and lifecycle acceptance evidence are not yet claimed in
  this initial report record.

## What changed in understanding

T0 needs a fixed evidence cut, but it does not need historical whole-file
equality for mutable reader-facing texts. Binding a Git revision plus explicit
selectors preserves reproducibility while letting unrelated prose maintenance
remain semantically neutral.

## Open questions

- None requiring owner input. M2's remaining work is the bounded fresh
  evaluation, acceptance validation, review, and closeout.

## Suggested next prompt

No prompt is required. Continue M2 autonomously until the profile artifact,
acceptance record, G0 exit, and T1 entry are validated and pushed.

## Plan update status

更新不要: Plan 247 remains M2-active until the profile artifact and acceptance close.

## Documentation.md update status

更新不要: reader-facing lifecycle status changes only with accepted G0 exit/T1 entry.

## docs/project-status.md update status

更新不要: profile adoption alone leaves official T0 unchanged.

## progress.md update status

更新不要: the current checkpoint stays M2 until its acceptance sequence closes.

## tasks.md update status

更新不要: the sole active package remains M2 through its full close contract.

## samples_progress.md update status

更新不要: M2 changes no runnable sample path, command, classification, or blocker.

## Reviewer findings and follow-up

- Pre-edit planner found no owner-reserved trigger. It requires v1/v2 artifact
  preservation, no v2 re-pin, a v3 artifact source cut, a semantic negative
  control, exact digest acceptance, and explicit non-claims.
- Final independent review follows fresh artifact and acceptance changes.

## Skipped validations and reasons

- Cargo, Lean, parser/checker/runtime/model suites are not M2 surfaces and are
  not claimed. M2 runs the producer/test/profile/documentation validation.

## Commit / push status

M2 is uncommitted at this initial report write. It will use at most three
`--no-gpg-sign` integration commits and verify remote parity after each push.

## Sub-agent session close status

- M2 pre-edit planner: complete, read-only.
- Final independent reviewer: not yet requested; it must be distinct from this
  author and review the fresh artifact/acceptance cut.
