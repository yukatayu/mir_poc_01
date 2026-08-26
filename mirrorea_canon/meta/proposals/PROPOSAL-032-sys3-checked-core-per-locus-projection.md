---
id: meta/proposal-032
status: L1-fixed
maturity: reviewed
depends_on: [root/north-star, root/design-constitution, adr/ADR-0026, adr/ADR-0027, adr/ADR-0028, arch/03-toolchain, arch/04-runtime-carriers, theory/13-evaluation-materialization, theory/14-maintained-relation-projection, spec/08-m7-checked-elaboration]
summary: SYS-3のbounded designated-consume source/Core edge、checked-Core-only per-locus artifact、generated plan、source/Core/artifact correspondenceをcut 3013e7feで有限受理した提案。
open_items: []
---

# PROPOSAL-032 — SYS-3 checked-Core per-locus projection

## Owner disposition and selected capability

Under ADR-0026, accept the smallest SYS-3 compiler boundary that lets SYS-4
start independently tagged locus artifacts without parsing source or
reconstructing semantic plans. The selected route is pure and crate-private:

```text
CheckedSurfaceV0 + DeclaredLogicalTopology
  -> project_checked_core
  -> GlobalProjectionResult | ProjectionDiagnostics
```

`DeclaredLogicalTopology` is bound to the exact checked program identity and
contains only the logical locus inventory. Its locus set must equal the union
of the checked static-environment loci and every locus referenced by checked
owner, relation, designated-evaluator, designated-input, and
designated-consume Core. It cannot
declare communication edges, authority, message schemas, failures, handlers,
or deployment hosts.

The result owns the typed checked fragments needed by the next consumer. It
does not borrow an AST or require the original checked artifact to remain
alive. Input order and topology insertion order do not affect the canonical
result.

## Close-review history and bounded E-CONSUME correction

Close review falsified the first `ded622fe...` candidate as a complete SYS-3
cut. It projected designated remote inputs into the evaluator and kept the
evaluator expression in the right artifact, but the ordinary source/AST/M6/M7
path named no result consumer. Consequently no checked Core fact could derive
the required evaluator-to-consumer delivery/consume edge; topology could not
repair that omission without becoming an illicit semantic input.

The exact bounded correction is the provisional internal Surface-v0 clause:

```text
designated consume E.result at C
```

It creates a distinct AST item, M6 template/source-map entry, and M7
`DesignatedResultConsume` checked Core edge. The edge binds the already-declared
designated result to exactly one explicitly declared consumer locus, preserving
its input/result frontiers, version, observation policy, policy stamp, and the
normative finite one-consumer rule of theory/13 `[E-CONSUME]`. It contains
neither the evaluator expression nor raw input. The static contract names
`ReturnExistingNoNewConsumption`: a future retry by the same named consumer
must return the existing decided result without a new semantic consumption,
while a competing consumer is a typed conflict.

`ReturnExistingNoNewConsumption` is a **new required SYS-4 endpoint refinement
contract**, not an existing M8 implementation behavior or evidence. The legacy
M8 direct-consume API keys consumption by delivery id: repeating the same
delivery rejects with `AlreadyConsumed`, while a different delivery id can be
consumed again. M10's accepted duplicate-delivery pressure case preserves that
rejection and remains an unchanged regression baseline. SYS-3 encodes only the
source/Core-bound semantic-consumption identity and the static refinement
requirement. SYS-4 must add a carrier-side idempotent return path or compatible
wrapper that, on the accepted path, invokes legacy M8 consumption exactly once
for that semantic identity and returns the stored decision on a same-consumer
retry without invoking M8 again. Actual positive, retry, and competing-
consumer endpoint tests belong to SYS-4.

This is the smallest ordinary-source fact required by SYS-3/SYS-4. It does not
select final/public grammar, API, ABI, JSON, or wire; admit multiple consumers;
infer a consumer from topology, schedule, relation, or deployment; or claim
runtime dispatch. The AST/M6/M7/projection implementation and falsifiers now
pass at accepted cut `3013e7fe075a7605a1ffe01e0b14f4a0856eaeb9`;
SYS-3 is closed, SYS-4 is active, and SYS-5 is next.

## Selected projection result

The internal `GlobalProjectionResult` contains:

