# Report 2146 - G1 ELAB-04 mixed visibility branch inventory

- Date: 2026-07-04 02:32 JST
- Author / agent: Codex
- Scope: LAB `ELAB-04` mixed visibility branch inventory
- Decision levels touched: L3 LAB repository memory only

## Objective

Record a docs-only branch inventory for `ELAB-04`, the mixed visibility /
non-visibility generated failure-row omission.

Close target for this package:

- keep executable output unchanged;
- keep `ELAB-04` no-repair;
- separate base remote-request failure pressure from `VisibilityDenied`
  pressure before any future mixed-row repair widening.

## Scope and assumptions

Scope included:

- `ELAB-04` fixture source and expected JSON.
- Current Rust repair payload structs and singleton emission guard.
- Current Rust and Python tests around `ELAB-04` no-repair and `ELAB-10`
  visibility singleton repair.
- `plan/86`, `plan/95`, `plan/96`, and `plan/97`.
- Snapshot docs that mention current G1 E-ROW repair status.

Assumptions:

- `mirrorea_canon/` remains normative.
- The current package is docs / repository-memory only.
- `ELAB-10` and `ELAB-13..16` remain current singleton repair-bearing evidence.
- `ELAB-04/07` remain no-repair unless a later package explicitly designs and
  tests set-insertion, branch grouping, or bundle semantics.

## Start state / dirty state

Start state for this package:

- Branch: `main`
- Upstream: `origin/main`
- Starting HEAD: `435e141d12a392128a045ea4893f187a9899496f`
- Start dirty state: clean at package start.
- Discord baseline: `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/oracle-chatgpt-pro-operations.md`
- `/home/codex/.codex/docs/oracle-chatgpt-pro.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `plan/00-index.md`
- `plan/86-g1-erow002-visibility-repair-carrier-prototype.md`
- `plan/90-source-traceability.md`
- `plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md`
- `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md`
- `plan/97-g1-erow07-set-insertion-gate-review.md`
- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `scripts/tests/test_surface_mir_samples.py`
- `scripts/surface_mir_samples.py`
- `samples/full-system-v1-surface/elaboration/matrix.json`
- `samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/README.md`
- `samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/main/src/undeclared-generated-failure-negative.mir`
- `samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/expected/elaboration.json`
- `docs/research_abstract/surface_mir_alpha_01.md`

External / delegated review:

- Oracle consult `we-are-continuing-autonomous-g1-2` failed during attachment
  submission.
- Oracle retry `retry-after-attachment-timeout-advisory` completed and returned
  advisory confirmation of the lossy-classifier and partial-coverage guard.
- Sub-agent `019f2909-65e1-7180-826e-561823208be9` completed a read-only
  ELAB-04 inventory and was closed.

## Actions taken

- Added `plan/98-g1-erow04-mixed-visibility-branch-inventory.md`.
- Updated `plan/00-index.md` and `plan/90-source-traceability.md`.
- Updated `plan/95`, `plan/96`, and `plan/97` to reference the new `ELAB-04`
  mixed branch inventory.
- Updated `README.md` to mention the current `ELAB-04` branch inventory and
  no-repair status.
- Updated `docs/research_abstract/surface_mir_alpha_01.md` so `ELAB-04` is not
  just "rejected" but also explicitly mixed-branch no-repair evidence.
- Updated `Documentation.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.

## Files changed

