# Report 0599 — phase5 public checker entry criteria package

- Date: 2026-04-11 18:40 JST
- Author / agent: Codex
- Scope: Phase 5 checker-side current promoted line continuation through public checker entry criteria package
- Decision levels touched: current L2 docs-first boundary only (`specs/` normative additions at L2; no L0/L1 change)

## 1. Objective

- Continue the self-driven Phase 5 promoted line after the minimal public checker API package.
- Determine whether public checker comparison should require a separate entry-criteria gate, and if so what the minimum gate should be.
- Update the repo mirrors so the new current package and next promoted line stay consistent.

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/34-current-l2-static-reason-code-entry-criteria.md`
- `specs/examples/49-current-l2-shared-family-checker-support-helper.md`
- `specs/examples/63-current-l2-try-rollback-ast-helper-public-checker-entry-criteria.md`
- `specs/examples/72-current-l2-try-rollback-first-tranche-generic-public-recheck.md`
- `specs/examples/256-current-l2-small-decidable-core-ready-checker-cluster-matrix-comparison.md`
- `specs/examples/275-current-l2-minimal-public-checker-payload-schema-ready-public-checker-api-comparison.md`
- `specs/examples/276-current-l2-public-checker-api-ready-minimal-public-checker-api-threshold.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `scripts/current_l2_detached_loop.py`
- `scripts/current_l2_family_checker_support.py`
- `scripts/current_l2_same_lineage_checker.py`
- `scripts/current_l2_missing_option_checker.py`
- `scripts/current_l2_capability_checker.py`
- `scripts/current_l2_try_rollback_structural_checker.py`

## 3. Actions taken

- Re-read the required status / invariant / roadmap docs and confirmed that the current promoted line remained `minimal-public-checker-api-ready public-checker-entry-criteria comparison`.
- Re-read the recent checker-side package (`275`〜`276`), the earlier checker/public entry criteria judgments (`30`, `49`, `63`, `72`), and the current code anchors in the detached loop / family-facade scripts.
- Added:
  - `specs/examples/277-current-l2-minimal-public-checker-api-ready-public-checker-entry-criteria-comparison.md`
  - `specs/examples/278-current-l2-public-checker-entry-criteria-ready-minimal-public-checker-entry-criteria-threshold.md`
- Fixed the current first choice as:
  - public checker comparison should require a separate docs-only entry-criteria gate
  - the minimum gate should be `minimal API fixed + source-backed family-facade command-surface pressure`
  - shared output contract / parser boundary should remain later
- Updated the mirrors:
  - `specs/00-document-map.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/12-open-problems-and-risks.md`
  - `plan/13-heavy-future-workstreams.md`
  - `plan/17-research-phases-and-autonomy-gates.md`
  - `plan/90-source-traceability.md`
  - `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## 4. Files changed

- `specs/examples/277-current-l2-minimal-public-checker-api-ready-public-checker-entry-criteria-comparison.md`
- `specs/examples/278-current-l2-public-checker-entry-criteria-ready-minimal-public-checker-entry-criteria-threshold.md`
- `specs/00-document-map.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0599-phase5-public-checker-entry-criteria-package.md`

## 5. Commands run and exact outputs

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
  - `Task baseline recorded.`
- `df -h .`
  - `Filesystem      Size  Used Avail Use% Mounted on`
  - `/dev/vda2        99G   94G 1015M  99% /`
- `free -h`
  - `Mem: 960Mi total / 781Mi used / 77Mi free / 178Mi available`
  - `Swap: 19Gi total / 1.5Gi used / 18Gi free`
- `date '+%Y-%m-%d %H:%M:%S %Z'`
  - `2026-04-11 18:36:32 JST`
- `python3 scripts/new_report.py --slug phase5-public-checker-entry-criteria-package`
  - `/home/yukatayu/dev/mir_poc_01/docs/reports/0599-phase5-public-checker-entry-criteria-package.md`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 598 numbered report(s).`
- `git diff --check`
  - no output

## 6. Evidence / findings

- `specs/examples/277` では、public checker API minimal shape と public-looking reopen condition を別段に扱い、first reopen target を command surface に限る current first choice を固定した。
- `specs/examples/278` では、その minimum を
  - docs-only minimal API fixed
  - source-backed family-facade command-surface pressure
  - heavier boundary deferred
  の 3 層に整理し、shared output contract / parser boundary を still later に残す current first choice を固定した。
- これにより current package は `specs/examples/126...278` まで close と読め、next promoted line は `minimal-public-checker-entry-criteria-ready public-checker-command-surface comparison` に進める状態になった。
- `python3 scripts/validate_docs.py` は通過し、docs scaffold / numbered report inventory は整合した。
- `git diff --check` は無出力で、今回の docs 更新に whitespace / conflict marker 問題はない。

## 7. Changes in understanding

- public checker API minimal relation が切れたこと自体は、public-looking reopen を始める十分条件ではない。
- current repo で concrete に見えている public-looking pressure は shared output contract や parser boundary ではなく、family facade smoke commands が並んでいる command surface 側である。
- `specs/examples/72` の pause judgment を維持するには、generic/public pressure が薄い family を current package に無理に混ぜず、command-surface comparison を先に narrow に切る方が自然である。

## 8. Open questions

- public checker command surface comparison の first candidate を family facade preservation / shared generic entry / query-token refinement のどれに置くべきか
- shared output contract を command surface comparison の次段に置くべきか
- `checker_subject` を later で dedicated query token / `*_ref` family に寄せるべきか

## 9. Suggested next prompt

- `Phase 5 の current promoted line である minimal-public-checker-entry-criteria-ready public-checker-command-surface comparison を進め、mirror sweep と report closeout までお願いします。`
