---
id: spec/02-surface-grammar
status: L1-fixed
maturity: draft
depends_on: [spec/01-lexical-and-modules, adr/ADR-0021, adr/ADR-0025, adr/ADR-0029, adr/ADR-0031, theory/13-evaluation-materialization, theory/14-maintained-relation-projection, theory/15-shared-formal-model]
summary: 実装済み M6 parser / fixture とSYS-3 designated-consume、SYS-5 provisional explicit relation-anchor locusを含むordinary Surface reference grammar。
open_items: []
---

# 02 — M6 bounded Surface grammar

This is the selected M6 grammar and the exact current parser/fixture profile.
It supersedes the retained pre-M6 grammar as the current source direction, but
it remains a bounded reference grammar, not a final public language or a
general expression parser.

```ebnf
Module          ::= "module" ModulePath { Item }
ModulePath      ::= Ident { "." Ident }
Item            ::= LocusDecl | PrincipalDecl | TypeDecl | StateDecl
                  | ActorBlock | RelationDecl | DesignatedEval | DesignatedConsume
                  | DeferredAuth | DeferredVerify

LocusDecl       ::= "locus" LocusName
PrincipalDecl   ::= "principal" PrincipalName
TypeDecl        ::= "type" TypeName
StateDecl       ::= "state" StateName "[" IndexName ":" TypeName "]"
                  "at" LocusName "{" { StateField } [ ObserverSafeFields ] "}"
StateField      ::= FieldName ":" TypeName
ObserverSafeFields ::= "visible" "observer_safe" "fields" "("
                       FieldName { "," FieldName } ")"

ActorBlock      ::= "Role" "[" "self" "]" "at" LocusName
                  "{" { Handler } "}"
Handler         ::= "when" HandlerName "(" [ Param { "," Param } ] ")"
                  "fails" "(" [ FailureName { "," FailureName } ] ")"
                  "{" { OwnerAction } "}"
Param           ::= Ident ":" TypeName
OwnerAction     ::= "at" LocusName "{" { ActionItem } "}"
ActionItem      ::= OwnerAssignment | RelationMutation
OwnerAssignment ::= StateRef "=" OwnerExpr
RelationMutation ::= "relation" RelationName "mutate" Ident

RelationDecl    ::= "relation" RelationName "at" LocusName "{"
                  "subject" Ident ":" TypeName
                  PrimaryAnchor FallbackAnchor
                  "bind" "frontier" BindingFrontierName
                  "publish" ("relation" | "value" Ident)
                  [ "project" "at" LocusName "local" ] "}"
PrimaryAnchor   ::= "primary" AnchorName [ "at" LocusName ]
                  "epoch" EpochName "transform" Transform
FallbackAnchor  ::= "fallback" AnchorName [ "at" LocusName ]
                  "epoch" EpochName "transform" Transform
Transform       ::= "identity"
                  | "translate" "(" SignedInt "," SignedInt ")"
SignedInt       ::= [ "-" ] IntLiteral

DesignatedEval  ::= "designated" "evaluate" LocusName "on" "tick"
                  ResultFrontierName "publish" ResultName "=" DesignatedExpr
DesignatedConsume ::= "designated" "consume" LocusName "." ResultName
                  "at" LocusName
DeferredAuth    ::= "with" "auth" AuthName
DeferredVerify  ::= "verify" DeferredName

StateRef        ::= Ident [ "[" Ident "]" ] [ "." Ident ]
FieldBearingStateRef ::= Ident [ "[" Ident "]" ] "." Ident
OwnerExpr       ::= M6ExprToken { M6ExprToken }
DesignatedExpr  ::= M6ExprToken { M6ExprToken }
M6ExprToken     ::= Ident | IntLiteral | "{" | "}" | "[" | "]"
                  | "(" | ")" | ":" | "," | "." | "=" | "+" | "-"
```

There are no semicolon terminators.  `OwnerExpr` is the parser's bounded token
collector through the closing `}` of its `at` block.  `DesignatedExpr` is the
corresponding collector through the next `with`, `verify`, or end of file; the
canonical fixture profile therefore places a designated evaluation after other
ordinary items and before any deferred forms.  This preserves the implemented
parser boundary without claiming a general M6 expression grammar.

