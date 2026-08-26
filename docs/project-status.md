# Project status

最終更新: 2026-08-27 01:09 JST

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
→ [~] SYS-3 per-locus artifact/communication generation (active)
→ [ ] SYS-4 in-process generated dispatch (next)
→ [ ] SYS-5 typed devtools + four-locus toy world
→ [ ] SYS-6 finite I2 assurance/lifecycle closeout
→ [ ] SYS-7 inactive I3 entry contract only
```

ADR-0026 authorizes the bounded SYS-0--SYS-7 program. The list above is an
implementation-program roadmap, not an official Phase acceptance checklist.

## 現在地

| 観点 | 状態 | 根拠 |
| --- | --- | --- |
| active frontier | **SYS-3 active** — checked global Coreからdeterministic per-locus artifactsとgenerated plansを導出する; **SYS-4 is next** | `mirrorea_canon/adr/ADR-0028.md`, `plan/249-mirrorea-i2-systems-foundation-current-roadmap.md` |
| authority | ADR-0026 / PROPOSAL-029 permit evidence-gated SYS-0--SYS-7 work. ADR-0015 / Plan 247 remain closed M0--M10 history | `mirrorea_canon/meta/proposals/PROPOSAL-029-mirrorea-i2-systems-foundation.md`, `mirrorea_canon/adr/ADR-0015.md` |
| official lifecycle | Theory remains **T1**. Broad PHASE-I1 exit, I2 lifecycle entry, and I2 exit are not accepted by program activation or SYS-0--SYS-2 completion | `mirrorea_canon/plan/01-phases.md` |
| accepted baseline | M10 R5 cut `23f5a8130334bf0c8516d51e9dcea38b92f50db1`; static 26/26, runtime 47/47, mismatch/missing 0, anchor true, waiver null | `mirrorea_canon/adr/ADR-0025.md`, `docs/reports/2591-mir-theory-v0-i1plus-milestone-10-conformance-closeout.md` |
| SYS-0 evidence | baseline HEAD/origin `49e6845...`; focused M10 groups 67+2+4+3+5 pass; config validator/9 tests/strict-help pass; final review ACCEPT no P0/P1/P2; integration cut `350e04b4...` pushed with clean remote parity | `docs/reports/2592-mirrorea-i2-systems-foundation-sys0-baseline-goal-alignment.md` |
| SYS-1 evidence | source cut `94e3707c...`; crate-private kernel on ordinary source/generic OwnerEvent; focused 13/13, runtime lib 25/25, M10 source 2/2, CLI 4/4, conformance 67/67, workspace/format/Clippy pass; semantics and code-quality review ACCEPT | `mirrorea_canon/adr/ADR-0027.md`, `docs/reports/2593-mirrorea-i2-systems-foundation-sys1-runtime-kernel-carrier.md` |
| SYS-2 evidence | source cut `920d3fe0...`; ST/OW1 selected result agreement, actual M8 LP/reads-from, same-seam M9 ack-before-publish; SYS-2 27/27, SYS-1 13/13, M10 2/4/67, full runtime/format/Clippy/diff pass; four review lanes ACCEPT | `mirrorea_canon/adr/ADR-0028.md`, `docs/reports/2594-mirrorea-i2-systems-foundation-sys2-concurrency-refinement.md` |
| proof/scenarios | Frozen SCN expectations and earlier proof status stay unchanged; OBL-058 is `model-checked-bounded`, OBL-059 is `runtime-monitored`; no Lean/general theorem added | `mirrorea_canon/theory/11-metatheory-ledger.md`, `mirrorea_canon/spec/06-conformance.md` |

SYS-2 closes the selected local execution residual: ST stays deterministic;
OW1 has one worker-exclusive M8 runtime for exactly one combined owner/source-
owner locus; M9 successor generation publishes only after complete
retranslation and owner acknowledgement. This remains a finite internal
contract. SYS-3 consumes its semantic requirements, not concrete Rust mailbox
or worker types.

## 現在の停止線

Current technical blocker: checked global Core does not yet emit deterministic
independently executable `LocusProgram`s and complete generated communication,
effect-handler, observation, persistence, source-map, and diagnostic plans.
SYS-3 must preserve owner/source provenance, explicit failure/effect/authority,
relation/fallback lineage, designated non-reexecution, and SYS-2 backend
requirements without manual interfaces or runtime semantic reconstruction.

Boundary sources: `mirrorea_canon/architecture/04-runtime-carriers.md`,
`mirrorea_canon/theory/04-ordering-and-cuts.md`, and
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

There is **no owner decision required for current SYS-3 work**. The following
remain reserved future checkpoints rather than current blockers:

Boundary sources: `mirrorea_canon/adr/ADR-0026.md` and
`plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`.

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
| SYS-1 internal contract | `mirrorea_canon/meta/proposals/PROPOSAL-030-sys1-runtime-kernel-internal-carrier.md`, `mirrorea_canon/adr/ADR-0027.md`, `mirrorea_canon/architecture/04-runtime-carriers.md` |
| official lifecycle | `mirrorea_canon/plan/01-phases.md` |
| operating rules | `mirrorea_canon/plan/02-operating-model.md` |
| current roadmap | `plan/249-mirrorea-i2-systems-foundation-current-roadmap.md` |
| M10 closed baseline | `mirrorea_canon/adr/ADR-0025.md`, `plan/247-mir-theory-v0-i1plus-current-roadmap.md`, `docs/reports/2591-mir-theory-v0-i1plus-milestone-10-conformance-closeout.md` |
| SYS-0 evidence | `docs/reports/2592-mirrorea-i2-systems-foundation-sys0-baseline-goal-alignment.md` |
| SYS-1 evidence | `docs/reports/2593-mirrorea-i2-systems-foundation-sys1-runtime-kernel-carrier.md` |
| SYS-2 internal execution contract | `mirrorea_canon/meta/proposals/PROPOSAL-031-sys2-st-ow1-concurrency-refinement.md`, `mirrorea_canon/adr/ADR-0028.md` |
| SYS-2 evidence | `docs/reports/2594-mirrorea-i2-systems-foundation-sys2-concurrency-refinement.md` |
| proof status | `mirrorea_canon/theory/11-metatheory-ledger.md` |
| runnable evidence dashboard | `samples_progress.md` |

## 更新規約

Derived snapshots do not create Canon, lifecycle, proof, or conformance facts.
At every SYS close, synchronize Plan 249, this view, `progress.md`, `tasks.md`,
and the one milestone report from actual evidence. Change official Phase state
only through its pre-existing criteria and an explicit authorized acceptance
record.
