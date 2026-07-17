# Report 2275 - OBL-003 Line-1 decidability boundary audit

## Objective

Determine which decidability fact follows directly from the finite Line-1
fragment, without inventing the complete unified checker judgment required by
OBL-003.

## Scope and assumptions

Canon remains normative. The disposable Lean model is LAB evidence about one
finite failure-row containment checker and a partial-checker counterexample.
It is not a canonical Surface parser, AST, unified judgment, declaration
environment, complete effect/failure carrier, diagnostic contract, proof of
OBL-003, or implementation decision.

## Start state / dirty state

The worktree was clean at `321691d8`. T-RESEARCH-022 recorded its Discord task
baseline before candidate reading and placed all Lean experiments only under
`/tmp`. No tracked source had changed before this audit record.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, and theory/11
- Canon theory/01, theory/02, theory/03, architecture/02, spec/02, spec/03,
  spec/04, spec/07, ADR-0010, plan/00, and plan/01
- LAB `plan/156`, `tasks.md`, `progress.md`, `docs/project-status.md`, and
  Report 2269
- `.docs/oracle-chatgpt-pro-operations.md` and the Oracle operating guide

## Actions taken

- Reopened OBL-003 after T-RESEARCH-016 had correctly declined it for lack of
  a derivation-ready full judgment/grammar interface. The current source cut
  is narrower: theory/01 states Line-1 decidability on a declared finite
  fragment; theory/02 makes effect/failure rows finite and containment a
  Line-1 check; spec/02 and spec/03 enumerate syntax and static obligations.
- Re-read the unified judgment's decidable `C` versus residual `O` split,
  finite index discipline, failure/effect row containment, declared-fragment
  diagnostic direction, and BND-002's prohibition on hidden success.
- Built a disposable finite two-bit failure-row checker. If it accepts, the
  two explicitly modeled generated failures are contained in the declared row.
- Built a separate candidate model in which the failure row is empty and
  accepted, while an undeclared external-adapter effect remains generated. This
  demonstrates that a failure-only checker cannot establish the full Line-1
  judgment.
- Rejected a generic list/Prop proof formulation after its proof machinery
  introduced `propext`; that is a Lean representation issue, not semantic
  evidence. The final model uses only explicit finite Boolean bits and two
  no-axiom theorems, avoiding any claim that a generic list carrier is canon.
- Did not retry Oracle for this package: the two immediately preceding
  selection tasks each exhausted the one allowed retry on the same pre-submit
  browser model-picker failure, so no advisory result was available.

## Files changed

- `docs/reports/2275-obl003-line1-decidability-boundary-audit.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`

The final disposable source remains outside the repository at
`/tmp/mirrorea-t-research-022/LineOneDecidabilityBoundaryFinal.lean`.

## Commands run

- focused canon/LAB source searches with `rg`, `sed`, and `find`
- isolated Lean probes for `List` and proposition-level checker encodings
- `lean --trust=0 /tmp/mirrorea-t-research-022/LineOneDecidabilityBoundaryFinal.lean`
- forbidden-element scan and `sha256sum` over the final disposable Lean source
- `df -h .` and `free -h` before broad validation
- `make check`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- final documentation/source-hierarchy and Git diff checks listed below

## Evidence / outputs / test results

- Source-adequacy result: **one direct finite Line-1 subcheck kernel** and
  **one remaining full-judgment decidability boundary**. Theory/02 directly
  makes failure rows finite and requires containment; theory/01 names that
  containment as a Line-1 check. A finite two-bit checker therefore gives a
  source-aligned conditional soundness kernel.
- The positive theorem says only that an accepted checker for the two modeled
  failure families (`StaleMembership` and `MissingCapability`) establishes
  their containment. It selects no final failure representation, row algebra,
  source syntax, or unified judgment.
