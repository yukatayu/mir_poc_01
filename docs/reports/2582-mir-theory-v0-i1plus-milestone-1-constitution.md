# Report 2582 — Mir Theory v0 / I1+ Milestone 1 Constitution

- Date: 2026-08-04 09:31 JST
- Author / agent: Codex orchestrator
- Scope: concise Canon Design Constitution and only the alignment necessary to
  remove its direct conflicts in the owner-approved M1 scope
- Decision levels touched: L1 design filter and its supporting L1 ADR. North
  Star purpose, official lifecycle, proof achievements, conformance results,
  runtime implementation, and public contract remain unchanged.

## Objective

Create one short canonical Constitution which lets a context-free agent make
ordinary Mir v0/I1+ design decisions without turning it into a parallel theory
or prematurely fixing syntax, carriers, public APIs, or implementation. Repair
the existing SCN-02 evaluation/authority contradiction and the semantic versus
presentation fallback ambiguity in the same integration unit.

## Scope and assumptions

- M1 starts from accepted M0 remote cut
  `d772678abfe345dc6a5de0538db537d64962368d`.
- `mirrorea_canon/` remains normative; this report, Plan 247, and all existing
  implementation/sample material are LAB evidence only.
- ADR-0015 and PROPOSAL-018 supply the owner authorization; this report does
  not create a further owner decision.
- M1 fixes cross-cutting decision rules and directly conflicting prose. M3--M5
  still own exact calculus/carriers/formal inventory, M6 grammar, and M7--M10
  implementation/conformance.

## Start state / dirty state

- `HEAD == origin/main == d772678abfe345dc6a5de0538db537d64962368d` before
  M1 edits; worktree was clean.
- Official lifecycle was `T0`; v2 T0 artifact remained valid `fail`; G0-D3,
  G0 exit, and T1 entry were absent; OBL-001..028 remained open in the
  pre-M1 ledger; SCN conformance and runtime implementation were unchanged.
- Root storage had about 24 GiB available (87% used); no heavy build or new
  generated artifact was started.

## Documents consulted

- Canon route: `mirrorea_canon/README.md`, `MAP.md`, `NORTH-STAR.md`,
  `GLOSSARY.md`, ADR index, ADR-0001..0011, ADR-0015, operating model, source
  hierarchy, style guide, theory/spec/scenario READMEs, theory 00--10, and
  SCN-02/08/09/10.
- Current LAB route: Plan 247, `Documentation.md`, `docs/project-status.md`,
  `progress.md`, `tasks.md`, `samples_progress.md`, and Report 2581. No bulk
  `docs/reports/` history was read.
- Independent planner review: read-only M1 Canon-first review.
- Advisory Oracle review: temporary `GPT-5.6 Sol + Pro` session
  `mir-m1-constituti-20260803`; advice was checked against local Canon and did
  not become normative by itself.

## Actions taken

1. Compared exactly two placement candidates: amend L0 North Star, or create
   a root-level subordinate Design Constitution. Adopted the latter to retain
   North Star as the project axis while giving all later layers one compact
   decision filter.
2. Added `root/design-constitution`, PROPOSAL-019, and ADR-0016; wired them
   into the Canon reading route, ADR index, source-hierarchy rule, changelog,
   and generated index.
3. Set stable C1--C12 clauses for source/authority, ordinary Surface,
   communication, evaluation/materialization, owner RMW, designated results,
   relations/projection, fallback, extensions/observation, save/load/patch,
   assurance, and priority/escalation.
4. Repaired SCN-02, theory/01, and theory/03 so BrowserClient is authority
   origin while S evaluates S-owned RMW in its owner queue; cross-owner input
   remains explicit-result-or-Diagnostic only.
5. Repaired theory/09 so transient consumer sample loss is presentation
   fallback, not semantic lineage advancement; aligned save/load and typed
   observation bridges with the Constitution. Repaired the behind-the-scenes
   mental model so it no longer describes requester-side reads of S-owned RHS.
6. Reclassified the old Surface EBNF as a pre-M6 candidate through ADR-0016,
   preserving historic LAB evidence without freezing final syntax early.
7. Kept the proof-ledger vocabulary/status migration in M5: no M1 proof claim
   or invented classification is made before its shared formal model/evidence.

## Files changed

