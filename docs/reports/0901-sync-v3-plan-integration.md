# Report 0901 — 旧 `sync_v3` 要素の現行 `plan/` への再配置

- Date: 2026-04-21
- Author / agent: Codex
- Scope: `旧資料_参考_ChatGPT_03_sync_v3/` から回収価値がある論点を、現行 repo の current-L2 / separable architecture / later-gate 境界を崩さず `plan/` に取り込む
- Decision levels touched: roadmap / repository memory の整理。`specs/` の規範変更なし

## 1. Objective

旧 `sync_v3` 資料のうち本質的に有益な論点だけを回収し、現行 repo の整合性を優先したまま `plan/` へ自然に取り込む。

特に次を満たすことを目的とした。

- current numbered self-driven queue を再開したように読めないこと
- Mir / Mirrorea / PrismCascade / Typed-Effect Wiring Platform の separable boundary を崩さないこと
- 「昔あった practical concern を忘れる」ことと、「昔の monolithic vision を current line に戻す」ことの両方を避けること
- `plan/` 全体の owner split が歪にならないこと

## 2. Scope and assumptions

- 現行 mainline は `current-L2 repo-local near-end` の読みを維持する。
- 旧資料は current adoption source ではなく、historical comparison / recovered concern source として扱う。
- 旧資料の fixed surface syntax、tool brand、product roadmap は再採用しない。
- `specs/` は normative source のままとし、今回の作業は `plan/` / `progress.md` / report の整理に留める。
- `tasks.md` は、current next order が実質的に変わらない限り更新しない。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `.docs/progress-task-axes.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `diff_investigation_02.md`
- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_00.md`
- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_04.md`
- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_05.md`
- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_06.md`
- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_09.md`
- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_10.md`
- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_11.md`
- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_14.md`
- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_15.md`
- sub-agent results:
  - Bacon
  - Godel
  - Linnaeus

## 4. Actions taken

1. 現行 repo の roadmap / risk / heavy-future / theory-lab owner split を再確認した。
2. `diff_investigation_02.md` と旧資料本文を突き合わせ、回収候補を次に分類した。
   - current invariant を補助する retained concern
   - heavy future に送る stressor family
   - theory-side memory としてだけ残す proof / fragment / ladder
   - 取り込まない fixed surface / monolithic roadmap
3. sub-agent 3 本で次を並列調査した。
   - recoverable / reserve-only / non-adoption の三分類
   - plan 文書の歪みやすい箇所
   - heavy future / risk / theory owner の候補整理
4. `plan/10` に historical recovery policy を追加し、owner split を top-level で固定した。
5. `plan/11` に historical recovery が numbered queue を reopen しない旨を追記した。
6. `plan/12` に historical risk と recovered concern owner map を追加した。
7. `plan/13` に portal / multi-world stressor、benchmark family catalog、operational trust / audit / registry / observability を heavy future として明示した。
8. `plan/18` に historical recovery note と proof promotion ladder memory を追加した。
9. `progress.md` に owner-split 済みであることと recent log を追記した。
10. stray に生成されていた sub-agent draft report は削除し、この report を新規作成した。

## 5. Evidence / outputs / test results

### 5.1 Files changed

- 更新:
  - `plan/10-roadmap-overall.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/12-open-problems-and-risks.md`
  - `plan/13-heavy-future-workstreams.md`
  - `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
  - `progress.md`
- 追加:
  - `docs/reports/0901-sync-v3-plan-integration.md`
- 削除:
  - `docs/reports/0900-plan-roadmap-distortion-audit.md`

補足:

- `tasks.md` 更新不要
- `specs/` 更新不要

### 5.2 Commands run

主要コマンドだけを記す。

```bash
$ python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
Task baseline recorded.

$ git status --short
?? docs/reports/0900-plan-roadmap-distortion-audit.md

$ find 旧資料_参考_ChatGPT_03_sync_v3 -maxdepth 2 -type f | sort | sed -n '1,120p'
旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_00.md
...
旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_16.md

