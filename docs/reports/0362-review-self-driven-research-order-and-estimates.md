# Report 0362 — review self driven research order and estimates

- Date: 2026-04-09T03:53:54.312417Z
- Author / agent: Codex
- Scope: review record / local evidence fallback for self-driven research order refresh
- Decision levels touched: L2 / L3

## 1. Objective

- `tasks.md` / `progress.md` / `plan/11` / `plan/17` の順番・rough estimate・phase 読みが整合しているかを review 記録として残す。

## 2. Inputs consulted

- `tasks.md`
- `progress.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `docs/reports/0361-self-driven-research-order-and-estimates.md`

## 3. Actions taken

- reviewer subagent を 1 回起動した。
- この session では reviewer completion を受け取る handle が返らず、結果取得を継続できなかった。
- そのため local evidence fallback として、対象文書の diff と key phrase search を使って mirror drift を点検した。
- 点検中に見つかった `progress.md` の自走可否 drift を補正した。

## 4. Files changed

- `progress.md`
- `docs/reports/0362-review-self-driven-research-order-and-estimates.md`

## 5. Commands run and exact outputs

- reviewer subagent 起動
  - completion handle を取得できず、結果取得不可
- `git diff -- tasks.md progress.md plan/11-roadmap-near-term.md plan/17-research-phases-and-autonomy-gates.md plan/90-source-traceability.md docs/reports/0361-self-driven-research-order-and-estimates.md`
  - local mirror drift 点検に使用
- `rg -n "detached validation loop の運用摩擦低減|authoritative room baseline|consistency / fairness / causal metadata|async-control boundary|reserve path" ...`
  - ordering / wording / provenance の cross-check に使用

## 6. Evidence / findings

- 実質的な finding は 1 件だった。
  - `progress.md` では Phase 4 / Phase 5 の docs-first package を immediate execution order に上げた一方、phase table / chapter table / `着手可能` section では still `一部自走可能` / `後段依存` / `要仕様確認` が残っていた。
- これを補正し、
  - Phase 4 / Phase 5 の docs-first inventory / comparison は `着手可能`
  - final catalog / final actualization は引き続き deferred
  に揃えた。

## 7. Changes in understanding

- self-driven order の refresh では、順番表だけでなく `progress.md` の `着手可否` mirror が最も drift しやすい。
- 以後も ordering refresh を行う task では、`tasks.md` と `progress.md` の `着手可能 / 後段依存 / 要仕様確認` の整合を必ず同時に点検する。

## 8. Open questions

- reviewer subagent completion handle を取れない場合の標準 fallback を、後で helper /運用文書に切り出すか。

## 9. Suggested next prompt

`tasks.md` の順番どおりに detached validation loop friction reduction を再開し、checkpoint close ごとに `progress.md` の `着手可否` mirror drift を先に点検してください。
