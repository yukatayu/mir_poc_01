---
id: meta/proposal-002
status: L1-fixed
maturity: draft
depends_on: [adr/ADR-0013, plan/00-gates, plan/01-phases, plan/02-operating-model, spec/06-conformance, arch/03-toolchain]
summary: 採択済み T0/G0 phase-governance profile の design memo。SCN 適合性、Gate exit、実装状態は動かさない。
open_items: []
---

# PROPOSAL-002 - T0/G0 Governance Profile

> Adopted design memo. The effective profile is defined by `plan/01-phases`
> under ADR-0013; this memo is not an exit record or a tool contract.
>
> It does not establish G0 exit, T1 entry, an SCN conformance result, or an
> implementation claim.

## Owner Disposition

Recorded on 2026-07-15 and applied through ADR-0013.

- G0-D1: accepted. The exact five ADRs, GLOSSARY baseline, and present
  LAB-demotion evidence are accepted for G0 at ADR-0013's pinned evidence cut.
- G0-D2: adopted. The T0-specific phase-governance JSON profile is defined in
  `plan/01-phases`.
- Production route: one ephemeral evaluation creates one derived artifact
  recorded by `LAB:plan/155`; no producer implementation, reusable helper,
  schema, CI, Make target, evidence lane, or report series is committed.
- G0-D3: deferred. No G0 exit, T1 entry, or canonical exit record follows.
- G0-D4: waived for this checkpoint; a concrete future drift can reopen a
  separately scoped audit.

## Target and Authority Boundary

The target was `G0-EXIT-001`: `plan/01-phases` requires JSON pass/fail plus
human acceptance for every Phase exit, but `spec/06-conformance` defines only
the SCN-based C-static, C-runtime, and C-distributed levels. T0 is a
non-executable vocabulary-and-decision phase. A document check must not be
misrepresented as SCN conformance or semantic proof. ADR-0013 resolves this
protocol question without applying an exit.

`plan/00-gates` remains authoritative for G0 criteria. `plan/01-phases`
remains authoritative for implementation state. Only the human owner may make
ADRs effective or approve a Gate exit. The adopted profile remains subject to
the normal canon process for later revision.

## Owner Inputs Recorded

The following inputs were recorded on 2026-07-15 and accepted through
ADR-0013. They are not a G0 exit record.

- `World` and `Game` are user-defined S5 domain/library concepts, not Mir
  primitives.
- Source-level domain handlers are permitted; occurrence, request, and
  publication mechanics remain non-source Core/Trace concerns.
- Authority is grant-lineage-only.
- `.mir` is the source of truth for program meaning; AST, Core IR,
  projected/generated artifacts, and generated code are derived.
- The response to G0-EXIT-001 is a T0-specific governance JSON profile, not
  an exception to the Phase JSON rule.
- The owner does not request a separate semantic/historical LAB-demotion audit
  at this checkpoint. A concrete future drift finding may still justify a
  separately scoped audit.

ADR-0013 separately records the exact five G0 ADR documents, current GLOSSARY
baseline, and present LAB-demotion evidence that the owner accepted at its
pinned evidence cut.

## Adopted Profile Boundary

ADR-0013 adds a narrowly named **T0/G0 phase-governance profile** to the
canon. It supplies the T0 meaning of the universal Phase JSON condition; it is
not `arch/03-toolchain`'s SCN-suite `mir-conform` output and is not a
C-static, C-runtime, or C-distributed conformance level.

The profile evaluates only the following conditions:

1. ADR-0013 expressly accepts the five ADRs as effective for G0, the named
   GLOSSARY baseline as prepared, and the present LAB-demotion evidence as
   complete for its checkpoint (G0-D1);
2. the canonical source-hierarchy controls cited by that record remain
   present; and
3. ADR-0013 records that the separate D4 audit is waived at this checkpoint.

The profile reports `pass`, `fail`, or `pending`. `pass` means only that the
defined T0 governance checks are satisfied. It does not itself approve G0
exit, make an ADR effective, or change the current Phase. A separate human
acceptance and the canonical G0-exit record remain mandatory.

## Derived Artifact Boundary

The result is a LAB-derived artifact with provenance to canon inputs. A valid
result binds the effective profile definition, evaluated canon/repository
revision, cited canonical owner record, accepted LAB-demotion evidence revision,
and its own digest. The human acceptance and G0-exit record must reference that
exact digest. Unbound, replayed, illustrative, or independently authored JSON
has no Phase- or Gate-exit effect. The result never becomes a source of program
meaning or an authority-bearing decision record.

The owner-authorized production route is an existing-LAB, one-off ephemeral
evaluation. It creates no producer implementation, `mir-conform` executable,
helper, schema, CI surface, Make target, evidence lane, or report series.
`spec/06-conformance` remains SCN-only, and `arch/03-toolchain` remains
unchanged because no tool contract is added.

## Alternatives Considered

1. **Selected: define the narrow governance profile.** It preserves the
   universal JSON rule while separating governance evidence from conformance.
2. **Amend the Phase rule with a theory-phase exception.** This avoids a new
   profile but creates a special case in the lifecycle rule.
3. **Hold T0 until an authorized existing process appears.** This is safest by
   inaction but leaves an already identified, narrow protocol gap unresolved.

## Remaining Decision

G0-D3 remains deliberately deferred. A later owner decision must approve or
continue to defer G0 exit, and an approval must identify the canonical exit
record and accept the exact evaluated artifact digest.

## Non-effects

This adopted memo and the profile do not:

- create an SCN `mir-conform` executable, helper, or new evidence lane;
- establish C-static, C-runtime, C-distributed, proof, runtime, product, or
  sample readiness; or
- approve G0 exit, T1 entry, or any later Gate.
