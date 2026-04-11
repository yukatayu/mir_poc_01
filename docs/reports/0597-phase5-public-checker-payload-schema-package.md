# Report 0597 — phase5 public checker payload schema package

- Date: 2026-04-11 17:46 JST
- Author / agent: Codex
- Scope: Phase 5 checker-side current promoted line continuation through public checker payload schema package
- Decision levels touched: current L2 docs-first boundary only (`specs/` normative additions at L2; no L0/L1 change)

## 1. Objective

- Continue the self-driven Phase 5 promoted line after checker payload supported kind summary.
- Determine whether public checker payload schema should now be cut as a docs-first line, and if so what its minimal wrapper shape should be.
- Update the repo mirrors so the new current package and next promoted line are consistent.

## 2. Scope and assumptions

- This task stays on the checker-side docs-first line and does not actualize public checker API, parser boundary, transport surface, or final checker implementation contract.
- The current Phase 5 basis remains: theorem-side retained bridge stops at `retained_payload_body_materialization_theorem_export_handoff_transport_channel_body`, and checker-side package already includes payload family / row family / row detail / row body / supported kind summary.
- `plan/`、`progress.md`、`tasks.md` は current package close と next promoted line の mirror として更新する。

## 3. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/132-current-l2-theorem-line-public-checker-migration-threshold.md`
- `specs/examples/198-current-l2-theorem-line-external-contract-facing-handoff-row-ready-actual-external-contract-threshold.md`
- `specs/examples/199-current-l2-theorem-line-actual-external-contract-ready-consumer-specific-external-contract-payload-threshold.md`
- `specs/examples/263-current-l2-supported-kind-summary-ready-actual-checker-payload-family-comparison.md`
- `specs/examples/264-current-l2-actual-checker-payload-family-ready-minimal-checker-payload-family-threshold.md`
- `specs/examples/271-current-l2-minimal-checker-payload-row-body-ready-checker-payload-supported-kind-summary-comparison.md`
- `specs/examples/272-current-l2-checker-payload-supported-kind-summary-ready-minimal-checker-payload-supported-kind-summary-threshold.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `crates/mir-semantics/src/lib.rs`
- `scripts/current_l2_reason_codes_assist.py`

## 4. Actions taken

- Re-read the required status / invariant / roadmap docs and confirmed that the current promoted line remained `minimal-checker-payload-supported-kind-summary-ready public-checker-payload-schema comparison`.
- Re-read the immediately preceding checker-side package (`263`〜`264`, `271`〜`272`) and theorem-side anti-premature-public-surface references (`132`, `198`, `199`) to keep the comparison narrow.
- Added:
  - `specs/examples/273-current-l2-minimal-checker-payload-supported-kind-summary-ready-public-checker-payload-schema-comparison.md`
  - `specs/examples/274-current-l2-public-checker-payload-schema-ready-minimal-public-checker-payload-schema-threshold.md`
- Fixed the current first choice as:
  - public checker payload schema should be cut as a docs-only wrapper line
  - its minimal shape should be `actual_checker_payload_family_ref + checker_payload_row_family_ref + checker_payload_row_detail_ref + checker_payload_row_body_ref + checker_payload_supported_kind_summary_ref`
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

## 5. Files changed

- `specs/examples/273-current-l2-minimal-checker-payload-supported-kind-summary-ready-public-checker-payload-schema-comparison.md`
- `specs/examples/274-current-l2-public-checker-payload-schema-ready-minimal-public-checker-payload-schema-threshold.md`
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
- `docs/reports/0597-phase5-public-checker-payload-schema-package.md`

## 6. Commands run and exact outputs

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
  - `Task baseline recorded.`
- `df -h .`
  - `Filesystem      Size  Used Avail Use% Mounted on`
  - `/dev/vda2        99G   94G  1.1G  99% /`
- `free -h`
  - `Mem: 960Mi total / 721Mi used / 85Mi free / 238Mi available`
  - `Swap: 19Gi total / 1.4Gi used / 18Gi free`
- `date '+%Y-%m-%d %H:%M:%S %Z'`
  - `2026-04-11 17:46:23 JST`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 596 numbered report(s).`
- `git diff --check`
  - no output
- `git diff --stat`
  - `10 files changed, 37 insertions(+), 21 deletions(-)` on tracked files
  - plus `specs/examples/273`, `specs/examples/274`, `docs/reports/0597` as newly added files

## 7. Evidence / outputs / test results

- `specs/examples/273` では、public checker payload schema を still prose や public checker API 直接移行ではなく、`public_checker_payload_schema_ready_sketch` docs-only wrapper として切る案を current first choice に固定した。
- `specs/examples/274` では、その wrapper の minimal shape を
  - `actual_checker_payload_family_ref`
  - `checker_payload_row_family_ref`
  - `checker_payload_row_detail_ref`
  - `checker_payload_row_body_ref`
  - `checker_payload_supported_kind_summary_ref`
  の 5 ref に留める current first choice を固定した。
- `python3 scripts/validate_docs.py` は通過し、docs scaffold / numbered report inventory は整合した。
- `git diff --check` は無出力で、今回の docs 更新に whitespace / conflict marker 問題はない。
- これにより current package は `specs/examples/126...274` まで close と読め、next promoted line は `minimal-public-checker-payload-schema-ready public-checker-api comparison` へ進める状態になった。

## 8. What changed in understanding

- supported kind summary の次段で自然なのは flattened public row schema でも public checker API でもなく、already-cut checker-side lines を束ねる docs-only schema wrapper である。
- public checker payload schema の minimal cut は family と summary だけでは弱く、row family / row detail / row body も separate ref として含めた方が current package の役割分担を保ちやすい。
- theorem-side の public contract comparison と同様に、schema wrapper と public API を同じ reopen で混ぜない方が ratchet がきれいである。

## 9. Open questions

- public checker API の first reopen cut をどの layer から始めるのが最小か
- flattened public checker row schema を later でどの line から reopen するか
- `*_ref` naming family を current phase でどこまで stable token として扱うか

## 10. Suggested next prompt

- `Phase 5 の current promoted line である minimal-public-checker-payload-schema-ready public-checker-api comparison を進め、mirror sweep と report closeout までお願いします。`
