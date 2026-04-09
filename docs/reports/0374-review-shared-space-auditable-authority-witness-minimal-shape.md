# Report 0374 — review shared space auditable authority witness minimal shape

- Date: 2026-04-09T16:35:40+09:00
- Author / agent: Codex
- Scope: Phase 4 shared-space / membership docs-first line の uncommitted mirror sweep を review し、`auditable_authority_witness` minimal shape の semantic drift を最小修正する
- Decision levels touched: L2 / L3

## 1. Objective

`specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md` を正本として、shared-space docs-first task の uncommitted mirror 群に semantic drift がないかを確認し、必要なら最小修正する。

## 2. Scope and assumptions

- review 対象は user 指定の 10 file と新規 report である。
- 規範判断の正本は `specs/` に置き、`plan/` / `progress.md` / `tasks.md` は mirror として扱う。
- 今回は final catalog、final exporter schema、final auth / identity integration を固定しない。
- 規範 spec の判断変更は行わない。

## 3. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `specs/examples/121-shared-space-authoritative-room-baseline.md`
- `specs/examples/122-shared-space-catalog-working-subset-comparison.md`
- `specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `tasks.md`
- `docs/reports/0373-shared-space-auditable-authority-witness-minimal-shape.md`

## 4. Actions taken

1. Read the mandated repo docs in order, then read the shared-space baseline chain (`121` / `122` / `123`) and the touched mirrors.
2. Compared uncommitted diffs against the requested review criteria: witness/provider separation, room-profile vs audit split, next-step handoff, anti-overclaim wording, and mirror consistency.
3. Found one stale mirror statement in `plan/16` that still said witness-shape comparison should come before provider placement.
4. Updated `plan/16` so it now matches `specs/examples/123...`, `plan/11`, `plan/17`, `progress.md`, `tasks.md`, and report `0373`.
5. Added this review report.

`progress.md 更新不要`

`tasks.md 更新不要`

## 5. Files changed

- `plan/16-shared-space-membership-and-example-boundary.md`
- `docs/reports/0374-review-shared-space-auditable-authority-witness-minimal-shape.md`

## 6. Commands run

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-09 16:35 JST

$ git diff -- Documentation.md specs/00-document-map.md specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md plan/11-roadmap-near-term.md plan/12-open-problems-and-risks.md plan/16-shared-space-membership-and-example-boundary.md plan/17-research-phases-and-autonomy-gates.md plan/90-source-traceability.md progress.md tasks.md docs/reports/0373-shared-space-auditable-authority-witness-minimal-shape.md

$ rg -n "auditable_authority_witness|delegated_rng_service|provider placement|provider-placement|room profile|audit / receipt|final catalog|final exporter|final .*schema|auth / identity|identity / auth|fairness_claim|witness_kind|draw_slot|draw_result|action_ref" Documentation.md specs/00-document-map.md specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md plan/11-roadmap-near-term.md plan/12-open-problems-and-risks.md plan/16-shared-space-membership-and-example-boundary.md plan/17-research-phases-and-autonomy-gates.md plan/90-source-traceability.md progress.md tasks.md docs/reports/0373-shared-space-auditable-authority-witness-minimal-shape.md

$ git diff -- plan/16-shared-space-membership-and-example-boundary.md docs/reports/0374-review-shared-space-auditable-authority-witness-minimal-shape.md

$ git diff --check

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 374 numbered report(s).
```

## 7. Evidence / outputs / test results

- `git diff` showed all top-level mirrors had already moved the current next narrow step to authoritative-room-side `delegated_rng_service` comparison except `plan/16`.
- `rg` over the target set found the stale wording at `plan/16` around the fairness trust model section, while `specs/examples/123...`, `plan/11`, `plan/17`, `progress.md`, `tasks.md`, and `docs/reports/0373...` already agreed that minimal witness core was closed and provider placement was next.
- After the patch, `plan/16` now states:
  - `auditable_authority_witness` minimal witness core is already cut in `specs/examples/123...`
  - the current next narrow step is the authoritative-room `delegated_rng_service` practical-cut comparison
  - provider placement and witness requirement remain separate axes
- `git diff --check` returned no output.
- `python3 scripts/validate_docs.py` exited 0 with `Documentation scaffold looks complete.` and `Found 374 numbered report(s).`

## 8. What changed in understanding

- The uncommitted work was largely consistent already; the main defect was a single stale mirror in `plan/16`, not a broader drift across the targeted files.
- The important boundary is now stable across the docs set: room profile carries only `fairness_claim`, while the witness payload stays on the audit / receipt side as `witness_kind + action_ref + draw_slot + draw_result`.

## 9. Open questions

- `action_ref` の final stable-ref policy をどの layer で固定するか。
- authoritative room で `delegated_rng_service` を practical candidate に置いたときも、minimal witness core を unchanged で保てるか。
- control-plane separated causal carrier を reopen する threshold。

## 10. Suggested next prompt

`shared-space の authoritative room について、delegated_rng_service を provider-placement candidate としてどこまで practical に読めるかを docs-first に比較し、authority_rng baseline・minimal witness core・room profile / audit split を崩さない最小 cut を整理してください。`