```text
checked-program and projection identity
LocusProgram[locus]
CommunicationPlan
EffectHandlerPlan
ProjectionRelationGraph
ObservationPlan
PersistencePlan
ProjectedSourceMap
static readiness and runtime-admission status
ST / OW1 backend requirements
```

Each operation fragment retains its locus tag, operation/fragment identity,
placement-specific typed checked Core, source and Core references, exact
declared/generated failure rows, semantic obligations, and the separate
requirements that a sealed runtime seam must later satisfy. A projection
result may be statically ready while runtime admission remains
`AwaitingRuntimeSeam` or `BlockedByResidual`; projection never issues
membership, capability, witness, producer-release, or evaluator authority.

ST remains eligible for every successfully projected finite artifact. OW1 is
eligible only when the checked artifact has exactly one combined semantic
owner/source-owner locus. These are semantic backend requirements from
ADR-0028, not Rust worker/channel layout or evidence that execution occurred.

## Placement and generated communication

The selected placement is by construction:

- an owner request invocation remains at the checked authority-origin locus;
  the whole same-owner RMW Core and local state schemas remain only at the
  checked owner;
- owner request and typed reply/receipt edges are generated between those
  fragments and transfer no authority;
- a maintained relation remains an owner publication plus an optional
  consumer-local projection. The consumer receives the checked relation and
  fallback lineage, not an absolute-value stream and not mutation authority;
- each designated remote-input dependency produces a source-owner service,
  request/result carrier plan, and evaluator fragment. The checked state-read
  source span remains distinct from the evaluator artifact span, and the
  evaluator expression is not copied to consumers; and
- each explicit designated-consume Core edge produces one evaluator-side
  result-delivery source fragment, one named-consumer consume fragment, and one
  generated `DesignatedResultDelivery` edge. The consumer fragment exposes no
  evaluator expression or raw input; and
- effect-handler rows are only the source/Core-bound owner service,
  designated source service, and designated evaluator rows required by this
  fragment. There is no generic provider registry.

Every communication edge directly names real source and target operation
fragment references plus its checked-Core identity. This includes the
evaluator-to-consumer edge only when the distinct designated-consume Core exists;
topology cannot invent it. Its carrier contract keeps
the applicable operation/request identity templates, source/Core provenance,
origin/target loci, declared failure/effect rows, authority requirements,
typed outcome, occurrence slots, frontiers, and receipt-consumption state.
Authority requirements are slots for the sealed runtime seam, not grants;
transport or receipt does not satisfy them.

The observation plan names only required future runtime occurrences and uses
reference-only redaction. The source map joins source -> Core -> artifact ->
edge/plan without inventing a runtime occurrence. Persistence planning assigns
the selected local store, carrier, authority-reference, relation/designated,
receipt, cut, and patch responsibilities to loci or the whole-fabric boundary;
it neither saves nor restores state in SYS-3.

## Relation graph boundary

Production projection preserves exactly the currently checked two-anchor
relation shape: one source-bound primary -> fallback edge per accepted
relation. It does not add nested relation syntax or dependencies to ordinary
source.

The implementation also retains a finite typed, acyclic, same-checked-program
extension boundary exercised only by tests with a shared/deeper dependency
pressure case. That boundary rejects a cycle or a foreign checked-program
source reference and cannot mint future semantic dependencies. It proves only
that the current representation has a conservative source-bound pressure
seam; it is not production nested-relation semantics or an arbitrary-DAG
theorem.

## Verification, falsifiers, and accepted evidence

`verify_projection` recomputes the pure canonical result and rejects a
candidate with an identity, backend, persistence, handler provenance, source
map, owner placement, edge completeness, or other structural mismatch. A
diagnostic returns no partial projection result.

The contract is falsified if topology can add an unknown locus or omit a Core
locus; two input orders produce different results; a candidate adds or removes
an edge; owner execution moves to a requester; an edge does not bind actual
source/target fragments; designated state-read/evaluator provenance collapses;
an observation row loses edge/fragment/occurrence/redaction identity;
finalization duplicates or erases a semantically distinct observation row; a
cycle/foreign relation dependency passes; or SYS-4 must re-read source to
execute an artifact. The reopened contract is additionally falsified if a
designated result with an explicit consumer lacks its consumer fragment or
evaluator-to-consumer edge, a consumer is inferred without the source clause,
the evaluator expression leaks into the consumer, the named consumer moves to
another locus, an undeclared/competing consumer passes, or same-consumer retry
is encoded without a stable source/Core semantic identity. Claiming the legacy
M8 `AlreadyConsumed` duplicate-delivery rejection as evidence of
`ReturnExistingNoNewConsumption` also falsifies this evidence boundary.

