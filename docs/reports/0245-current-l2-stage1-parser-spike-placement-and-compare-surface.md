# Report 0245 — current L2 stage 1 parser spike placement and compare surface

- Date: 2026-04-05T22:27:09.279675Z
- Author / agent: Codex
- Scope: docs-only comparison for actual stage 1 parser spike placement and compare surface
- Decision levels touched: L2

## 1. Objective

`specs/examples/77-current-l2-stage1-parser-smoke-family-working-set.md` の次段として、
actual stage 1 parser spike をどこに置き、何を compare surface にするのが最小かを比較し、
non-production / private / test-driven な initial cut を決める。

## 2. Scope and assumptions

- scope は stage 1 parser spike の private placement と compare surface に限定する。
- current L2 semantics、final parser grammar、public parser API、`lib.rs` / `harness.rs` boundary は変更しない。
- 比較対象は `scripts/`、`crates/mir-semantics/examples/support/`、`crates/mir-ast/tests/support/` の 3 placement と、
  raw AST snapshot / lowered fixture-subset / full fixture compare の 3 compare surface に留める。
- `plan/` mirror は `plan/11`、`plan/12`、`plan/90` のみ更新対象とする。

## 3. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/31-current-l2-detached-aggregate-transform-helper.md`
- `specs/examples/37-current-l2-detached-bundle-transform-helper.md`
- `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`
- `specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md`
- `specs/examples/76-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md`
- `specs/examples/77-current-l2-stage1-parser-smoke-family-working-set.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `crates/mir-semantics/examples/current_l2_emit_detached_bundle.rs`
- `crates/mir-semantics/examples/support/current_l2_detached_bundle_support.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 4. Actions taken

1. stage 1 smoke family judgment を前提に、次段が actual spike の配置と compare surface であることを確認した。
2. placement について次の 3 案を比較した。
   - `scripts/`
   - `crates/mir-semantics/examples/support/`
   - `crates/mir-ast/tests/support/`
3. compare surface について次の 3 案を比較した。
   - parser-side raw AST snapshot compare
   - lowered fixture-subset compare
   - full fixture compare
4. `specs/examples/78-current-l2-stage1-parser-spike-placement-and-compare-surface.md` を追加し、`crates/mir-ast/tests/support/` 配置の private helper + lowered fixture-subset compare を current judgment として整理した。
5. `Documentation.md`、`specs/00-document-map.md`、`specs/examples/77-current-l2-stage1-parser-smoke-family-working-set.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/90-source-traceability.md`、`progress.md` を mirror 更新した。

## 5. Evidence / outputs / test results

- Files changed:
  - Added: `specs/examples/78-current-l2-stage1-parser-spike-placement-and-compare-surface.md`
  - Added: `docs/reports/0245-current-l2-stage1-parser-spike-placement-and-compare-surface.md`
  - Modified: `Documentation.md`
  - Modified: `specs/00-document-map.md`
  - Modified: `specs/examples/77-current-l2-stage1-parser-smoke-family-working-set.md`
  - Modified: `plan/11-roadmap-near-term.md`
  - Modified: `plan/12-open-problems-and-risks.md`
  - Modified: `plan/90-source-traceability.md`
  - Modified: `progress.md`
- Commands run:
  - `git status --short --branch`
    - Output: `## main...origin/main`
  - `python3 scripts/new_report.py --slug current-l2-stage1-parser-spike-placement-and-compare-surface`
    - Output: `/home/yukatayu/dev/mir_poc_01/docs/reports/0245-current-l2-stage1-parser-spike-placement-and-compare-surface.md`
  - local file reads:
    - `crates/mir-semantics/examples/current_l2_emit_detached_bundle.rs`
    - `crates/mir-semantics/examples/support/current_l2_detached_bundle_support.rs`
    - `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- detached helper 群は `examples/support/` に置かれているが、主題は emitter / transform helper であり、parser spike の ownership をそこへ寄せる必要はない。
- parser spike の current 主題は stage 1 structural floor と lowered fixture-subset compare なので、fixture corpus に近い `tests/support/` の方が責務が素直である。
- compare surface を lowered fixture-subset compare に留めると、spec 75 / 76 の handoff cut と直接整合する。
- full fixture compare は current stage では広すぎ、runtime / trace / audit expectation を巻き込みやすい。
- test results:
  - 実行系 test はなし。docs-only comparison のため、この段階では file inspection と mirror 更新のみを行った。

## 6. What changed in understanding

- actual stage 1 parser spike は detached helper family と同じ置き方を真似るより、fixture compare に近い `crates/mir-ast/tests/support/` へ置く方が自然である。
- compare surface は parser-side temporary AST ではなく lowered fixture subset に留める方が、temporary carrier drift を防ぎやすい。
- ここまでで、non-production / private / test-driven な actual stage 1 parser spike に入る前提はかなり揃った。

## 7. Open questions

- actual temporary carrier の Rust type / function name をどこまで narrow に先に決めるか。
- input surface を test inline string にするか、別 fixture file にするか。
- 実装と同時に `e4` / `e7` の compare harness をどこまで generic にするか。

## 8. Suggested next prompt

`specs/examples/78-current-l2-stage1-parser-spike-placement-and-compare-surface.md` を前提に、actual non-production stage 1 parser spike を `crates/mir-ast/tests/support/` 配置で始めるときの temporary carrier 名、private helper 名、input surface を narrow に比較し、実装へ進める最小 cut を整理してください。

## 9. plan/ update note

- `plan/` 更新あり: `plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/90-source-traceability.md`
