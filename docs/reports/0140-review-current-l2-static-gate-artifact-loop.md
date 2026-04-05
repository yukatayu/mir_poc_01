# Report 0140 — review current L2 static gate artifact loop

## 1. Title and identifier

- Report 0140
- review current L2 static gate artifact loop

## 2. Objective

Review the current uncommitted diff for the static gate artifact loop task with emphasis on:

- accidental public API promotion or boundary leaks
- mismatch between docs and code
- traceability / maintenance omissions
- semantic overstatement beyond current code

## 3. Scope and assumptions

- Scope was limited to the task files named in the review request plus the necessary existing anchors needed to verify semantics and boundary behavior.
- Review was performed against the current uncommitted worktree on 2026-04-05.
- No normative statement was edited in this review task.
- `plan/ 更新不要`
- `progress.md 更新不要`

## 4. Documents consulted

1. `README.md`
2. `Documentation.md`
3. `specs/00-document-map.md`
4. `specs/01-charter-and-decision-levels.md`
5. `specs/02-system-overview.md`
6. `specs/03-layer-model.md`
7. `specs/09-invariants-and-constraints.md`
8. `plan/00-index.md`
9. `plan/07-parser-free-poc-stack.md`
10. `plan/09-helper-stack-and-responsibility-map.md`
11. `plan/11-roadmap-near-term.md`
12. `plan/12-open-problems-and-risks.md`
13. `plan/15-current-l2-fixture-authoring-template.md`
14. `specs/examples/32-current-l2-static-gate-artifact-loop.md`
15. `docs/reports/0139-current-l2-static-gate-artifact-loop.md`
16. `crates/mir-semantics/src/lib.rs`
17. `crates/mir-semantics/src/harness.rs`

## 5. Actions taken

1. Read the repository instructions and required normative documents in the prescribed order.
2. Inspected the targeted new example, support, test, script, spec, mirror, and report files.
3. Compared the new static gate helper cut against the existing aggregate helper pattern and `lib.rs` / `harness.rs` public surface.
4. Verified the stability of `checker_core.reasons` by constructing a temporary malformed fixture with multiple static-gate failures and running the new emitter repeatedly across fresh processes.
5. Checked document-map traceability for the new anchors and compared it against the updated helper inventory in `Documentation.md` and `plan/`.

## 6. Evidence / outputs / test results

```text
$ cargo run -q -p mir-semantics --example current_l2_emit_static_gate -- "$tmpdir/multi-static.json" | python3 -c 'import json,sys; print(json.load(sys.stdin)["checker_core"]["reasons"])'
['missing lineage assertion for primary -> mirror', 'lineage assertion does not describe primary -> archive']
['lineage assertion does not describe primary -> archive', 'missing lineage assertion for primary -> mirror']
```

```text
$ nl -ba specs/00-document-map.md | sed -n '235,263p'
235	## 実装 anchor
...
243	- `crates/mir-semantics/examples/current_l2_emit_detached_bundle.rs`
245	- `crates/mir-semantics/examples/current_l2_emit_detached_aggregate.rs`
247	- `crates/mir-semantics/examples/support/current_l2_detached_aggregate_support.rs`
249	- `scripts/current_l2_diff_detached_artifacts.py`
251	- `scripts/current_l2_diff_detached_aggregates.py`
253	- `scripts/current_l2_detached_loop.py`
255	- `scripts/current_l2_scaffold_fixture.py`
257	- `scripts/current_l2_detached_loop.py`
260	- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
```

## 7. What changed in understanding

- The new helper cut does keep the static gate artifact outside `lib.rs` / `harness.rs`; there is no direct crate-level public API promotion in this diff.
- However, the new exact-compare contract for `checker_core.reasons` exposes an existing nondeterministic iteration order from `static_gate_detailed()`, so the artifact loop is not yet semantically stable for multi-reason fixtures.
- The mirror docs mostly describe the new helper correctly, but `specs/00-document-map.md` still does not map the new code anchors under `## 実装 anchor`, which weakens repo-bootstrap traceability for future agents.

## 8. Open questions

- Should the static gate compare contract sort / normalize `reasons`, or should `static_gate_detailed()` itself emit reasons in deterministic program order?
- Should `specs/00-document-map.md` treat helper/test anchors as mandatory when a new detached-loop helper is added, to avoid this class of omission repeating?

## 9. Suggested next prompt

`Fix the static gate artifact loop review findings: make checker_core.reasons deterministic for multi-reason fixtures, add a regression test that proves the order is stable, and update specs/00-document-map.md implementation anchors for the new static gate files without widening public API.`
