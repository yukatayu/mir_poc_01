# 173 - Current local predicate constructive-decidability selection

## Purpose

Record the next bounded L3 research candidate after the foundational proof-obligation revalidation. This is LAB selection memory. Canon remains normative; this document neither defines a Mir checker nor changes an obligation, gate, or phase.

## Selection result

The selected candidate is a constructive-decidability boundary for the exact current LAB predicate `captureSubset` in `samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean`. `outlives` and `remoteCallAllowed` are positive controls only. The candidate is useful only if it demonstrates all-input decision evidence for `captureSubset` without changing the existing predicate definitions, adding a generic finite-carrier abstraction, exporting a global instance, or relying on classical machinery.

`theory/11` OBL-003 is an exclusionary anchor, not the target of this work. The experiment must not claim any component of `MirCore.Check.Decidable`, Line-1 coverage, a `C` / `O` boundary, grammar, environment lookup, type/effect/failure/capability checking, diagnostics, residual obligations, or progress toward OBL-003. The current helper predicate is not a canonical finite-index algorithm, and `remoteCallAllowed : Nat -> Prop` is arithmetic, not finite-cardinality evidence.

## Source reading

- Canon theory/01 fixes the unified judgment but not an executable complete checker. Theory/02 permits finite decidable index families but does not select this helper carrier or a Lean encoding. Theory/11 leaves OBL-003 open.
- WRK-0001 already reproduces the three finite-index helper shapes and their selected positive/negative lemmas. It does not establish a uniform decision term for arbitrary `captureSubset` functions.
- T-RESEARCH-022 / Report 2275 already retain a finite failure-row subcheck and an effect-omission countermodel. They show why a subcheck is not the unified judgment; this candidate must not repeat that model or widen its conclusion.
- Existing OBL-005 evidence already proves the structural-output reassociation kernel and records the missing source unit, validity, orientation, and confluence interfaces. Existing OBL-015 evidence already isolates the mutation-origin/authorization boundary. Neither is selected again.

## Candidate comparison

| Candidate | Disposition | Reason |
| --- | --- | --- |
| Uniform local `captureSubset` decision evidence | selected for L3 pre-registration | It is input-parametric over the existing two-constructor LAB carrier, has an adverse opaque-domain control, and need not select a Canon interface. |
| `outlives` / `remoteCallAllowed` decision evidence | controls only | Constructor split and natural-number comparison are low-information checks; treating them as coequal results would overstate the experiment. |
| OBL-005 nested reassociation | rejected as duplicative | Report 2262 already establishes the structural output result; a broader theorem would select a context, relation, or equality boundary. |
| OBL-015 Boolean lineage absence audit | rejected | It would establish only non-representation in an IFC helper and has no identified consumer that treats that Boolean as Canon grant-lineage evidence. |

## Required pre-registration cut

Before any outcome command or source edit, create a fresh L3 working record without reusing the historical uncreated `WRK-0015` stale-fence label. The record must pin the clean commit, Canon anchors, the exact foundation source, and this plan, and declare both `plan` and `samples/lean` as its permitted LAB locations.

Its question must be limited to whether Lean can construct a named, non-instance term of type `Decidable (captureSubset lhs rhs)` for arbitrary `lhs rhs : CaptureSet`, by eliminating the exact current `Capability` constructors and primitive Boolean decisions. It must register:

- `outlives` and `remoteCallAllowed` as positive controls, not a combined finite-index theorem;
- a no-import, no-`Classical`, no-`Fintype`, no-`Finset`, no-choice, no-global-instance, no-definition-change lexical boundary;
- an adverse opaque-domain probe that must fail without a supplied finite interface, to detect hidden instance or classical leakage;
- a correctness check against the existing positive/negative capture lemmas;
- immediate freeze/stop on a new helper/API, source catalog/synchronizer/CI change, generic finite-carrier abstraction, global instance, or any conclusion about Line-1 or OBL-003.

The evidence package may add only the bounded proof terms to the existing foundation and its direct report. It is compile-check-only L3 evidence, not a runnable workflow change. Any retained result must be manifested later through the record's append-only `Evidence commits` list; unretained disposable output may not be cited as WRK evidence.

## Open boundaries

- Whether a future Canon formalization should define the complete Line-1 checker and its decidability proof remains open and owner/canon controlled.
- Whether this bounded proof succeeds is deliberately unknown until after the immutable pre-registration is committed and pushed.
- The validator root tuple / ADR-0014 lane-catalog correspondence remains UNRESOLVED and unchanged; this candidate stays inside the already admitted `samples/lean` root.