- The negative candidate has no generated failures, so the failure-row checker
  accepts, but it generates an undeclared external-adapter effect. This is not
  a counterexample to the canon, which already requires effect-row containment;
  it demonstrates that one finite subcheck does not decide the complete
  judgment.
- A full OBL-003 statement remains under-specified: canon does not select the
  executable Line-1 rule set, parser/AST-to-judgment relation, declaration and
  name-resolution algorithm, complete finite carrier/equality algorithms,
  residual-obligation boundary per rule, or rejection/diagnostic result
  relation. The spec grammar and lists of obligations are constraints on that
  future checker, not its decidability proof.
- Before broad validation, the repository filesystem had 21 GiB available
  (89% used). Memory had 9.5 GiB available and 14 GiB free swap; this package
  added no repository-local heavy artifact.
- `make check` passed: source hierarchy `704 / 704`, documentation validation
  passed with 1,429 numbered reports, and `cargo check` finished successfully.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` passed
  all 21 tests. `lean --trust=0` reported that both final scratch theorems
  depend on no axioms. The forbidden-element scan had no matches and
  `git diff --check` passed.
- Scratch hash:
  `72bea7ea349148c87088799ccc86d362c2188aed437af4dbbb49a7ec2b41ba84`.

## What changed in understanding

The important distinction is not finite versus infinite in the abstract. A
finite declared fragment yields concrete decidable kernels only after the
particular checked relation and its carriers are specified. The canon already
fixes several such kernels, including row containment, but a full OBL-003
proof must show that every Line-1 rule is both covered and separated from
residual obligations. This narrows the future work without treating the
current helper or a scratch bitset as the language checker.

## Open questions

- What exact rules comprise Line-1, and what relation distinguishes them from
  `ResidualObligation` cases for every judgment branch?
- What parser/AST, declaration environment, name-resolution, and equality
  carriers make the selected finite fragment executable?
- How are types, effects, failures, capability evidence, and index families
  given complete deciders rather than just finite examples?
- What rejected-result/diagnostic relation exposes a failed check without
  silently converting it into dynamic failure or hidden success?

## Suggested next prompt

Select another independent existing-lane source cut, or prepare a canon
formalization decision only when an OBL-003 package must choose the complete
Line-1 rule and checker-result interface.

## Plan update status

Updated: plan/156 records the direct finite subcheck, the effect omission
countermodel, the full-judgment stop threshold, and non-claims.

## Documentation.md update status

`Documentation.md` update unnecessary: reader entry points did not change.

## docs/project-status.md update status

更新済み: the human view now separates the direct finite row-check kernel from
the unselected complete Line-1 judgment and checker-result interface.

## progress.md update status

Updated: current research and the dated recent log include T-RESEARCH-022.

## tasks.md update status

Updated: T-RESEARCH-022 is closed as direct conditional LAB evidence and the
next source selection excludes silently choosing its full checker interface.

## samples_progress.md update status

`samples_progress.md` update unnecessary: no runnable sample changed.

## Reviewer findings and follow-up

No advisory Oracle conclusion was requested for this package. The immediately
preceding two candidate-selection tasks each had two concrete pre-submit
browser model-picker failures, so retrying the same unavailable consultation
path without an environmental change would not add independent review. Local
review instead re-read theory/01--03, BND-002, spec/02--04, spec/07, and the
earlier T-RESEARCH-016 selection limit. The final scratch was checked with
`#print axioms`; both the finite row-check kernel and effect-omission
countermodel have no axioms. No local sub-agent service was available.

## Skipped validations and reasons

Runtime, distributed execution, conformance, and product checks do not apply
to this documentation and disposable-Lean source audit. The runnable sample
corpus was not rerun because no sample, runner, or implementation source
changed.

## Commit / push status

The final package commit uses `--no-gpg-sign` and is pushed immediately after
this status record is included.

## Sub-agent session close status

No local sub-agent service was available. No Oracle session was started for
this package because the repeated pre-submit picker failure was already
concrete and unresolved; no advisory review is represented as completed.
