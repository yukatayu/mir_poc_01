# Report 0595 — current promoted line audit and mirror wording cleanup

- Date: 2026-04-11 17:07 JST
- Author / agent: Codex
- Scope: current status audit, self-driven research availability check, mirror wording cleanup
- Decision levels touched: none (`specs/` normative statements unchanged)

## 1. Objective

- Determine whether the repo still has a reasonably chunked self-driven research package, or whether unresolved questions must be settled first.
- Audit `Documentation.md`, `progress.md`, `tasks.md`, and relevant `plan/` mirrors so the current promoted line and later blocker/open-question distinction are stated explicitly.

## 2. Scope and assumptions

- This task is mirror / status maintenance only; no new normative requirement is introduced.
- Judgments remain grounded in `specs/` and existing `plan/` memory; snapshot docs are adjusted only where wording had become stronger than the underlying state.
- `plan/17` is already aligned with the current phase reading, so `plan/17 更新不要` とした。

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
- `specs/examples/271-current-l2-minimal-checker-payload-row-body-ready-checker-payload-supported-kind-summary-comparison.md`
- `specs/examples/272-current-l2-checker-payload-supported-kind-summary-ready-minimal-checker-payload-supported-kind-summary-threshold.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## 4. Actions taken

- Ran lightweight resource checks before further audit work.
- Re-read the required status / roadmap / invariant documents in repository order.
- Ran a grep-based mirror sweep for:
  - `specs/examples/126...272`
  - `minimal-checker-payload-supported-kind-summary-ready public-checker-payload-schema comparison`
  - `payload_row_family_ref + supported_kind_scope + supported_kind_refs`
- Confirmed that the current promoted line is consistently mirrored across `Documentation.md`, `progress.md`, `tasks.md`, `plan/11`, `plan/12`, `plan/17`, `plan/13`, `plan/90`, and the Phase 5 research abstract.
- Updated mirror wording where the documents read as if the large unresolved questions were immediate blockers, even though the current Phase 5 line remains self-driven.
- Updated source traceability for this wording audit.
- Files changed:
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/90-source-traceability.md`
  - `docs/reports/0595-current-promoted-line-audit-and-mirror-wording-cleanup.md`

## 5. Evidence / outputs / test results

- `df -h .`
  - `/dev/vda2 99G 94G 1.1G 99% /`
- `free -h`
  - `Mem: 960Mi total / 713Mi used / 109Mi free / 246Mi available`
  - `Swap: 19Gi total / 1.4Gi used / 18Gi free`
- `rg -n "minimal-checker-payload-supported-kind-summary-ready public-checker-payload-schema comparison|126\\.\\.\\.272|payload_row_family_ref \\+ supported_kind_scope \\+ supported_kind_refs" README.md Documentation.md progress.md tasks.md specs plan docs/research_abstract`
  - `Documentation.md` / `progress.md` / `tasks.md` / `plan/11` / `plan/12` / `plan/17` / `plan/13` / `plan/90` / research abstract で current package と next promoted line の mirror 一致を確認した。
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 594 numbered report(s).`
- `git diff --check`
  - no output
- `rg -n "current blocker|いま研究の障害になっている|## いまの blocker" Documentation.md progress.md tasks.md plan/11-roadmap-near-term.md plan/17-research-phases-and-autonomy-gates.md`
  - exit code `1` で一致なし
- `git status --short`
  - `M Documentation.md`
  - `M plan/11-roadmap-near-term.md`
  - `M plan/90-source-traceability.md`
  - `M progress.md`
  - `M tasks.md`
  - `?? docs/reports/0595-current-promoted-line-audit-and-mirror-wording-cleanup.md`

## 6. What changed in understanding

- There is still a reasonably chunked self-driven research package: Phase 5 later reopen as `minimal-checker-payload-supported-kind-summary-ready public-checker-payload-schema comparison`.
- Cross-phase checkpoint maintenance is also independently self-driven and can run in the same task package.
- The large unresolved questions in shared-space finalization and final parser / public checker boundary are real, but they are later blocker / open-question items, not immediate blockers for the current Phase 5 promoted line.
- The main gap was not semantic drift in `specs/`, but wording drift in snapshot mirrors.

## 7. Open questions

- How narrow the next `public-checker-payload-schema` comparison should remain before any public checker API wording is introduced.
- When shared-space final activation / authority / fairness decisions become concrete enough to reopen Phase 4 beyond checkpoint maintenance.
- When practical pressure is strong enough to reopen the Phase 3 reserve path.

## 8. Suggested next prompt

- `Phase 5 の current promoted line である minimal-checker-payload-supported-kind-summary-ready public-checker-payload-schema comparison を進め、同じ task の中で mirror sweep と report closeout までお願いします。`
