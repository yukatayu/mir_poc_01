# Report 0348 — review phase3 reopen threshold and abstracts

- Date: 2026-04-08T11:15:39.237718Z
- Author / agent: Codex
- Scope: current uncommitted docs-only changesについて、Phase 3 self-driven reopen-threshold judgment と `docs/research_abstract/` の overclaim、ならびに `plan/` / `progress.md` mirror が Phase 3 を reserve path として一貫して読ませているかを narrow に review する。
- Decision levels touched:
  - review only; normative statement は未変更

## 1. Objective

- Phase 3 self-driven reopen-threshold judgment が既存 source-backed evidence を越えて completion / certainty を言い過ぎていないか確認する。
- `docs/research_abstract/` の condensed summary が、summary の域を越えて root evidence のように振る舞っていないか確認する。
- `plan/11`、`plan/17`、`progress.md` が Phase 3 を active mainline ではなく reserve path として一貫して mirror しているか確認する。

## 2. Scope and assumptions

- current normative / source-backed root は `specs/examples/107`〜`120`、特に `specs/examples/119-current-l2-reconnect-freeze-threshold.md` と `specs/examples/120-current-l2-phase3-self-driven-reopen-threshold.md` とする。
- `docs/research_abstract/`、`plan/`、`progress.md`、`README.md`、`Documentation.md`、`docs/reports/0347...` は mirror / summary / traceability layer として読む。
- review 対象は user 指定の uncommitted docs-only changes に限定し、code path review は行わない。
- `plan/ 更新不要`
- `progress.md 更新不要`

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/107-current-l2-stage3-suite-reopen-conditions.md`
- `specs/examples/111-current-l2-stage3-request-contract-subset-freeze-sequencing.md`
- `specs/examples/112-current-l2-phase3-resume-side-selection.md`
- `specs/examples/113-current-l2-first-checker-reconnect-family-selection.md`
- `specs/examples/114-current-l2-stage1-first-checker-reconnect-first-tranche-actualization.md`
- `specs/examples/115-current-l2-stage1-widening-vs-stage2-try-rollback-reconnect.md`
- `specs/examples/116-current-l2-stage2-try-rollback-reconnect-first-tranche-actualization.md`
- `specs/examples/117-current-l2-stage2-contrast-vs-stage1-summary-preserving-widening.md`
- `specs/examples/118-current-l2-stage1-summary-preserving-widening-actualization.md`
- `specs/examples/119-current-l2-reconnect-freeze-threshold.md`
- `specs/examples/120-current-l2-phase3-self-driven-reopen-threshold.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/phase0-repository-memory-and-decision-boundary.md`
- `docs/research_abstract/phase1-current-l2-semantics-stabilization.md`
- `docs/research_abstract/phase2-parser-free-poc-and-detached-validation-loop.md`
- `docs/research_abstract/phase3-parser-boundary-and-first-checker-cut.md`
- `docs/reports/0345-current-l2-phase3-closeout-sweep.md`
- `docs/reports/0346-review-phase3-closeout-checkpoint.md`
- `docs/reports/0347-phase3-self-driven-reopen-threshold-and-research-abstracts.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`

## 4. Actions taken

- AGENTS.md の読書順に合わせて foundational docs を確認し、その後に Phase 3 freeze / closeout / reopen-threshold chain を source として読んだ。
- uncommitted diff を対象ファイルごとに確認し、mirror wording が `specs/examples/120` を越えていないかを見た。
- `plan/11` / `plan/17` / `progress.md` の current-mainline wording を相互照合し、Phase 3 reserve-path reading に internal contradiction がないかを確認した。
- `docs/research_abstract/README.md` の summary-positioning と `plan/90-source-traceability.md` の addendum wording を突き合わせ、traceability inversion が起きていないかを確認した。
- `docs/reports/0347...` の action summary を actual diff と突き合わせ、report 自体の scope overclaim がないかを確認した。

## 5. Files changed

- Added: `docs/reports/0348-review-phase3-reopen-threshold-and-abstracts.md`

## 6. Commands run and exact outputs

- `git status --short`
  - ` M Documentation.md`
  - ` M README.md`
  - ` M plan/11-roadmap-near-term.md`
  - ` M plan/17-research-phases-and-autonomy-gates.md`
  - ` M plan/90-source-traceability.md`
  - ` M progress.md`
  - ` M specs/00-document-map.md`
  - `?? docs/reports/0347-phase3-self-driven-reopen-threshold-and-research-abstracts.md`
  - `?? docs/research_abstract/`
  - `?? specs/examples/120-current-l2-phase3-self-driven-reopen-threshold.md`
- `python3 scripts/new_report.py --slug review-phase3-reopen-threshold-and-abstracts`
  - created `docs/reports/0348-review-phase3-reopen-threshold-and-abstracts.md`
- `git diff --stat`
  - `7 files changed, 99 insertions(+), 70 deletions(-)`
- `rg -n "Phase 3|reserve path|主線|immediate execution order|止めるべき線" plan/17-research-phases-and-autonomy-gates.md progress.md plan/11-roadmap-near-term.md Documentation.md README.md specs/00-document-map.md docs/research_abstract/*.md`
  - Phase 3 reserve-path wording appears in `plan/11` / `plan/17` / `progress.md` / `docs/research_abstract/phase3...`
- `rg -n "narrative_explanations|must_explain|payload_core|explanation" specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md specs/examples/23-current-l2-detached-export-loop-consolidation.md specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md specs/examples/25-current-l2-detached-aggregate-emitter-sketch.md specs/examples/26-current-l2-detached-aggregate-compare-helper.md docs/research_abstract/phase2-parser-free-poc-and-detached-validation-loop.md`
  - Phase 2 abstract の detached-loop summary は `narrative_explanations` / `must_explain` placement について source docs と整合していた

review task のため、`cargo test` は実行していない。

## 7. Evidence / findings

1. **Medium — `progress.md` still contains an internal “mainline” contradiction for Phase 3.**
   `progress.md` 冒頭では、現在の主線を `Phase 2 maintenance tail + Phase 4 side line + Phase 5 inventory line` と定義し、Phase 3 は later pressure が出たときだけ reopen する reserve path だと明記している。一方で後半には `## 現在の mainline` という section title の下に `### 3. parser boundary staged spike` が残っており、同じファイルの中で Phase 3 を mainline 構成要素として再掲している。state snapshot として読み直した新規 agent には、Phase 3 が reserve path なのか active mainline なのかを曖昧にする。

2. **Medium — `plan/90` now inverts provenance by listing research abstracts as primary roots.**
   `docs/research_abstract/README.md` は abstracts を `condensed summary` と定義し、完全な traceability は `docs/reports/` / `specs/` / `plan/90` を参照するよう明示している。にもかかわらず `plan/90-source-traceability.md` の 2026-04-08 addendum は、top-level mirror updates の「主根拠」に各 phase abstract を並べている。これは summary を root evidence の側へ持ち上げる書き方であり、source-backed chain を弱める。特に `README.md` の今回の変更は directory listing 追加に留まるため、`specs/examples/120` や abstract 群を README 更新の primary basis として束ねるのも過剰である。

3. **No direct semantic overclaim found inside the abstract prose itself.**
   `docs/research_abstract/phase0..3` は summary / non-normative positioning を維持しており、Phase 3 abstract も `specs/examples/120` が与える reserve-path reading を大きく越えていない。問題は abstract の本文より、traceability layer がそれらを root evidence 扱いしている点にある。

## 8. What changed in understanding

- Phase 3 reserve-path judgment 自体は `specs/examples/120`、`plan/11`、`plan/17` でかなり整っており、mainline 移行の読みも概ね安定している。
- 今回の主問題は reopen-threshold judgment 本体の overclaim ではなく、mirror wording の取り残しと provenance layering の崩れだった。

## 9. Open questions

- `progress.md` の `## 現在の mainline` section は、title を変えて Phase 3 snapshot inventory にするのか、それとも Phase 3 subsection 自体を reserve-path appendix へ移すのか。
- `plan/90-source-traceability.md` では abstract 群を root から外し、`specs/examples/120` と `docs/reports/0347...` を primary source にしたうえで abstract は derived output として別記する方がよいか。
- `docs/research_abstract/` 各ファイルに、minimal な source pointer を個別に持たせる必要があるか。

## 10. Suggested next prompt

```text
review finding に合わせて、
1. `progress.md` の `## 現在の mainline` section が Phase 3 reserve-path reading と衝突しない wording / placement へ直るよう整理し、
2. `plan/90-source-traceability.md` で `docs/research_abstract/` を primary root ではなく derived summary として扱う traceability wording に補正し、
3. その修正を新しい report に記録してください。
```
