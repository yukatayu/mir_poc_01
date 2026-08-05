---
id: meta/proposal-024
status: L1-fixed
maturity: draft
depends_on: [meta/proposal-018, adr/ADR-0015, adr/ADR-0016, adr/ADR-0020, theory/15-shared-formal-model]
summary: M6 で retained pre-M6 profile を置換し、M5 shared model に total に分類する bounded ordinary Surface を選ぶ提案。
open_items: []
---

# PROPOSAL-024 — M6 bounded ordinary Surface

## Owner disposition

Under the owner-approved ADR-0015 program, select one bounded M6 Surface
profile as the direct source input to M7.  The selected source forms are:

```text
module / locus / principal / type / indexed state-with-fields
Role[self] at L_actor { when h(...) fails (...) { at L_owner { field = expr } } }
relation name at S { subject/primary/fallback/bind/publish/project }
designated evaluate E on tick F publish result = expr
with auth MembershipAuth
verify finite_refinement
```

There are no semicolon terminators. The relation form retains its owner site,
subject type, primary/fallback anchor plus epoch/transform, binding frontier,
`publish relation`, and optional `project at C local`. Its syntactically valid
`publish value` sibling is a typed diagnostic, not a relation lowering.
`fails(...)` is mandatory, including `fails()` for an empty declared row.
The actor form has literal `self`, not an inferred authority binder.
`Role[self] at L_actor` records authority origin, while nested `at L_owner`
records the owner evaluation/request site. The loci may differ without an
authority grant; the target state's declared owner must instead equal
`L_owner`. Same-owner RHS reads resolve at `L_owner`, and `RouteUnavailable`
remains in the generated failure row.

The parser rejects any non-`self` role actor as `RoleActorMustBeLiteralSelf`.
Before owner-RMW lowering, a fieldless target is
`FieldlessAssignmentTarget`, and a target state whose owner differs from the
action site is `CrossOwnerWriteTargetOutsideActionLocus`; both reject at the
target span with no CoreTemplate. These are distinct from the RHS
`CrossOwnerOperandRequiresReceipt` diagnostic.

The ordinary same-owner action lowers only to an M6 `CoreTemplate` for the M5
owner-RMW fragment.  It has explicit request-to-owner, owner-write, and
separate source-to-Core owner-local-RHS-dependency entries, plus capability and
witness obligations. It emits no receipt or receipt-release fact. A source action
whose resolved RHS state dependency belongs to another locus is instead the
explicit receipt-required static diagnostic; this proposal neither adds a
receipt syntax nor changes M5's finite elaborator.

`=` in an `at` action is a one-shot owner mutation. `result = expr` names one
designated publication template; it is not state mutation.  Neither form
creates a maintained relationship. Only `relation ... at S { ... publish
relation ... }` elaborates to the M5 relation publication template with a
typed binding-frontier field and `publish-relation`; `publish value` is
rejected. `relation Name mutate Field` is syntactically parsed only to produce
the consumer-relation-mutation diagnostic.

`with auth` and `verify finite_refinement` parse without braced bodies and
classify successfully as explicit non-executable typed deferred CoreTemplates.
`with auth` supplies required-authority metadata only. Neither form introduces
an M6 grant, membership decision, verification verdict, state, effect, or
runtime action.
`PresentationContext` remains a M5 read-side input and has no source-state
syntax in this profile.

## Selected alternative and falsifier

The one smallest viable alternative is the retained ADR-0008 pre-M6 profile
with place/role blocks, `chain`, and unconstrained statement families.  It is
not selected because it does not give one bounded total classifier over the
M5 fragment boundary and leaves source forms that can imply unrelated
authority/effect behavior.

The selected route is falsified if any accepted M6 constructor needs an
implicit communication, authority, receipt, result/relation frontier
conversion, presentation state, or runtime behavior in order to classify; or
if a generated CoreTemplate/diagnostic/obligation cannot retain its source
span.  In that event, retain the failed evidence and reopen the grammar
package rather than adding a hidden default.

## Scope and non-effects

This authorizes ADR-0021, the M6 specification rewrite, exact finite Lean
classifier evidence, and the associated ledger entry.  It does not implement
a parser, checker, elaborator, SCN corpus, runtime, transport, receipt
protocol, membership system, proof engine, final JSON/wire/API/ABI, or public
grammar freeze.  It does not alter M5's `SurfaceFragment`, Core, Config,
Step, presentation context, or theorem claims.  Legacy grammar and LAB source
remain historical compatibility evidence rather than deleted or silently
promoted syntax.