- New Canon decision artifacts: `mirrorea_canon/DESIGN-CONSTITUTION.md`,
  PROPOSAL-019, and ADR-0016.
- Canon navigation/governance: root README/MAP/North Star, ADR README,
  ADR-0008, CHANGELOG, source hierarchy, and INDEX.
- Direct semantic alignment: theory 01--04, 07, 09; the behind-the-scenes
  mental model; spec README/spec 02; scenarios README/SCN-02.
- Structural validation: `scripts/check_source_hierarchy.py`.
- Report: this file.

## Commands run

- Canon-first source inventory/search and read-only independent planner review.
- Oracle status/output review with the project-local operating policy.
- `cd mirrorea_canon && python3 meta/build-index.py`, followed by repository
  root `make docs` (the latter is the correct Makefile location).
- `make docs`; `python3 -m unittest scripts.tests.test_validate_docs
  scripts.tests.test_validate_agent_configs -v`; `codex --strict-config -C .
  --help`; `git diff --check`; focused stale-wording/schema searches; and the
  Constitution word-count check.
- Independent reviewer initial pass with three P1 findings, one correction
  cycle, then narrow re-review pass. Commit/push/parity commands are recorded
  below when completed.

## Evidence / outputs / test results

- `root/design-constitution` has 1,340 words and stable clauses C1--C12;
  it has no grammar production, Core constructor inventory, transport choice,
  runtime algorithm, public ABI/wire declaration, lifecycle verdict, or proof
  completion claim.
- Canon index generation reported `ok: 139 files indexed`.
- Final pre-commit `make docs` passed: agent configuration, Canon index
  (`139` files), source hierarchy (`798/798`), and documentation validation
  all exited zero. Focused documentation/config unittest, strict Codex config,
  `git diff --check`, stale-wording/schema scans also passed.
- Closeout formatting removed one trailing blank line from PROPOSAL-019; its
  byte metadata therefore required one final INDEX regeneration, after which
  `make docs` and `git diff --check` passed again.
- The fixed decision corpus below is the M1 positive/adverse evidence and the
  input for the final Constitution-only reviewer classification.

## What changed in understanding

The key M1 distinction is not a new event system: it is the independent
coordinates of authority origin and evaluation site. That makes same-owner RMW
safe without giving actor code owner authority, and it makes a true cross-owner
operand visible rather than turning it into a hidden transaction.

The second key distinction is between a semantic relation/fallback lineage and
a consumer's presentation shortage. The former belongs in the occurrence/cut
world; the latter must not rewrite it. The Constitution can fix both
cross-cutting invariants while deliberately leaving their M3/M4 carriers open.

## Open questions

- Exact evaluation/materialization judgment, designated-result carrier, and
  ambiguity diagnostic are M3.
- Relation DAG algorithm, relation save state, and finite projection theorem
  are M4/M5.
- Proof-ledger migration and theorem/implementation correspondence are M5;
  M1 makes no `lean-*` or model-check claim.
- Final Surface grammar is M6. The retained grammar is one candidate, not an
  owner-reserved public freeze.

## Suggested next prompt

No prompt is required. After M1 independent review and closeout, continue
autonomously with M2 semantic-assertion T0/G0 closeout.

## Plan update status

Updated: Plan 247 remains the sole roadmap, records the accepted Constitution
path, M2 direct consumer, M1 payload `aa0771ec`, and its pushed close.

## Documentation.md update status

Updated: it links the M1 decision filter and preserves the T0 non-effect.

## docs/project-status.md update status

Updated: it names M2 as the active frontier and records M1's pushed payload and
unchanged T0 non-effect.

## progress.md update status

Updated: it records the M1 close checkpoint with command-derived timestamp and
retains the official T0 state until M2 acceptance.

## tasks.md update status

Updated: it moves M1 to closed and makes M2 the only active package.

## samples_progress.md update status

`samples_progress.md` update unnecessary: M1 changes no runnable sample path,
command, debug surface, classification, or sample blocker.

## Reviewer findings and follow-up

- Pre-edit planner review recommended a root-level Constitution rather than a
  North Star rewrite; it identified SCN-02 owner-side RMW, semantic versus
  presentation fallback, and premature grammar finality as direct conflicts.
- It also suggested a proof-ledger and I1 phase wording change. Direct source
  review found no I1 entry/exit contradiction: C-static is entry/readiness and
  C-runtime is exit evidence. Ledger status migration belongs to the explicit
  M5 shared-model milestone; changing status before evidence would be false.
