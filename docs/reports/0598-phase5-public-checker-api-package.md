# Report 0598 — phase5 public checker api package

- Date: 2026-04-11 18:25 JST
- Author / agent: Codex
- Scope: Phase 5 checker-side current promoted line continuation through public checker API package
- Decision levels touched: current L2 docs-first boundary only (`specs/` normative additions at L2; no L0/L1 change)

## 1. Objective

- Continue the self-driven Phase 5 promoted line after public checker payload schema.
- Determine whether public checker API should now be cut as a docs-first line, and if so what its minimal read relation should be.
- Update the repo mirrors so the new current package and next promoted line are consistent.

## 2. Scope and assumptions

- This task stays on the checker-side docs-first line and does not actualize command naming, shared output contract, parser boundary, transport surface, or final checker implementation contract.
- The current Phase 5 basis remains: theorem-side retained bridge stops at `retained_payload_body_materialization_theorem_export_handoff_transport_channel_body`, and checker-side package already includes payload family / row family / row detail / row body / supported kind summary / public checker payload schema.
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
- `specs/examples/49-current-l2-shared-family-checker-support-helper.md`
- `specs/examples/62-current-l2-try-rollback-ast-helper-generic-family-boundary.md`
- `specs/examples/63-current-l2-try-rollback-ast-helper-public-checker-entry-criteria.md`
- `specs/examples/132-current-l2-theorem-line-public-checker-migration-threshold.md`
- `specs/examples/198-current-l2-theorem-line-external-contract-facing-handoff-row-ready-actual-external-contract-threshold.md`
- `specs/examples/199-current-l2-theorem-line-actual-external-contract-ready-consumer-specific-external-contract-payload-threshold.md`
- `specs/examples/256-current-l2-small-decidable-core-ready-checker-cluster-matrix-comparison.md`
- `specs/examples/257-current-l2-checker-cluster-matrix-ready-minimal-checker-cluster-row-threshold.md`
- `specs/examples/273-current-l2-minimal-checker-payload-supported-kind-summary-ready-public-checker-payload-schema-comparison.md`
- `specs/examples/274-current-l2-public-checker-payload-schema-ready-minimal-public-checker-payload-schema-threshold.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `scripts/current_l2_detached_loop.py`
- `scripts/current_l2_family_checker_support.py`
- `scripts/current_l2_reason_codes_assist.py`

## 4. Actions taken

- Re-read the required status / invariant / roadmap docs and confirmed that the current promoted line remained `minimal-public-checker-payload-schema-ready public-checker-api comparison`.
- Re-read the checker-side payload package (`273`〜`274`), checker-side generic/public-entry guard judgments (`49`, `62`, `63`), and theorem-side anti-premature-public-surface references (`132`, `198`, `199`) to keep the comparison narrow.
- Added:
  - `specs/examples/275-current-l2-minimal-public-checker-payload-schema-ready-public-checker-api-comparison.md`
  - `specs/examples/276-current-l2-public-checker-api-ready-minimal-public-checker-api-threshold.md`
- Fixed the current first choice as:
  - public checker API should be cut as a docs-only read relation line
  - its minimal shape should be `checker_subject + public_checker_payload_schema_ref`
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

- `specs/examples/275-current-l2-minimal-public-checker-payload-schema-ready-public-checker-api-comparison.md`
- `specs/examples/276-current-l2-public-checker-api-ready-minimal-public-checker-api-threshold.md`
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
- `docs/reports/0598-phase5-public-checker-api-package.md`

## 6. Commands run and exact outputs

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
  - `Task baseline recorded.`
- `df -h .`
  - `Filesystem      Size  Used Avail Use% Mounted on`
  - `/dev/vda2        99G   94G 1017M  99% /`
- `free -h`
  - `Mem: 960Mi total / 724Mi used / 80Mi free / 235Mi available`
  - `Swap: 19Gi total / 1.5Gi used / 18Gi free`
- `date '+%Y-%m-%d %H:%M:%S %Z'`
  - `2026-04-11 18:25:41 JST`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 597 numbered report(s).`
- `git diff --check`
  - no output

## 7. Evidence / outputs / test results

- `specs/examples/275` では、public checker API を still prose や actual command surface 直接移行ではなく、`public_checker_api_ready_sketch` docs-only read relation として切る案を current first choice に固定した。
- `specs/examples/276` では、その minimal shape を
  - `checker_subject`
  - `public_checker_payload_schema_ref`
  の 2 field に留める current first choice を固定した。
- `python3 scripts/validate_docs.py` は通過し、docs scaffold / numbered report inventory は整合した。
- `git diff --check` は無出力で、今回の docs 更新に whitespace / conflict marker 問題はない。
- これにより current package は `specs/examples/126...276` まで close と読め、next promoted line は `minimal-public-checker-api-ready public-checker-entry-criteria comparison` へ進める状態になった。

## 8. What changed in understanding

- public checker payload schema の次段で自然なのは actual command surface ではなく、どの `checker_subject` に対してどの schema を返すかを示す docs-only read relation である。
- current family facade / detached loop smoke surface は non-production helper として十分 source-backed だが、それ自体を public checker API に昇格させる段階ではまだない。
- generic/public entry criteria は API minimal shape とは別 reopen として扱う方が checker-side ratchet を保ちやすい。

## 9. Open questions

- actual command surface / shared output contract / parser boundary を reopen してよい public checker entry criteria をどこに置くか
- `checker_subject` を later で dedicated query token / `*_ref` family に寄せるべきか
- family facade pattern と future public checker API comparison をどこで接続するべきか

## 10. Suggested next prompt

- `Phase 5 の current promoted line である minimal-public-checker-api-ready public-checker-entry-criteria comparison を進め、mirror sweep と report closeout までお願いします。`