The parser recognizes the syntactically valid `publish value Name` and
`relation Name mutate Field` variants so that classification can return a
typed diagnostic with their exact spans.  They are not accepted semantic
relation operations.  `publish relation` is the only relation form that
classifies to the maintained relation template.

`ResultFrontierName` and `BindingFrontierName` are distinct source references.
The former becomes the finite M5 result frontier for designated evaluation;
the latter becomes the finite binding activation frontier for maintained
relation.  M6 adds no separate declaration production for either.

## Fixed source distinctions

- `Role[self] at L_actor` has literal `Role[self]` authority origin. A nested
  `at L_owner` is its explicit evaluation site and owner-directed request
  site; `L_actor != L_owner` is accepted and never mints authority. The
  target state's declared owner must equal `L_owner`, or the target-reference
  diagnostic is `CrossOwnerWriteTargetOutsideActionLocus`. Same-owner RHS
  state reads resolve at `L_owner`, not at `L_actor`; `RouteUnavailable`
  remains a generated failure of the owner action. A non-`self` bracket token
  is rejected by the parser as `RoleActorMustBeLiteralSelf`, at the token
  span.
- The parser accepts the broad `StateRef = OwnerExpr` shape in an `at` block,
  but a lowerable owner mutation requires `FieldBearingStateRef` on the left.
  A fieldless target is `FieldlessAssignmentTarget` at the target span. A
  field-bearing target whose declared state owner differs from the action site
  is `CrossOwnerWriteTargetOutsideActionLocus`, also at the target span. Both
  have no CoreTemplate.
- A resolved state-valued RHS dependency owned at another locus is separately
  `CrossOwnerOperandRequiresReceipt` at the RHS-reference span, never implicit
  communication. It is not the cross-owner write-target diagnostic.
- `designated evaluate E on tick F publish result = Expr` is one
  designated-result template binding, not a state mutation.  `result` is an
  ordinary result name in this fixture profile, not a reserved punctuation
  token.
- `designated consume E.result at C` is the bounded SYS-3 internal source
  clause that names the one consumer locus of an existing designated result.
  It is a distinct source item with its own result-reference and consumer-locus
  spans; it neither repeats the evaluator expression nor infers `C` from
  topology, schedule, relation, or deployment metadata. This exact finite
  profile accepts at most one consumer for `(E, result)`. An undeclared `C` or
  a competing second consumer is rejected by M7. This clause is provisional
  reference Surface v0, not final/public grammar or compatibility syntax.
- A maintained relation owns an explicit relation/binding frontier and may
  describe a consumer-local projection site.  It publishes a relation carrier,
  not an early materialized absolute value.
- A maintained relation anchor may name its existence locus with provisional
  internal `at L`. The explicit locus is distinct from relation owner and
  consumer projection locus. It is retained exactly for M7/projection/runtime;
  an undeclared explicit locus rejects. The earlier form without `at` remains
  accepted with no inferred anchor locus. SYS-5 leave/fresh requires an
  explicit primary anchor locus. This is not final/public grammar or a
  compatibility promise.
- A `StateDecl` may list a source-bound observer-safe subset with
  `visible observer_safe fields (...)` after its field declarations. At most
  one such clause is syntactically present. Unlisted fields are private by
  default. Each listed name must denote a field of that declaration exactly
  once; an unknown or duplicate name is rejected at that declaration token
  span. This finite declaration is an M10 consumer seam, not final grammar/API
  surface.
- `with auth MembershipAuth` and `verify finite_refinement` have no braced
  body.  Each classifies successfully to a non-executable typed deferred
  CoreTemplate with a source span; neither executes an effect nor settles M9
  membership, grant, or verification semantics.

There is no Surface production for `send`, `receive`, receipt/release,
occurrence/envelope/witness, `PresentationContext`, `chain`, `try`, patch,
transport, provider, or renderer control.  The historical ADR-0008
place/role/chain profile remains LAB compatibility evidence only.
