# 174 - Local predicate proposition-decidability successor selection

## Role and authority

This is LAB repository memory and a future L3 pre-registration plan. Canon
remains normative. It does not alter `theory/11`, select a MirCore checker
interface, or reopen frozen WRK-0016.

## Start point

WRK-0016 froze the conjunction of two constraints: a persistent source-visible
top-level name for a `Decidable (captureSubset lhs rhs)` *value*, and a ban on
new data-valued declaration forms. Lean rejects the attempted `theorem` because
its target is `Type`, not `Prop`. The direct proof body was therefore not
validated. This is neither a positive nor a negative constructivity theorem.

Temporary Oracle review and two independent read-only reviews agree on three
facts: an anonymous `example`, a local binding, and a retained value declaration
are not the same successor; a retained `def`/`abbrev`/`opaque` presently has no
identified bounded consumer; and a proposition-valued theorem can isolate body
constructivity without selecting any value/API policy.

## Selection result

Select only this future successor candidate:

```lean
captureSubset lhs rhs ∨ ¬ captureSubset lhs rhs
```

for arbitrary exact current-L2 `lhs rhs : CaptureSet`, as a named local Lean
**theorem**. This is proposition-valued, so it does not repair or reinterpret
WRK-0016's rejected data-valued declaration form. A positive result would say
only that the explicit two-constructor decision body is expressible under the
registered local constraints. A negative result would show that the restricted
body is not established without a new finite interface or other excluded
mechanism.

## Candidate comparison

| Candidate | Disposition | Reason |
| --- | --- | --- |
| Proposition-valued `captureSubset lhs rhs ∨ ¬ captureSubset lhs rhs` | selected for fresh pre-registration | Distinguishes logical body feasibility from WRK-0016's declaration persistence stop. It has exact existing source, a finite carrier, an opaque-domain adverse probe, and a clear freeze condition without selecting an API. |
| Anonymous `example : Decidable (captureSubset lhs rhs)` | not selected | It is disposable and has lower retained evidence value. It changes the persistent-name condition but does not itself provide a theorem-shaped artifact. |
| Local `let` / `have` matrix | rejected | No downstream signal beyond the anonymous example. |
| Retained `def` / `abbrev` / `opaque` decision value | deferred | It conflates body feasibility with value persistence/helper/API policy; no bounded consumer requires it. |
| Failed-subset witness extraction | reserve | It may be useful only if a concrete proof-level witness consumer is identified. It otherwise overlaps the selected finite case split and risks being misread as diagnostics work. |
| OBL-001/020/021, OBL-024/025/027, OBL-005, OBL-015 | rejected | Existing evidence is duplicative, or a positive branch needs an owner-reserved Core/result, coverage, outcome-totality, diagnostic, cut, repair, or authority interface. |

## Required pre-registration cut

Before any candidate source edit or Lean outcome command, create a fresh L3
working record with a new WRK identifier. It must pin this plan and the exact
foundation, stay in existing `samples/lean`, and declare `plan` and
`samples/lean` as permitted locations.

The question must be limited to a proposition-valued theorem with a fresh name,
for example `capture_subset_excluded_middle_constructive`. Its source may use
only explicit elimination of `Capability.roomHistory` and
`Capability.ephemeralToken` plus primitive Bool equality decisions. It must
not add or select a persistent data-valued declaration, `def`, `abbrev`,
`opaque`, `instance`, `Fintype`, `Finset`, `Classical`, choice, import, generic
carrier, reusable value helper, predicate change, checker rule, or API.

The pre-registration must include:

- a semantic declaration-form boundary, not only a token scan;
- an opaque arbitrary-domain adverse theorem attempt whose local `by_cases`
  cannot synthesize the needed decision without a finite interface;
- source regression by compiling the existing positive/negative capture lemmas;
- immediate freeze if the theorem needs any excluded declaration/mechanism,
  cannot compile, or creates an OBL/Line-1/checker/core/interface pressure;
- append-only evidence-commit manifestation for any retained outcome.

## Non-claims

This future experiment does not prove or move OBL-003 or any other obligation;
define a generic decision procedure; make the unified judgment decidable; alter
MirCore, syntax, type checking, elaboration, diagnostics, effect/failure,
authority, transport, runtime, contracts, Gate, Phase, conformance, workflow,
or public API; or establish constructive decidability/undecidability beyond the
exact LAB carrier and stated theorem.

## Open boundaries

- Whether the selected theorem compiles remains deliberately unknown until a
  committed and pushed successor registration.
- Whether a retained value declaration has a bounded consumer remains open and
  is not a reason to relax the successor.
- Whether failed-subset witnesses have a proof-level consumer remains open.
- Owner-reserved OBL-001 Core/result, OBL-020 coverage, OBL-021 totality, and
  grammar/contract questions remain outside this route.
