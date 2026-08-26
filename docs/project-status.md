# Project status

最終更新: 2026-08-27 07:07 JST

**Canon notice:** `mirrorea_canon/` is the normative source for direction,
theory, ADRs, conformance, and process. Everything outside `mirrorea_canon/`
is LAB; canon wins. This document is a LAB derived view.

## この文書の役割

これは人間向けの短い **派生ビュー** である。規範判断は
`mirrorea_canon/`、current execution controlはPlan 249、詳細履歴はPlan 247と
milestone reportsにある。この文書はGate/Phase、OBL、SCN、適合性、実装完了を
決めない。

## 全体の進行チェックリスト

```text
closed M0--M10 finite reference baseline
→ [x] SYS-0 baseline/goal alignment (completed / closed)
→ [x] SYS-1 kernel/conformance separation + internal carrier (completed / closed)
→ [x] SYS-2 ST/OW1 concurrency refinement (completed / closed)
→ [x] SYS-3 per-locus artifact/communication generation (completed / closed)
→ [~] SYS-4 in-process generated dispatch (active)
→ [ ] SYS-5 typed devtools + four-locus toy world (next)
→ [ ] SYS-6 finite I2 assurance/lifecycle closeout
→ [ ] SYS-7 inactive I3 entry contract only
```

ADR-0026 authorizes the bounded SYS-0--SYS-7 program. The list above is an
implementation-program roadmap, not an official Phase acceptance checklist.

## 現在地

| 観点 | 状態 | 根拠 |
| --- | --- | --- |
| active frontier | **SYS-4 active** — accepted per-locus artifactsをindependent in-process endpointsでactual dispatchする; **SYS-5 is next** | `mirrorea_canon/adr/ADR-0029.md`, `plan/249-mirrorea-i2-systems-foundation-current-roadmap.md` |
| authority | ADR-0026 / PROPOSAL-029 permit evidence-gated SYS-0--SYS-7 work. ADR-0015 / Plan 247 remain closed M0--M10 history | `mirrorea_canon/meta/proposals/PROPOSAL-029-mirrorea-i2-systems-foundation.md`, `mirrorea_canon/adr/ADR-0015.md` |
| official lifecycle | Theory remains **T1**. Broad PHASE-I1 exit, I2 lifecycle entry, and I2 exit are not accepted by program activation or SYS-0--SYS-3 completion | `mirrorea_canon/plan/01-phases.md` |
| accepted baseline | M10 R5 cut `23f5a8130334bf0c8516d51e9dcea38b92f50db1`; static 26/26, runtime 47/47, mismatch/missing 0, anchor true, waiver null | `mirrorea_canon/adr/ADR-0025.md`, `docs/reports/2591-mir-theory-v0-i1plus-milestone-10-conformance-closeout.md` |
| SYS-1 evidence | source cut `94e3707c...`; crate-private kernel on ordinary source/generic OwnerEvent; focused 13/13 and preserved M10/workspace validation; semantics/code-quality ACCEPT | `mirrorea_canon/adr/ADR-0027.md`, `docs/reports/2593-mirrorea-i2-systems-foundation-sys1-runtime-kernel-carrier.md` |
| SYS-2 evidence | source cut `920d3fe0...`; selected ST/OW1 result agreement, actual M8 LP/reads-from, same-seam M9 ack-before-publish; 27/27 and four review lanes ACCEPT | `mirrorea_canon/adr/ADR-0028.md`, `docs/reports/2594-mirrorea-i2-systems-foundation-sys2-concurrency-refinement.md` |
| SYS-3 evidence | accepted cut `3013e7fe...`; source-bound exactly-one E-CONSUME consumer/artifact/delivery, 9/13/25/8/27/7/2/67 focused rows, full runtime/workspace, scoped Clippy, and final semantic/code-quality ACCEPT; `ded622fe...` remains partial history | `mirrorea_canon/adr/ADR-0029.md`, `docs/reports/2595-mirrorea-i2-systems-foundation-sys3-per-locus-projection.md` |
| proof/scenarios | Frozen SCN expectations and earlier proof status stay unchanged; OBL-058 is `model-checked-bounded`, OBL-059 is `runtime-monitored`, OBL-060 is `runtime-monitored` for static finite compiler/projector evidence only; no new Lean/general theorem or runtime dispatch claim | `mirrorea_canon/theory/11-metatheory-ledger.md`, `mirrorea_canon/spec/06-conformance.md` |

SYS-3 closed the static-generation milestone. Its bounded non-final clause
`designated consume E.result at C` produces a distinct AST/M6/M7 Core edge,
consumer-only artifact, and `DesignatedResultDelivery(E→C)`; `C` comes only
from source and cannot be invented by topology, schedule, or relation. SYS-3
records the stable source/Core semantic-consumption identity and the static
`ReturnExistingNoNewConsumption` requirement only. It does not claim that
current M8 implements the return: legacy M8 rejects the same delivery id with
`AlreadyConsumed` and may consume a different id, while M10 keeps its accepted
same-delivery rejection. SYS-4 must implement the carrier-side idempotent
return or compatible wrapper and actual first/retry/competing-consumer endpoint
tests. No actual endpoint, runtime occurrence, admission, save/restore, or patch
execution is claimed by SYS-3.

