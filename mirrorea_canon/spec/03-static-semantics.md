---
id: spec/03-static-semantics
status: L1-fixed
maturity: draft
depends_on: [spec/02-surface-grammar, theory/03-elaboration, theory/10-diagnostics, theory/13-evaluation-materialization, theory/14-maintained-relation-projection, theory/15-shared-formal-model, adr/ADR-0021]
summary: 実装済み M6 source form の M5 Core / typed CoreTemplate / typed Diagnostic classification と source-span obligation。
open_items: []
---

# 03 — M6 static classification

M6 first parses the grammar in spec/02, then classifies each semantic source
form to existing M5 Core evidence, an inspectable typed `CoreTemplate`, or a
typed `StaticDiagnostic`. This is a bounded M6 source contract, neither an
M7 checker nor a general elaboration theorem.

Every parser node has a canonical file-qualified span. The classifier derives
an M5 `SourceRef` from it and retains that span in every emitted CoreTemplate,
source-to-Core entry, and diagnostic. A span is neither an M5 operation key,
runtime occurrence, nor presentation input.

| Source form | Current M6 classification | Required retained boundary |
|---|---|---|
| `module`, `locus`, `principal`, `type`, indexed `state` | parsed declaration input | declaration span/name data; no M5 action or runtime step is implied |
| `Role[self] at S` / `when ... fails(...)` | handler authority context | literal `Role[self]` authority origin, declared header locus, failure row, source span |
| `Role[Name]` where `Name` is not `self` | parser diagnostic `RoleActorMustBeLiteralSelf` | actor-token span; no parsed role, CoreTemplate, or static classification |
| site-aligned owner `at S { StateRef = Expr }` | M5 `ownerRmw` Core through owner-mutation template | actor authority origin separate from owner evaluation; `store`; request-to-owner and owner-write generated edges; capability/witness obligations; no receipt/receipt-release fact |
| fieldless assignment target | `FieldlessAssignmentTarget` diagnostic | target-reference span; no CoreTemplate or panic path |
| field-bearing target state owned outside the action locus | `CrossOwnerWriteTargetOutsideActionLocus` diagnostic | target-reference span; no CoreTemplate or implicit cross-owner write |
| local RHS dependency of that assignment | source-to-Core map entry | same assignment span records `OwnerRmw`, `OwnerLocalRead`, and `OwnerLocalWrite`; local dependency is not a receipt edge |
| owner action site differs from enclosing `Role[self] at S` site | `OwnerActionLocusMismatch` diagnostic | action span; no owner Core or hidden authority |
| RHS state dependency owned at another locus | `CrossOwnerOperandRequiresReceipt` diagnostic | offending RHS reference span; no hidden request, receipt, release, snapshot, capability, or witness |
| `relation ... publish relation [project at C local]` | maintained-relation template | M5 relation bind/publication/projection boundary, typed binding frontier, `publish-relation`, optional consumer-local projection site, no result frontier |
| `relation ... publish value Name` | `RelationMustPublishRelationCarrier` diagnostic | publication-clause span; no absolute-value relation carrier |
| `relation Name mutate Field` in an owner action | `ConsumerRelationMutationDenied` diagnostic | mutation span; no semantic relation mutation |
| `designated evaluate E on tick F publish result = Expr` | designated-result template | `publish-value`, typed result frontier/version, no relation frontier |
| `with auth MembershipAuth` | successful typed deferred CoreTemplate | source span and typed deferred-policy marker; supplies required authority metadata only, with no M6 grant/membership state/effect |
| `verify finite_refinement` | successful typed deferred CoreTemplate | source span and typed deferred-policy marker; no proof verdict/state/effect |

## Checked source obligations

1. The current bounded name checks reject an unresolved handler parameter type,
   a locus name that collides with a declared type name, and an unresolved RHS
   state base. Each diagnostic carries the exact token/reference span. M7 owns
   general namespace, type, and diagnostic-completeness claims.
2. A nested owner action's explicit `at` site must equal its enclosing
   `Role[self] at` site. The classifier keeps the Role authority origin and
   owner evaluation site separate; the nested `at` never mints authority.
3. An accepted owner mutation has a field-bearing target whose declared state
   owner equals the action site. Fieldless targets and cross-owner targets are
   rejected at the target reference before Core construction.
4. For an accepted owner mutation, every resolved RHS state reference must be
   owned by the action site. A cross-owner operand is rejected at the operand
   span, and the M6 grammar offers no receipt syntax to recover it.
5. Same-owner lowering records separate capability and witness obligations and
   two generated M5 edges (request-to-owner, owner-write). The third
   owner-local RHS dependency is a distinct source-to-Core map entry. The
   receipt-fact list is exactly empty; this does not claim that a genuine
   cross-owner operand needs no receipt.
6. `ResultFrontierName` and `BindingFrontierName` remain distinct nominal
   values. A designated result has the former and `publish-value`; a
   maintained relation has the latter and `publish-relation`.
7. `with auth` and `verify` are successful source classifications only as
   non-executable typed deferred templates. They retain no authorization
   grant, membership decision, proof verdict, state update, or runtime step.

## Rejections and non-effects

The parser rejects `send`/`receive`, `occurrence`, and `envelope` with typed
unsupported-syntax diagnostics before classification. No accepted M6 form
emits transport/communication syntax, occurrence/envelope machinery,
receipt/release facts, runtime steps, membership state, proof verdict, or
presentation state. Capability/witness *obligations* in an owner template are
not source-level witness or transport objects.

The exact finite classifier evidence is OBL-048 in theory/11. M7's separate
finite refinement/evidence is spec/08 and OBL-049; neither changes this M6
classification contract. General parser coverage, elaboration determinism,
diagnostic soundness/completeness, checker decidability, and M9 semantics
remain separately deferred.
