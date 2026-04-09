# Report 0361 — self driven research order and estimates

- Date: 2026-04-09T03:48:22.128273Z
- Author / agent: Codex
- Scope: next self-driven research ordering / rough estimate / task snapshot refresh
- Decision levels touched: L2 / L3

## 1. Objective

- current repo state に合わせて、次に自走で進める research task の順番を整理する。
- 各 task の rough weight / rough 所要を、手戻り前提の概算として明記する。
- `tasks.md` を current snapshot として書き直し、必要な roadmap / progress mirror を揃える。

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
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`

## 3. Actions taken

- `tasks.md` の task package を、「何を先に進めるか」「rough weight / rough 所要」「自走可否」が見える順に再配置した。
- `progress.md` の immediate execution order / Priority A / next tasks を `tasks.md` と揃えた。
- `plan/11-roadmap-near-term.md` を、候補列挙ではなく current self-driven order と rough estimate が読める形に更新した。
- `plan/17-research-phases-and-autonomy-gates.md` の immediate execution order も同じ順番に補正した。
- `plan/90-source-traceability.md` に provenance addendum を追加した。

## 4. Files changed

- `tasks.md`
- `progress.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/reports/0361-self-driven-research-order-and-estimates.md`

## 5. Commands run and exact outputs

- `date '+%Y-%m-%d %H:%M JST'`
  - `2026-04-09 12:44 JST`
- `df -h .`
  - `Filesystem /dev/vda2 Size 99G Used 89G Avail 5.1G Use% 95%`
- `free -h`
  - `Mem: 960Mi total / 720Mi used / 73Mi free / 239Mi available`
- `python3 scripts/new_report.py --slug self-driven-research-order-and-estimates`
  - `docs/reports/0361-self-driven-research-order-and-estimates.md`

## 6. Evidence / findings

- current self-driven order は、Phase 読みを変えずに次の 4 package へ具体化できる。
  1. detached validation loop friction reduction
  2. authoritative room baseline refinement
  3. consistency / fairness / causal metadata catalog comparison
  4. static analysis / type / theorem prover / async-control boundary inventory
- Phase 3 は current checkpoint では reserve path のままでよく、active package に戻す pressure はまだ不足している。
- rough estimate は実装 promise ではなく、比較 / report / review / validation / drift suppression まで含む概算として書くのが自然である。

## 7. Changes in understanding

- これまでは「Phase 2 / 4 / 5 が主線」という phase 読みは揃っていたが、その中で何を先にやるかが `tasks.md` / `progress.md` / `plan/11` で少し分散していた。
- 今回の refresh で、Phase 2 maintenance tail を先に使って validation loop throughput を上げ、その後に Phase 4 を authoritative baseline → catalog comparison の 2 段に分け、最後に Phase 5 inventory line を進める順が current first choice だと明示できた。

## 8. Open questions

- `tasks.md` に書いた rough 所要を、将来さらに task 実績ベースで補正するか。
- authoritative room baseline を厚くした後、consistency / fairness / causal metadata comparison をどこまで一つの package として扱うか。
- Phase 5 inventory line を shared-space side line とどこまで交互に進めるか。

## 9. Suggested next prompt

`tasks.md` の順番どおりに、まず detached validation loop friction reduction から自走で進め、その checkpoint closeout ごとに `tasks.md` / `progress.md` / `plan/11` の順番と rough estimate を見直してください。
