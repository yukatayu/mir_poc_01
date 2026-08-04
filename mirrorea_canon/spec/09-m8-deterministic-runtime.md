---
id: spec/09-m8-deterministic-runtime
status: L1-fixed
maturity: draft
depends_on: [spec/05-runtime-semantics, spec/08-m7-checked-elaboration, theory/17-m8-deterministic-runtime, adr/ADR-0023]
summary: M8 reference runtimeのchecked-artifact-only input、finite admission、deterministic trace/state output boundary。
open_items: []
---

# 09 — M8 deterministic runtime admission

## Input boundary

The selected M8 route takes one M7 `CheckedSurfaceV0` artifact plus typed M8
runtime admission evidence.  The artifact is the only source-program input.
The route must retain and validate a structural checked identity containing
the static environment, evaluation/Core shape, effect/obligation shape, and
stable source map.  Every evidence row is bound to this identity and to the
residual's canonical `SourceRef`.

The route may use a request name only to deterministically select a checked
plan already retained by the admitted artifact.  It must not parse/reclassify
source, use an AST, look up a fixture or evaluation by a reconstructed name,
consult an externally supplied/reconstructed evaluator side table, or accept
report/expected JSON/helper output as a semantic shortcut.  Internal M8
carrier names are not a public API, ABI, wire, or diagnostic catalog.

## Admission result

| Condition | Result | Required non-effect |
|---|---|---|
| residual-free checked owner artifact with matching identity | `RuntimeAdmitted` | no source reconstruction |
| complete relation payload `⟨SourceRef, label, redaction, declared lease ref, binding frontier, primary epoch, fallback epoch⟩` | `RuntimeAdmitted` | install that exact declared payload in owner-held relation state only; do not resolve dynamic inventory here |
| complete designated payload `⟨SourceRef, label, redaction⟩` | `RuntimeAdmitted` | retain input/result frontier, policy, stamp, label, and redaction |
| missing, mismatched, duplicate, or conflicting base evidence / identity / source ref | `Rejected` | semantic state unchanged |
| operation-time absent, expired, or mismatched relation lease-inventory row | operation rejected | no projection, transition, or re-acquire mutation |
| `AuthDeferred` or `VerifyDeferred` | `DeferredToM9` | no authority, proof verdict, effect, mutation, or success |

`RuntimeAdmitted` is an M8 result.  It does not mutate M7
`execution_is_admissible` or make an unresolved M7 residual disappear.

Admission validates only the declared lease reference, binding frontier, and
primary/fallback epochs. It does not prove or read a current live lease
inventory. A duplicate residual-evidence key
`⟨kind, name, SourceRef⟩`, including a bytewise-identical duplicate, is also
rejected: M8 never selects a first/last duplicate row. An M8 re-acquire accepts
only the declared distinct fresh witness and an exact fresh lease-inventory row
`⟨relation, owner, declared ref, live, binding frontier, lease epoch⟩` whose
frontier/epoch agree with the selected new primary epoch and lineage. This
dynamic row is rechecked for every projection, transition, and re-acquire. Its
absence, expiry, or any relation/owner/ref/frontier/epoch mismatch rejects that
operation without a relation mutation; it does not retroactively change M8
admission.

## Deterministic reference state and output

One M8 semantic state contains the admitted identity/evidence and retained
checked-plan set, owner queues/stores, live validation context,
relation/designated stores, local cut/save data, patch lifecycle, and one
occurrence/dependency trace `H`. `H` retains typed authority-validation,
witness-validation/rejection, declared-failure, relation, and designated facts
with their source references, including local raw authority/witness/capability
failure payloads. It is the sole semantic/save carrier. Raw `H` is internal and
not observer-exportable. Observer-safe output is a separate typed derived
projection whose failure rows retain explicit label/redaction but erase every
raw authority/witness/capability payload.

Under the deterministic testing profile, loci run in declared order and each
owner serves at most one FIFO request per turn.  The route records explicit
state/effect/failure/witness/history/cut trace rows and returns a deterministic
replay result for the same frozen runtime profile. Rejected/deferred patch rows
preserve the semantic snapshot while recording their lifecycle result. The one
selected accepted candidate atomically replaces the active checked identity,
admission evidence, and retained checked-plan set at its local cut; it does
not create a second runtime state. A consumer-local fallback retains the
admitted relation label and redaction exactly; in particular, a private relation
cannot become a weaker or unredacted fallback output.

## Boundary

This specification is a bounded reference-runtime contract.  It does not
claim transport, multi-process delivery, M9 auth/verification, final public
interfaces, or C-static/C-runtime conformance.  Official frozen SCN-01..10
conformance remains M10 fresh release-profile evidence.