- Initial independent Constitution-only falsification review found no P0 and
  three P1 findings: M1 had named future relation/result persistence fields,
  the derived-stream adverse corpus conflated stream use with transfer of
  semantic ownership, and INDEX needed fresh regeneration after edits.
- The single correction cycle removed the field schema/serialization choice,
  made a derived stream an M4-only explicit preserving boundary, split the
  corpus into ownership and deferred-stream cases, regenerated INDEX, and
  reran focused validation. Narrow independent re-review passed with no P0/P1.

## Skipped validations and reasons

- Rust, Cargo, Lean, runtime replay, model checking, parser/checker, save/load,
  patch, relation, evaluator, auth, and release conformance were not run for
  the initial M1 semantic-document cut because no corresponding executable or
  proof source changed. M3--M10 own those validations.
- No heavy build was started because M1 has no build target and the configured
  external workdir remains absent.

## Commit / push status

M1 payload `aa0771ecdec4a7cec8f9f454dcbb455025ede8dc` (`Adopt Mir v0 design
constitution`) was committed with `--no-gpg-sign`, pushed to `origin/main`, and
verified `HEAD == origin/main == aa0771ecdec4a7cec8f9f454dcbb455025ede8dc`.
The closeout-record commit follows this report update and is pushed with its
own parity check before M2 begins.

## Sub-agent session close status

- M1 pre-edit planner: complete, read-only.
- Oracle: complete, advisory only; no external transcript is committed.
- Final independent reviewer: initial review and narrow correction re-review
  complete, read-only, pass after one correction cycle.

## Fixed Constitution decision corpus

The following corpus is intentionally self-contained. A reviewer receives this
table and `root/design-constitution`, not Plan 247 or historical reports, and
must classify every row using its stated controlling clause and one of C1's
five outcomes.

| Pair | Positive case | Adverse case | Controlling clause / expected distinction |
| --- | --- | --- | --- |
| 1 | Ordinary assignment elaborates generated owner request/edges. | Surface requires handwritten RPC/envelope before meaning. | C2/C3: allowed vs prohibited. |
| 2 | One S owner evaluates S-owned `hp = hp - atk` serially. | An actor reads S-private `atk` then blind-writes, or cross-owner atomicity is inferred. | C4/C5: allowed only with explicit boundary vs prohibited/deferred. |
| 3 | E publishes a versioned result bound to frontier and authority evidence. | C treats an unversioned/stale result as current or re-evaluates E's semantic decision. | C6: allowed only with explicit semantic boundary vs prohibited. |
| 4 | B owns bird-follow relation; C derives pose in its admitted local presentation context. | C becomes the relation's semantic owner. | C7: allowed vs prohibited. |
| 5 | M4 evaluates whether a derived-value stream is admissible at an explicit relation-, authority-, and policy-preserving boundary. | A stream silently replaces the maintained relation or transfers its ownership. | C7: deferred to M4 vs prohibited. |
| 6 | Anchor membership/lease loss advances semantic fallback; reacquire is fresh. | Packet jitter/sample loss advances the semantic fallback position. | C8: allowed vs prohibited. |
| 7 | Authenticated principal receives an admission-bound capability and validates it at owner service. | Transport/session/locus/role/key alone authorizes mutation. | C9: allowed only with explicit semantic boundary vs prohibited. |
| 8 | Observer-safe relation projection is redacted to every input's policy. | Raw debug/telemetry releases witness/private state without typed authority/redaction. | C9: allowed only with explicit semantic boundary vs prohibited. |
| 9 | Consistent cut saves lineage/version/consumption and load validates before mutation. | Load revives stale membership, witness, lease, consumption, or relation lineage. | C10: allowed only with explicit semantic boundary vs prohibited. |
| 10 | Checked patch activates only after verdict and activation cut. | Rejected/deferred patch mutates semantic state. | C10: allowed only with explicit semantic boundary vs prohibited. |
| 11 | Bounded evidence stays explicitly bounded. | Bounded evidence is claimed as a general proof. | C11: allowed vs prohibited. |
| 12 | Existing deferred scope remains temporary. | A new permanent v0 non-goal is introduced. | C12: allowed vs owner escalation required. |
