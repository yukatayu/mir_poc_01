# Report 2572 — WRK-0046 P017 X1 K0 q-fiber U/L lift execution

- Date: 2026-07-31
- Author / agent: Codex
- Scope: Execute exactly the already registered WRK-0046 finite conditional
  lemma, retain source/evidence only if all registered controls pass, and make
  no Canon or reader-status change in this package.
- Decision levels touched: L3 evidence only; no L0/L1/L2 decision, theorem/OBL,
  Gate, Phase, implementation contract, or public claim changes.

## Objective

Confirm that the registered A0 preservation condition excludes two counted
consumes on one supplied finite line, while the A1 control produces its exact
adverse trace after removing only restore preservation.

## Scope and assumptions

`mirrorea_canon/` is normative. WRK-0046 was committed and pushed as a
source-free L3 preregistration. This package retains only its declared
Markdown-held Lean source and this direct report. Results are filled after the
registered execution and review steps below; Canon Results/MAP/status updates
are deliberately a later package.

## Start state / dirty state

`main` and `origin/main` were equal and clean at
`3068e73cd060f482129e705af663e13e6f124534`. WRK-0046 was an already
committed and pushed, source-free `L3-open` record with `Reliance status:
not-promoted`; its declared source path was absent. The package starts by
creating exactly that source path and this direct report.

## Documents consulted

- Canon: `mirrorea_canon/README.md`, `MAP.md`, ADR-0014, P012, P013, P017,
  theory/01, theory/02, theory/04, theory/05, theory/07, and WRK-0046.
- LAB: Plans 230, 231, and 245; Reports 2564, 2565, 2570, and 2571; the
  retained WRK-0045 source; `Documentation.md`, `docs/project-status.md`,
  `progress.md`, `tasks.md`, `samples_progress.md`, and the report template.
- Operations: the Oracle instructions, source-hierarchy validator,
  documentation validator, and Canon index generator.

## Actions taken

1. Requested a temporary Oracle review of the pre-registered question before
   editing. It recommended a finite ordinary-before / designated-restore /
   ordinary-after construction, with A1 as an omission control rather than an
   A0 falsifier.
2. Obtained a Canon-first planner review before materialization. It accepted
   the semantic boundary but rejected the first fixture proof style because
   `simp` introduced `propext`.
3. Obtained an independent formal review. It found that the first fixture was
   conditional on an external restore witness and therefore did not prove the
   registered nonvacuity bundle inhabited.
4. Corrected only the fixture layer: the general theorem remains parameterized
   by an arbitrary relation, while the fixture uses a local universal
   Prop-valued relation and a supplied witness. The fixture predicates are
   indexed inductive propositions, and all proofs are direct constructors.
   This eliminates `propext` without adding identity, functionality, a restore
   function, or a global uniqueness premise.
5. Extracted the one Lean block to disposable temporary files, executed Lean
   under `--trust=0`, printed axioms for every retained declaration, scanned
   prohibited surfaces, and ran a disposable A0/A1 compatibility harness.

## Files changed

- `plan/wrk-0046-p017-x1-k0-qf-ul-lift.md`
- `docs/reports/2572-wrk0046-p017-x1-k0-qf-ul-lift-execution.md`

## Commands run

- Extracted the unique `lean` fenced block with `awk` into a disposable
  `mktemp` file and ran `lean --trust=0` twice, including once with the A0/A1
  harness appended.
- Checked the source SHA-256 after extraction, the one-block/two-fence count,
  imports and prohibited proof/identity/functionality constructs, and the only
  proposition equalities (`counted = true`).
- Ran `python3 scripts/check_source_hierarchy.py` and
  `python3 scripts/validate_docs.py`.
- Ran resource, Git-dirty-state, local/remote-head, and whitespace-diff
  checks before the wider validation step.

## Evidence / outputs / test results

The retained source has exactly one Lean block, 434 extracted lines, and
SHA-256 `07538caaf5e1c369e4baf1a1f3b3dac1a957b8f3947998c6bb2d2304d3349efd`.
The fresh extraction passed `lean --trust=0`. All 53 declarations printed by
`#print axioms` report no axiom dependency, including the generic A0 theorem,
the true-falsifier wrapper, the local fixture relation and predicates, the
closed A0 nonvacuity fixture, the A0 exclusion, the A1 adverse control, and
the separately derived A1 non-preservation fact.

