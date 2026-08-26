# Project status

最終更新: 2026-08-26 23:09 JST

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
→ [~] SYS-2 ST/OW concurrency refinement (active)
→ [ ] SYS-3 per-locus artifact/communication generation (next)
→ [ ] SYS-4 in-process generated dispatch
→ [ ] SYS-5 typed devtools + four-locus toy world
→ [ ] SYS-6 finite I2 assurance/lifecycle closeout
→ [ ] SYS-7 inactive I3 entry contract only
```

ADR-0026 authorizes the bounded SYS-0--SYS-7 program. The list above is an
implementation-program roadmap, not an official Phase acceptance checklist.

## 現在地

| 観点 | 状態 | 根拠 |
| --- | --- | --- |
| active frontier | **SYS-2 active** — ST/OW concurrency, memory, and bounded effect ordering refinement; **SYS-3 is next** | `mirrorea_canon/adr/ADR-0027.md`, `plan/249-mirrorea-i2-systems-foundation-current-roadmap.md` |
| authority | ADR-0026 / PROPOSAL-029 permit evidence-gated SYS-0--SYS-7 work. ADR-0015 / Plan 247 remain closed M0--M10 history | `mirrorea_canon/meta/proposals/PROPOSAL-029-mirrorea-i2-systems-foundation.md`, `mirrorea_canon/adr/ADR-0015.md` |
| official lifecycle | Theory remains **T1**. Broad PHASE-I1 exit, I2 lifecycle entry, and I2 exit are not accepted by program activation or SYS-0/SYS-1 completion | `mirrorea_canon/plan/01-phases.md` |
| accepted baseline | M10 R5 cut `23f5a8130334bf0c8516d51e9dcea38b92f50db1`; static 26/26, runtime 47/47, mismatch/missing 0, anchor true, waiver null | `mirrorea_canon/adr/ADR-0025.md`, `docs/reports/2591-mir-theory-v0-i1plus-milestone-10-conformance-closeout.md` |
| SYS-0 evidence | baseline HEAD/origin `49e6845...`; focused M10 groups 67+2+4+3+5 pass; config validator/9 tests/strict-help pass; final review ACCEPT no P0/P1/P2; integration cut `350e04b4...` pushed with clean remote parity | `docs/reports/2592-mirrorea-i2-systems-foundation-sys0-baseline-goal-alignment.md` |
| SYS-1 evidence | source cut `94e3707c...`; crate-private kernel on ordinary source/generic OwnerEvent; focused 13/13, runtime lib 25/25, M10 source 2/2, CLI 4/4, conformance 67/67, workspace/format/Clippy pass; semantics and code-quality review ACCEPT | `mirrorea_canon/adr/ADR-0027.md`, `docs/reports/2593-mirrorea-i2-systems-foundation-sys1-runtime-kernel-carrier.md` |
| proof/scenarios | Proof ledger and frozen SCN expectations unchanged; bounded M3--M9 evidence retains exact classes | `mirrorea_canon/theory/11-metatheory-ledger.md`, `mirrorea_canon/spec/06-conformance.md` |

SYS-1 separated the semantic runtime kernel from conformance/release/profile/
CLI orchestration for the accepted ordinary source and generic checked-owner
paths. Specialized historical SCN-04/09/10/route-patch runners remain M10
regression-only. SYS-2 consumes that narrow kernel/carrier contract.

## 現在の停止線

Current technical blocker: the SYS-1 kernel seals an immutable M9 authority
snapshot. SYS-2 must map abstract happens-before, linearization, reads-from/
coherence where needed, and revoke/publication/activation/cut visibility to ST
and one-owner-worker execution. Required edge removal must produce bounded
counterexamples; ordinary Surface must not acquire `memory_order_*`.

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

There is **no owner decision required for current SYS-2 work**. The following
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
| proof status | `mirrorea_canon/theory/11-metatheory-ledger.md` |
| runnable evidence dashboard | `samples_progress.md` |

## 更新規約

Derived snapshots do not create Canon, lifecycle, proof, or conformance facts.
At every SYS close, synchronize Plan 249, this view, `progress.md`, `tasks.md`,
and the one milestone report from actual evidence. Change official Phase state
only through its pre-existing criteria and an explicit authorized acceptance
record.
