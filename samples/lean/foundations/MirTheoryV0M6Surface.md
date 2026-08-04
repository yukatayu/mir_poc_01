# MirTheoryV0M6Surface.lean

## Summary

This is the M6 trusted finite grammar-form classifier evidence for the
implemented bounded parser/fixture profile in ADR-0021 and spec/01--04. It
defines a fresh finite parsed-form domain and a total classifier to either an
inspectable `CoreTemplate` or a typed `StaticDiagnostic`. It is not a parser,
checker, runtime, public grammar, or general elaboration theorem.

## Exact finite coverage

- Module, locus, principal, type, state, `Role[self]`, and handler forms
  classify to declaration templates with their canonical source span. The
  concrete parser syntax has no semicolon terminators and has state field
  blocks. `Role[self]` is parser-enforced: another actor token is the
  `RoleActorMustBeLiteralSelf` parser diagnostic at that token span.
- An accepted `at S { StateRef = Expr }` form carries `Role[self]` authority
  origin separately from owner evaluation site `S`. It lowers to the M5
  `ownerRmw` fragment with request-to-owner and owner-write edges plus
  capability and witness obligations. The same assignment span carries the
  separate `ownerRmw`/owner-local-read/owner-local-write source-to-Core map
  entries; the local RHS dependency is not a third generated edge. Its
  `receiptFacts` list is exactly empty.
- An actor/action locus mismatch is a typed static diagnostic at the nested
  `at` span. A RHS state dependency owned at another locus is the typed
  `CrossOwnerOperandRequiresReceipt` diagnostic at that RHS reference span, not a
  hidden request or receipt lowering.
- A fieldless assignment target is `FieldlessAssignmentTarget` at the target
  span, and a field-bearing target whose declared owner is outside the action
  locus is `CrossOwnerWriteTargetOutsideActionLocus` at that same target
  span. Neither produces a CoreTemplate; both are distinct from the RHS
  `CrossOwnerOperandRequiresReceipt` diagnostic.
- `relation Name at S { ... publish relation [project at C local] }` carries
  the M5 relation-bind/publication/projection boundary, `publish-relation`,
  its nominal binding frontier, and optional consumer-local projection site.
  `publish value` and `relation Name mutate Field` are typed diagnostics.
  `designated evaluate E on tick F publish result = Expr` carries
  `designatedEvaluation`, `publish-value`, and only the nominal result
  frontier.
- Non-braced `with auth MembershipAuth` and `verify finite_refinement` are
  successful span-preserving non-executable typed deferred templates. They
  introduce no M6 membership grant, verification result, state, effect, or
  runtime behavior.

`send`, `receive`, occurrence/envelope/witness source machinery, receipt
source syntax, and `PresentationContext` are deliberately absent from the
finite `SurfaceForm` constructor domain. The retained witness obligation is
template metadata, not a source-level witness or transport object.

## Evidence and boundary

The theorems prove deterministic classification and canonical span retention
for the finite domain, including templates, generated edges, obligations, and
diagnostics. Exact literal-self, site-aligned no-receipt, fieldless,
target-owner, RHS cross-owner, forbidden relation publication/mutation, and
deferred-template cases are stated as reduction theorems. Lean prints
dependency information for the selected theorems; the source declares no user
`axiom` and uses no `sorry`/`admit`.

This evidence is recorded as OBL-048. It does not amend M5's
`SurfaceFragment`, `Core`, `Config`, `Step`, `WellFormed`, or its theorems. It
does not establish parsing, arbitrary source spans, diagnostic soundness or
completeness, a general elaboration theorem, M7 implementation, M8 runtime,
M9 semantics, transport, or a public API/wire contract.

## Compile

```bash
lean --trust=0 samples/lean/foundations/MirTheoryV0M6Surface.lean
```

## Inventory sync

```bash
python3 scripts/current_l2_lean_sample_sync.py
```