The source scan found no import, `sorry`, `admit`, axiom declaration,
`unsafe`, `partial`, `implemented_by`, classical/choice/quotient construct,
decidable equality, injectivity/surjectivity premise, `Nodup`, unique-existence
operator, endpoint restore function, or q-equality. The only proposition
equalities are three uses of the internal `counted = true` edge annotation.

The closed A0 fixture has one consume before the designated bridge, a supplied
restore witness, post-restore acceptance, and preserved `Spent`; the A0
theorem excludes two counted consumes. The closed A1 fixture retains the same
bridge and ordinary-edge rules, omits only restore preservation, and constructs
the two-consume trace with post-restore acceptance and `Not Spent`. A separate
disposable harness showed that adding an arbitrary A0 preservation argument to
the A1 trace yields `False` through `no_true_a0_falsifier`. It therefore
confirms the classification: A1 is an omission/reset control, not an A0
falsifier.

`check_source_hierarchy.py` passed `795/795`; `validate_docs.py` completed
successfully. No Canon file, generated Canon index, reader status snapshot,
runtime, parser, sample, helper, schema, CI, or Make surface changed.

## What changed in understanding

The bounded K0 U/L question has a non-vacuous, axiom-free finite model without
choosing the future semantic residence of `Spent`. The positive result is only
conditional sufficiency for one supplied finite experimental line: ordinary
preservation plus local restore preservation excludes the reset/re-enable
trace. Removing that local preservation admits the exact adverse control.

This does not make `Spent` primitive or derived, choose a P017 X1 model,
establish every restored continuation, or provide an implementation contract.
The record remains `not-promoted`; a separate immutable-evidence link adds
artifact and commit references without promotion, and it remains L3 evidence
only.

## Open questions

The final primitive-versus-uniquely-derived classification of `Spent`, its
eventual semantic residence, its relation to actual admissible loads, and the
full P017 no-reset/no-re-enable requirement remain OPEN. This source does not
address no-merge/no-duplicate, global exactly-once, receipt semantics,
matching, authority, failure, `Gamma`/`Delta`, actual transition, persistence,
runtime behavior, theorem/OBL status, Gate/Phase movement, or public behavior.

## Suggested next prompt

Review the immutable source/evidence commit, then make a separate append-only
WRK-0046 evidence-link package that records the positive conditional result
without promotion. Only after that link is durable should reader/status
snapshots be refreshed.

## Plan update status

`plan/` 更新済み: the registered source path is materialized in this package.

## Documentation.md update status

`Documentation.md` 更新不要: reader-facing state waits for a separate durable
Canon evidence-link package.

## docs/project-status.md update status

更新不要: reader status waits for the separate Canon evidence-link package.

## progress.md update status

`progress.md` 更新不要: this source/evidence package does not yet change the
durable working-record result.

## tasks.md update status

`tasks.md` 更新不要: this package selects no new autonomous task.

## samples_progress.md update status

`samples_progress.md` 更新不要: this L3 Lean evidence creates no runnable Mir
sample, runner, debug surface, or sample workflow.

## Reviewer findings and follow-up

The temporary Oracle review was advisory and was checked against the registered
boundary before use. The first planner review found fixture `propext`
dependencies; the independent reviewer additionally found the open restore
witness. Both findings were addressed by the bounded fixture-only correction
described above.

The final exact-source reviewer approved the 434-line source, all 53 axiom-free
declarations, the closed fixture, the A0/A1 premise split, and the stated
non-claims. The final planner review initially corrected two overstatements:
the arbitrary relation does not *assume* functionality or disjointness, and a
future evidence link does not promote the record. After those exact wording
corrections, the planner approved with no residual issue.

## Skipped validations and reasons

No parser, runtime, transport, sample, or product command applies because this
is a Markdown-held L3 Lean evidence package with no executable Mir layer. The
Canon Results/MAP/index and reader status validations are intentionally
separate: this package must be pushed before immutable evidence is linked.

## Commit / push status

Pending at report write. This source/evidence package will be committed and
pushed before the working record is linked to the resulting immutable evidence.

## Sub-agent session close status

Planner `Plato` and formal reviewer `Mendel` completed their final reviews and
were closed after approval. The completed temporary Oracle consultation is
advisory only and is mirrored here as evidence reasoning rather than external
normative state.