- `README.md`
- `Documentation.md`
- `docs/reports/2146-g1-erow04-mixed-visibility-branch-inventory.md`
- `docs/research_abstract/surface_mir_alpha_01.md`
- `plan/00-index.md`
- `plan/90-source-traceability.md`
- `plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md`
- `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md`
- `plan/97-g1-erow07-set-insertion-gate-review.md`
- `plan/98-g1-erow04-mixed-visibility-branch-inventory.md`
- `progress.md`
- `samples_progress.md`
- `tasks.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short --branch
git rev-parse HEAD origin/main
date '+%Y-%m-%d %H:%M %Z'
sed -n '1,170p' README.md
sed -n '1,220p' Documentation.md
sed -n '1,280p' progress.md
sed -n '1,260p' tasks.md
sed -n '1,230p' samples_progress.md
sed -n '1,220p' mirrorea_canon/README.md
sed -n '1,220p' mirrorea_canon/MAP.md
sed -n '1,260p' mirrorea_canon/theory/03-elaboration.md
sed -n '1,300p' mirrorea_canon/theory/10-diagnostics.md
sed -n '1,260p' mirrorea_canon/spec/07-diagnostics-format.md
sed -n '1,240p' mirrorea_canon/theory/11-metatheory-ledger.md
sed -n '1,260p' plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md
sed -n '1,360p' plan/96-g1-erow-set-insertion-bundle-payload-inventory.md
sed -n '1,280p' plan/97-g1-erow07-set-insertion-gate-review.md
sed -n '1,260p' plan/86-g1-erow002-visibility-repair-carrier-prototype.md
sed -n '1,220p' samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/README.md
sed -n '1,220p' samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/main/src/undeclared-generated-failure-negative.mir
jq '.' samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/expected/elaboration.json
jq '.rows[] | select(.sample_id=="ELAB-04" or .sample_id=="ELAB-07" or .sample_id=="ELAB-10" or .sample_id=="ELAB-13" or .sample_id=="ELAB-14" or .sample_id=="ELAB-15" or .sample_id=="ELAB-16")' samples/full-system-v1-surface/elaboration/matrix.json
nl -ba crates/mir-semantics/src/surface_to_core_elaboration.rs | sed -n '36,120p;1090,1260p'
nl -ba crates/mir-semantics/tests/surface_to_core_elaboration.rs | sed -n '560,650p;1030,1145p'
nl -ba scripts/tests/test_surface_mir_samples.py | sed -n '392,455p;520,700p'
rg -n "ELAB-04|UnderdeclaredGeneratedFailure|VisibilityDenied|suggested_repair" crates/mir-semantics/tests/surface_to_core_elaboration.rs scripts/tests/test_surface_mir_samples.py crates/mir-semantics/src/surface_to_core_elaboration.rs
nl -ba scripts/tests/test_surface_mir_samples.py | sed -n '700,760p'
nl -ba crates/mir-semantics/tests/surface_to_core_elaboration.rs | sed -n '480,560p'
sed -n '1,260p' /home/codex/.codex/docs/oracle-chatgpt-pro.md
sed -n '1,240p' .docs/oracle-chatgpt-pro-operations.md
ask-chatgpt-pro -p "We are continuing autonomous G1 LAB work in the Mirrorea repo. Please review a docs-only ELAB-04 mixed visibility branch inventory package before any executable repair widening. Source hierarchy: mirrorea_canon/ is normative; legacy specs/ and plan/ are LAB evidence / repository memory; this consult is advisory only. Current evidence: ELAB-10 is E-ROW-002 / VisibilityDenied singleton repair-bearing; ELAB-13..16 are E-ROW-001 non-visibility singleton repair-bearing; ELAB-07 is non-visibility multi-missing no-repair after plan/97; ELAB-04 is mixed visibility/non-visibility multi-missing with MissingWitness, RouteUnavailable, StaleMembership, and VisibilityDenied, currently no-repair. Question: what should plan/98 record for ELAB-04 branch decomposition so we do not collapse visibility/auth/observe-authority concerns into base remote-request failures, do not misread partial branch guidance as OBL-025 coverage, and do not prematurely define repair ranking/bundle semantics? Please identify hidden failure modes, minimum docs/tests/predicates needed before any future executable mixed-row widening, and the smallest safe next package. Do not propose canon edits, final ABI, proof discharge, conformance, or G1 exit." --file plan/86-g1-erow002-visibility-repair-carrier-prototype.md --file plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md --file plan/96-g1-erow-set-insertion-bundle-payload-inventory.md --file plan/97-g1-erow07-set-insertion-gate-review.md --file samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/expected/elaboration.json --file samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/expected/elaboration.json --file samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json
oracle session we-are-continuing-autonomous-g1-2
/home/codex/.local/bin/ask-chatgpt-pro -p "Retry after attachment-timeout; advisory only. We are working on docs-only ELAB-04 mixed visibility branch inventory. Source hierarchy: mirrorea_canon normative; legacy specs and plan are LAB evidence and repository memory. Current evidence: ELAB-04 source is visible hp read with only MissingCapability declared; required failures are MissingCapability, MissingWitness, RouteUnavailable, StaleMembership, VisibilityDenied; missing set is base branch MissingWitness, RouteUnavailable, StaleMembership plus visibility branch VisibilityDenied; code classifies singleton VisibilityDenied as E-ROW-002 but mixed set as E-ROW-001; repair emission is singleton-gated so ELAB-04 omits suggested_repair. ELAB-10 singleton visibility and ELAB-13..16 non-visibility singletons are repair-bearing. ELAB-07 remains no-repair after set-insertion gate review. Question: what should plan/98 record to avoid collapsing visibility into base failures, avoid partial guidance as OBL-025 coverage, avoid premature ranking/bundle semantics? Return hidden failure modes, minimum docs/tests/predicates before executable mixed-row widening, and smallest safe next package. No canon edits, final ABI, proof, conformance, or G1 exit."
python3 scripts/surface_mir_samples.py --format json run ELAB-04
python3 scripts/surface_mir_samples.py --format json run ELAB-10
python3 scripts/surface_mir_samples.py --format json check-all
python3 scripts/surface_mir_samples.py --format json check-all > /tmp/mirrorea-surface-check-all-2146.json
jq '{sample_count, failed_count:(.failed|length), workflow_ready, elab04_repair:([.results[] | select(.sample_id=="ELAB-04") | .actual.lab_diagnostic_details[] | has("suggested_repair")][0]), elab10_repair_count:([.results[] | select(.sample_id=="ELAB-10") | .actual.lab_diagnostic_details[].suggested_repair | length][0])}' /tmp/mirrorea-surface-check-all-2146.json
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples
python3 scripts/check_source_hierarchy.py
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/validate_docs.py
git diff --check
cargo fmt --check
rg -n "<repo-local secret-patterns>" README.md Documentation.md docs/research_abstract/surface_mir_alpha_01.md plan/00-index.md plan/90-source-traceability.md plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md plan/96-g1-erow-set-insertion-bundle-payload-inventory.md plan/97-g1-erow07-set-insertion-gate-review.md plan/98-g1-erow04-mixed-visibility-branch-inventory.md progress.md samples_progress.md tasks.md docs/reports/2146-g1-erow04-mixed-visibility-branch-inventory.md
```

