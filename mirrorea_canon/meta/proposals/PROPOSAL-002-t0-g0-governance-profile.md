---
id: meta/proposal-002
status: L3-open
maturity: draft
depends_on: [meta/agent-instructions, plan/00-gates, plan/01-phases, plan/02-operating-model, spec/06-conformance, arch/03-toolchain]
summary: G0-EXIT-001 のための T0/G0 phase-governance profile 提案。SCN 適合性、Gate exit、実装状態は動かさない。
open_items: [G0-EXIT-001]
---

# PROPOSAL-002 - T0/G0 Governance Profile

> Decision-request artifact only.
>
> This proposal defines neither an effective profile nor a `mir-conform`
> result. It does not establish G0 exit, T1 entry, an ADR effectivity, or an
> implementation/conformance claim.

## Target and Authority Boundary

The target is `G0-EXIT-001`: `plan/01-phases` requires JSON pass/fail plus
human acceptance for every Phase exit, but `spec/06-conformance` defines only
the SCN-based C-static, C-runtime, and C-distributed levels. T0 is a
non-executable vocabulary-and-decision phase. A document check must not be
misrepresented as SCN conformance or semantic proof.

`plan/00-gates` remains authoritative for G0 criteria. `plan/01-phases`
remains authoritative for implementation state. Only the human owner may make
ADRs effective or approve a Gate exit. This proposal may be adopted, rejected,
or returned for revision through the normal canon process.

## Owner Inputs Recorded

Recorded on 2026-07-15. These are inputs to this proposal, not an effective
profile or G0 exit record.

- `World` and `Game` are user-defined S5 domain/library concepts, not Mir
  primitives.
- Source-level domain handlers are permitted; occurrence, request, and
  publication mechanics remain non-source Core/Trace concerns.
- Authority is grant-lineage-only.
- `.mir` is the source of truth for program meaning; AST, Core IR,
  projected/generated artifacts, and generated code are derived.
- The requested response to G0-EXIT-001 is a T0-specific governance JSON
  profile, not an exception to the Phase JSON rule.
- The owner does not request a separate semantic/historical LAB-demotion audit
  at this checkpoint. A concrete future drift finding may still justify a
  separately scoped audit.

These inputs do **not** yet record collective acceptance of the exact five
G0 ADR documents, the current GLOSSARY baseline, and the present LAB-demotion
evidence as satisfying G0. That is the remaining G0-D1 decision.

## Proposal

Add a narrowly named **T0/G0 phase-governance profile** to the canon. It is a
`mir-conform` profile only in the operational sense that it emits a structured
derived report; it is not a C-static, C-runtime, or C-distributed conformance
level.

The adopted profile would evaluate only the following conditions:

1. an effective canonical owner record expressly accepts the five ADRs as
   effective for G0, the named GLOSSARY baseline as prepared, and the present
   LAB-demotion evidence as complete (G0-D1);
2. the canonical source-hierarchy controls cited by that record remain
   present; and
3. an explicit canonical owner record states whether the separate D4 audit is
   waived or required. If required, the profile cannot pass before the scoped
   audit is completed and accepted.

The profile must report `pass`, `fail`, or `pending`. A G0-D1 deferral yields
`pending` or `fail`; it can never yield `pass`. `pass` means only that the
defined T0 governance checks are satisfied. It does not itself approve G0
exit, make an ADR effective, or change the current Phase. A separate human
acceptance and the canonical G0-exit record remain mandatory.

## Illustrative Derived Output

The following is a non-normative shape only. Field names, schema version, and
input encoding have no force until an adopted canon change defines them.

```json
{
  "profile": {"id": "<effective-profile-id>", "version": "<version>", "hash": "<hash>"},
  "phase": "T0",
  "evaluated_revision": "<canon-and-repository-revision>",
  "owner_records": {"g0_d1": "<canonical-record-id>", "g0_d4": "<canonical-record-id>"},
  "lab_demotion_evidence_revision": "<revision>",
  "artifact_digest": "<digest>",
  "result": "pending",
  "checks": [
    {"id": "g0-substantive-owner-record", "result": "pending"},
    {"id": "g0-source-hierarchy-controls", "result": "pending"},
    {"id": "g0-demotion-audit-scope", "result": "pending"}
  ],
  "non_claims": ["SCN conformance", "proof", "Gate exit", "T1 entry"]
}
```

The result is a LAB-derived artifact with provenance to canon inputs. A valid
result must bind the effective profile definition, evaluated canon/repository
revision, cited canonical owner records, accepted LAB-demotion evidence
revision, and its own digest. The human acceptance and G0-exit record must
reference that exact digest. Unbound, replayed, illustrative, or independently
authored JSON has no Phase- or Gate-exit effect. The result never becomes a
source of program meaning or an authority-bearing decision record.

## Proposed Canon Change Set

If adopted, make the smallest coordinated change set:

1. define the exact T0/G0 profile in a plan-level governance source, and amend
   `plan/01-phases` to name it as the T0 interpretation of the universal JSON
   rule without changing G0 exit criteria;
2. leave `spec/06-conformance` SCN-only, adding at most a cross-reference that
   phase-governance JSON is outside C-static/C-runtime/C-distributed; and
3. amend `architecture/03-toolchain` only for any consequential `mir-conform`
   input/output contract, not as the normative profile definition.

Adoption changes the normative T0 Phase-exit protocol. It therefore requires
the owner-approved canonical decision/ADR and associated ledger or CHANGELOG
updates required by the canon process, followed by `INDEX.json` regeneration
and reference checks. No executable tool is proposed before that adoption.

The adoption transaction must also resolve the T1-moratorium constraint before
any JSON producer exists. It must either identify an already-authorized,
one-off derived artifact that creates no committed helper, evidence lane, or
report series, or explicitly amend the moratorium through the canon process.
Separate package authorization cannot override that constraint.

## Alternatives Considered

1. **Recommended: define the narrow governance profile.** It preserves the
   universal JSON rule while separating governance evidence from conformance.
2. **Amend the Phase rule with a theory-phase exception.** This avoids a new
   profile but creates a special case in the lifecycle rule.
3. **Hold T0 until an authorized existing process appears.** This is safest by
   inaction but leaves an already identified, narrow protocol gap unresolved.

The owner selected alternative 1 at the mechanism-choice level. The exact
profile remains subject to adoption or revision.

## Requested Owner Decisions

1. Adopt, reject, or revise this proposed profile boundary, normative location,
   and moratorium-compatible producer route.
2. Separately answer G0-D1 in exact terms: accept or defer the named five ADRs,
   GLOSSARY baseline, and present LAB-demotion evidence as the three G0
   substantive criteria.
3. After an adopted profile is defined and evaluated, approve or defer G0 exit
   and identify the effective canonical ADR/ledger record (G0-D3).

## Non-effects

This proposal does not:

- edit an ADR, status, maturity, Gate/Phase criterion, or conformance level;
- make the five ADRs effective or accept the G0 substantive evidence;
- create a `mir-conform` executable, helper, result, or new evidence lane;
- claim C-static, C-runtime, C-distributed, proof, runtime, product, or sample
  readiness; or
- approve G0 exit, T1 entry, or any later Gate.
