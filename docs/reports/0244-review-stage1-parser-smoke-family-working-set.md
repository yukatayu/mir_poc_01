# Report 0244 — review stage1 parser smoke family working set

- Date: 2026-04-05T22:23:50.319392Z
- Author / agent: Codex
- Scope: maintainer-style review of the current L2 parser-boundary docs-only task around spec 77 and report 0243
- Decision levels touched: L2

## 1. Objective

`specs/examples/77-current-l2-stage1-parser-smoke-family-working-set.md` の judgment が
`specs/examples/74`、`75`、`76`、`02` と fixture `e3` / `e4` / `e7` の役割整理に整合するか、
`e4` + `e7` を最小 working set にして `e3` を later-stage contrast に残す cut が stage split を保てているか、
mirror / traceability と report 0243 の hygiene に欠落がないかを review する。

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `plan/00-index.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`
- `specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md`
- `specs/examples/76-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md`
- `specs/examples/77-current-l2-stage1-parser-smoke-family-working-set.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `docs/reports/0243-current-l2-stage1-parser-smoke-family-working-set.md`
- fixture `crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json`
- fixture `crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`
- fixture `crates/mir-ast/tests/fixtures/current-l2/e7-write-fallback-after-expiry.json`

## 3. Actions taken

1. Required repo context を `README.md` → `Documentation.md` → `progress.md` → `specs/00`〜`03` → `specs/09` → `plan/00` の順で読み、review judgment の土台を揃えた。
2. spec 74 → 75 → 76 → 77 の chain を読み、stage 1 accepted parse cluster、opaque slot handoff、`decl_guard_slot` naming、option-level bridge、smoke family の関係を確認した。
3. `e3` / `e4` / `e7` fixture 実体を読み、`lease` / `admit` / chain edge / runtime pressure source の役割を照合した。
4. `Documentation.md`、`specs/00-document-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/90-source-traceability.md`、`progress.md` の mirror と traceability を spot-check した。
5. report 0243 の section order、consulted inputs、actions、evidence、open questions、next prompt、`plan/` / `progress.md` note を確認した。

## 4. Files changed

- Added: `docs/reports/0244-review-stage1-parser-smoke-family-working-set.md`
- `plan/` 更新不要
- `progress.md` 更新不要

## 5. Commands run and exact outputs

- `rg -n "77-current-l2-stage1-parser-smoke-family-working-set|e4-malformed-lineage|e7-write-fallback-after-expiry|e3-option-admit-chain|0243|0244|stage 1 actual parser spike の smoke family" Documentation.md specs/00-document-map.md plan/11-roadmap-near-term.md plan/12-open-problems-and-risks.md plan/90-source-traceability.md progress.md`
  - `Documentation.md:112` / `159`、`specs/00-document-map.md:284` / `325`、`plan/11-roadmap-near-term.md:213`、`plan/12-open-problems-and-risks.md:197`、`progress.md:232`、`plan/90-source-traceability.md:507`〜`512` に spec 77 mirror があることを確認した。
- `rg -n "malformed fallback branch|malformed lineage|lineage edge mismatch|lineage assertion" specs/examples/02-current-l2-ast-fixture-schema.md specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md specs/examples/77-current-l2-stage1-parser-smoke-family-working-set.md crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`
  - `specs/examples/02-current-l2-ast-fixture-schema.md:203` に `e4` を `malformed fallback branch` と書いている一方、spec 77 と fixture 本体は lineage mismatch として読んでいることを確認した。
- `tail -n 8 progress.md`
  - 末尾 work log が `07:23 JST` の直後に `07:02 JST` を置いており、時系列順ではないことを確認した。
- `python3 scripts/new_report.py --slug review-stage1-parser-smoke-family-working-set`
  - `docs/reports/0244-review-stage1-parser-smoke-family-working-set.md` を生成した。

## 6. Evidence / findings

1. Medium — [specs/examples/02-current-l2-ast-fixture-schema.md](/home/yukatayu/dev/mir_poc_01/specs/examples/02-current-l2-ast-fixture-schema.md#L203) の fixture catalog が `e4-malformed-lineage` を `malformed fallback branch` と説明しており、spec 74 の stage-1 anchor、spec 77 の working-set judgment、fixture 本体の `lineage_assertion_edge_mismatch` とずれています。stage 1 parser smoke family の rationale は `e4` を declaration / chain structural floor 上の lineage mismatch anchor として使っているので、この catalog wording のままだと `e4` の役割を fallback/admissibility 側へ誤って寄せ、spec 77 の cut を弱めます。
2. Low — [progress.md](/home/yukatayu/dev/mir_poc_01/progress.md#L232) の task-close log が時系列順になっていません。`2026-04-06 07:23 JST` の entry の後に `2026-04-06 07:02 JST` が残っているため、parser-boundary 主線への切り替え順が追いにくくなっています。内容自体は spec 77 judgment と整合していますが、mirror / traceability の読みやすさは落ちています。

## 7. Changes in understanding

- spec 77 自体の cut は spec 74 / 75 / 76 と整合しており、`e4` + `e7` を active smoke、`e3` を later-stage contrast に残す判断は stage split を保っている。
- main mismatch は judgment 本体ではなく、fixture catalog 側の `e4` role wording に残っている。
- report 0243 は section hygiene と consulted/evidence の最低要件を満たしている。review report 0244 を追加したことで、`plan/90-source-traceability.md` の 0244 参照も現在状態では実在 source になった。

## 8. Open questions

- `specs/examples/02-current-l2-ast-fixture-schema.md` の `e5-underdeclared-lineage` row も `underdeclared fallback case` と書かれているため、同じ lineage-oriented wording へ合わせてよいかは次の mirror cleanup で確認してよい。

## 9. Suggested next prompt

`specs/examples/02-current-l2-ast-fixture-schema.md` の fixture catalog wording を current parser-boundary reading に合わせて見直し、必要なら `e4` / `e5` の主題表現を lineage-oriented に修正してください。必要なら `progress.md` の末尾作業ログも時系列順に整えてください。
