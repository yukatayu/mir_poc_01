---
id: theory/17-m8-deterministic-runtime
status: L1-fixed
maturity: draft
depends_on: [theory/04-ordering-and-cuts, theory/05-authority, theory/07-observation, theory/08-patch-hotplug, theory/13-evaluation-materialization, theory/14-maintained-relation-projection, theory/15-shared-formal-model, theory/16-m7-checked-elaboration, adr/ADR-0023]
summary: M7 checked artifactを唯一のsource inputとする、有限M8 deterministic runtime admission・lowering・state model。
open_items: []
---

# 17 — M8 deterministic runtime

This chapter defines the bounded M8 runtime handoff.  It is a finite,
single-process, logical multi-locus reference model.  It is neither a general
runtime calculus nor an implementation/public representation.

## 1. Checked-artifact-only admission

Let `A7` be an immutable M7 checked artifact.  Its M8 identity is the
structural tuple:

```text
I8(A7) = ⟨retained program identity, static environment,
          checked evaluation/Core shape, effect/obligation shape,
          stable source-to-Core map, residual-source-reference rows⟩
```

The retained M7 program identity and every residual `SourceRef` are part of
this tuple.  An M8 admission input is `⟨I8(A7), E8⟩`; it is invalid when any
identity component, source reference, or required evidence row differs.  M8
reads no source text/AST.  A request name may deterministically select only a
checked plan retained in `A7`; it must not rebuild a plan from source or
consult an externally supplied or reconstructed name-keyed evaluator side
table.

M7 static admission and M8 runtime admission are distinct judgments:

```text
A7.execution_is_admissible       M7 static, empty residual row only
⟨A7, E8⟩ ⇝8 RuntimeAdmitted | Rejected | DeferredToM9
```

The second judgment never changes the first.  In particular, M8 does not
claim that a relation/designated-result artifact became M7 residual-free.

`Visibility`, `RelationLifetime`, `FallbackValidity`, and
`ValueVisibilityRedaction` require the finite source-ref-bound M8 evidence
selected by ADR-0023.  Absent or mismatched evidence produces `Rejected`
before semantic mutation.  `AuthDeferred` and `VerifyDeferred` always yield
`DeferredToM9`; they create no authority, capability, verification verdict,
effect, semantic mutation, or runtime admission.

For the exact relation profile, the three evidence rows are respectively
`⟨SourceRef, label, redaction⟩`,
`⟨SourceRef, declared lease ref, binding frontier⟩`, and
`⟨SourceRef, primary epoch, fallback epoch⟩`. The designated row is
`⟨SourceRef, label, redaction⟩`. Every declared component is matched before the
relation or designated carrier is installed; source-only or kind-only matching
is not M8 admission. Admission does not read or prove a live `L` inventory.
A missing or mismatched declared row, or a duplicate/conflicting
`⟨kind, name, SourceRef⟩` row, is `Rejected`; in particular M8 never selects
duplicate evidence by input order.

## 2. Deterministic lowering and state

Lowering preserves the stable source-map order and converts only retained
checked actions:

```text
OwnerRmw              request → owner-local-read → owner-write
PublishRelation       owner relation publish → consumer-local projection
DesignatedPublishValue request → receipt-use → value publish
```

For this finite profile the one semantic state is:

```text
K8 = ⟨I8, E8, P, H, Q, S, M, G, W, L, D, J, C, X⟩
P  active retained checked artifact/Core/effect/obligation/source-map plan set
H  append-only occurrence/dependency trace with source references
Q  per-owner FIFO requests and logical deterministic turn data
S  owner state
M/G/W/L membership, capability, witness, and lease validation records
D  designated result/frontier/version/policy/stamp store
J  owner-held maintained relation/binding/published relation
C  local atomic-cut/save provenance
X  bounded patch frontier/lifecycle rows
```

`I8`, `E8`, `P`, `H`, `Q`, `D`, `J`, `C`, and `X` are save-relevant semantic state.
Observer projection and presentation context are read-side and are not a
second semantic state or a hidden persistence carrier.

Owner service consumes at most one FIFO request per owner turn. It validates
the finite live membership/capability/witness/lease context before evaluating
the owner RMW. `H` records the selected authority/witness validation facts or
witness-rejection plus declared failure, including the local raw
authority/witness/capability failure payload; a failure does not change `S`,
`D`, or `J`. The selected two-request profile evaluates at service and
therefore reaches `100 → 90 → 80`.

## 3. Relation, designated value, cut, patch, and observation

Relation publication installs the exact admitted payload only in owner-held
`J`; consumer local projection does not semantically mutate `J`. The selected
re-acquire rejects its reused/forged witness and accepts only its declared
distinct fresh witness with a new binding/primary epoch, exact fresh
`⟨relation, owner, declared ref, live, binding frontier, lease epoch⟩`
inventory tuple, and lineage. Projection, transition, and re-acquire each
recheck that dynamic `L` tuple. Its absence, expiry, or relation/owner/ref/
frontier/epoch mismatch rejects that operation without a relation mutation; it
does not revise `⟨A7, E8⟩ ⇝8`. Consumer-local fallback remains read-side and
preserves the admitted relation label and redaction exactly; it cannot weaken a
private relation. A designated value is keyed by its evaluator/result/input
frontier/result frontier/version and keeps its evaluation policy, observation
policy, policy stamp, label, and redaction. A duplicate uses the stored decision
rather than creating another semantic value.

A local save object contains the whole declared `K8` profile.  Restore checks
program/admission provenance and the finite membership/capability/witness/
lease context; a stale record is rejected.  This is not THM-003 or a general
cut/load algorithm.

For the selected bounded patch rows, `Rejected` and `Deferred` append only a
patch-lifecycle occurrence. Their `K8` semantic snapshot is unchanged. The
one selected `Accepted` row atomically replaces `I8`, `E8`, and `P` at its
local cut inside the same `K8`. Observer-safe output is projected from `H` and
labels/redactions. Raw `H` is internal and non-observer-exportable; its carrier
has authority, witness, capability, and raw-failure payload fields. An observer
failure row retains its explicit label/redaction but has no raw witness,
capability, authorization, or raw-failure payload field.

## 4. Boundary

OBL-050--056 state exact finite Lean evidence only. OBL-057 is
`runtime-monitored` for validation correspondence between this finite carrier
and the current bounded typed/source-bound Rust M8 route: 53 focused M8 tests,
full runtime/semantics all-target checks, format and clippy, trusted Lean
28-theorem axiom-free checking, raw-public-API absence and observer
label/redaction scans, and Canon/hierarchy/docs checks passed on the current
cut. This is implementation and fixture-matrix evidence, not a general
correspondence theorem. This chapter does not prove a general scheduler/DAG
theorem, general owner preservation, general noninterference, general
patch/cut theorem, transport/receipt delivery, M9 auth/verification, M10 or
official SCN conformance, or public API/ABI/wire behavior.
