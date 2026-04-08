# 0271 — review shared-space activation rule comparison

## Objective

Review the current uncommitted docs-only shared-space activation rule comparison, with emphasis on layer separation, avoidance of premature algorithm / consensus fixation, careful status of `authority-ack` as a minimal operational candidate, compile-time vs runtime control-plane separation, authoritative-room vs append-friendly-room consistency, and report / mirror hygiene.

## Scope and assumptions

- Review-only task. No normative spec edits and no fixes applied as part of this task.
- `plan/` 更新不要.
- `progress.md` 更新不要.
- The reviewed change set is limited to:
  - `plan/10-roadmap-overall.md`
  - `plan/12-open-problems-and-risks.md`
  - `plan/16-shared-space-membership-and-example-boundary.md`
  - `progress.md`
  - `docs/reports/0270-shared-space-activation-rule-comparison.md`

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/05-mirrorea-fabric.md`
- `specs/08-cross-system-relations.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/00-index.md`
- `plan/10-roadmap-overall.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/90-source-traceability.md`
- `plan/91-maintenance-rules.md`
- `docs/reports/0268-shared-space-authority-ownership-and-consistency-comparison.md`
- `docs/reports/0269-review-shared-space-authority-doc-change.md`
- `docs/reports/0270-shared-space-activation-rule-comparison.md`

## Actions taken

1. Read the required baseline repo documents in the prescribed order, including the relevant shared-space / Mirrorea context.
2. Inspected the uncommitted diff for the five target files.
3. Checked surrounding context in `plan/16` and the immediately preceding shared-space reports to verify that the new activation comparison does not collapse Mir core, Mirrorea, and L3 shared-space responsibilities.
4. Checked report / mirror hygiene against `plan/90-source-traceability.md` and `plan/91-maintenance-rules.md`.
5. Ran lightweight doc validation and whitespace checks.
6. Wrote this review report.

## Evidence / outputs / test results

Commands run:

```text
$ git status --short
 M plan/10-roadmap-overall.md
 M plan/12-open-problems-and-risks.md
 M plan/16-shared-space-membership-and-example-boundary.md
 M progress.md
?? docs/reports/0270-shared-space-activation-rule-comparison.md

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 270 numbered report(s).

$ git diff --check -- plan/10-roadmap-overall.md plan/12-open-problems-and-risks.md plan/16-shared-space-membership-and-example-boundary.md progress.md docs/reports/0270-shared-space-activation-rule-comparison.md
[no output]
```

Focused review result:

- No substantive finding was identified for layer separation, premature algorithm fixation, or compile-time/runtime boundary drift in the compared activation-rule wording itself.
- Low-severity findings were identified in report / mirror hygiene.

## What changed in understanding

- The new activation-rule comparison is materially careful about keeping `authority-ack`, `full-coverage-like activation`, and `quorum-like activation` at the shared-space operational-policy layer rather than promoting them into Mir core semantics or Mir-1 cut-profile vocabulary.
- `authority-ack` is stated as a current minimal operational candidate, not as a finalized protocol rule, and the text keeps replicated-authority agreement and consensus realization outside the current fixed spec surface.
- The compile-time over-approximation vs runtime control-plane split remains consistent in `plan/16`, `plan/10`, `plan/12`, `progress.md`, and `docs/reports/0270`.
- The main remaining weaknesses are not semantic; they are traceability / report-hygiene omissions.

## Open questions

- Should `plan/90-source-traceability.md` be updated whenever `plan/10`, `plan/12`, or `plan/16` gain a new primary report source, or only on larger memory reorganizations? Current practice in nearby tasks suggests the former.
- Should `docs/reports/0270` record the actual local validation and reviewer completion state before the task is considered closed, instead of leaving them as pending items?

## Suggested next prompt

`docs/reports/0270-shared-space-activation-rule-comparison.md` と `plan/90-source-traceability.md` の hygiene だけを直してください。activation rule の substantive wording は変えず、0270 の validation / reviewer evidence を実績ベースに更新し、plan/10・plan/12・plan/16 の traceability に 0270 を反映してください。
