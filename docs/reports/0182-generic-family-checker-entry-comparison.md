# Report 0182 — generic family checker entry comparison

- Date: 2026-04-06T01:52:00+09:00
- Author / agent: Codex
- Scope: shared family checker support helper 導入後の次段として、generic checker-side shared family compare entry を actualize する価値があるかを docs-only で比較する
- Decision levels touched: L2

## 1. Objective

- generic checker-side shared family compare entry を今切るべきか比較する
- family facade script 維持で止める current judgment を docs / plan / progress に反映する
- production checker API や generic public CLI には進まない

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/46-current-l2-same-lineage-first-checker-spike.md`
- `specs/examples/47-current-l2-missing-option-second-checker-spike.md`
- `specs/examples/48-current-l2-capability-third-checker-spike.md`
- `specs/examples/49-current-l2-shared-family-checker-support-helper.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`

## 3. Actions taken

- `specs/examples/50-current-l2-generic-family-checker-entry-comparison.md` を追加した
- generic entry を今は切らず、family facade 維持で止める current judgment を Documentation / document map / roadmap / open problems / traceability / progress に反映した
- code change は行わず、shared support helper 導入後の command surface を docs-only で固定した

## 4. Files changed

- 追加:
  - `specs/examples/50-current-l2-generic-family-checker-entry-comparison.md`
  - `docs/reports/0182-generic-family-checker-entry-comparison.md`
  - `docs/reports/0183-review-generic-family-checker-entry-comparison.md`
- 変更:
  - `Documentation.md`
  - `specs/00-document-map.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/12-open-problems-and-risks.md`
  - `plan/90-source-traceability.md`
  - `progress.md`

## 5. Commands run and exact outputs

- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 183 numbered report(s).`
- `git diff --check`
  - 無出力

## 6. Evidence / findings

- duplicated core contract は already `scripts/current_l2_family_checker_support.py` に寄っており、generic entry を今足しても duplication 削減の利得は小さい
- family facade script と `smoke-*` command 名は detached validation loop の readability を保っている
- generic checker-side shared entry は current phase では public-looking surface を増やす副作用の方が大きい

## 7. Changes in understanding

- support helper 導入の次段は generic entry actualization ではなく、public checker cut comparison との接続 timing を見極めることになる
- current phase では family facade 維持で十分である

## 8. Open questions

- generic entry を later public checker API comparison と同時に扱うべきか
- family taxonomy が増えたとき facade 維持コストがどう変わるか
- final public checker API をどこへ置くか

## 9. Suggested next prompt

`shared support helper と family facade 維持が揃った current checker family spike の上で、public checker cut をどの local / structural judgment まで actualize してよいかを narrow に比較してください。`
