# 0612 — phase6 actual checker / runtime skeleton first tranche package

## Objective

Phase 6 front-half の next actualization として、`mir-semantics` / `mir-runtime` をまたぐ
current L2 checker/runtime first tranche を non-production minimal cut で compile-ready にする。

## Scope and assumptions

- parser side の accepted floor は `mir-ast` stage 1 option/chain と stage 2 try/fallback structural carrierに留める。
- actual parser-to-`Program` lowering、stage 3 request / predicate reconnect、richer host interface、final public runtime / checker API、formal hook concrete binding は今回 fixed しない。
- `plan/07-parser-free-poc-stack.md` と `docs/research_abstract/README.md` は今回の current line 変更に対して追加 mirror を要しないと判断し、**plan/07 更新不要**、**research_abstract/README 更新不要** とした。
- checker/runtime first tranche では、parser bridge input と semantic `Program` を別 artifact とみなし、mismatch は fail-closed に止める。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/287-current-l2-minimal-verifier-handoff-surface-ready-minimal-parser-subset-freeze-comparison.md`
- `specs/examples/288-current-l2-minimal-parser-subset-freeze-ready-minimal-parser-subset-freeze-threshold.md`
- `specs/examples/289-current-l2-minimal-parser-subset-freeze-ready-parser-to-checker-reconnect-freeze-comparison.md`
- `specs/examples/290-current-l2-parser-to-checker-reconnect-freeze-ready-minimal-parser-to-checker-reconnect-freeze-threshold.md`
- `specs/examples/299-current-l2-phase5-proof-protocol-runtime-policy-handoff-closeout-ready-phase6-actual-parser-ast-carrier-first-tranche-comparison.md`
- `specs/examples/300-current-l2-phase6-actual-parser-ast-carrier-first-tranche-ready-minimal-phase6-actual-parser-ast-carrier-first-tranche-threshold.md`
- `docs/reports/0611-phase6-actual-parser-ast-carrier-first-tranche-package.md`
- `crates/mir-runtime/src/lib.rs`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`

## Actions taken

1. Phase 6 next cut を再確認し、actual parser first tranche の次段では full runtime widening ではなく、program-level semantic entry + thin runtime orchestrator の first tranche に留めるのが current minimum だと整理した。
2. TDD の red として `crates/mir-runtime/tests/current_l2_runtime_skeleton.rs` を追加し、stage 1 bridge / stage 2 bridge の happy path を要求したうえで `cargo test -p mir-runtime --test current_l2_runtime_skeleton` を実行し、`unresolved import mir_runtime::current_l2` compile failure を確認した。
3. `crates/mir-semantics/src/lib.rs` に `static_gate_program_detailed`、`DirectStyleEvaluator::from_program`、`run_program_to_completion` を追加し、`CurrentL2Fixture` wrapper から program-level entry を薄く切り出した。
4. `crates/mir-semantics/src/harness.rs` に `FixtureHostStub::run_program` を追加し、host plan validation / oracle coverage check を維持したまま semantic `Program` 直実行 path を足した。
5. `crates/mir-runtime/src/current_l2.rs` を追加し、`Program + FixtureHostPlan + optional parser bridge input` を受ける non-production thin orchestrator を actualize した。checker floor では stage 1 reconnect summary、stage 2 try/rollback structural summary、semantic static gate report を返す。
6. reviewer 指摘を受け、parser bridge input と semantic `Program` が食い違うときは combined report を返さず fail-closed に止める validation を追加した。あわせて stage 1 capability strengthening summary を existing reconnect helper contract と同じ `read -> write` floor に戻し、regression tests を追加した。
7. `specs/examples/301...302`、`Documentation.md`、`progress.md`、`tasks.md`、`specs/00-document-map.md`、`plan/09`、`plan/10`、`plan/11`、`plan/12`、`plan/17`、`plan/90`、`docs/research_abstract/phase6-compile-ready-minimal-actualization.md` を current mainline へ合わせて更新した。

## Files changed

- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-runtime/Cargo.toml`
- `crates/mir-runtime/src/lib.rs`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/tests/current_l2_runtime_skeleton.rs`
- `specs/examples/301-current-l2-phase6-actual-parser-ast-carrier-first-tranche-ready-phase6-actual-checker-runtime-skeleton-first-tranche-comparison.md`
- `specs/examples/302-current-l2-phase6-actual-checker-runtime-skeleton-first-tranche-ready-minimal-phase6-actual-checker-runtime-skeleton-first-tranche-threshold.md`
- `specs/00-document-map.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `docs/reports/0612-phase6-actual-checker-runtime-skeleton-first-tranche-package.md`

## Commands run and exact outputs

```bash
cargo test -p mir-runtime --test current_l2_runtime_skeleton
cargo test -p mir-runtime
cargo test -p mir-semantics --test current_l2_minimal_interpreter
cargo test -p mir-ast
python3 scripts/validate_docs.py
git diff --check
git status --short
```

- `cargo test -p mir-runtime --test current_l2_runtime_skeleton`
  - red phase では `error[E0432]: unresolved import mir_runtime::current_l2`
- `cargo test -p mir-runtime`
  - current L2 runtime skeleton suite を含めて通過
- `cargo test -p mir-semantics --test current_l2_minimal_interpreter`
  - current parser-free interpreter suite が通過
- `cargo test -p mir-ast`
  - stage 1 / stage 2 parser carrier suite を含めて通過
- `python3 scripts/validate_docs.py`
  - docs scaffold と numbered report count の整合を確認
- `git diff --check`
  - no output
- `git status --short`
  - task close 直前は Task 2 関連の edited/new file が並ぶ uncommitted 状態

## Evidence / outputs / test results

- red verification:
  - `cargo test -p mir-runtime --test current_l2_runtime_skeleton`
  - `error[E0432]: unresolved import mir_runtime::current_l2`
- green verification:
  - `cargo test -p mir-runtime --test current_l2_runtime_skeleton`
  - `4 passed; 0 failed`
- reviewer re-check:
  - parser/semantic mismatch guard と stage 1 capability-strengthening freeze mismatch は解消済み

## What changed in understanding

- Phase 6 checker/runtime first tranche で必要なのは actual parser lowering ではなく、semantic `Program` と parser evidence の boundary を明示した thin orchestrator だった。
- `mir-semantics` program-level entry は fixture wrapper を壊さずに切り出せる。`CurrentL2Fixture` は compile path の唯一入口ではなくてよい。
- parser bridge input を consistency guard なしで combined report に混ぜると boundary bug になるため、first tranche でも fail-closed validation が必要だった。

## Open questions

- tool-neutral formal hook を theorem-side / model-check sideのどの detached artifact family から first tranche 化するか
- actual parser-to-`Program` lowering を second tranche widen でどこまで reopen するか
- semantic static gate の capability lattice と stage 1 reconnect summary floor を later にどこまで揃えるか

## Suggested next prompt

```text
Phase 6 compile-ready verification and formal hook first tranche を進め、tool-neutral relation exporter と cargo / smoke gate を current checkpoint minimum まで揃えてください。
```
