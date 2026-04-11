# Report 0619 — phase6 sample corpus roadmap and doc sync

- Date: 2026-04-11T17:51:43.597119Z
- Author / agent: Codex
- Scope: Phase 6 sample-corpus 到達経路の再分解、snapshot / roadmap / policy 文書の整合化、stale wording 監査
- Decision levels touched: L1 / L2

## 1. Objective

- 現行 `tasks.md` が compile-ready checkpoint までは十分でも、具体的な source text sample と verification ladder までの道のりを表すには不足しているかを確認する。
- fixed subset の executable sample 拡張、LLVM-family backend timing、higher-level async-control / low-level memory-order-like surface の timing について current recommendation を docs に落とす。
- `tasks.md` / `progress.md` / `Documentation.md` / `plan/` / `.docs` の wording drift と stale snapshot を洗い、repo memory を current line に揃える。

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
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `.docs/continuous-task-policy.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `AGENTS.md`

## 3. Actions taken

- disk / memory 状況を確認したうえで、sample corpus / formal staging / `atomic_cut` 周辺の current constraints を local read と subagent inspection で整理した。
- `tasks.md` を全面書き直し、old `fixed-subset sample/program corpus staging` 1 項目を
  - parser-side follow-up sequencing
  - parser-side follow-up actualization
  - source corpus scope / layout
  - representative / fixture / source mapping
  - parser-to-`Program` lowering
  - syntax-backed runner
  - verification ladder
  - authoring / bless policy
  の ordered package map に再分解した。
- `progress.md` で compile-ready checkpoint 96%+ と、syntax-backed sample verification milestone 35%前後を分離し、Phase 6 全体 rough % を source-sample path を含む読みへ補正した。
- `Documentation.md`、`plan/01`、`plan/08`、`plan/10`、`plan/11`、`plan/12`、`plan/13`、`plan/15`、`plan/17`、Phase 6 abstract、`.docs/continuous-task-policy.md` を更新し、sample-path ordering、backend timing、async-control timing を current recommendation に揃えた。
- reviewer subagent を 1 回だけ使い、semantic drift を点検した。指摘された `plan/08` / `plan/01` / `.docs` / blank report の問題は this task 内で修正した。
- `plan/90` は今回の task では traceability 断面が変わっていないため **更新不要** と判断した。

## 4. Files changed

- `.docs/continuous-task-policy.md`
- `Documentation.md`
- `docs/reports/0619-phase6-sample-corpus-roadmap-and-doc-sync.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `progress.md`
- `tasks.md`

## 5. Commands run and exact outputs

- `df -h .`
  - `Filesystem Size Used Avail Use% Mounted on`
  - `/dev/vda2 99G 93G 1.5G 99% /`
- `free -h`
  - `Mem: 960Mi total / 589Mi used / 90Mi free / 370Mi available`
- `df -h .` after user cleanup
  - `Filesystem Size Used Avail Use% Mounted on`
  - `/dev/vda2 99G 76G 19G 80% /`
- `free -h` after user cleanup
  - `Mem: 960Mi total / 597Mi used / 120Mi free / 363Mi available`
- `python3 scripts/new_report.py --slug phase6-sample-corpus-roadmap-and-doc-sync`
  - `docs/reports/0619-phase6-sample-corpus-roadmap-and-doc-sync.md`
- `date '+%Y-%m-%d %H:%M JST'`
  - `2026-04-12 02:51 JST`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 618 numbered report(s).`
- `git diff --check`
  - no output
- `git status --short`
  - modified tracked docs plus new `docs/reports/0619-phase6-sample-corpus-roadmap-and-doc-sync.md`

## 6. Evidence / findings

- Current `tasks.md` だけでは、compile-ready checkpoint の先にある **syntax-backed source sample + static/interpreter/formal ladder** までの経路を表すには不足していた。
- current recommendation は次である。
  - fixed subset の executable sample を増やすこと自体はよい
  - representative prose / fixture corpus / source-sample corpus は別 layer として扱う
  - actual parser-to-`Program` lowering と syntax-backed runner は fail-closed first cut を先に置く
  - tool-neutral formal hook を current top とし、concrete theorem/model-check binding はその後段 reserve に置く
  - LLVM-family backend / external codegen は、この narrow ladder より前に入れない
  - higher-level async-control / low-level memory-order-like surface も、settled subset sample 拡張とは分けて later workstream に残す
- reviewer final check では、当初
  - `plan/08` の source corpus 既成事実化
  - `plan/01` の stale sequencing / stale checkpoint wording
  - `.docs` の旧用語
  - blank report
  が finding になった。いずれも this task 内で修正した。

## 7. Changes in understanding

- compile-ready checkpoint close と syntax-backed sample verification readiness は、同じ Phase 6 の中でも別 progress axis として明示した方が repository memory として正確である。
- source-sample corpus は「もうある layer」ではなく、「これから current fixed subset に対して作る target layer」と明記すべきである。
- backend / codegen timing の判断は、単に implementation convenience の問題ではなく、syntax / lowering / runtime boundary を早期固定しないための guard として書いた方が drift を防ぎやすい。

## 8. Open questions

- shared single attachment frame を parser-side follow-up current package に含めるか
- source corpus の file layout / sample ID policy をどこに置くか
- parser-to-`Program` lowering first cut をどの carrier / API で止めるか
- theorem-first concrete tool pilot を sample verification ladder のどこで reopen するか
- `plan/90` traceability をこの新 task chain に合わせて再編する必要が後続 task で出るか

## 9. Suggested next prompt

- `tasks.md` の先頭 task である `Phase 6 parser-side follow-up package sequencing` を source-backed comparison と report まで閉じてください。その後、`fixed-subset source-sample corpus scope / file layout` へ進み、representative / fixture / source の 3 層を current subset に対して narrow に固定してください。
