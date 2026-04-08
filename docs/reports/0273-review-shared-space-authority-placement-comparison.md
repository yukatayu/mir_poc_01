# Report 0273 — review shared-space authority placement comparison

- Date: 2026-04-08 11:21 JST
- Author / agent: Codex
- Scope: Uncommitted docs-only review of the shared-space authority placement comparison and its mirrors
- Decision levels touched: L1/L2 wording review only; no normative change

## 1. Objective

Review the current uncommitted authority placement comparison in:

- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `progress.md`
- `docs/reports/0272-shared-space-authority-placement-comparison.md`

with focus on:

- shared-space / Mirrorea / implementation separation
- whether `single room authority` is still framed as a current-phase candidate
- whether participant carrier, resource owner slot, and authority placement remain separated
- report hygiene / traceability

## 2. Inputs consulted

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
- `plan/00-index.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `docs/reports/0272-shared-space-authority-placement-comparison.md`

## 3. Actions taken

1. Read the repo’s required normative stack and relevant plan memory before reviewing the diff.
2. Inspected the uncommitted diff for the four target files.
3. Checked the new wording against the separation rules in `specs/03`, `specs/05`, `specs/08`, and `specs/09`.
4. Reviewed report 0272 against the repository reporting requirements.

## 4. Files changed

- `docs/reports/0273-review-shared-space-authority-placement-comparison.md` (new)
- `plan/` 更新不要
- `progress.md` 更新不要

## 5. Commands run and exact outputs

```text
$ git status --short --branch
## main...origin/main
 M plan/12-open-problems-and-risks.md
 M plan/16-shared-space-membership-and-example-boundary.md
 M progress.md
?? docs/reports/0272-shared-space-authority-placement-comparison.md

$ git diff -- plan/12-open-problems-and-risks.md plan/16-shared-space-membership-and-example-boundary.md progress.md docs/reports/0272-shared-space-authority-placement-comparison.md
Reviewed the uncommitted docs-only diff for the four target files.

$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 11:21 JST
```

## 6. Evidence / findings

1. Medium: The progress mirror broadens a room-scoped, current-phase candidate into a repo-level snapshot line that reads like a general shared-space first choice. `plan/16` keeps the narrower judgment explicit: authoritative game room only, current first choice, current phase candidate, with append-friendly rooms still open to `relaxed projection authority` later (`plan/16-shared-space-membership-and-example-boundary.md:1033`, `plan/16-shared-space-membership-and-example-boundary.md:1045`). But the summary in `progress.md` compresses this to “authority placement の current first choice を `single room authority`、next candidate を `replicated authority`” without restating the authoritative-room scope or the still-open append-friendly branch (`progress.md:21`). That wording risks reading as premature cross-room finalization rather than the narrower current-phase comparison stated in the plan.

2. Medium: Report 0272 is missing the required command-trace section, so the review trail is incomplete. The repository policy requires reports to include commands run; 0272 jumps from `Files changed` directly to `Evidence / outputs / test results` and only records pending validation, not the command list that produced the draft state (`docs/reports/0272-shared-space-authority-placement-comparison.md:58`, `docs/reports/0272-shared-space-authority-placement-comparison.md:65`). This is a concrete hygiene / traceability omission even though the substantive comparison itself is mostly sound.

No substantive finding on focus item 1: the new plan wording still preserves shared-space / Mirrorea / implementation separation. `plan/16` explicitly keeps algorithm and deployment topology out of the authority definition (`plan/16-shared-space-membership-and-example-boundary.md:926`) and later places route / authority placement with Mirrorea while keeping participant / board / room rule at shared-space / app layers (`plan/16-shared-space-membership-and-example-boundary.md:1099`). `plan/12` also keeps `single room authority` as a current-phase minimal candidate rather than a fixed implementation commitment (`plan/12-open-problems-and-risks.md:24`).

No substantive finding on focus item 3: the wording still keeps participant carrier, resource owner slot, and authority placement as separate axes. The preexisting ownership section distinguishes owner slot from delegated capability (`plan/16-shared-space-membership-and-example-boundary.md:742`, `plan/16-shared-space-membership-and-example-boundary.md:811`), and the new authority section closes by explicitly saying membership registry, activation rule, resource owner slot, and authority placement should not be collapsed into one carrier (`plan/16-shared-space-membership-and-example-boundary.md:1051`).

## 7. Changes in understanding

- The main semantic line remains intact; the risk is in mirror wording and report traceability, not in the comparison structure inside `plan/16`.
- `single room authority` is correctly treated in `plan/16` as a current-phase operational candidate for authoritative rooms, but `progress.md` currently states it too broadly.

## 8. Open questions

- None beyond the findings above. The remaining issues are wording / hygiene corrections, not new design questions.

## 9. Suggested next prompt

`progress.md の shared-space 要約を authoritative room に scoped した current-phase candidate だと分かるように補正し、docs/reports/0272-shared-space-authority-placement-comparison.md に Commands run section を追加して traceability を埋めてください。`
