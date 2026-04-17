# Report 0717 — inspect current l2 source sample pipeline

- Date: 2026-04-17T01:33:40.524500Z
- Author / agent: Codex (GPT-5)
- Scope: Narrow inspection of the current current-L2 source-text sample pipeline only
- Decision levels touched: None changed; inspection only

## 1. Objective

Identify, from the current repository state, which source-text constructs are presently parsed / lowered / runner-supported in the current-L2 source-sample path, which files implement that path, and what the minimum edits would be to add a few new runnable source samples without widening grammar more than necessary.

## 2. Scope and assumptions

- Read only the mandatory top-level docs plus the current-L2 source-sample / parser-free PoC documents needed for this question.
- Treated `specs/` and `plan/` as design context, but used Rust implementation plus tests as the source of truth for current accepted constructs.
- Interpreted “runner-supported” as accepted by `run_current_l2_source_sample` and executable through the current runtime skeleton; noted the narrower formal-hook status separately where relevant.
- No normative statements were edited.
- `plan/` 更新不要
- `progress.md` 更新不要
- `tasks.md` 更新不要

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md` (current-L2/source-sample-related anchors only)
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/06-surface-notation-status.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `samples/current-l2/README.md`
- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_source_lowering.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- `crates/mir-runtime/tests/current_l2_runtime_skeleton.rs`
- `scripts/current_l2_source_sample_regression.py`

## 4. Actions taken

1. Read the required top-level docs and narrowed to current-L2 source-sample / parser-free PoC documents.
2. Located the code path from source text to semantic `Program` to runtime runner.
3. Inspected the accepted construct floor in `mir-ast` stage 1 / stage 2 / stage 3 predicate helpers.
4. Inspected the fixed-subset lowerer and sample runner in `mir-runtime`.
5. Checked authored sample inventory and ran the source-lowering and source-sample-runner tests.
6. Wrote this report with implementation-backed findings.

## 5. Files changed

- `docs/reports/0717-inspect-current-l2-source-sample-pipeline.md` (new report only)

## 6. Commands run and exact outputs

1. `python3 scripts/current_l2_source_sample_regression.py inventory`
   - Output: current authored inventory listed 14 present source samples: `e1`, `e2`, `e3`, `e4`, `e14`, `e15`, `e16`, `e13`, `e19`, `e21`, `e22`, `e18`, `e20`, `e23`.
   - Output: `e3-option-admit-chain` remains `formal_hook = not_reached_guarded`; all other authored rows matched their expected file presence.
2. `cargo test -p mir-runtime --test current_l2_source_lowering`
   - Output: `14 passed; 0 failed`.
3. `cargo test -p mir-runtime --test current_l2_source_sample_runner`
   - Output: `16 passed; 0 failed`.
4. `date '+%Y-%m-%d %H:%M:%S %Z'`
   - Output: `2026-04-17 10:33:48 JST`

## 7. Evidence / findings

- The current source-sample runner accepts exactly 14 hard-coded sample paths and rejects files outside that accepted set.
- The current lowerer accepts only a narrow fixed subset: `place { ... }`, one `try { ... } fallback { ... }`, `atomic_cut`, `option ... on ... capability ... lease ...` with optional inline `admit`, `chain name = head`, `fallback successor @ lineage(pred -> succ)`, `perform op on target`, `perform op via chain_ref`, and single-line `require` / `ensure` clauses with minimal predicate fragments.
- Multiline request-clause suites are documented and test-spiked in `mir-ast`, but the actual source lowerer still rejects them.
- The stage-1 parser bridge only covers option/chain/lineage rows without `admit`; as soon as inline `admit` appears, lowering still works but the stage-1 bridge is intentionally disabled for that sample.
- The stage-2 parser bridge tracks at most one try/fallback block and reduces direct child statement heads to `AtomicCut` vs `Other`; nested blocks are collapsed structurally rather than fully parsed.
- Minimal-addition path for new runnable samples is to stay inside the existing fixed subset and add new authored rows plus accepted-set / docs / regression inventory wiring, rather than widening grammar.

## 8. What changed in understanding

- The repository docs describe a broader companion notation than the actual lowering floor. The real runnable subset is narrower and is fixed in `crates/mir-runtime/src/current_l2.rs`, not in the prose examples alone.
- “Current L2 companion notation” includes multiline clauses and hanging lineage presentation, but the runnable source-sample path currently only accepts inline lineage rows and single-line clauses.
- The sample pipeline is intentionally split:
  - parser carrier floor in `mir-ast`
  - source-to-`Program` lowering and sample runner in `mir-runtime`
  - semantic execution and host-plan runtime in `mir-semantics`
  - repo-local inventory/regression orchestration in Python

## 9. Open questions

- If new runnable samples are added, should they target runner-only coverage, or should they also be admitted into the formal-hook smoke subset? Current `e3` shows those are distinct gates.
- If the next desired samples need multiline `require:` / `ensure:` blocks, should the repo accept a minimal lowering widening now, or should those remain parser-spike-only and new samples stay single-line?

## 10. Suggested next prompt

Map 3 candidate new source samples onto the existing fixed subset and tell me exactly which ones can be added with no lowering changes, plus the smallest code/doc edits for each.