The changed-file secret scan exited 1 with no matches, which is the expected
no-match result for `rg`.

## Evidence / outputs / test results

Evidence so far:

- `ELAB-04` source declares only `MissingCapability` on `when render`.
- `ELAB-04` generated read request requires `MissingCapability`,
  `MissingWitness`, `RouteUnavailable`, `StaleMembership`, and
  `VisibilityDenied`.
- `ELAB-04` expected JSON omits `suggested_repair`.
- Current Rust code classifies singleton `VisibilityDenied` as `E-ROW-002`, but
  mixed missing sets as `E-ROW-001`.
- Current Rust repair emission returns no repair when
  `missing_failures.len() != 1`.
- Current Rust and Python tests assert `ELAB-04` no-repair behavior.
- Current Rust and Python tests assert `ELAB-10` singleton `VisibilityDenied`
  repair behavior.
- Sub-agent inventory reached the same current-state reading.
- Oracle retry agreed that the current `E-ROW-001` classifier result is lossy
  for mixed visibility rows and should not be treated as base-only evidence.
- Oracle retry also reinforced that partial branch guidance must not count as
  OBL-025 coverage unless every missing failure in the associated row is
  covered.

Validation results:

- `python3 scripts/surface_mir_samples.py --format json run ELAB-04`: passed;
  `ELAB-04` still has no `suggested_repair`.
- `python3 scripts/surface_mir_samples.py --format json run ELAB-10`: passed;
  `ELAB-10` still has one singleton `suggested_repair`.
- `python3 scripts/surface_mir_samples.py --format json check-all`: passed
  with 52 samples and 0 failed.
- Saved check-all summary: `sample_count=52`, `failed_count=0`,
  `workflow_ready=false`, `elab04_repair=false`, `elab10_repair_count=1`.
- `cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture`:
  passed, 20 tests. The printed panic belongs to an expected `should panic`
  placeholder-detector test.
- `python3 -m unittest scripts.tests.test_surface_mir_samples`: passed, 45
  tests.
- `python3 scripts/check_source_hierarchy.py`: passed, 602 required paths
  present.
- `python3 -m unittest scripts.tests.test_validate_docs`: passed, 20 tests.
- `python3 scripts/validate_docs.py`: passed; documentation scaffold complete,
  1298 numbered reports found.
- `git diff --check`: passed.
- `cargo fmt --check`: passed.
- Changed-file secret scan: no matches.

## What changed in understanding

The `ELAB-04` blocker is not only multi-missing coverage. It is mixed-branch
diagnostic ownership:

- the base branch has the same set/bundle/partiality pressure as `ELAB-07`;
- the visibility branch has `E-ROW-002`-like pressure and possible future
  visibility / observe-authority alternatives;
- emitting one branch without the other would not discharge the whole local
  row-containment premise.

## Open questions

- Should a future mixed row stay one diagnostic or split into E-ROW-001 and
  E-ROW-002 diagnostics with explicit association?
