# Report 0708 — faq005 literature mapping and theory-lab followup

- Date: 2026-04-17T07:22:00+09:00
- Author / agent: Codex
- Scope: FAQ 05 explanatory expansion plus two docs-first theory-lab packages
- Decision levels touched: L2 / L3

## 1. Objective

1. Expand `faq_005.md` so it explains:
   - the modality literature line (`lambda-circle-box`, guarded lambda-calculus, MDTT, MTT)
   - the current order / handoff / `memory_order` reinterpretation line
   - a concrete dice-owner handoff explanation without overstating settled syntax
2. Close the next two self-driven theory-lab packages:
   - model-check small-cluster projection keep/drop refresh
   - order/handoff source-surface wording reserve note
3. Update snapshot / plan / traceability mirrors so no stale current-line wording remains.

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
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `specs/examples/405-current-l2-order-cut-family-comparison.md`
- `specs/examples/407-current-l2-order-visibility-witness-family-comparison.md`
- `specs/examples/410-current-l2-modal-foundation-comparison.md`
- `specs/examples/411-current-l2-order-handoff-adequacy-corpus-and-verification-boundary-matrix.md`
- `specs/examples/420-current-l2-model-check-carrier-to-projection-bridge-note.md`
- `specs/examples/427-current-l2-sample-visible-theorem-model-check-property-summary-wording.md`
- `specs/examples/428-current-l2-order-handoff-property-language-bridge-note.md`
- `specs/examples/429-current-l2-modal-promotion-threshold-note.md`
- `specs/examples/436-current-l2-order-handoff-emitted-artifact-schema-reserve-note.md`
- `specs/examples/422-current-l2-modal-foundation-comparison-follow-up.md`
- `faq_004.md`
- `faq_005.md`

## 3. Actions taken

1. Added `specs/examples/441...442` to close the next two theory-lab packages.
2. Fixed the model-check bridge at a **same-subject stage-local small-cluster** keep line and explicitly dropped typed reserve / theorem discharge / room protocol family from that current cut.
3. Fixed the order/handoff explanation line at a **two-layer wording**:
   - snake_case relation family names as principal wording
   - plain-language stage wording as explanation layer
   while keeping low-level fence-like wording and room macro wording out of the current cut.
4. Rewrote `faq_005.md` so it now includes:
   - current literature mapping for modality
   - dice-owner handoff explanation for order/handoff and cut family terms
   - explicit cautions that these are current explanation labels, not final syntax
5. Updated `Documentation.md`, `progress.md`, `tasks.md`, relevant `plan/`, `specs/00-document-map.md`, `specs/10-open-questions.md`, and relevant research abstracts.

## 4. Files changed

- `specs/examples/441-current-l2-model-check-small-cluster-projection-keep-drop-refresh.md`
- `specs/examples/442-current-l2-order-handoff-source-surface-wording-reserve-note.md`
- `faq_005.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `faq_004.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `docs/reports/0708-faq005-literature-mapping-and-theory-lab-followup.md`

## 5. Commands run and exact outputs

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
df -h .
free -h
date '+%Y-%m-%d %H:%M:%S %Z'
python3 scripts/new_report.py --slug faq005-literature-mapping-and-theory-lab-followup
date '+%Y-%m-%d %H:%M JST'
python3 .agents/skills/discord-report/scripts/discord_notify.py progress --summary "model-check small-cluster projection keep/drop を閉じ、same-subject stage-local cluster を current cut に固定しました。次は order/handoff source-surface wording reserve を閉じます。" --cwd .
```

## 6. Evidence / findings

- resource snapshot at start:
  - `df -h .` -> `/dev/vda2 99G 77G 18G 82% /`
  - `free -h` -> `Mem 960Mi total / 762Mi used / 69Mi free / 282Mi buff-cache / 197Mi available`
- task baseline:
  - `Task baseline recorded.`
- first package close:
  - model-check small-cluster projection keep/drop was narrow enough to fix now; no concrete tool or room protocol projection was needed.
- second package close:
  - snake_case relation family names are workable as principal docs vocabulary and are less drift-prone than short aliases or fence-like surface hints.
- current FAQ expansion is supportable from source-backed docs without promoting OPEN candidates to settled vocabulary.

## 7. Changes in understanding

- The model-check line no longer needs a near-term “keep/drop” open item; the next meaningful question is now the later gate for settled property language and concrete tool seam.
- The order/handoff line no longer needs a near-term wording reserve item; the next pressure is modality internalization and later source-surface / property-language gates.
- `faq_005.md` can safely carry literature mapping and a concrete motivating example, but only if it keeps the explicit distinction between:
  - current explanation
  - current working line
  - OPEN / later gate

## 8. Open questions

- modality internalization trigger
- malformed duplicate-cluster exact actualization cut
- stronger typed-surface promotion threshold
- theorem discharge transport / public-contract later gate
- model-check property-language / concrete tool seam later gate

## 9. Update necessity notes

- `plan/` updated
- `progress.md` updated
- `tasks.md` updated
- `Documentation.md` updated
- `AGENTS.md` updated unnecessary
- `.docs/` updated unnecessary

## 10. Suggested next prompt

`では引き続き、tasks.md 先頭の modality internalization trigger note と malformed duplicate-cluster source-authored static-stop pair actualization comparison を自走でお願いします。`
