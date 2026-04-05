# 0186 — try/rollback structural floor と restore-scope boundary

## Objective

`TryFallback` / `AtomicCut` まわりの current runtime reading を representative runtime fixture で 1 段 concretize し、
first checker cut には structural floor だけを残し、`place_anchor == current_place` gate と restore scope を runtime / proof boundary に残す current judgment を docs / plan / tests へ揃える。

## Scope and assumptions

- current L2 parser-free PoC を前提にする。
- runtime semantics の全面 redesign は行わない。
- final type system / theorem prover boundary 自体はまだ固定しない。
- `TryFallback` / `AtomicCut` の checker-boundary 比較と、その representative runtime actualization だけを扱う。

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/04-current-l2-step-semantics.md`
- `specs/examples/09-current-l2-bundle-loader.md`
- `specs/examples/10-current-l2-batch-runner.md`
- `specs/examples/11-current-l2-selection-helper.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/39-current-l2-static-reason-code-readiness-scan.md`
- `specs/examples/44-current-l2-checked-reasons-coexistence-and-shrink-policy.md`
- `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md`
- `plan/01-status-at-a-glance.md`
- `plan/04-core-semantics-current-l2.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `crates/mir-ast/tests/fixtures/current-l2/e2-try-fallback.json`
- `crates/mir-ast/tests/fixtures/current-l2/e21-try-atomic-cut-frontier.json`

## Actions taken

1. `TryFallback` / `AtomicCut` の current code anchor を再確認し、rollback frame が `place_anchor` と whole `place_store` snapshot を保持し、`AtomicCut` frontier update が `place_anchor == current_place` で gate されることを確認した。
2. runtime contrast fixture `e22-try-atomic-cut-place-mismatch` と sidecar を追加し、nested place 内 `AtomicCut` が event-only に留まり、rollback が try-entry snapshot へ戻る case を actual corpus に入れた。
3. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs` に focused runtime test を追加し、event sequence、final `place_store`、directory/profile counts を `e22` を含む current corpus に合わせて更新した。
4. representative examples / AST fixture schema / step semantics / bundle-loader / batch-runner / selection helper を更新し、E22 と runtime contrast reading を human-facing docs に反映した。
5. `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md` と新規文書 `specs/examples/51-current-l2-try-rollback-structural-floor-and-restore-scope.md` を追加し、checker cut には structural floor だけを残し、dynamic gate と restore scope を runtime / proof boundary に残す current judgment を固定した。
6. readiness scan 実測に合わせ、stable cluster 8 kind / 9 fixture と capability floor `2` を current state docs / plan / progress に反映した。

## Files changed

- `crates/mir-ast/tests/fixtures/current-l2/e22-try-atomic-cut-place-mismatch.json`
- `crates/mir-ast/tests/fixtures/current-l2/e22-try-atomic-cut-place-mismatch.host-plan.json`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/04-current-l2-step-semantics.md`
- `specs/examples/09-current-l2-bundle-loader.md`
- `specs/examples/10-current-l2-batch-runner.md`
- `specs/examples/11-current-l2-selection-helper.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/39-current-l2-static-reason-code-readiness-scan.md`
- `specs/examples/44-current-l2-checked-reasons-coexistence-and-shrink-policy.md`
- `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md`
- `specs/examples/51-current-l2-try-rollback-structural-floor-and-restore-scope.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/01-status-at-a-glance.md`
- `plan/04-core-semantics-current-l2.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Commands run

```bash
cargo test -p mir-semantics
python3 scripts/current_l2_detached_loop.py smoke-fixture crates/mir-ast/tests/fixtures/current-l2/e22-try-atomic-cut-place-mismatch.json --reference-fixture crates/mir-ast/tests/fixtures/current-l2/e21-try-atomic-cut-frontier.json --run-label e22-atomic-cut-mismatch --reference-label e21-atomic-cut-frontier --overwrite
python3 scripts/current_l2_detached_loop.py scan-reason-code-readiness crates/mir-ast/tests/fixtures/current-l2 --run-label readiness-after-e22 --overwrite
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- `cargo test -p mir-semantics`
  - unit tests `2` pass
  - detached aggregate support tests `2` pass
  - detached bundle support tests `2` pass
  - `current_l2_minimal_interpreter` tests `46` pass
  - static gate support tests `8` pass
- `smoke-fixture ... e22 ... --reference-fixture ... e21 ...`
  - bundle diff では `payload_core.event_kinds` だけが差分となり、`e21` だけが second `perform-success` を含む
  - aggregate diff では `e22` 単体 run と reference run の coarse summary 差分だけが出る
- `scan-reason-code-readiness`
  - static-only fixtures scanned: `11`
  - runtime fixtures skipped: `11`
  - fixtures with checked_reasons: `9`
  - fixtures with reason_codes suggestions: `9`
  - fixtures with checked_reason_codes: `9`
  - fixtures with stable coexistence anchors: `9`
  - `same_lineage_evidence_floor: 4`
  - `capability_strengthening_floor: 2`
  - `missing_option_structure_floor: 3`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 187 numbered report(s).`
- `git diff --check`
  - 無出力

## What changed in understanding

1. `AtomicCut` event の存在だけでは rollback frontier update を意味しないことが、`e21` / `e22` の contrast で runtime corpus 上に固定された。
2. current parser-free PoC の rollback locality は、user-facing explanation としては place-aware に読めても、actual code anchor は whole-store snapshot restore である。したがって checker cut には structural floor だけを入れ、gate / restore scope は runtime / proof boundary に残す方が source-backed である。
3. readiness scan の現況は、stable cluster 8 kind を覆う 9 fixture と capability floor `2` まで進んでおり、古い `8 fixture` / `capability 1` 表現は current state とずれていた。

## Open questions

- whole-store snapshot restore を将来 place-local restore scope へ refinement する必要があるか。
- `place_anchor == current_place` gate を future checker / typed carrier に一部持ち上げる価値があるか。
- rollback locality を theorem prover / model checker 向けにどう有限化するか。
- first checker cut の public helper / API をどこに置くか。

## Suggested next prompt

`TryFallback` / `AtomicCut` の structural floor を current first checker spike family とどう接続するかを、same-lineage / missing-option / capability の existing checker support helper を前提に narrow に比較し、必要なら helper-local fourth checker spike を切ってください。
