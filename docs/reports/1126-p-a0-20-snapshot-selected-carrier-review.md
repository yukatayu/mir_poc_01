# Report 1126 — P-A0-20 snapshot-selected carrier review

- Date: 2026-05-03 02:27 JST
- Author / agent: Codex
- Scope: review-only inspection of existing helper/test patterns for a proposed `LIF-13` snapshot-selected carrier
- Decision levels touched: no new normative decision; review findings against existing `L1`/`L2` carrier boundaries

## Objective

Review the existing helper patterns in:

- `scripts/current_l2_family_checker_support.py`
- `scripts/current_l2_family_acceptance_support.py`
- `scripts/current_l2_family_runtime_mirror_support.py`
- `scripts/alpha_lifetime_fallback_checker.py`
- `scripts/alpha_lifetime_fallback_acceptance.py`

and the current tests in:

- `scripts/tests/test_current_l2_family_acceptance_support.py`
- `scripts/tests/test_alpha_lifetime_fallback_acceptance.py`

to identify likely implementation pitfalls, missing tests, carrier-mixing risks, or scope-confinement gaps for the proposed `LIF-13` snapshot-selected carrier:

- `snapshot_scope = alpha-snapshot-selected-floor`
- `expected_snapshot.checked_snapshot_rows`
- `detached_noncore.snapshot_rows`

## Scope and assumptions

- This was a review-only task. No helper/test behavior was changed.
- Normative intent was read from `specs/13-type-system-lifetime-fallback.md`, especially the distinction between the existing acceptance floor and the deferred `LIF-13` selected-option snapshot semantics.
- The proposed carrier is evaluated as a new positive-carrier family, not as an extension of `reason_codes`, `acceptance_rows`, or `runtime_mirror`.
- The worktree already contained an untracked proposal implementation for the snapshot helper/tests plus a modified `LIF-13` sidecar, so the review was updated to inspect those concrete files rather than only hypothetical future edits.
- Findings focus on semantic correctness, carrier separation, scope confinement, and test adequacy, not style.

## Start state / dirty state

- The worktree already contained local proposal files related to this review:
  - `scripts/current_l2_family_snapshot_support.py`
  - `scripts/alpha_lifetime_fallback_snapshot.py`
  - `scripts/tests/test_current_l2_family_snapshot_support.py`
  - `scripts/tests/test_alpha_lifetime_fallback_snapshot.py`
  - modified `samples/alpha/lifetime-fallback/lif-13-snapshot_selected_anchor.expected.json`
