# Report 0036 — recheck option local declared contract surface review

- Date: 2026-03-31T06:22:25.481163Z
- Author / agent: Codex
- Scope: 直近の未コミット変更に限った再確認 review。対象は `specs/examples/01-current-l2-surface-syntax-candidates.md`、`specs/examples/00-representative-mir-programs.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md`
- Decision levels touched: L2

## 1. Objective

前回 report 0035 の 2 finding が解消されたかを、最新の未コミット差分だけで再確認する。

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `docs/reports/0035-review-option-local-declared-contract-surface.md`

## 3. Actions taken

1. 指定 4 ファイルの最新 `git diff` を確認した。
2. D-033 / D-036 周辺、`admit` omission と underdeclared handling 周辺の実行番号を確認した。
3. 前回 finding 2 件について、差分が実際に wording を修正しているかを突き合わせた。

## 4. Files changed

- `docs/reports/0036-recheck-option-local-declared-contract-surface-review.md`

## 5. Commands run and exact outputs

- `git diff -- <4 files>`
  - D-033 が `request-local surface` へ狭められ、D-036 参照が追加されていた。
  - `admit` が必要な case では「明示しなければならない」「省略したまま判定に進む branch は underdeclared 側に残る」という文言が `specs/10`、`specs/examples/01`、`specs/examples/00` に反映されていた。
- `nl -ba specs/12-decision-register.md | sed -n '36,44p'`
- `nl -ba specs/10-open-questions.md | sed -n '74,82p'`
- `nl -ba specs/examples/01-current-l2-surface-syntax-candidates.md | sed -n '83,113p;303,321p'`
- `nl -ba specs/examples/00-representative-mir-programs.md | sed -n '18,22p;205,228p'`

## 6. Evidence / findings

### 結論

- no findings

### 根拠

- [specs/12-decision-register.md:39](/home/yukatayu/dev/mir_poc_01/specs/12-decision-register.md#L39) は D-033 を request-local surface に限定し、option-local admission metadata は D-036 で別扱いと明示した。これで前回の D-033 / D-036 wording conflict は解消した。
- [specs/12-decision-register.md:42](/home/yukatayu/dev/mir_poc_01/specs/12-decision-register.md#L42)、[specs/10-open-questions.md:77](/home/yukatayu/dev/mir_poc_01/specs/10-open-questions.md#L77)、[specs/examples/01-current-l2-surface-syntax-candidates.md:87](/home/yukatayu/dev/mir_poc_01/specs/examples/01-current-l2-surface-syntax-candidates.md#L87)、[specs/examples/01-current-l2-surface-syntax-candidates.md:111](/home/yukatayu/dev/mir_poc_01/specs/examples/01-current-l2-surface-syntax-candidates.md#L111)、[specs/examples/00-representative-mir-programs.md:21](/home/yukatayu/dev/mir_poc_01/specs/examples/00-representative-mir-programs.md#L21) は、必要 case での `admit` omission を underdeclared 側に残すことを明示しており、static evidence floor / underdeclared handling の読みも揃った。
- [specs/examples/01-current-l2-surface-syntax-candidates.md:106](/home/yukatayu/dev/mir_poc_01/specs/examples/01-current-l2-surface-syntax-candidates.md#L106) から [specs/examples/01-current-l2-surface-syntax-candidates.md:112](/home/yukatayu/dev/mir_poc_01/specs/examples/01-current-l2-surface-syntax-candidates.md#L112) では、`admit` を admission-side metadata に限定し、statement-local clause attachment との分離、option-local outcome metadata の未決定維持、final parser syntax の非固定も保てている。

## 7. Changes in understanding

前回は `admit` の optionality が underdeclared handling を弱めて見えたが、今回の wording 修正で「必要 case では mandatory、さもなくば underdeclared」という current L2 の読みが mirror 文書まで揃った。

## 8. Open questions

- 残るのは未決事項として既に明示されている範囲、すなわち `admit` の最終 keyword / punctuation、option-local outcome metadata の要否、final parser syntax である。

## 9. Suggested next prompt

current L2 の option-local `admit` failure を、non-admissible skip、explicit failure、dynamic `Reject` のどれとして読むのが最も整合的かを、比較用 variant を使って詰めてください。
