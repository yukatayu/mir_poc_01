# Report 2392 - Core preservation readiness and candidate re-screen

- Date: 2026-07-23 04:51 JST
- Author / agent: Codex
- Scope: OBL-020 proof-readiness audit and ADR-0014 candidate re-screen
- Decision levels touched: none; LAB evidence classification only

## Objective

Test whether the present Core preservation evidence has an unrecorded logical
gap, then select a non-duplicative autonomous L3 target only if both outcomes
would affect an existing permitted downstream decision.

## Scope and assumptions

`mirrorea_canon/` remains normative. OBL-020 remains open in `theory/11`.
This task does not change the definition of `WellFormed`, select a formal
runtime carrier, add a queue invariant, reinterpret a patch invariant, decide
a label lattice, or create a working record. The prior owner decisions
G0-D3, OBL-001 Core/result route, and PROPOSAL-008 remain out of scope.

## Start state / dirty state

Started clean at synchronized commit `f1f5302f`. Root storage had 6.9 GiB
free of 188 GiB (97% used), with no mounted external work directory. The task
used source reads, Lean compilation, a small disposable external `.olean`
directory, and the existing Lean synchronizer. That synchronizer invokes its
own narrow `cargo run` path and refreshes existing generated Lean/manifest
material; it left no repository diff. No broad Cargo, runtime, distributed, or
large generated-artifact workflow was started.

## Documents consulted

Read Canon README, MAP, ADR-0013, ADR-0014, theory/00 through theory/08 where
relevant, theory/11, spec/05, plan/00 and plan/01, WRK-0006, WRK-0014,
WRK-0018, and working/README. Read LAB plans 126, 161, 163, 176, and 180;
the OBL-020 Lean draft, its two retained experiments, the current label model,
its explanation, Lean manifest, progress/tasks snapshots, and prior reports.

## Actions taken

1. Replayed the active OBL-020 statement draft, its sync guard, and the
   standalone same-carrier relation experiment.
2. Compiled the import-bearing familywise experiment by placing the dependency
   `.olean` under its repository module path in a fresh disposable external
   directory and supplying that directory through `LEAN_PATH`.
3. Compared `theory/01`'s five explicitly named well-formedness clauses with
   Core configuration fields, the request lifecycle, patch theorem, and the
   existing familywise coverage result.
4. Obtained independent reviewer, Lean-evidence mapper, and planner analyses.
   A temporary Oracle review independently assessed both the OBL-020 boundary
   and a proposed label-model L3 candidate.
5. Rejected both a duplicate OBL-020 component-vacuity experiment and, under
   the current LAB priority screen, the proposed two-point label-lattice
   closure as new L3 records.

## Files changed

- this report
- `progress.md`

## Commands run

- `df -h .`, `free -h`, `du -sh . .git target .cargo .lake`
- `lean --version`
- `lean samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`
- `python3 scripts/current_l2_lean_sample_sync.py`
- `lean samples/lean/foundations/CurrentL2ProofSkeleton.lean`
- `lean -o <temporary-module-root>/samples/lean/lab-statements/obl020/StepWFStatementDraft.olean ...`
- `LEAN_PATH=<temporary-module-root> lean samples/lean/lab-statements/obl020/FamilywiseGlobalBoundary.lean`
- `lean samples/lean/lab-statements/obl020/SameCarrierVarianceBoundary.lean`
- source/ledger/reference searches, Git status checks, and two temporary
  Oracle consultations

## Evidence / outputs / test results

Lean 4.29.1 compiled the OBL-020 abstract draft, its sync guard, the current
proof-skeleton fragment, the standalone same-carrier experiment, and the
import-bearing familywise experiment with the documented external module-root
procedure. The forbidden-token search over the three OBL-020 sources found no
`sorry`, `admit`, `axiom`, `unsafe`, `partial`, `implemented_by`, or `opaque`
declaration.

The global OBL-020 target is correctly stronger than a familywise proof
wrapper: WRK-0006 already proves the conditional coverage bridge and a
non-vacuous countermodel when an actual step is unclassified. Repeating this
result, or adding an opaque component predicate that merely restates the lack
of a bridge from `WellFormed`, would not alter a live decision and was not
registered.

