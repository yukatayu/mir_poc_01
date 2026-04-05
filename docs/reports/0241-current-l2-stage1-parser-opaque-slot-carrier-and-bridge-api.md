# Report 0241 — current L2 stage 1 parser opaque slot carrier and bridge API

- Date: 2026-04-05T22:13:30Z
- Author / agent: Codex
- Scope: docs-only comparison for parser-side opaque slot naming and thin lowering bridge API surface
- Decision levels touched: L2

## 1. Title and identifier

- Report 0241
- current L2 stage 1 parser opaque slot carrier and bridge API

## 2. Objective

`specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md` の次段として、
stage 1 parser-side opaque slot carrier の naming candidate と thin lowering bridge の private API surface を比較し、
actual parser spike 前の docs-only judgment を narrow に固める。

## 3. Scope and assumptions

- current L2 の core semantics は変更しない。
- stage 1 declaration-side guard slot は opaque attached slot のままにする。
- predicate fragment parse / normalization は stage 1 に入れない。
- public `lib.rs` / `harness.rs` API は増やさない。
- `plan/` は今回更新対象である。

## 4. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/31-current-l2-detached-aggregate-transform-helper.md`
- `specs/examples/37-current-l2-detached-bundle-transform-helper.md`
- `specs/examples/73-current-l2-first-parser-spike-staging.md`
- `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`
- `specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- fixture `crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json`
- fixture `crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`
- fixture `crates/mir-ast/tests/fixtures/current-l2/e7-write-fallback-after-expiry.json`

## 5. Actions taken

1. spec 75 の handoff judgment と parser-free fixture schema を読み直し、naming と bridge API surface で何が未決かを切り出した。
2. parser-side carrier naming について次の 3 案を比較した。
   - `lease` / `lease_slot`
   - `guard_slot`
   - `decl_guard_slot`
3. thin lowering bridge の private API surface について次の 3 案を比較した。
   - slot-only bridge
   - option-level bridge
   - program-level bridge
4. `specs/examples/76-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md` を追加し、`decl_guard_slot` + option-level structural transfer を current judgment として整理した。
5. `Documentation.md`、`specs/00-document-map.md`、`specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/90-source-traceability.md`、`progress.md` を mirror 更新した。

## 6. Evidence / outputs / test results

### Commands run

- `git status --short --branch`
- `python3 scripts/new_report.py --slug current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api`
- `sed -n '1,240p' specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md`
- `sed -n '1,220p' specs/examples/02-current-l2-ast-fixture-schema.md`
- `sed -n '1,220p' specs/examples/31-current-l2-detached-aggregate-transform-helper.md`
- `sed -n '1,220p' specs/examples/37-current-l2-detached-bundle-transform-helper.md`
- `date -u '+%Y-%m-%dT%H:%M:%SZ' && date '+%Y-%m-%d %H:%M JST'`
- `python3 scripts/validate_docs.py`
- `git diff --check`

### Command outputs

- `git status --short --branch`
  - `## main...origin/main`
- `python3 scripts/new_report.py --slug current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api`
  - `/home/yukatayu/dev/mir_poc_01/docs/reports/0241-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md`
- `date -u '+%Y-%m-%dT%H:%M:%SZ' && date '+%Y-%m-%d %H:%M JST'`
  - `2026-04-05T22:13:30Z`
  - `2026-04-06 07:13 JST`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 241 numbered report(s).`
- `git diff --check`
  - no output

### 読み取り evidence

- current fixture-side anchor は `OptionDecl.lease` であり、parser-side carrier と同名に寄せると handoff judgment が崩れやすい。
- `e3-option-admit-chain` は request / admissibility cluster の later-stage pressure を示しており、generic な `guard_slot` は declaration-side か request-side かを曖昧にしやすい。
- `e4-malformed-lineage` と `e7-write-fallback-after-expiry` は declaration structural floor だけを見る current stage で `lease` slot が narrow companion carrier として十分機能していることを示す。
- current detached helper 群は slot-only helper ではなく whole-carrier transform を narrow private API として切っており、option-level bridge の方が current repo の helper boundary と整合する。

## 7. What changed in understanding

- parser-side carrier 名は fixture-side `lease` から切り離して読む必要があり、generic すぎる `guard_slot` より `decl_guard_slot` の方が declaration-side opaque slot という current meaning を保ちやすい。
- bridge API surface は slot-only helper ではなく option declaration 単位の structural transfer として切る方が、hidden parsing / normalization を避けやすい。
- したがって current next narrow step は、`decl_guard_slot` の内部 carrier と option-level structural transfer の actual private API surface を比べることであり、program-wide bridge や public API 化ではない。

## 8. Open questions

- `decl_guard_slot` の内部 carrier は raw text / token slice / opaque leaf のどれが最小か。
- option-level structural transfer の actual private API 名をどこまで docs-only で先に決めるべきか。
- stage 1 smoke family を `e4` / `e7` / `e3` のどこまで含めるのが最小か。

## 9. Suggested next prompt

`specs/examples/76-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md` を前提に、stage 1 option-level structural transfer を actual parser spike の smoke family へどこまで narrow に接続するかを比較し、最小 working set を整理してください。

## 10. Files changed

- Added: `specs/examples/76-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md`
- Modified: `Documentation.md`
- Modified: `specs/00-document-map.md`
- Modified: `specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md`
- Modified: `plan/11-roadmap-near-term.md`
- Modified: `plan/12-open-problems-and-risks.md`
- Modified: `plan/90-source-traceability.md`
- Modified: `progress.md`

## 11. plan/ update note

- `plan/` 更新あり: `plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/90-source-traceability.md`