Production relation projection remains the current checked two-anchor shape.
The deeper/shared finite DAG case is test-only source-bound extension pressure,
not production nested-relation semantics or a general theorem.

## 現在の停止線

Current technical blocker: SYS-4 must start the accepted SYS-3 artifacts as
independent locus-local stores/queues/endpoints and actually cross only the
generated carrier boundaries without source reparse, handwritten routes, or
direct cross-locus store access. It must bind runtime occurrences to source/
Core/artifact/edge identities, preserve ST/OW1 selected semantics, fail closed,
support deterministic replay and a local whole-fabric cut/save/restore/patch,
and implement the theory/13 carrier-side idempotent return/wrapper before
calling M8 exactly once. That endpoint behavior cannot be inferred from static
OBL-060 or M8/M10 regression behavior. The direct consumer is SYS-5.

Boundary sources: `mirrorea_canon/spec/12-sys3-per-locus-projection.md`,
`mirrorea_canon/architecture/04-runtime-carriers.md`, and
`plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`.

OPEN-030 is resolved only as the ADR-0027 I2-internal owner/designated-input
contract. Architecture/04 remains L2-working; OPEN-026/027 and full internal
carrier freeze still block broad PHASE-I1 acceptance.

Stop for owner input only if the North Star/guarantees must weaken, domain
vocabulary must become Core, a hidden multi-owner transaction becomes
unavoidable, a public API/ABI/wire must be frozen, real transport must be
selected/implemented now, production/publication or risky data/secret/paid
resource action is required, an irreversible semantic tie remains, or the
parent goal contradicts North Star. Official T1, deferred general OBLs, open
public contracts, later I3+, and unoptimized performance are not stop reasons.

## オーナーの確認・判断待ち

There is **no owner decision required for current SYS-4 implementation**. The following
remain reserved future checkpoints rather than current blockers:

The reserved boundary is defined by
`mirrorea_canon/DESIGN-CONSTITUTION.md` and the owner-direction limits in
`mirrorea_canon/adr/ADR-0026.md`.

| Item | Earliest effect | Current handling |
| --- | --- | --- |
| public API/ABI/wire freeze | external compatibility | keep internal/provisional; stop before irreversible freeze |
| real transport selection/implementation | future I3 program | SYS-7 writes an inactive entry contract only |
| production/publication | external state/product | owner-reserved |
| North Star or safety/privacy weakening | project guarantee | owner-reserved stop |

## 根拠と詳細

| 知りたいこと | 正本またはLAB evidence |
| --- | --- |
| project axis / decision filter | `mirrorea_canon/NORTH-STAR.md`, `mirrorea_canon/DESIGN-CONSTITUTION.md` |
| active authority | `mirrorea_canon/meta/proposals/PROPOSAL-029-mirrorea-i2-systems-foundation.md`, `mirrorea_canon/adr/ADR-0026.md` |
| SYS-1 internal contract | `mirrorea_canon/meta/proposals/PROPOSAL-030-sys1-runtime-kernel-internal-carrier.md`, `mirrorea_canon/adr/ADR-0027.md` |
| SYS-2 internal execution contract | `mirrorea_canon/meta/proposals/PROPOSAL-031-sys2-st-ow1-concurrency-refinement.md`, `mirrorea_canon/adr/ADR-0028.md` |
| SYS-3 projection contract | `mirrorea_canon/meta/proposals/PROPOSAL-032-sys3-checked-core-per-locus-projection.md`, `mirrorea_canon/adr/ADR-0029.md`, `mirrorea_canon/spec/12-sys3-per-locus-projection.md` |
| official lifecycle | `mirrorea_canon/plan/01-phases.md` |
| current roadmap | `plan/249-mirrorea-i2-systems-foundation-current-roadmap.md` |
| M10 closed baseline | `mirrorea_canon/adr/ADR-0025.md`, `plan/247-mir-theory-v0-i1plus-current-roadmap.md`, `docs/reports/2591-mir-theory-v0-i1plus-milestone-10-conformance-closeout.md` |
| SYS-0--SYS-3 accepted evidence / Reports 2592--2595 | `docs/reports/2592-mirrorea-i2-systems-foundation-sys0-baseline-goal-alignment.md` through `docs/reports/2595-mirrorea-i2-systems-foundation-sys3-per-locus-projection.md` |
| proof status | `mirrorea_canon/theory/11-metatheory-ledger.md` |
| runnable evidence dashboard | `samples_progress.md` |

## 更新規約

Derived snapshots do not create Canon, lifecycle, proof, or conformance facts.
At every SYS close, synchronize Plan 249, this view, `progress.md`, `tasks.md`,
and the one milestone report from actual evidence. Change official Phase state
only through its pre-existing criteria and an explicit authorized acceptance
record.