The reviewer identified proof-model requirements that must later be made
explicit: exhaustive transition representation, safe occurrence insertion,
and request/queue admissibility. These are not presently contradictions in
Canon. `theory/01` defines OBL-020 over its five named clauses and its global
step relation includes `[E-PATCH]`. THM-006/OBL-019 separately constrain
rejected or deferred patch verdicts to no mutation except lifecycle rows; they
do not prove general patch-lifecycle preservation or remove `[E-PATCH]` from
OBL-020. Whether queue admissibility or further runtime state belongs in a
future proof carrier or transition premise is a proof-interface decision, not
an autonomous Core extension.

The planner proposed proving antisymmetry and least-upper-bound laws for the
existing two-point label model. The independent Oracle review rejected it
under the current LAB priority screen: both lemmas are expected consequences
of a fixed helper-local toy definition, and neither outcome changes an
identified immediate downstream decision. This is not an ADR-0014 eligibility
rule and does not narrow future L3 research. No working record, theorem tail,
or outcome command was created.

## What changed in understanding

The project has a sound current separation: the global preservation target is
not being silently replaced by familywise evidence, and THM-006's
rejected/deferred patch property is not being treated as a substitute for
OBL-020's `[E-PATCH]` case. The next formal model must make transition
coverage and dynamic admissibility explicit, but current evidence does not
justify choosing their final representation. The current LAB priority screen
uses decision value in addition to easy Lean provability; ADR-0014 itself is
not narrowed by that heuristic.

## Open questions

- What exact formal relation defines all legal Core transitions and their
  coverage when a proof-facing OBL-020 package is authorized?
- What explicit request/queue admissibility and reachability premises, if any,
  are needed to prove Canon's existing global preservation statement without
  changing its scope?
- Do queue/request admissibility and any additional runtime-state invariants
  belong in a future proof carrier, or stay as explicit transition premises?
- The existing owner decisions G0-D3, OBL-001 route selection, and
  PROPOSAL-008 remain unchanged.

## Suggested next prompt

Prioritize autonomous research from a new source locus that has a concrete
non-reserved consumer and distinct positive/adverse downstream branches. This
is a current LAB prioritization heuristic, not an added ADR-0014 eligibility
condition. Do not reopen OBL-020 coverage or the two-point label model merely
to add more toy lemmas.

## Plan update status

`plan/` 更新不要: plans 126, 161, 163, 176, and 180 already record the
abstract OBL-020 boundary, coverage result, candidate discipline, and the
proof-facing dependencies. This task adds no new route or decision.

## Documentation.md update status

`Documentation.md` 更新不要: the entry path and current research map did not
change.

## docs/project-status.md update status

更新不要: Canon lifecycle, proof status, implementation status, and compact
status map are unchanged.

## progress.md update status

更新済み: recent log now records the replay, the explicit non-expansion of
OBL-020, and the rejected no-consumer label candidate.

## tasks.md update status

`tasks.md` 更新不要: its autonomous-research and owner-decision maps already
express the same boundaries. No new executable package was selected.

## samples_progress.md update status

`samples_progress.md` 更新不要: no active sample source, validation command,
or dashboard status changed.

## Reviewer findings and follow-up

The evidence mapper correctly separated actual Lean declarations from abstract
assumptions. The reviewer surfaced transition coverage, queue admissibility,
and occurrence-freshness requirements; source comparison showed that these
are future proof-model questions, not grounds for a silent Canon change. The
planner's label-model candidate was challenged by an independent temporary
Oracle review and rejected under the current LAB priority screen for lacking
an identified live downstream decision. That screen is not an ADR-0014
eligibility condition. All three sub-agents completed without edits and were
closed.

## Skipped validations and reasons

No broad Cargo, runtime, distributed, or large generated-artifact suite was
run: this was a theory-only audit and root storage remains constrained. The
existing Lean synchronizer was run and invoked its narrow `cargo run` path
without leaving a diff. No new Lean outcome command was run for a candidate
because no pre-registration was selected under the current LAB priority screen.
Documentation validation is pending before commit.

## Commit / push status

Pending at report write. This report and the progress log will be validated,
committed with `--no-gpg-sign`, and pushed.

## Sub-agent session close status

The read-only reviewer, Lean-evidence mapper, and planner completed without
edits and are closed.
