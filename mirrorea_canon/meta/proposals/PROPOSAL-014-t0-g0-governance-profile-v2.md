---
id: meta/proposal-014
status: L1-fixed
maturity: draft
depends_on: [meta/proposal-002, plan/00-gates, plan/01-phases, meta/source-hierarchy]
summary: T0/G0 phase-governance profile v2 と一回限りの fresh artifact を記録する採択済み design memo。G0-D3、Gate/Phase、実装状態は動かさない。
open_items: []
---

# PROPOSAL-014 - T0/G0 governance profile v2

> Adopted design memo. The effective profile is defined by `plan/01-phases`
> under the 2026-07-28 amendment to ADR-0013. This memo records the exact
> owner disposition; it is not an exit record, a conformance result, or a tool
> contract.

## Owner disposition

Recorded on 2026-07-28 and applied through the amendment to ADR-0013:

> `phase-governance/t0-g0` v2 を作り、success literal は `pass` のみとする。v1
> artifact は nonconforming historical evidence として保持し、v2 に対する fresh
> exact evaluation を一回だけ許可する。この判断は G0-D3 acceptance、G0 exit、
> T1 entry、I1 authorization を含まない。

## Adopted scope

Version 2 replaces version 1 only as the current T0 interpretation of the
phase-governance JSON condition. The stored version-1 JSON at `LAB:plan/155`
is retained byte-for-byte as nonconforming historical evidence; it is not
rewritten, regenerated, renamed, or reused as version-2 evidence.

The accepted G0-D1 repository evidence cut, its ordered hashes, the three
checks, the G0-D4 waiver, and the source-hierarchy control pins are not
reaccepted or rebased by this disposition. The fresh evaluation must therefore
report the derived result from the fixed predicates, including `fail` if a
current control no longer matches its pin. A valid `fail` is distinct from a
malformed or nonconforming artifact.

Exactly one fresh version-2 JSON artifact is authorized at
`LAB:plan/198`, path
`plan/198-t0-g0-governance-profile-v2-evaluation.json`. The version-2 profile
defines its direct-parent and source-binding conditions. No producer,
generator, helper, schema, CI surface, Make target, or new evidence lane is
authorized.

## Alternatives rejected

1. **Reuse the version-1 artifact after a corrigendum.** Rejected because its
   bound source contract is internally inconsistent and its exact digest does
   not repair that inconsistency.
2. **Re-pin current controls to obtain `pass`.** Rejected because this would
   make a new G0 evidence decision beyond the recorded owner disposition.
3. **Defer the repair.** Valid but rejected by the owner decision above; it
   would retain an unresolvable profile-format ambiguity.

## Non-effects

This disposition does not accept G0-D3, exit G0, enter T1, authorize I1, alter
the accepted evidence cut, change the G0-D4 waiver, establish SCN
conformance, change a proof or OBL status, or change implementation or public
readiness. The resulting LAB artifact is evidence only and cannot make an
additional ADR effective.
