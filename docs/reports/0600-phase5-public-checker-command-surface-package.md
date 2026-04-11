# Report 0600 — phase5 public checker command surface package

- Date: 2026-04-11 19:02 JST
- Author / agent: Codex
- Scope: Phase 5 checker-side current promoted line continuation through public checker command surface package
- Decision levels touched: current L2 docs-first boundary only (`specs/` normative additions at L2; no L0/L1 change)

## 1. Objective

- Continue the self-driven Phase 5 promoted line after the public checker entry criteria package.
- Determine what the first docs-only public checker command-surface cut should be, and what its minimum shape should be.
- Update `tasks.md` / `progress.md` / `plan/` mirrors so the completed package disappears from the top task and the next promoted line is explicit.

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
- `specs/examples/50-current-l2-generic-family-checker-entry-comparison.md`
- `specs/examples/63-current-l2-try-rollback-ast-helper-public-checker-entry-criteria.md`
- `specs/examples/72-current-l2-try-rollback-first-tranche-generic-public-recheck.md`
- `specs/examples/275-current-l2-minimal-public-checker-payload-schema-ready-public-checker-api-comparison.md`
- `specs/examples/276-current-l2-public-checker-api-ready-minimal-public-checker-api-threshold.md`
- `specs/examples/277-current-l2-minimal-public-checker-api-ready-public-checker-entry-criteria-comparison.md`
- `specs/examples/278-current-l2-public-checker-entry-criteria-ready-minimal-public-checker-entry-criteria-threshold.md`
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

## 3. Actions taken

- Re-read the required base docs, `plan/00-index.md`, the current status snapshots, and the relevant Phase 5 / checker-side comparison chain before touching the new package.
- Re-checked the command-surface-related source anchors and the earlier judgments that kept family facades while deferring a generic shared entry.
- Added:
  - `specs/examples/279-current-l2-minimal-public-checker-entry-criteria-ready-public-checker-command-surface-comparison.md`
  - `specs/examples/280-current-l2-public-checker-command-surface-ready-minimal-public-checker-command-surface-threshold.md`
- Fixed the current first choice as:
  - public checker command surface should start from the existing family facade pattern rather than a generic shared entry
  - the minimum command-surface shape should be `command_surface_kind + family_facade_command_refs + public_checker_api_ref`
  - detached loop `smoke-*` wrappers, shared generic entry, shared output contract, and parser boundary should remain later
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

- `specs/examples/279-current-l2-minimal-public-checker-entry-criteria-ready-public-checker-command-surface-comparison.md`
- `specs/examples/280-current-l2-public-checker-command-surface-ready-minimal-public-checker-command-surface-threshold.md`
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
- `docs/reports/0600-phase5-public-checker-command-surface-package.md`

## 5. Commands run and exact outputs

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
  - `Task baseline recorded.`
- `df -h .`
  - `Filesystem      Size  Used Avail Use% Mounted on`
  - `/dev/vda2        99G   94G 1012M  99% /`
- `free -h`
  - `Mem: 960Mi total / 747Mi used / 75Mi free / 212Mi available`
  - `Swap: 19Gi total / 1.5Gi used / 18Gi free`
- `date '+%Y-%m-%d %H:%M:%S %Z'`
  - `2026-04-11 19:02:03 JST`
- `python3 scripts/new_report.py --slug phase5-public-checker-command-surface-package`
  - `/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).`
  - `  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")`
  - `/home/yukatayu/dev/mir_poc_01/docs/reports/0600-phase5-public-checker-command-surface-package.md`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 599 numbered report(s).`
- `git diff --check`
  - no output
- `git status --short`
  - ` M Documentation.md`
  - ` M docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
  - ` M plan/11-roadmap-near-term.md`
  - ` M plan/12-open-problems-and-risks.md`
  - ` M plan/13-heavy-future-workstreams.md`
  - ` M plan/17-research-phases-and-autonomy-gates.md`
  - ` M plan/90-source-traceability.md`
  - ` M progress.md`
  - ` M specs/00-document-map.md`
  - ` M tasks.md`
  - `?? docs/reports/0600-phase5-public-checker-command-surface-package.md`
  - `?? specs/examples/279-current-l2-minimal-public-checker-entry-criteria-ready-public-checker-command-surface-comparison.md`
  - `?? specs/examples/280-current-l2-public-checker-command-surface-ready-minimal-public-checker-command-surface-threshold.md`

## 6. Evidence / findings

- `specs/examples/279` では、public checker command surface comparison の first candidate を existing family facade pattern に置き、generic shared entry ではなく family-facade anchored line から始める current first choice を固定した。
- `specs/examples/280` では、その minimum を
  - `command_surface_kind`
  - `family_facade_command_refs`
  - `public_checker_api_ref`
  の 3 field に留め、detached loop `smoke-*` wrapper は source evidence にとどめる current first choice を固定した。
- これにより current package は `specs/examples/126...280` まで close と読め、next promoted line は `minimal-public-checker-command-surface-ready shared-output-contract comparison` に進める状態になった。
- `tasks.md` と `progress.md` はこの package close に合わせて更新され、前回の top task は snapshot から消えた。
- `python3 scripts/validate_docs.py` は通過し、docs scaffold / numbered report inventory は整合した。
- `git diff --check` は無出力で、今回の docs 更新に whitespace / conflict marker 問題はない。

## 7. Changes in understanding

- current public checker command-surface pressure は generic shared entry ではなく、shared support helper の下で維持されている family facade pattern 自体にある。
- detached loop の `smoke-*` wrapper は current repo-visible evidence として重要だが、first minimal command-surface shape にそのまま混ぜる必要はない。
- command surface comparison を 1 段切ったことで、shared output contract を次段として比較してよい一方、parser boundary は still later に残す方が current ratchet に合う。

## 8. Open questions

- shared output contract の minimum を command surface の次段で何 field までに留めるべきか
- detached loop `smoke-*` wrapper を later public surface に含めるべきか
- generic shared public checker entry を later でどの trigger から切るべきか

## 9. Suggested next prompt

- `Phase 5 の current promoted line である minimal-public-checker-command-surface-ready shared-output-contract comparison を進め、mirror sweep と report closeout までお願いします。`
