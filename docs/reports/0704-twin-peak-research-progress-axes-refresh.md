# Report 0704 — twin peak research progress axes refresh

- Date: 2026-04-16T14:18:33.602730Z
- Author / agent: Codex
- Scope: `progress.md` に twin peaks 専用の進捗節を追加し、typed/theorem/model-check 線と order/memory-order/authority-handoff 線の current reading を snapshot 化する。あわせて user 向け説明を `faq_005.md` に新規作成する。
- Decision levels touched: `L2` current reading / progress snapshot only

## 1. Objective

- 型システム / 定理証明 / モデル検査線と、ordering / `memory_order` / authority-handoff 線について、repo current reading を簡潔かつ誤読しにくい形で `progress.md` に特別扱いで載せる。
- それぞれについて、現段階、rough progress、source-backed floor、OPEN、次の research package、user の元構想から current repo でどう整理されたかを残す。
- 規範判断や roadmap sequencing は勝手に変更せず、snapshot 強化に留める。

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
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `faq_004.md`

## 3. Actions taken

- current snapshot と `plan/18` の theory-lab tracks を突き合わせ、twin peaks として独立追跡するのに必要な項目を抽出した。
- `progress.md` に 2 本の専用節を追加した。
  - 型システム / 定理証明 / モデル検査
  - fence / atomic / `memory_order` / authority-handoff 再構築
- 各節に、位置づけ、現段階、rough progress、採用済みの骨格、最初の構想からの整理、OPEN、直近計画、自走範囲を記載した。
- recent log に今回の snapshot 強化を追記した。
- `faq_005.md` を新規追加し、上記 2 本の heavy line について current reading を user 向けにまとめた。

## 4. Files changed

- `progress.md`
- `faq_005.md`
- `docs/reports/0704-twin-peak-research-progress-axes-refresh.md`

## 5. Commands run and exact outputs

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
  - `Task baseline recorded.`
- `df -h .`
  - `/dev/vda2 99G 77G 18G 82% /`
- `free -h`
  - `Mem: 960Mi total / 731Mi used / 92Mi free / 291Mi buff/cache`
  - `Swap: 19Gi total / 1.1Gi used`
- `date '+%Y-%m-%d %H:%M:%S %Z'`
  - `2026-04-16 23:14:11 JST`
- `date '+%Y-%m-%d %H:%M JST'`
  - `2026-04-16 23:20 JST`
- `python3 scripts/new_report.py --slug twin-peak-research-progress-axes-refresh`
  - `Created docs/reports/0704-twin-peak-research-progress-axes-refresh.md`
- exploratory reads:
  - `sed -n ... progress.md`
  - `sed -n ... tasks.md`
  - `sed -n ... plan/18-type-proof-modelcheck-and-ordering-research-program.md`
  - `sed -n ... faq_004.md`

## 6. Evidence / findings

- typed/theorem/model-check line is no longer future-only. It already has `formal_hook`, `proof_notebook_review_unit`, row-local `model_check_concrete_carriers`, emitted artifact wiring, and sample-facing theorem/model-check summary.
- ordering / authority-handoff line is also no longer future-only. It already has cut-family decomposition, relation decomposition, adequacy corpus, falsifier coverage, candidate reduction, and shared-space authority-room baseline.
- both lines are still theory-lab / docs-first lines rather than implementation-ready lines.
- the heaviest remaining work is not “can we start at all?” but “how far to promote before concrete tools, final syntax, or final public contracts.”
- no immediate user decision is currently blocking either line at the current stop line.

## 7. Changes in understanding

- The repo already has enough source-backed material to treat these two lines as the heavyweight twin peaks explicitly in `progress.md`.
- The typed/theorem/model-check line is farther along than the order/memory-order line in implementation/ops, but both are still in active boundary hardening rather than finalization.
- The user’s original broad ambitions were not rejected; they were decomposed into bridge programs with clearer stop lines:
  - strong types / theorem / model-check -> obligation allocation, attachment, pilot evidence, projection/tool seams
  - fence / atomic / `memory_order` rebuild -> cut/order/visibility/witness/authority decomposition plus adequacy/falsifier loop
- The `progress.md` twin peaks sections should stay on repo-defined axes (`Macro N`, `S0-S6`, rough per-row percentages) and should not invent a private step ladder.

## 8. Open questions

- typed-core `request / predicate / try` cluster shape
- admissible evidence contraction and proof-object transport
- model-check concrete tool binding and first settled property language
- order / handoff emitted-artifact schema
- final handoff source surface
- final low-level `memory_order` reinterpretation stance
- fairness / replay / authority final catalog

## 9. Suggested next prompt

- `progress.md` の twin peaks 節を踏まえて、typed/theorem/model-check 側と order/handoff 側の current reserve packages を current lane を崩さずに進めてください。

## 10. Update necessity notes

- `plan/` 更新不要
- `tasks.md` 更新不要
- `Documentation.md` 更新不要
- `faq_004.md` 更新不要
