# Report 2256 - OBL-001 concrete-evidence bridge preflight

- Date: 2026-07-17 09:51 JST
- Author / agent: Codex
- Scope: existing Surface elaboration and OBL-001 LAB statement lanes only
- Decision levels touched: no canon decision; one owner-facing LAB evidence-route blocker recorded

## Objective

Determine whether an existing lane can evaluate a closed-RHS, private-field
foreign-locus write subcase of THM-001 / OBL-001 without adding a fixture,
helper, schema, wrapper, interpretation, or semantic decision.

## Scope and assumptions

The preflight follows `plan/156` and uses only the existing Surface elaborator,
the committed `ELAB-07` negative, the existing OBL-001 statement draft, and an
untracked `/tmp` source that changes only the `fails` row. It is source evidence,
not an implementation-to-theorem bridge or a canon statement.

## Start state / dirty state

Started after clean, pushed commit `50e44a2a` with no active research unit.
The prior T-RESEARCH-003 result prohibited reopening graph append without an
explicit trigger. The disposable scratch and Oracle output were outside the
repository and occupied 32 KiB total; this package's documentation edits are
the files listed below.

## Documents consulted

- `mirrorea_canon/theory/01-mircore-v0.md`, `03-elaboration.md`, and
  `11-metatheory-ledger.md`
- `mirrorea_canon/plan/00-gates.md`, `01-phases.md`, and `02-operating-model.md`
- `plan/121`, `124`, `126`, `137`, `145`, `147`, and `156`
- `samples/lean/lab-statements/obl001/THM001StatementDraft.lean` and its guide
- the ELAB-02, ELAB-05, and ELAB-07 source/expected evidence and the Surface runner

## Actions taken

- Asked Oracle to rank eligible next units under the autonomy envelope.
- Performed the recommended candidate's existing-lane preflight before selecting it.
- Created a disposable literal-RHS positive source from `ELAB-07` by completing
  only its failure row, then executed the existing elaborator directly.
- Compared the positive and negative JSON structurally and requested an Oracle
  follow-up on the missing authority-carrier bridge.

## Files changed

- `docs/reports/2256-obl001-concrete-evidence-bridge-preflight.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`

## Commands run

- `ask-chatgpt-pro` with the canon, plan, and snapshot source cut attached
- `ask-chatgpt-pro-followup` with the concrete preflight result
- `python3 scripts/surface_mir_samples.py --format json run ELAB-02|ELAB-05|ELAB-07`
- `cargo run -q -p mir-semantics --example surface_to_core_elaborate -- /tmp/mirrorea-t-research-004/literal-write-positive.mir --format json`
- structured JSON assertions for the positive/negative pair, plus source diff,
  source/plan inspection, Git-status, and resource checks

## Evidence / outputs / test results

- The temporary positive differs from `ELAB-07` only by completing the declared
  failure row. It was accepted by the existing elaborator.
- The committed negative was rejected with `generated_failure_not_declared`.
- Both outputs contain one `BrowserClient -> S` remote write request, zero
  dependency/publication/observation rows, and six source-span entity kinds.
  The positive has `failure_row_complete: true`; the negative has `false`.
- The current elaborator JSON exposes neither structured capability/witness
  references nor a mapping to OBL-001's abstract `Pred` or canonical `C ∪ O`.
- The OBL-001 statement draft compile-checks a predicate shape, but it is not an
  interpretation of this concrete elaborator result. Existing `plan/124` also
  keeps helper JSON out of the predicate surface.

## What changed in understanding

The existing lane is sufficient for a narrow source-level request/failure/span
pair, but insufficient for the proposed OBL-001 authority-carrier subcase. The
missing bridge is an evidence-route limitation, not a canon counterexample and
not evidence that the request lacks authorization.

## Open questions

- Should an OBL-001 proof-facing package later authorize a concrete bridge from
  Surface elaboration evidence to abstract OBL-001 predicates?
- If so, should it expose carrier data, introduce a read-only interpretation,
  use a reviewed binding, or retain the current deferral?

## Suggested next prompt

Decide whether to keep the OBL-001 concrete-evidence bridge deferred, or
explicitly promote a bounded decision request that compares bridge designs and
their artifact-identity consequences.

## Plan update status

`plan/` 更新済み: `plan/156` records the preflight, its exact falsifier, and
the separate owner decision without selecting a research unit.

## Documentation.md update status

`Documentation.md` 更新不要: the top-level source hierarchy and roadmap remain accurate.

## docs/project-status.md update status

更新済み: the concise control view now shows the OBL-001 concrete-evidence
bridge as decision-ready and keeps T-RESEARCH-004 unselected.

## progress.md update status

`progress.md` 更新済み: records the preflight outcome, remaining unselected
work, and the evidence-route boundary without changing canon status.

## tasks.md update status

`tasks.md` 更新済み: adds the not-selected preflight and separates the owner
bridge decision from future autonomous research selection.

## samples_progress.md update status

`samples_progress.md` 更新不要: no committed runnable sample, command, status,
or dashboard classification changed; the temporary source is not an active row.

## Reviewer findings and follow-up

The first Oracle review recommended the literal-RHS preflight as the only
candidate. Its follow-up reviewed the actual result and concluded that the
missing concrete authority carrier is an existing-lane falsifier: do not select
or call T-RESEARCH-004 `research-complete`; record a separate decision-ready
bridge question instead. Oracle advice remains advisory. The wrapper metadata
did not independently verify model selection, so the result was additionally
checked against the local canon and plan before being distilled here.

## Skipped validations and reasons

- No new code, fixture, helper, schema, or Lean artifact was added, so no new
  feature test applies.
- No JSON-to-`Pred` bridge was prototyped: doing so would itself create the
  evidence route that the preflight found missing.
- Broad product/runtime validation was not repeated because this package touched
  only research documentation and an external scratch source; the prior front-door
  audit remains the runnable evidence baseline.

## Commit / push status

Pending at report write; documentation validation and focused review will run
before commit with `--no-gpg-sign` and push.

## Sub-agent session close status

No local sub-agent was opened. Both Oracle sessions are complete; their useful
advisory conclusions are distilled above and no external session is pending.
