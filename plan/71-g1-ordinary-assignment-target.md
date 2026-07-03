# plan/71 - G1 ordinary assignment target draft

## Purpose

This file is LAB repository memory. It drafts the next safe G1 target for
ordinary Surface simple-assignment elaboration, using `mirrorea_canon/` as the
normative source. In this file, "ordinary assignment" means the simple
assignment form before compound read-modify-write assignment.

This file does not edit canon, does not discharge any theorem, and does not
claim G0/G1/T1 exit.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB evidence / history: legacy `specs/`, `plan/`, samples, helpers, and
  reports outside `mirrorea_canon/`
- Advisory review input: sub-agent findings and Oracle consults, after local
  source-hierarchy review

When this document cites legacy files, read them as `LAB:` evidence unless the
same point is mirrored into canon.

## Canon anchors

| Anchor | Reading for this target |
|---|---|
| `mirrorea_canon/plan/00-gates.md` | G1 is ordinary assignment. Exit requires the G1 row's proof/statement obligations; this draft does not satisfy the gate. |
| `mirrorea_canon/plan/01-phases.md` | Current position remains T0/G0 rebaseline. T1 is later and not claimed. |
| `mirrorea_canon/theory/01-mircore-v0.md` | Read/write rules distinguish owner-local write, owner-directed request, read dependency, cross-locus observe/read request, and locus-block non-authority. |
| `mirrorea_canon/theory/02-types-effects-failures.md` | Generated effects and generated failures must be explicit and row-contained. |
| `mirrorea_canon/theory/03-elaboration.md` | BND-001 and THM-001 are the core target shape for assignment elaboration. |
| `mirrorea_canon/theory/11-metatheory-ledger.md` | OBL-001, OBL-020, and OBL-021 are the required G1 exit obligations; OBL-002 is proof work, and OBL-004 is a corollary target. |
| `mirrorea_canon/spec/02-surface-grammar.md` | The Surface assignment syntax and place/locus block syntax are the source-side reference. |
| `mirrorea_canon/spec/03-static-semantics.md` | Cross-locus access and failure-row containment must be checked statically where the canon says so. |
| `mirrorea_canon/spec/04-core-ir.md` | Core IR shape is L2-working; do not freeze JSON field names from this draft. |
| `mirrorea_canon/spec/06-conformance.md` | SCN-01 and SCN-02 define the conformance reading to preserve. |
| `mirrorea_canon/scenarios/SCN-01-sugoroku-roll.md` | Ordinary Sugoroku roll assignment must expose owner-directed write, dependency, publish, spans, and authority obligations. |
| `mirrorea_canon/scenarios/SCN-02-attack.md` | Attack assignment must expose cross-locus read/write, failure containment, and no ambient authority from nested locus blocks. |
| `mirrorea_canon/architecture/02-boundary-contracts.md` | BND-001 Surface to Core elaboration boundary is the immediate boundary. Runtime, transport, projection, and devtools boundaries are later. |

## Target statement

Working G1 target:

> For every successful Surface assignment elaboration under the canon unified
> judgment, each generated Core write is either owner-local or an explicit
> owner-directed request; every generated cross-locus consequence is represented
> in the explicit request / publish / observe / dependency vocabulary required
> by canon; every generated failure is contained in the declared failure row;
> every required authority/capability/witness obligation is represented in the
> context or obligation set; source spans map generated consequences back to the
> assignment; and elaboration is deterministic for the fixed input judgment.

This is intentionally narrower than broad "Surface Mir program soundness." It
targets simple assignment elaboration, not whole-program runtime behavior.
Compound assignment is a read-plus-write case and should be handled by a
separate lemma or extension after the simple-assignment read and write
obligations are explicit.

## Case split

| Case | Target reading | Must not claim |
|---|---|---|
| Owner-local write | A write to state owned by the current locus is a local write occurrence. Visible fields may generate publication consequences when the canon permits it. | Do not treat local write success as evidence for remote write, transport, or runtime dispatch. |
| Cross-locus write | A write to another owner is an explicit owner-directed request with authority/capability/witness obligations and generated failures. | No direct remote store and no authority from syntax, role name, key, transport, provider, or package artifact alone. |
| Local read | A read creates dependency information where audited/cross-cut relevant. | No occurrence is created merely by a local read. |
| Cross-locus read | A read of another locus is an observe/read-request consequence with failure containment. | OPEN-014 means the exact materialization/optimization policy is not frozen here. |
| Visible publish / observe | Visibility consequences must be explicit enough for conformance and diagnostics. | No runtime dispatch, queue delivery, transport completion, LAB `MessageEnvelope` promotion, or final telemetry ABI claim. |
| Nested locus block | A nested block checks against that locus, but does not switch ambient authority from the caller. | No `S { ... }` ambient authority grant. |
| Underdeclared failure | If elaboration generates a failure absent from the declared row, the assignment is rejected or otherwise fails the static boundary required by canon. | Do not collapse static errors into a generic runtime reject. |

