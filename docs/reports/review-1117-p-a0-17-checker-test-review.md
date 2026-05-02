# Report 1117 — P-A0-17 Checker/Test Review

1. Title and identifier

- Report 1117 — `P-A0-17` checker/test review
- Date: 2026-05-02 15:13 JST
- Author / agent: Codex

2. Objective

- Review `scripts/current_l2_family_checker_support.py`, `scripts/alpha_lifetime_fallback_checker.py`, `scripts/alpha_contract_variance_checker.py`, their existing tests under `scripts/tests/`, and the six positive sidecars `LIF-02/03/11` and `VAR-01/04/06`.
- Focus on helper scope confinement, on preventing accept-side rows from reusing `reason_codes`, and on missing test coverage for `matched`, `out_of_scope`, `sample_expected_acceptance_rows_missing`, `mismatch`, and `scope_mismatch`.

3. Scope and assumptions

- This was a review-only task. No checker/runtime behavior was modified.
- Normative reading was taken from `specs/13`, `specs/14`, and the repository stop-line recorded for `P-A0-17`.
- `P-A0-17` was evaluated against the current repository state, where `P-A0-16` is closed and accept-side evidence is still documented as blocked/unpromoted.

4. Start state / dirty state

- Worktree started clean for tracked files (`git status --short --untracked-files=no` returned no output).
- No pre-existing dirty state was observed in the reviewed files.

5. Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/13-type-system-lifetime-fallback.md`
- `specs/14-contract-subtyping-layer-compatibility.md`
- `specs/17-mirrorea-spaces-alpha-scope.md`
- `plan/00-index.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/39-type-system-freeze-roadmap.md`
- `plan/40-layer-compatibility-freeze-roadmap.md`
- `progress.md`
- `tasks.md`
- `docs/reports/1116-p-a0-16-checker-widening-closeout.md`

6. Actions taken

- Read the shared family-checker helper and both Alpha-0 checker entrypoints.
- Read the existing checker tests for the shared helper, lifetime checker, and contract-variance checker.
- Read the six positive sidecars `lif-02`, `lif-03`, `lif-11`, `var-01`, `var-04`, and `var-06`.
- Compared current code/test behavior with the repository memory that says accept-side evidence is not yet promoted.
- Ran the focused Python unittest subset for the shared helper and the two Alpha-0 checker test modules.

7. Files changed

- `docs/reports/1117-p-a0-17-checker-test-review.md`

8. Commands run

```bash
sed -n '1,220p' /home/yukatayu/.codex/skills/superpowers/skills/using-superpowers/SKILL.md
sed -n '1,220p' .agents/skills/discord-report/SKILL.md
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
sed -n '1,220p' README.md
sed -n '1,260p' Documentation.md
sed -n '1,220p' specs/00-document-map.md
sed -n '1,240p' specs/01-charter-and-decision-levels.md
sed -n '1,240p' specs/02-system-overview.md
sed -n '1,240p' specs/03-layer-model.md
sed -n '1,260p' specs/09-invariants-and-constraints.md
sed -n '1,220p' plan/00-index.md
sed -n '1,260p' specs/13-type-system-lifetime-fallback.md
sed -n '1,260p' specs/14-contract-subtyping-layer-compatibility.md
sed -n '1,240p' specs/17-mirrorea-spaces-alpha-scope.md
sed -n '1,240p' plan/09-helper-stack-and-responsibility-map.md
sed -n '1,220p' plan/39-type-system-freeze-roadmap.md
sed -n '1,220p' plan/40-layer-compatibility-freeze-roadmap.md
sed -n '1,240p' scripts/current_l2_family_checker_support.py
sed -n '1,220p' scripts/alpha_lifetime_fallback_checker.py
sed -n '1,220p' scripts/alpha_contract_variance_checker.py
sed -n '1,260p' scripts/tests/test_current_l2_family_checker_support.py
sed -n '1,260p' scripts/tests/test_alpha_lifetime_fallback_checker.py
sed -n '1,260p' scripts/tests/test_alpha_contract_variance_checker.py
python3 -m unittest scripts.tests.test_current_l2_family_checker_support scripts.tests.test_alpha_lifetime_fallback_checker scripts.tests.test_alpha_contract_variance_checker
python3 scripts/validate_docs.py
git diff --check
date '+%Y-%m-%d %H:%M %Z'
git status --short --untracked-files=no
```

9. Evidence / outputs / test results

- `python3 -m unittest scripts.tests.test_current_l2_family_checker_support scripts.tests.test_alpha_lifetime_fallback_checker scripts.tests.test_alpha_contract_variance_checker`
  passed: `Ran 15 tests in 0.016s`, `OK`.
- `python3 scripts/validate_docs.py`
  reported `Documentation scaffold looks complete.` and `Found 1119 numbered report(s).`
- `git diff --check`
  returned no output.
- `date '+%Y-%m-%d %H:%M %Z'`
  returned `2026-05-02 15:13 JST`.
- `git status --short --untracked-files=no`
  returned no output.

10. What changed in understanding

- The current codebase still matches the documented stop-line: the helper stack is intentionally negative-row-only and does not yet carry accept-side evidence.
- The main review risk is not a hidden regression in the current negative floor; it is that `P-A0-17` would be easy to overclaim by extending the same helper/test pattern without first introducing a distinct accept-side carrier and status matrix.

11. Open questions

- Should `P-A0-17` introduce a new accept-side fixture/artifact carrier in the shared helper, or should positive rows continue to wait for a parser/runtime-backed bridge?
- What exact artifact field and scope name should be used for accept-side evidence so it is structurally distinct from `detached_noncore.reason_codes`?
- Should positive-row comparisons be exact ordered row equality, or should the carrier define canonical sorting/keys before comparison?

12. Suggested next prompt

- Design the explicit accept-side carrier for `P-A0-17` first, then add real-sidecar tests for `matched`, `out_of_scope`, `sample_expected_acceptance_rows_missing`, `mismatch`, and `scope_mismatch` before promoting any positive LIF/VAR row.

13. `plan/` update status

- `plan/` 更新不要. This task was review-only and did not change repository memory.

14. `Documentation.md` update status

- `Documentation.md` 更新不要. The review found missing promotion work, not a mismatch in current snapshot wording.

15. `progress.md` update status

- `progress.md` 更新不要. The current snapshot already states that safe `P-A0-17` is not promoted yet.

16. `tasks.md` update status

- `tasks.md` 更新不要. The current task map already records `P-A0-17` as blocked on an explicit accept-side carrier or parser/runtime bridge.

17. `samples_progress.md` update status

- `samples_progress.md` 更新不要. No sample implementation or validation floor changed.

18. Reviewer findings and follow-up

- No additional reviewer was invoked for this review-only task.
- Follow-up is captured directly in this report: the concrete findings are missing accept-side carrier separation and missing acceptance-path tests.

19. Skipped validations and reasons

- No Rust/runtime/sample runner validations were executed because the task was limited to Python helper/test review.
- No source-hierarchy or runtime/Cargo validation was rerun because the task was limited to checker/test review plus a new report.

20. Commit / push status

- No commit created.
- No push performed.

21. Sub-agent session close status

- No sub-agent sessions were opened for this task.
