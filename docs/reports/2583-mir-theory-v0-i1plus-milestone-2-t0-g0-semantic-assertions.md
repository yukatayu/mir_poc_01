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
3. Added the v3 profile/ADR/proposal route and the profile producer. The fresh
   source-bound artifact, its validator, and the lifecycle acceptance record
   are recorded below.
4. The first fresh run rejected two selectors whose English adjacency did not
   match their Japanese/line-wrapped Canon witnesses. Tightened them to the
   minimal stable semantic phrases; the focused negative controls remain
   rejecting.

## Files changed

- Profile/governance: `mirrorea_canon/plan/04-t0-g0-semantic-assertion-profile.md`,
  `mirrorea_canon/adr/ADR-0017.md`, `mirrorea_canon/plan/00-gates.md`,
  `mirrorea_canon/plan/01-phases.md`, `mirrorea_canon/CHANGELOG.md`,
  PROPOSAL-020, ADR-0013/README, Plan 01/README, Canon README/MAP/INDEX.
- Producer/test/taxonomy: `scripts/evaluate_t0_semantic_assertions.py`,
  `scripts/tests/test_t0_semantic_assertions.py`, and `scripts/README.md`.
- LAB artifact/index/snapshots: `plan/248-t0-g0-semantic-assertion-v3-evaluation.json`,
  `plan/00-index.md`, `plan/247-mir-theory-v0-i1plus-current-roadmap.md`,
  `Documentation.md`, `progress.md`, and `tasks.md`.
- `docs/project-status.md`
- Report: this file.

## Commands run

- Pre-edit planner review and source/artifact inventory.
- TDD RED/GREEN test commands for the producer.
- The first GREEN command was accidentally launched from `mirrorea_canon/` and
  failed to import the repository `scripts` package; rerunning it from the
  repository root passed all focused tests.
- The combined focused-test invocation after index regeneration repeated that
  wrong working-directory import failure; it made no source change and the
  same command from the repository root passed.
- `python3 -m unittest scripts.tests.test_t0_semantic_assertions -v` passed
  four tests: selector stability, clean/agent drift rejection, digest tamper
  rejection, and checked-artifact validation rejection.
- `python3 scripts/evaluate_t0_semantic_assertions.py --validate-artifact
  plan/248-t0-g0-semantic-assertion-v3-evaluation.json` reproduced the declared
  source cut and passed. A second `--revision 644ec1cd…` output was byte-equal
  to the checked-in artifact.
- `sha256sum plan/155* plan/198*` and `git diff --exit-code b9dcaa05… --
  plan/155* plan/198*` confirmed historical v1/v2 files were not modified.
- Canon index, docs/hierarchy, diff, review, commit/push/parity commands are
  recorded when they complete.

## Evidence / outputs / test results

- RED: the new producer test initially failed with missing producer file.
- GREEN: all four focused producer tests pass after implementation from the
  repository root.
- Profile-adoption validation passed: Canon index has `142` files; `make docs`
  passed its agent configuration, source hierarchy (`798/798`), and complete
  documentation scaffold (`1737` numbered reports) checks.
- Fresh v3 artifact: `plan/248-t0-g0-semantic-assertion-v3-evaluation.json`,
  source revision `644ec1cdfa7d69600af3463ab60a6b7d745913c8`, profile source
  SHA-256 `45cf5da9c7be03de89a47f74d37d3454c7c9029e0ac59bb7538f7c11b5974f37`,
  producer SHA-256 `1234566706bcfda51279bb6d0a88176be85c84f812e7f368171bc0ae6070a59f`,
  and canonical digest
  `b32bd2c87e1dc77ca2a4f7a7426cda0bff8bcbf80155d19addd7db3a8288aa23`; all six
  semantic assertions have root result `pass`.
- The exact acceptance record in Canon applies pass digest acceptance, G0-D3,
  G0 exit, and T1 entry in that order. It claims no SCN/conformance,
  proof/OBL, runtime, I1, public API/ABI/wire, or deployment result.
- Final `make docs` passed after the reviewer correction: agent configuration,
  Canon index (`142` files), source hierarchy (`798/798`), and documentation
  scaffold (`1737` numbered reports). The focused doc suite, artifact validator,
  strict Codex config dry-run, historical-artifact diff, and `git diff --check`
  also passed.

## What changed in understanding

T0 needs a fixed evidence cut, but it does not need historical whole-file
equality for mutable reader-facing texts. Binding a Git revision plus explicit
selectors preserves reproducibility while letting unrelated prose maintenance
remain semantically neutral.

## Open questions

- None requiring owner input. M3 begins the evaluation/materialization
  calculus under the accepted T1 lifecycle.

## Suggested next prompt

No prompt is required. Continue autonomously with M3 evaluation/materialization
calculus after the M2 validation/push closeout.

## Plan update status

更新済み: Plan 247 now records M2 close and M3 as the only active semantic milestone.

## Documentation.md update status

更新済み: Documentation now routes readers to the v3 acceptance and current M3 frontier.

## docs/project-status.md update status

更新済み: docs/project-status.md was updated in this package.

## progress.md update status

更新済み: progress snapshot now records accepted T1 entry, M3, and a dated log row.

## tasks.md update status

更新済み: tasks map now marks M2 complete and M3 active.

## samples_progress.md update status

更新不要: M2 changes no runnable sample path, command, classification, or blocker.

## Reviewer findings and follow-up

- Pre-edit planner found no owner-reserved trigger. It requires v1/v2 artifact
  preservation, no v2 re-pin, a v3 artifact source cut, a semantic negative
  control, exact digest acceptance, and explicit non-claims.
- Final independent review initially found one P1: the validator read the
  declared producer blob only to hash it, then evaluated the current checkout.
  Acceptance was rejected until corrected.
- One correction replaces that path with execution of the declared Git producer
  blob and exact rendered-byte comparison. A new digest-valid, valid-revision
  altered-artifact test rejects `artifact reproduction mismatch`.
- Narrow independent re-review passed: no remaining P0/P1 finding. It also
  confirmed the v3 semantic (not whole-file) controls, v1/v2 preservation,
  acceptance ordering/non-claims, sole M3 frontier, focused tests, and docs
  validation scope.

## Skipped validations and reasons

- Cargo, Lean, parser/checker/runtime/model suites are not M2 surfaces and are
  not claimed. M2 runs the producer/test/profile/documentation validation.

## Commit / push status

The profile adoption commit `e1d085792ac91395696fdbe10d762b71025b20a8` and
selector-correction commit `644ec1cdfa7d69600af3463ab60a6b7d745913c8` are
pushed. This final acceptance/snapshot integration cut uses `--no-gpg-sign`,
is pushed before M3 starts, and records remote parity in the milestone close.

## Sub-agent session close status

- M2 pre-edit planner: complete, read-only.
- Final independent reviewer: complete, read-only, distinct from this author;
  one P1 provenance correction and narrow re-review completed.