- This report was added on top of that existing dirty state. No user changes were modified.

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/13-type-system-lifetime-fallback.md`
- `plan/00-index.md`
- `plan/39-type-system-freeze-roadmap.md`
- `samples/alpha/lifetime-fallback/README.md`
- `samples/alpha/lifetime-fallback/lif-02-fallback_extends_access_path.expected.json`
- `samples/alpha/lifetime-fallback/lif-03-nested_inherit_chain_valid.expected.json`
- `samples/alpha/lifetime-fallback/lif-04-plain_ref_does_not_inherit.expected.json`
- `samples/alpha/lifetime-fallback/lif-13-snapshot_selected_anchor.expected.json`
- `samples/alpha/lifetime-fallback/lif-13-snapshot_selected_anchor.mir`
- `scripts/current_l2_family_snapshot_support.py`
- `scripts/alpha_lifetime_fallback_snapshot.py`
- `scripts/tests/test_current_l2_family_snapshot_support.py`
- `scripts/tests/test_alpha_lifetime_fallback_snapshot.py`
- `docs/reports/1118-post-p-a0-17-widening-review-blocker.md`
- `docs/reports/1124-p-a0-19-remaining-positive-row-carrier-inventory-closeout.md`
- `docs/reports/review-1117-p-a0-17-checker-test-review.md`
- `docs/reports/TEMPLATE.md`

## Actions taken

1. Read the required repo entry docs and base specs in the mandated order.
2. Located the current normative wording for `LIF-13` and the recorded blocker that says it needs a selected-option snapshot carrier rather than reuse of the acceptance floor.
3. Inspected the shared checker, acceptance, and runtime-mirror helper implementations.
4. Inspected the Alpha lifetime checker/acceptance entrypoints and the current lifetime acceptance sidecars.
5. Inspected the current helper tests and Alpha lifetime helper tests to see what behaviors are already frozen and what remains untested.
6. Detected local proposal files in the worktree, inspected the proposed snapshot helper, snapshot tests, and modified `LIF-13` sidecar.
7. Ran the focused unittest subset for both the current helper/tests and the local snapshot proposal tests.
8. Recorded the review findings in this report.

## Files changed

- `docs/reports/1126-p-a0-20-snapshot-selected-carrier-review.md`

## Commands run

```bash
sed -n '1,220p' /home/yukatayu/.codex/skills/superpowers/skills/using-superpowers/SKILL.md
sed -n '1,220p' .agents/skills/discord-report/SKILL.md
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
sed -n '1,220p' README.md
sed -n '1,260p' Documentation.md
sed -n '1,220p' specs/00-document-map.md
sed -n '1,220p' specs/01-charter-and-decision-levels.md
sed -n '1,260p' specs/02-system-overview.md
sed -n '1,260p' specs/03-layer-model.md
sed -n '1,240p' specs/09-invariants-and-constraints.md
sed -n '1,220p' plan/00-index.md
rg -n "LIF-13|snapshot-selected|checked_snapshot_rows|snapshot_rows|alpha-snapshot-selected-floor|detached_noncore" specs plan scripts samples docs -S
git status --short
date '+%Y-%m-%d %H:%M:%S %z'
sed -n '260,380p' specs/13-type-system-lifetime-fallback.md
sed -n '90,160p' plan/39-type-system-freeze-roadmap.md
sed -n '1,240p' docs/reports/1124-p-a0-19-remaining-positive-row-carrier-inventory-closeout.md
sed -n '1,220p' docs/reports/1118-post-p-a0-17-widening-review-blocker.md
sed -n '1,220p' docs/reports/review-1117-p-a0-17-checker-test-review.md
sed -n '1,240p' scripts/current_l2_family_checker_support.py
sed -n '1,240p' scripts/current_l2_family_acceptance_support.py
sed -n '1,260p' scripts/current_l2_family_runtime_mirror_support.py
sed -n '1,240p' scripts/alpha_lifetime_fallback_checker.py
sed -n '1,260p' scripts/alpha_lifetime_fallback_acceptance.py
sed -n '1,220p' samples/alpha/lifetime-fallback/lif-13-snapshot_selected_anchor.expected.json
sed -n '1,220p' samples/alpha/lifetime-fallback/README.md
sed -n '1,320p' scripts/tests/test_current_l2_family_acceptance_support.py
sed -n '1,320p' scripts/tests/test_alpha_lifetime_fallback_acceptance.py
sed -n '1,320p' scripts/tests/test_current_l2_family_runtime_mirror_support.py
sed -n '1,280p' scripts/tests/test_current_l2_family_checker_support.py
sed -n '1,280p' scripts/tests/test_alpha_lifetime_fallback_checker.py
sed -n '220,340p' scripts/current_l2_family_runtime_mirror_support.py
nl -ba scripts/current_l2_family_acceptance_support.py | sed -n '1,220p'
nl -ba scripts/current_l2_family_runtime_mirror_support.py | sed -n '1,320p'
nl -ba scripts/current_l2_family_checker_support.py | sed -n '1,220p'
sed -n '1,220p' samples/alpha/lifetime-fallback/lif-02-fallback_extends_access_path.expected.json
sed -n '1,220p' samples/alpha/lifetime-fallback/lif-03-nested_inherit_chain_valid.expected.json
sed -n '1,220p' samples/alpha/lifetime-fallback/lif-04-plain_ref_does_not_inherit.expected.json
sed -n '1,220p' samples/alpha/lifetime-fallback/lif-13-snapshot_selected_anchor.mir
python3 -m unittest scripts.tests.test_current_l2_family_checker_support scripts.tests.test_current_l2_family_acceptance_support scripts.tests.test_current_l2_family_runtime_mirror_support scripts.tests.test_alpha_lifetime_fallback_checker scripts.tests.test_alpha_lifetime_fallback_acceptance
sed -n '1,260p' docs/reports/TEMPLATE.md
ls docs/reports | sort | tail -n 5
nl -ba scripts/tests/test_current_l2_family_acceptance_support.py | sed -n '1,320p'
nl -ba scripts/tests/test_alpha_lifetime_fallback_acceptance.py | sed -n '1,320p'
nl -ba scripts/tests/test_current_l2_family_runtime_mirror_support.py | sed -n '1,340p'
nl -ba scripts/tests/test_current_l2_family_checker_support.py | sed -n '1,280p'
nl -ba scripts/tests/test_alpha_lifetime_fallback_checker.py | sed -n '1,280p'
python3 scripts/validate_docs.py
git diff --check
git status --short
sed -n '1,260p' scripts/current_l2_family_snapshot_support.py
sed -n '1,220p' scripts/alpha_lifetime_fallback_snapshot.py
sed -n '1,320p' scripts/tests/test_current_l2_family_snapshot_support.py
sed -n '1,320p' scripts/tests/test_alpha_lifetime_fallback_snapshot.py
sed -n '1,260p' samples/alpha/lifetime-fallback/lif-13-snapshot_selected_anchor.expected.json
python3 -m unittest scripts.tests.test_current_l2_family_snapshot_support scripts.tests.test_alpha_lifetime_fallback_snapshot
nl -ba scripts/current_l2_family_snapshot_support.py | sed -n '1,240p'
nl -ba scripts/tests/test_current_l2_family_snapshot_support.py | sed -n '1,320p'
nl -ba scripts/tests/test_alpha_lifetime_fallback_snapshot.py | sed -n '1,320p'
nl -ba samples/alpha/lifetime-fallback/lif-13-snapshot_selected_anchor.expected.json | sed -n '1,260p'
nl -ba specs/13-type-system-lifetime-fallback.md | sed -n '288,360p'
nl -ba plan/39-type-system-freeze-roadmap.md | sed -n '100,140p'
git diff --no-index -- /dev/null scripts/current_l2_family_snapshot_support.py
git diff --no-index -- /dev/null scripts/alpha_lifetime_fallback_snapshot.py
```

## Evidence / outputs / test results

- `python3 -m unittest scripts.tests.test_current_l2_family_checker_support scripts.tests.test_current_l2_family_acceptance_support scripts.tests.test_current_l2_family_runtime_mirror_support scripts.tests.test_alpha_lifetime_fallback_checker scripts.tests.test_alpha_lifetime_fallback_acceptance`
  passed: `Ran 25 tests in 0.024s`, `OK`.
- `python3 -m unittest scripts.tests.test_current_l2_family_snapshot_support scripts.tests.test_alpha_lifetime_fallback_snapshot`
  passed: `Ran 11 tests in 0.020s`, `OK`.
- `python3 scripts/validate_docs.py`
  reported `Documentation scaffold looks complete.` and `Found 1127 numbered report(s).`
- `git diff --check`
  returned no output.
- `git status --short`
  showed local proposal files and a modified `LIF-13` sidecar in addition to this new report.
- `date '+%Y-%m-%d %H:%M:%S %z'`
  returned `2026-05-03 02:27:35 +0900`.

## What changed in understanding

- The existing helper stack already establishes three distinct comparison families:
  `reason_codes`, `acceptance_rows`, and `runtime_mirror`.
  `LIF-13` will be the first case that adds a fourth family under the same repository line while still living in `detached_noncore`.
- The main risk is no longer hypothetical copy-paste. The worktree already contains a concrete snapshot helper/test proposal, and its largest issue is that it actualizes a new helper-local carrier without the accompanying spec/roadmap updates that the current stop-line still requires.
- The proposal already adds one-direction carrier-separation tests for the new helper, so the remaining test gaps are narrower: reciprocal regression coverage on the existing acceptance helper, and explicit-scope failure coverage.

## Open questions

- Is `P-A0-20` intended to be the first explicit actualization after `P-A0-19`, or was this proposal meant to stay local until the normative/spec snapshot is widened in the same task?
- Should a snapshot-selected helper require `checked_snapshot_scope` explicitly whenever `checked_snapshot_rows` is present, instead of inheriting the helper default?
- Should malformed or unsupported `expected_snapshot.checked_snapshot_rows` fail closed, or should they keep the current `out_of_scope` behavior?

## Suggested next prompt

Review the local `P-A0-20` snapshot-helper proposal against the current `specs/13` / `plan/39` stop-line, then either reject it as premature or land it together with the exact normative/snapshot updates and the missing scope/confinement tests.

## Plan update status

`plan/` 更新不要: this task was review-only and did not change repository memory.

## Documentation.md update status

`Documentation.md` 更新不要: the review found implementation/test risks, not a top-level snapshot wording mismatch.

## progress.md update status

`progress.md` 更新不要: this task did not change package status or validation floor.

## tasks.md update status

`tasks.md` 更新不要: this task did not change the queued package map.

## samples_progress.md update status

`samples_progress.md` 更新不要: no sample/runtime/helper floor was widened.

## Reviewer findings and follow-up

- Finding 1:
  the proposal already widens `LIF-13` from docs-first blocker inventory into a concrete helper-local carrier (`scripts/current_l2_family_snapshot_support.py:103-165`, `scripts/alpha_lifetime_fallback_snapshot.py:1-29`, `samples/alpha/lifetime-fallback/lif-13-snapshot_selected_anchor.expected.json:16-45`), but the normative and roadmap sources still say `LIF-13` is only blocker inventory and not yet actualized (`specs/13-type-system-lifetime-fallback.md:333-360`, `plan/39-type-system-freeze-roadmap.md:105-117`).
  This is a spec/implementation mismatch and an overclaim risk: the code/test side has created a new helper-local floor before the repo’s explicit stop-line has been widened in the same task.
- follow-up:
  either keep the proposal out of tree until `P-A0-20` is promoted, or update the normative/snapshot documents in the same package so the new carrier is explicitly authorized and scoped.

- Finding 2:
  the snapshot helper inherits the current filter-first pattern (`scripts/current_l2_family_snapshot_support.py:71-96`), and the new tests explicitly freeze `unsupported kind => out_of_scope` for the snapshot floor (`scripts/tests/test_current_l2_family_snapshot_support.py:104-139`).
  For a dedicated single-row `LIF-13` carrier, this is likely too permissive: a mistyped `kind` or a copied acceptance-style row can disappear into `out_of_scope` rather than failing as malformed snapshot evidence.
- follow-up:
  decide whether the snapshot helper should fail closed on unsupported `expected_snapshot` / `snapshot_rows` kinds, and add a test that locks whichever rule is chosen.

- Finding 3:
  the proposal adds good one-direction coexistence coverage for the snapshot helper ignoring `reason_codes` and `acceptance_rows` (`scripts/tests/test_current_l2_family_snapshot_support.py:228-279`, `scripts/tests/test_alpha_lifetime_fallback_snapshot.py:41-94`), but it does not add the reciprocal regression on the existing acceptance helper.
  Once `snapshot_rows` exists inside `detached_noncore`, there should also be a test proving that `run_family_acceptance_checker` still ignores `snapshot_rows`; otherwise a later refactor can reintroduce carrier mixing without any failing test on the already-landed acceptance floor.
- follow-up:
  add a focused acceptance-support test with live `snapshot_scope` / `snapshot_rows` noise and assert the acceptance helper remains `matched`.

- Finding 4:
  explicit-scope enforcement is only partially frozen.
  The helper rejects wrong-floor scope (`scripts/current_l2_family_snapshot_support.py:129-159`) and the tests cover that (`scripts/tests/test_current_l2_family_snapshot_support.py:141-184`, `scripts/tests/test_alpha_lifetime_fallback_snapshot.py:96-132`), but there is no test for the two more brittle cases:
  fixture rows present with missing `checked_snapshot_scope`, and artifact rows present with missing `snapshot_scope`.
  Because the helper currently defaults fixture scope from `expected_scope` (`scripts/current_l2_family_snapshot_support.py:125-139`), the fixture-side omission case would silently pass today.
- follow-up:
  add explicit tests for both missing-scope cases, and decide whether fixture-side scope omission should raise/fail instead of inheriting the helper default.

## Skipped validations and reasons

- No runtime/Cargo/sample-runner validations were executed because this was limited to helper/test and sidecar review.
- No source edits were made to the user’s local snapshot proposal files; the task remained review-only.

## Commit / push status

Pending at report write.
No commit created.
No push performed.

## Sub-agent session close status

- No sub-agent sessions were opened for this review-only task.
