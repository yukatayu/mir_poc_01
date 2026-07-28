# Report 2503 - C2-B/C3 B-primary candidate selection

**Identifier:** `LAB-REPORT-2503`
**Date:** 2026-07-28 17:50 JST
**Status:** local validation passed; content commit pending

## Objective

Determine whether a reversible existing-lane experiment can test one B-primary
request-occurrence presentation without treating it as a selected Mirrorea
carrier, then select the smallest admissible candidate for pre-registration.

## Scope and assumptions

This is LAB candidate selection only. Canon remains normative. P012 V1/R1 and
P013 M1 are bounded directions, not a carrier or rule. The package does not
select Family A/B/C, Core, Config, history, SaveObject, identity/equality,
source syntax, runtime, transport, OBL, Gate, Phase, or public behavior.

## Start state / dirty state

Started from clean `main` at `74f1355dfdc10455944f2688528fd2e0d2cd95d5`,
equal to `origin/main`, after the Plan 210 package. No user-authored dirty
change was present.

## Documents consulted

- `AGENTS.md`, Canon README/MAP, ADR-0014, working/README, and agent instructions
- P012, P013, theory/01, theory/04, and theory/05
- Plans 199, 200, 207, 208, 209, and 210
- `samples/lean/README.md`, `Documentation.md`, `docs/project-status.md`,
  `progress.md`, `tasks.md`, and `samples_progress.md`
- a temporary GPT-5.6 Sol Pro advisory review with nine attached source files

## Actions taken

1. Re-read the delegated-research boundary and verified that a model-local
   existing-lane experiment is distinct from selecting a Mirrorea carrier.
2. Located the existing `plan/` fenced-Lean evidence pattern and confirmed
   Lean 4.29.1 is installed.
3. Used an independent Oracle challenge review to test the proposed boundary,
   required explicit state, and adverse cases.
4. Selected `B2-OPAQUE-PRE` only for future pre-registration: two opaque
   request atoms, equal incidental observations, direct q-indexed projections,
   and an explicit injective restore mapping.
5. Registered the falsifiers that prevent hidden identity, duplicate receipt,
   failure/success coexistence, lost held context, and implicit authority.
6. Synchronized LAB plan indexes and current status documents without claiming
   a Canon selection or runnable evidence.

## Files changed

- `plan/211-c2b-c3-b-primary-opaque-anchor-candidate-selection.md`
- `plan/199-selected-semantic-composition-and-inference-boundary.md`
- `plan/200-reanchored-semantic-composition-research-plan.md`
- `plan/00-index.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- this report

## Commands run

- Focused Canon/LAB source reads, WRK-history validator inspection, path
  inventory, working-tree checks, and diff reviews
- `ask-chatgpt-pro-temp` temporary advisory review with nine attached sources
- `lean --version` and `lake --version`
- `git diff --check`
- `make docs`

## Evidence / outputs / test results

The temporary Oracle review completed under GPT-5.6 Sol Pro in 10m00s. Its
response SHA-256 is
`2991356aae01f50b70dc42a3fee582d7b6b3e5c0668ebe320c71db968173d346`.
It confirmed that ADR-0014 permits a reversible finite B-primary hypothesis in
an existing LAB lane, provided it remains model-local and preregistered. It
also identified the strongest guard: equivariance under swapping two opaque
requests with identical incidental observations.

No Lean artifact has been created or run. The selected candidate has no result
until WRK-0037 is committed, pushed, then executed on its own registered command.
`make docs` passed: Canon index reported 123 indexed files; source hierarchy
reported 761 required and 761 present paths; documentation validation reported
a complete scaffold and 1,657 numbered reports.

## What changed in understanding

The relevant distinction is not whether B has a mathematical carrier: every
q-indexed partial map is one. The boundary is whether that carrier is asserted
as Mirrorea semantic state. A finite model can use explicit candidate-local
state to test coherence without making q equality, restore, receipt, or pending
facts part of Canon.

## Open questions

- Can the registered B2-OPAQUE model retain all staged obligations through its
  explicit restore map without a second attempt identity?
- Does any registered falsifier force a Family C comparison?
- If the finite model passes, which portions are a useful design input rather
  than merely an existence witness for one bounded presentation?
- The owner/Canon decision on A/B/C as a project carrier remains open.

## Suggested next prompt

Continue with the pre-registered B2-OPAQUE experiment only: first commit and
push WRK-0037, then materialize its fenced finite Lean model and run every
registered falsifier. Do not promote a model-local result into Canon semantics.

## Plan update status

更新済み: Plan 211 and Plans 199/200/00-index record the candidate selection and its non-effects.

## Documentation.md update status

更新済み: `Documentation.md` links Plan 211 and states the no-incidental-identity boundary.

## docs/project-status.md update status

更新済み: `docs/project-status.md` records pre-registration selection only, with no semantic or implementation advance.

## progress.md update status

更新済み: `progress.md` records B2-OPAQUE as the autonomous bounded experiment and keeps owner selection open.

## tasks.md update status

更新済み: `tasks.md` records the separate autonomous registration/execution package before the owner decision.

## samples_progress.md update status

更新不要: no active sample root, runnable workflow, validation command, debug surface, or blocker changed.

## Reviewer findings and follow-up

The Oracle review was advisory only. Its key requirements are incorporated:
model-local state, opaque q distinction, direct staged projections, explicit
injective restore, branch/linearity guards, scoped one-shot behavior, and no
authority inference. No callable sub-agent session was available.

## Skipped validations and reasons

No Lean/model command has run because ADR-0014 requires the WRK-0037
pre-registration to be committed and pushed before outcome evidence is relied
on. No runtime, sample, transport, or end-to-end validation applies to this
candidate-selection package.

## Commit / push status

Pending: the selection package and a report-status closeout will each be
committed with `--no-gpg-sign`, pushed to `origin/main`, and verified by
fetching `origin/main` after documentation validation.

## Sub-agent session close status

No callable sub-agent session was opened.