- Is a mixed branch wrapper a repair witness, a repair plan, or guidance only?
- What ordering / ranking policy applies when base and visibility branches both
  have candidate repairs?
- Should partial branch guidance live in `suggested_repair[]` or outside it?
- Which future relation states whole rejected-gap coverage for mixed rows?

## Suggested next prompt

Continue autonomously with either:

- `ELAB-07` set-insertion executable preflight design, without Rust output
  widening yet; or
- a docs-only mixed branch payload model for `ELAB-04`; or
- OBL-024 Lean statement-shape draft after diagnostic association vocabulary is
  stable enough.

## Plan update status

`plan/` 更新済み:

- Added `plan/98-g1-erow04-mixed-visibility-branch-inventory.md`.
- Updated `plan/00-index.md`.
- Updated `plan/90-source-traceability.md`.
- Updated `plan/95`, `plan/96`, and `plan/97` to reference `plan/98`.

## Documentation.md update status

`Documentation.md` 更新済み:

- The Surface Mir LAB summary now includes the `ELAB-04` mixed visibility
  branch inventory and keeps `ELAB-04/07` no-repair.

## progress.md update status

`progress.md` 更新済み:

- Added the `ELAB-04` mixed visibility branch inventory note, updated the
  next-gap paragraph, updated the Surface-to-Core row, and appended a recent
  log entry.

## tasks.md update status

`tasks.md` 更新済み:

- Moved the mixed visibility branch inventory into current memory and replaced
  the candidate with a future mixed branch payload-model preflight.

## samples_progress.md update status

`samples_progress.md` 更新済み:

- Added the `ELAB-04` mixed visibility branch inventory to current focus,
  Surface Mir row, elaboration root row, and recent validation log.

## Reviewer findings and follow-up

Sub-agent `019f2909-65e1-7180-826e-561823208be9` findings:

- `ELAB-04` is a mixed row today: base remote-request missing failures are
  `MissingWitness`, `RouteUnavailable`, `StaleMembership`, and the visibility
  branch is `VisibilityDenied`.
- Current code classifies `VisibilityDenied` plus other failures as
  `E-ROW-001`; `E-ROW-002` is only used for singleton `VisibilityDenied`.
- Current code omits `suggested_repair` for mixed rows because repair emission
  is singleton-gated.
- Current Rust and Python tests assert `ELAB-04` no-repair and `ELAB-10`
  singleton visibility repair.
- The plan should warn against one-repair-per-missing-failure alternatives,
  collapsing `VisibilityDenied` into base failures, standardizing
  `suggested_repair: []`, claiming row repair authorizes visibility/runtime
  success, and implementing mixed repair before branch association, ordering /
  ranking, set-insertion or bundle semantics, and OBL-025 coverage status are
  explicit.

Oracle retry findings incorporated:

- keep `ELAB-04` as LAB inventory only, not executable repair evidence;
- record the exact required / declared / missing failure split;
- treat `VisibilityDenied` as an independent branch component even though the
  current mixed-row classifier result is `E-ROW-001`;
- require complete missing-set coverage before any future mixed-row repair can
  be counted toward OBL-025;
- keep singleton repair-bearing rows out of mixed-row coverage claims.

Reviewer sub-agent `019f2918-f1a1-79f2-b9c3-4f4ed055ee9c` found no semantic
blockers. It reported one minor completeness gap: `mirrorea_canon/README.md`
and `mirrorea_canon/MAP.md` should be listed for this package report and the
`plan/90` traceability row. Follow-up: read both canon entry files and added
them to this report and `plan/90`.

## Skipped validations and reasons

No intended validation skips for docs / repository memory updates.

Broad Cargo workspace test was not run because this package changed docs,
reports, and LAB plan memory only; the focused Rust elaboration integration
test, Python helper tests, source hierarchy check, docs validators, Surface
helper check-all, diff check, and format check were run instead.

## Commit / push status

Content commit:

- `96318e1363cccdc27847462710bd04e3a3f78c01`
  `Record ELAB-04 mixed visibility branch inventory`

Push status:

- Pushed to `origin/main`.
- Verified after push that `HEAD` and `origin/main` both resolve to
  `96318e1363cccdc27847462710bd04e3a3f78c01`.
- Worktree was clean after the content push.
- This report status update is recorded in a follow-up bookkeeping commit.

## Sub-agent session close status

Sub-agent `019f2909-65e1-7180-826e-561823208be9` completed and was closed.
Reviewer sub-agent `019f2918-f1a1-79f2-b9c3-4f4ed055ee9c` completed and was
closed.