$ rg -n "durable_cut|barrier|atomic_cut|portal|world|audit|registry|revocation|SMT|timeout|state-space|trace|solver|witness|lease|gc_epoch|patch" 旧資料_参考_ChatGPT_03_sync_v3
旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_04.md:135:## 4.5 `durable_cut` — 永続コミット
旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_10.md:67:### 10.3.1 `portal` デクラレータ
旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_14.md:8:### 14.1.1 SMT タイムアウト問題
...

$ date '+%Y-%m-%d %H:%M %Z'
2026-04-21 13:11 JST
```

### 5.3 何を回収し、どこへ置いたか

- `plan/10`
  - historical recovery を current roadmap にどう接続するかの top-level policy を追加した。
  - これにより、「旧資料を読む = old queue を再開する」という誤読を防ぐ。
- `plan/12`
  - historical scope-collapse、historical requirement amnesia、solver / projection / artifact-trace blow-up を risk として登録した。
  - さらに recovered concern owner map を追加し、topic ごとの行き先を 1 箇所で読めるようにした。
- `plan/13`
  - portal / multi-world / replay / fairness catalog を heavy future の stressor family として固定した。
  - audit / registry / observability / key rotation / revocation を current mainline から分離した。
  - benchmark family catalog を heavy future の evaluation family として整理した。
- `plan/18`
  - finite decidable fragment、proof promotion ladder、non-adoption boundary を theory-side memory として固定した。
  - old fixed EBNF / bundled primitive / single-pass omnipotence story / concrete brand fixation を明示的に退けた。

### 5.4 今回あえて戻さなかったもの

- old fixed EBNF / reserved-word catalog
- old bundled `atomic_cut` / `durable_cut` / `barrier` surface
- Stage `0...4` / `1.0` product roadmap
- theorem / model-check / runtime の concrete brand 固定
- portal / multi-world を current authoritative-room default の先頭目標にする読み

### 5.5 plan 全体の歪み確認

今回の整理では、mixed gate / heavy future / risk / theory-memory の owner を増やしすぎないことを優先した。

- `plan/10`:
  top-level policy owner
- `plan/11`:
  near-term queue は reopen しないとだけ明示
- `plan/12`:
  risk と owner map の窓口
- `plan/13`:
  heavy future scenario / governance concern
- `plan/18`:
  theory-side retained memory

この split により、旧資料由来の論点を取り込んでも near-term queue は増えず、current mainline の reading は変わらない。

## 6. What changed in understanding

- 旧資料の価値は「そのまま戻す設計書」であることではなく、
  current repo が後段で再びぶつかる practical concern を先回りして列挙している点にある、と整理できた。
- とくに有益だったのは、
  solver / SMT スケール問題、
  portal / multi-world stressor、
  audit / registry / revocation / observability、
  `fallback` / `lease` / `patch` / `gc_epoch` の相互作用
  であり、これらは current-L2 mainline に戻すより owner 別 memory として保持する方が整合的だった。
- 逆に、旧資料の monolithic vision を current roadmap に混ぜると、
  `repo-local near-end` と `final public` の境界が再び曖昧になると判断した。

## 7. Open questions

- benchmark family のうち、最初に user と一緒に acceptance criteria を詰める候補をどれにするかは未決である。
- operational trust / audit / registry を `Macro 7` のどの reopen 条件で mainline 化するかは、packaging / host-facing target が固まるまで未決である。
- solver / projection / artifact-trace scaling の threshold を定量目標としてどこまで書くかは、実際の widening task を再開するときの open question である。

## 8. Suggested next prompt

`plan/12` の recovered concern owner map を起点に、次に reopen する 1 本だけを選び、`repo-local near-end` を崩さない具体的な package と validation loop に落として整理してください。たとえば `Problem 1 final-public-seam reopen` または `Problem 2 final-public-seam reopen` のどちらか 1 本に絞ってください。
