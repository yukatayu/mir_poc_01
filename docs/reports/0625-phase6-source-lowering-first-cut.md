# Report 0625 — phase6 source lowering first cut

- Date: 2026-04-12T03:36:00Z
- Author / agent: Codex
- Scope: Phase 6 actual parser-to-Program lowering first cut、source sample first trio actualization、runtime lowering tests、snapshot / roadmap / helper-boundary mirror 更新
- Decision levels touched: L1 / L2

## 1. Objective

- `tasks.md` 先頭 task だった Phase 6 actual parser-to-`Program` lowering first cut を actual code と docs の両方で閉じる。
- fixed-subset source sample を semantic `Program` と optional parser bridge evidence へ fail-closed に lower する non-production helper を `mir-runtime::current_l2` に置く。
- first authored source sample trio `e4` / `e2` / `e23` を追加し、next mainline を syntax-backed sample runner first cut へ進める。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/315...318`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `samples/current-l2/README.md`
- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/tests/current_l2_runtime_skeleton.rs`
- `crates/mir-ast/tests/fixtures/current-l2/e2-try-fallback.json`
- `crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`
- `crates/mir-ast/tests/fixtures/current-l2/e23-malformed-try-fallback-missing-fallback-body.json`

## 3. Actions taken

- `mir-runtime::current_l2` に `lower_current_l2_fixed_source_text` と helper-local source parser を追加し、place / option / chain / perform / single-line clause / single try / `atomic_cut` / inline admit の fixed subset を semantic `Program` + optional stage 1 / stage 2 bridge evidence へ lower する path を actualize した。
- source sample first trio として `samples/current-l2/e4-malformed-lineage.txt`、`e2-try-fallback.txt`、`e23-malformed-try-fallback-missing-fallback-body.txt` を追加した。
- `crates/mir-runtime/tests/current_l2_source_lowering.rs` を追加し、stage 1 static-only pair と stage 2 runtime/static pair が runtime skeleton まで通ることを確認した。
- `specs/examples/319...320` を追加し、lowering first cut の comparison / threshold を fixed した。
- `Documentation.md`、`progress.md`、`tasks.md`、relevant `plan/`、Phase 6 abstract、`specs/00-document-map.md` を更新し、current mainline を `syntax-backed sample runner first cut` に切り替えた。

## 4. Files changed

- `Documentation.md`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/tests/current_l2_source_lowering.rs`
- `docs/reports/0625-phase6-source-lowering-first-cut.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `samples/current-l2/README.md`
- `samples/current-l2/e2-try-fallback.txt`
- `samples/current-l2/e4-malformed-lineage.txt`
- `samples/current-l2/e23-malformed-try-fallback-missing-fallback-body.txt`
- `specs/00-document-map.md`
- `specs/examples/319-current-l2-representative-fixture-source-mapping-matrix-ready-actual-parser-to-program-lowering-first-cut-comparison.md`
- `specs/examples/320-current-l2-actual-parser-to-program-lowering-first-cut-ready-minimal-actual-parser-to-program-lowering-first-cut-threshold.md`
- `tasks.md`

## 5. Commands run and exact outputs

- `cargo test -p mir-runtime --test current_l2_source_lowering`
  - first run:
    - `error[E0432]: unresolved import mir_runtime::current_l2::lower_current_l2_fixed_source_text`
  - second run:
    - `running 3 tests`
    - `3 passed; 0 failed`
- `date '+%Y-%m-%d %H:%M JST'`
  - `2026-04-12 12:36 JST`

## 6. Evidence / findings

- lowering helper は `mir-runtime::current_l2` に置く方が、`mir-ast` public carrier を widen せずに source path を actualize できる。
- stage 2 bridge source は raw snippet 全体ではなく direct statement head row に正規化しないと、contract clause row まで `Other` として混ざり、semantic program subset と一致しない。
- first authored sample trio は `e4` / `e2` / `e23` で十分だった。stage 1 static-only、stage 2 runtime success、stage 2 structural malformed の 3 系統を current first cut で押さえられる。
- multiline clause suite、second try、unsupported row を fail-closed に止めることで、later parser/runtime public boundary を先取りせずに済む。

## 7. Changes in understanding

- lowering task の本質は parser 本体を広げることではなく、current parser carrier と semantic runtime のあいだに fail-closed bridge を 1 本置くことにある。
- stage 2 bridge の constraint は parser helper 自身よりも runtime-side consistency guardで顕在化しやすい。
- source sample authoring は cluster 全量を一度に入れなくてもよく、runner / ladder に必要な最小 trio から ratchet 方式で増やせる。

## 8. Open questions

- runner first cut で fixture path defaulting と CLI surface をどこまで helper-local に置くか
- `e1` / `e3` / `e21` source sample の actualization を runner task と ladder task のどちらで行うか
- multiline clause suite later reopen を parser-side widen と source-lowerer widen のどちらにぶら下げるか

## 9. Suggested next prompt

- `tasks.md` 先頭 task の `syntax-backed sample runner first cut` を進めてください。actual source sample first trio を file path から読み、lowerer と runtime skeleton を束ねる thin runner を `mir-runtime` 側 helper に切り、fixture path defaulting と fail-closed reporting を narrow に固定してください。