The former candidate cut is
`ded622fef91bab2cadc571ba944e5ee2c69a7b63`, following the RED test commits
`c10a1bceb882ef57b057e54b59febacb188f8f09`,
`dae31bbe296f1a3919b2cf10803d1fdeb1c11392`,
`db4358d1e3accb2b7b76380dd650f952327b24dc`,
`cd98d81fbcbdb83abb67b6ba0e5997e4c3e76ea8`, and
`e8c9570f4c320df9c7011b974868fe4791145ea7`. Twenty-five focused projection
tests, the preserved SYS-2/SYS-1/Full-System-V1/M10/runtime/workspace test
regressions, formatting, package-scoped warnings-denied Clippy, and independent
semantics and code-quality reviews passed for the fragment they covered. Full-
workspace Clippy is not claimed. Close review then
found the missing E-CONSUME path, so that cut is retained as partial regression
evidence and is not an accepted SYS-3 cut.

The corrected RED sequence is `b39f3e76` (source-bound designated consume),
`f37be73c` (M6 metadata), `27e42658` (missing producer), and `30be30bb`
(ambiguous/silent signature shadow), followed by production cut
`3013e7fe075a7605a1ffe01e0b14f4a0856eaeb9`. Review first found the missing
E-CONSUME path and reopened SYS-3; subsequent review found and fixed the M6
metadata P1, missing-producer P2, and silent-signature-shadow P1. Final
semantic and code-quality reviews are ACCEPT.

Accepted validation is AST Surface M6 9/9, M6 classification 13/13, M7
pipeline 25/25, M9 8/8, SYS-3 projection 27/27, M8 admission 7/7, M10 source
2/2, M10 conformance 67/67, full `mir-runtime`, full workspace tests, format,
package-scoped warnings-denied Clippy for `mir-ast`, `mir-semantics`, and
`mir-runtime`, and diff check. Full-workspace Clippy is not claimed.

This evidence establishes only the static finite projection identity,
placement, generated plans, diagnostics, and correspondence contract. Actual
endpoint idempotent-return behavior remains a SYS-4 obligation. OBL-060 is
therefore `runtime-monitored` at that static compiler/projector scope only. No
Lean statement, general projection determinism/completeness/owner-preservation
theorem, arbitrary relation-DAG proof, runtime dispatch, or retry-runtime
correspondence is added.

## Direct consumer and stop condition

```text
Direct consumer: SYS-4 in-process generated dispatch
Blocker reduced: checked Core omitted the source-named evaluator-result
  consumer and therefore could not emit the complete E-CONSUME artifact and
  visible delivery plan without forbidden topology inference; the retry row
  also lacked an explicit static semantic-identity / SYS-4 runtime-evidence
  boundary
Acceptance use: SYS-4 locus startup/endpoint dispatch, SYS-5 causal devtools,
  and the later SYS-6 finite projection correspondence profile
```

SYS-3 is closed at `3013e7fe075a7605a1ffe01e0b14f4a0856eaeb9` after the
bounded designated-consume clause became source/Core-bound through AST, M6,
M7, projection fragments, generated delivery, observation/persistence/
correspondence plans, positive/negative tests, preserved regressions, and
independent review. Reopen only if
SYS-4 cannot execute the owned fragments without source/AST reconstruction, a
selected accepted Core operation has a missing/extra/owner-moving edge, two
equivalent inputs project differently, or the retained relation representation
cannot conservatively accept the later source-derived finite DAG extension.

## Non-effects

This proposal does not dispatch a message, start a locus runtime, admit an
artifact into M8/M9, create an actual runtime occurrence, execute save/load or
patch, define deployment placement, support production nested relations,
freeze public API/ABI/wire/JSON, select transport, add multi-consumer semantics,
claim a general theorem, accept broad PHASE-I1/I2 lifecycle, or change theory
T1. It changes only the bounded non-final internal Surface-v0/AST/M6/M7 Core
path named above; it does not freeze final grammar or a general Core exchange
form.