## Proof-boundary split

| Work item | Status in this draft |
|---|---|
| OBL-001: Lean statement of THM-001 | Targeted next; statement only, no proof discharge. |
| OBL-020: assignment case WF preservation | Required for G1 exit; this draft only identifies the role. |
| OBL-021: assignment determinism | Required for G1 exit; this draft only identifies the role. |
| OBL-002: proof of THM-001 | Later proof work; not claimed here. |
| OBL-004: no undeclared communication corollary | Later corollary once THM-001 and failure/effect containment are ready. |
| OBL-003: decidability / diagnosability | Related support obligation; not part of this draft's close claim. |
| THM-004 / OBL-015 / OBL-016 | Authority delegation theorem family; keep separate from G1 ordinary assignment. |
| THM-005 / OBL-017 / OBL-018 | Observation/privacy theorem family; keep separate from G1 ordinary assignment. |

## Scenario mapping

| Scenario | G1 target contribution | Remaining boundary |
|---|---|---|
| SCN-01 Sugoroku roll | The assignment to player position should elaborate as an owner-directed write request when issued from a non-owner locus, record dependencies, preserve spans, and expose publication where visible. | Runtime scheduling, transport delivery, and final public sample/API status remain outside G1. |
| SCN-02 attack | The assignment to target HP should expose read dependencies, cross-locus write request, failure-row containment, and non-ambient nested-locus authority. | Runtime stale-membership behavior and distributed execution remain outside the static elaboration target. |

## LAB evidence to cite carefully

Useful LAB evidence:

- `LAB:specs/39-surface-mir-placement-elaboration.md`
- `LAB:plan/64-surface-mir-placement-roadmap.md`
- `LAB:plan/69-consultation-synthesis-and-management-roadmap.md`
- `LAB:plan/70-lab-to-canon-reconciliation-ledger.md`
- `LAB:samples/full-system-v1-surface/elaboration/README.md`
- `LAB:scripts/surface_mir_samples.py`
- `LAB:crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `LAB:crates/mir-semantics/tests/surface_to_core_elaboration.rs`

Read these as runnable/sample evidence only. They do not prove THM-001, do not
establish G1 exit, and do not freeze final Core JSON, grammar, runtime,
transport, devtools, or public API.

In this evidence class, LAB `MessageEnvelope` rows are helper/sample evidence
for explicit communication consequences. They are not part of this target's
canon vocabulary unless a later canon process promotes equivalent wording.

## Non-claims

- No G0 exit.
- No G1 exit.
- No T1 transition.
- No theorem discharge.
- No Lean proof completion.
- No final grammar or public API freeze.
- No final Core IR JSON field freeze.
- No runtime MessageEnvelope dispatch.
- No C-runtime or C-distributed conformance claim.
- No production identity, authentication, authorization, membership, capability,
  witness, transport, devtools, telemetry, or hot-plug completion.
- No promotion of `World`, `Room`, `Avatar`, `Game`, role names, keys,
  provider names, or package artifacts into Mir core primitives.

## Open questions

- OPEN-014: what exact materialization policy should transparent cross-locus
  reads use before optimization?
- What is the smallest Lean statement for OBL-001 that preserves THM-001
  without overfitting LAB helper field names?
- Which SCN-01/SCN-02 rows need line-level LAB trace before OBL-001 can be
  written confidently?
- Should canon receive a short human-approved mental-model clarification for
  "ordinary assignment" before the Lean statement package, or is the current
  canon text sufficient?

## Next safe packages

1. G1 SCN-01/SCN-02 static trace drilldown.
2. G1 OBL-001 Lean statement inventory, statement only.
3. LAB claim-family line-level drilldown for assignment-related rows.
4. Canon mental-model clarification proposal if the drilldown reveals an
   actual wording gap.
