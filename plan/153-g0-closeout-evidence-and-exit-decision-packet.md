# plan/153 -- G0 closeout evidence and non-applied exit decision packet

## Purpose and authority boundary

This is LAB repository memory, not a canon decision. It records the evidence
audit requested for the T0/G0 closeout and packages the remaining owner
decisions. `mirrorea_canon/` remains normative. In particular, this document
does not declare G0 exit, T1 entry, an ADR effective, a conformance result, or
any OBL status movement.

The current owner direction for `PROPOSAL-001` is deliberately orthogonal: the
abstract OBL-020 Lean statement may be used as a G1-supporting scope artifact
for later proposal preparation, while full OBL-020 completion and all status,
ledger, proof, artifact-identity, wrapper, Gate, and Phase changes remain
open. It supplies no G0 criterion.

## Canonical exit rule

`mirrorea_canon/plan/00-gates.md` defines G0 as the axis and vocabulary gate.
Its three criteria are the five stated ADRs being effective, a prepared
GLOSSARY, and completed LAB demotion. The same document says that every Gate
exit is established by a human decision plus ADR/ledger update.

`mirrorea_canon/plan/01-phases.md` defines the current implementation state as
T0 and T0 exit as G0 exit. It also states that every Phase exit requires
`mir-conform` JSON pass/fail and human acceptance. `plan/02-operating-model.md`
reserves Gate exit approval and ADR effectiveness to the human owner. These
are controlling constraints, not suggestions that a LAB audit may replace.

## Evidence audit

| G0 criterion | Primary evidence found | Audit conclusion |
|---|---|---|
| ADR-0001, 0002, 0005, 0009, 0012 effective | All five primary ADRs exist with L0-frozen or L1-fixed status. `CHANGELOG.md` v0.1 records their initial establishment. | The substance and authoritative documents are present. Their collective `effective` determination must be explicitly made by the owner; status labels alone do not constitute an applied G0 record. |
| GLOSSARY prepared | `GLOSSARY.md` is L1-fixed, defines CON-001 through CON-040, and includes the LAB migration table. OPEN-001 is confined to future refinement of CON-040. | The intended baseline is present. Whether that is sufficient for the word `prepared` remains part of the owner G0 decision; OPEN-001 must not be silently treated as resolved. |
| LAB demotion complete | `meta/source-hierarchy.md` gives five T0 steps. `CANON.md`, root `README.md`, and `AGENTS.md` contain the source hierarchy notice. `mir_hilight.html` now omits both `world` and `game` from current core keyword/declaration handling and labels both as legacy LAB vocabulary. `samples/clean-near-end/README.md` identifies the suite as LAB evidence. | Direct textual evidence covers the declared notices and highlighter boundary. Step 5 is an ongoing process constraint, so an audit can show its current controls but cannot prove all future additions compliant. The owner must accept the current demotion state as complete for G0. |

The command-level audit also confirmed the canon index, index JSON syntax,
documentation scaffold, hierarchy guard, focused validator tests, `make check`,
and whitespace check. These validate repository consistency. They are not a
substitute for the G0 human decision or a conformance profile.

## Audit limitations found and corrected

The audit corrected a reader-facing LAB vocabulary leak: `game` had remained a
current highlighter keyword/declaration pattern even though ADR-0001 and the
GLOSSARY put game packages in S5 domain/library vocabulary. A RED/GREEN test
now requires both `world` and `game` to remain outside current core handling.
A generic `package` declaration pattern still recognizes the following domain
package name as a definition, so the correction does not degrade viewer
readability or reintroduce `game` as core syntax.

The current LAB task map also now says explicitly that historical
`self-driven` wording is evidence/feasibility history, not AI roadmap or Gate
authority. The P109 audit itself was directly owner-authorized; that bounded
authorization does not create a Gate/Phase decision and is not inferred from
the independent P001 scope answer.

These corrections do not make generic validators semantic proof of LAB
demotion. Path/heading/wording checks remain supplementary only. In particular,
the audit does not yet establish a full historical provenance account for all
main-merged implementation activity during T0's stated implementation freeze.
That historical compliance question is not silently counted as a passed G0
criterion.

## T0 exit protocol resolution

ADR-0013 resolves G0-EXIT-001 without applying an exit. `plan/01-phases`
defines `phase-governance/t0-g0` as the T0 interpretation of the universal
Phase JSON condition. It is not `arch/03-toolchain`'s SCN-suite `mir-conform`
tool output and is not C-static, C-runtime, or C-distributed conformance.

The version-1 route and artifact at `plan/155` are now nonconforming historical
evidence because its bound profile contract had incompatible root-result
vocabulary. The amended ADR-0013 defines version 2 and authorizes exactly one
fresh artifact at `plan/198`. Its result is structured T0 evidence, not an AI
conclusion, docs-validator result, executable tool result, or exit record.
G0-D3 remains independently deferred.

## Owner decision record and applied profile

The owner decisions of 2026-07-15 are now reflected by canon
`adr/ADR-0013`; this LAB packet remains evidence, not a Gate exit record.

1. **G0-D1 accepted:** the exact five ADRs, `root/glossary` baseline, and
   present LAB-demotion evidence are accepted as the G0 substantive criteria at
   ADR-0013's pinned repository evidence cut.
2. **G0-D2 adopted:** the T0-specific `phase-governance/t0-g0` JSON profile is
   defined in canon `plan/01-phases`. It is neither the SCN `mir-conform` tool
   contract nor a C-static/C-runtime/C-distributed result.
3. **Evaluation:** `plan/198` is the one authorized v2 artifact. Its checks
   are `pass` / `fail` / `pass`, so its derived root result is `fail`; it cannot
   support G0-D3 acceptance. The v1 `plan/155` artifact is retained only as
   nonconforming historical evidence.
4. **G0-D3 deferred:** the owner deliberately did not apply G0 exit or T1
   entry. A future approval must separately accept the exact artifact digest and
   identify the canonical exit record.
5. **G0-D4 waived:** no additional semantic/historical LAB-demotion audit is
   requested at this checkpoint. A concrete later drift finding may still
   justify a separately scoped audit.

## Non-claims and reopen point

This packet is not a proof, C-static/C-runtime/C-distributed claim, runnable
sample status change, public-product claim, or G1 preparation promotion. It
does not alter `plan/141` unresolved status slots. The remaining reopen point
is G0-D3 after valid `pass` evidence, or a separately scoped decision about the
concrete fixed-control drift now recorded by `plan/198`.

## Current conclusion

The original audit/acceptance history and the v2 profile are evidence-backed,
but the sole v2 evaluation result is `fail` on fixed control drift. They still
do not prove every semantic aspect of LAB demotion. **G0 exit is not currently
established** because G0-D3 is explicitly deferred and lacks a valid `pass`
artifact. The canonical implementation state consequently remains T0.
