---
id: meta/proposal-028
status: L1-fixed
maturity: draft
depends_on: [meta/proposal-027, adr/ADR-0015, adr/ADR-0024, spec/06-conformance]
summary: 凍結SCN-01..10をordinary sourceから直接通す、有限M10 I1+ conformance profileの提案。
open_items: []
---

# PROPOSAL-028 — M10 I1+ finite conformance

## Owner disposition

Select one finite M10 conformance profile for the frozen `SCN-01..10` suite.
Each SCN correspondence has one primary versioned ordinary `.mir` source plus
finite named negative source variants; a primary source may be shared across
SCNs while retaining its exact identity. SCN-09 additionally has finite named
candidate patch sources. Every source unit runs once through M6 parse/classification. A
negative unit terminates at its required typed diagnostic; an accepted
executable unit continues through M7 checked elaboration, M8 `DeferredToM9`,
source-bound M9 resolution, the provisional M9-to-M8 authority-inventory
bridge, the deterministic runtime, and observer-safe projection. Each unit,
not a fixture name or reconstructed expectation, retains its own identity to
its terminal.

Setup prose is a versioned typed conformance input. It is neither a substitute
source program nor an expected output. A profile must not derive a result from
expected JSON, reports, fixture names, or any other sidecar. A waiver carrier
is prohibited and is always empty.

The profile binds its version, source revision, every source-unit hash, runtime
inputs, policy stamps, predicates, and an overall `profile_hash`. It requires each source
unit's continuity to its terminal—diagnostic for negatives and source-to-Core-
to-trace-to-projection for executable units—plus exact negative
diagnostic/span/no-mutation checks, deterministic replay, and fresh-checkout
reproduction. It does not require identity to be shared across variants.

The selected finite carrier has one hash-bound correspondence declaration per
frozen expectation. A declaration records its source/patch-source/typed-
carrier/context/action role, exact artifact identity, diagnostic location where
applicable, source-derived reference, external schedule action, and evidence
predicate; the verifier appends only `pass` or `fail`. Missing correspondence
fails; it is never `N/A` or a waiver. A schedule stays
at the typed exogenous request/context boundary and cannot manufacture semantic
state, authority, verdicts, fallback position, patch declarations, history,
projection, or expected results. Generated evidence and predicate verification
are separate stages, with program-artifact and schedule-action provenance kept
distinct. SCN-08 uses the non-Surface finite typed three-option fallback
carrier; SCN-09 uses candidate source plus patch-intent pairs checked before
compatibility, so self-grant/capability-less-write rejects cannot be supplied by
a schedule. SCN-05/07 use explicit source-bound observation-policy carriers
for their non-Surface private-policy constraints, never an absent M6/M7 check
or metadata-only widening route.

## One M6/M7 direct-consumer seam

M10 is the direct consumer that reopens exactly one bounded M6/M7 seam. A
`StateDecl` may declare `visible observer_safe fields (FieldName {, FieldName})`.
The one optional clause follows the field declarations. Fields are private by
default. The declared list is a unique subset of the fields of that state:
unknown or duplicate entries reject at their source span. Only a write to a
listed field creates the source-bound observer-publish effect and its
`VisibilityDenied` failure entry; a private-field write creates neither.

The same seam makes the frozen SCN-01/02 owner-directed reading explicit:
`Role[self] at L_actor` is authority origin and nested `at L_owner` is the
evaluation/request site. `L_actor != L_owner` is accepted without minting
authority. The declared target-state owner must equal `L_owner`; same-owner
RHS reads resolve at `L_owner`, not at the actor origin, and the generated
`RouteUnavailable` failure remains required.

The one smaller alternative is metadata-only field visibility in the M10
profile. It is rejected because a profile-only policy would become hidden
observation semantics outside the `.mir` source and checked identity. This
does not freeze final grammar, diagnostics catalog, or public API.

## M9-to-M8 bridge and lifecycle

The bridge is crate-private and provisional. It exposes no provider proof and
losslessly translates only M9-issued active membership, capability, and
witness records into the typed M8 authority inventory. Direct M8 admission for
`AuthDeferred` and `VerifyDeferred` remains `DeferredToM9`, and the bridge
does not auto-attach a `ContractUpdate`.

The M9 resolution envelope is sealed to the exact residual-bearing checked
artifact and its complete residual row. A partial or mismatched envelope stays
deferred and cannot admit execution.

A profile pass is evidence, not the phase lifecycle decision. The lifecycle
requires a separate explicit acceptance record that names the profile hash,
source revision, validation cut, independent review, non-claims, and the
accepting authority.

## Non-effects

This proposal creates no general theorem or new OBL, changes no existing proof
status, and does not promote SCN-11/12: they remain pressure rows distinct from
the frozen 10/10 suite. It does not claim C-distributed, sockets, public
ABI/wire/grammar, deployment, product completion, or I2+ behavior.
