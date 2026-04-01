# Report 0046 — review 0045 short rereview

- Date: 2026-04-01T00:09:56.094610Z
- Author / agent: Codex
- Scope: `0045-capability-mismatch-non-admissible-taxonomy-current-l2.md` の短い再 review と、その結果記録
- Decision levels touched: L2

## 1. Objective

`0045` で整理した capability mismatch の taxonomy 位置づけについて、current L2 の既存 reading と矛盾がないかを短く再確認する。

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
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `docs/reports/0045-capability-mismatch-non-admissible-taxonomy-current-l2.md`

## 3. Actions taken

1. `0045` の差分対象だった current L2 reading を読み直した。
2. 特に次の 3 点だけに絞って再確認した。
   - capability mismatch を formal subreason に上げず narrative audit explanation に留める方針
   - event surface を増やしていないこと
   - `specs/examples/` と `specs/10` / `specs/12` の wording conflict がないこと
3. reviewer subagent に短い re-review を依頼した。

## 4. Files changed

- Changed:
  - `docs/reports/0046-review-0045-short-rereview.md`
- Checked but not changed:
  - `specs/examples/00-representative-mir-programs.md`
  - `specs/examples/01-current-l2-surface-syntax-candidates.md`
  - `specs/10-open-questions.md`
  - `specs/12-decision-register.md`
- Task-start dirty state:
  - task 開始時点では clean worktree だった。

## 5. Commands run and exact outputs

- reviewer subagent completion
  - `No findings.`

## 6. Evidence / findings

- no findings
- 手元確認でも、`admit-miss` / `lease-expired` を formal subreason に残し、capability mismatch を narrative explanation に留める current L2 reading は、D-038 / D-039 / D-040 の並びと矛盾していない。
- `option ref` / `subreason` を最小 shape に留める current reading も保たれており、event surface の追加は生じていない。

## 7. Changes in understanding

`0045` の整理は、non-admissible taxonomy を増やさずに E6 の説明力だけを確保する、という current L2 の最小化方針に沿っていることを再確認した。

## 8. Open questions

- capability mismatch を将来 formal subreason に昇格させる必要があるか。
- もし昇格させる場合、`option ref` / `subreason` だけでは足りず request-local carrier が必要になるか。

## 9. Suggested next prompt

current L2 の representative Mir programs を parser なしで machine-readable に扱えるように、最小 AST fixture schema と example fixture を整理してください。特に static verdict、runtime outcome、trace / audit expectation を fixture 側でどう持つかに絞ってください。
