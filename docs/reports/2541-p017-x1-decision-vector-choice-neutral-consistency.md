# Report 2541 — P017 X1 decision vector and choice-neutral consistency matrix

- Date: 2026-07-30
- Author / agent: Codex
- Scope: LAB decision preparation after P017 X1 and the closed WRK-0043 / P0A screens.
- Decision levels touched: LAB only. No Canon decision level changed.

## Objective

Prepare the next ordinary-design review surface for P017 X1 without selecting
the relation's concrete semantics, representation, or implementation.

## Scope and assumptions

The work is limited to P017's V1/R1 cross-locus read scope. Canon is
authoritative. The owner has already accepted X1, but X1 authorizes only a
bounded integration design package. The report assumes the committed
WRK-0040--0043 and Plans 225--226 classifications are current evidence.

## Start state / dirty state

Started at `e8be4a5b90cce433d0d3270a0d25dac412e28071` on `main`,
equal to `origin/main`, with a clean worktree. Discord task baseline
was recorded before this package. No build artifact or source sample was
changed.

## Documents consulted

- Canon entry/map; P017, P012, P013, P008, ADR-0014, theory/01--07, and
  spec/04--05.
- LAB Plans 215--221 and 225--226; `Documentation.md`,
  `docs/project-status.md`, `progress.md`, `tasks.md`,
  `samples_progress.md`, `plan/00-index.md`, and the report
  template.
- Temporary Oracle scope review. Its advice was checked against direct Canon
  sources and is not a normative input.

## Actions taken

1. Re-read the P017 X1 envelope and its direct theory, authority, observation,
   and save/load constraints.
2. Confirmed that Plans 225 and 226 close only duplicate research lines; they
   do not supply a positive relation model.
3. Created Plan 227 with a direct-source ledger, R/B/T/U/C/L decision vector,
   dependency graph, explicit escalation boundary, choice-neutral adversarial
   matrix, and candidate-native comparison-card contract.
4. Ran an independent Oracle final review against P017 and direct theory
   sources. It found four substantive omissions; each was rechecked locally
   against P017 and corrected without changing the plan's LAB-only scope.
5. Ran a narrow independent re-review. It returned `PASS`; its residual
   risks were clarified by naming the request-declared dynamic failure row,
   adding ADR-0014 narrow-question/status-quo card fields, and identifying
   restore-state rows as frontiers rather than a common lifecycle.
6. Synchronized the reader index and LAB status/task snapshots. No runnable
   sample dashboard change was needed.

## Files changed

- `plan/227-p017-x1-decision-vector-and-choice-neutral-consistency.md`
- `plan/00-index.md`
- `plan/221-c2b-c3-canon-proposal-preparation.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `scripts/validate_docs.py`
- This report.

## Commands run

- Read-only Canon/LAB source inspection with `sed` and `rg`.
- `sha256sum` for the direct-source ledger and advisory-review audit.
- `make docs`.
- `python3 scripts/validate_docs.py`.
- `git diff --check` and staged diff/secret scans.

## Evidence / outputs / test results

The direct-source ledger pins P017, ADR-0014, theory/01--07, P012/P013, and
spec/04--05 at the start cut. The initial independent final review found that
the first draft had reopened the carrier family, omitted semantic-receipt
acceptance and typed-rejection disposition, omitted L3 package controls, and
omitted M1 binding/provenance immutability. The corrected Plan 227 now states
those P017 requirements. The preparation creates no executable semantic
artifact, so no Lean or runtime command is applicable. Documentation
validation, hierarchy/index checks, diff checks, and staged secret scan are
re-run after the correction. The narrow re-review returned `PASS` with
only the three terminology/record-shape clarifications applied above.

## What changed in understanding

The next real work is not another finite permutation detector and not a
positive model hidden in a plan. It is an X1-bounded relation-state design
package that must choose a coherent answer across six coupled questions:
residence/reference scope, owner branch/provenance, requester
receipt/rejection, restricted use, occurrence/causal integration, and
persistence/restore. P017 already fixes the relation-state family, at-most-one
semantic acceptance, M1 bind/provenance floor, and the pre-registration
controls. Observation is a separate conditional typed-effect gate, not a
storage consequence.

## Open questions

- Which candidate-native semantic residence and restore correspondence should a
  normal Canon proposal adopt?
- What is the selected requester-rejection policy, branch representation,
  `Gamma`/`Delta` disposition, occurrence mapping, and
  persistence placement?
- Those questions are not answered here; each may require ordinary Canon
  amendment.

## Suggested next prompt

Continue autonomous source-led preparation by screening for a non-duplicate,
ADR-0014-eligible literal or conditional research candidate outside the closed
P017 fixture and P0A restore-quantifier lines. Escalate rather than silently
instantiating the Plan 227 tuple.

## Plan update status

`plan/` 更新済み: Plan 227, its index entry, and Plan 221 now record the
ordinary-design preparation boundary.

## Documentation.md update status

`Documentation.md` 更新済み: added the Plan 227 reader entry and role.

## docs/project-status.md update status

更新済み: the semantic-kernel row and owner-facing X1 status now identify Plan
227 as decision preparation only.

## progress.md update status

`progress.md` 更新済み: recorded Plan 227 in the logical-specification
snapshot, research table, and dated recent log.

## tasks.md update status

`tasks.md` 更新済み: current package and ordered self-driven work now
identify Plan 227's coupled ordinary-design tuple.

## samples_progress.md update status

`samples_progress.md` 更新不要: no active sample, validation command,
debug surface, or runnable-evidence classification changed.

## Reviewer findings and follow-up

The temporary Oracle final review initially returned `FAIL` with four source-
backed findings: Plan 227 had reopened a non-relation carrier family, missed
P017's semantic-acceptance/rejection rules, omitted record-specific L3
controls and allowed proof/observation/freshness escalation too broadly, and
missed the M1 bind-time/immutable-provenance floor. The local source review
confirmed all four against P017 lines 55--172 and applied the minimal
corrections. The independent narrow re-review then returned `PASS`:
no concrete schema, shared carrier, common lifecycle, or unsupported Canon
claim remained. Its residual terminology/record-shape risks were incorporated
before the final validation suite. No callable sub-agent capability is
available in this environment.

## Skipped validations and reasons

No Lean, build, or runtime validation was run because this package creates only
LAB documentation and no executable or formal source. The repository
documentation validation suite was run instead.

## Commit / push status

The validated package is committed with `--no-gpg-sign` and pushed to
`origin/main`. `HEAD` is compared with `origin/main` after
push; the final command evidence is reported in the task close message.

## Sub-agent session close status

No callable sub-agent session was available or opened. Oracle was used as an
advisory independent review and its result was incorporated only after local
source verification.
