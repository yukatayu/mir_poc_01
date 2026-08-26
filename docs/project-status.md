# Project status

最終更新: 2026-08-26 19:31 JST

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
→ [~] SYS-0 baseline/goal alignment (closing review/integration)
→ [ ] SYS-1 kernel/conformance separation + internal carrier (next)
→ [ ] SYS-2 ST/OW concurrency refinement
→ [ ] SYS-3 per-locus artifact/communication generation
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
| active frontier | **SYS-0 closing** — correction review, integration commit, push, and remote parity remain; **SYS-1 is next, not active** | `mirrorea_canon/adr/ADR-0026.md`, `plan/249-mirrorea-i2-systems-foundation-current-roadmap.md` |
| authority | ADR-0026 / PROPOSAL-029 permit evidence-gated SYS-0--SYS-7 work. ADR-0015 / Plan 247 remain closed M0--M10 history | `mirrorea_canon/meta/proposals/PROPOSAL-029-mirrorea-i2-systems-foundation.md`, `mirrorea_canon/adr/ADR-0015.md` |
| official lifecycle | Theory remains **T1**. Broad PHASE-I1 exit, I2 lifecycle entry, and I2 exit are not accepted by program activation/SYS-0 | `mirrorea_canon/plan/01-phases.md` |
| accepted baseline | M10 R5 cut `23f5a8130334bf0c8516d51e9dcea38b92f50db1`; static 26/26, runtime 47/47, mismatch/missing 0, anchor true, waiver null | `mirrorea_canon/adr/ADR-0025.md`, `docs/reports/2591-mir-theory-v0-i1plus-milestone-10-conformance-closeout.md` |
| SYS-0 evidence | baseline HEAD/origin `49e6845...`; focused M10 groups 67+2+4+3+5 pass; config validator/9 tests/strict-help pass; prechange review GO; first close review REJECT without P0 and correction cycle active | `docs/reports/2592-mirrorea-i2-systems-foundation-sys0-baseline-goal-alignment.md` |
| proof/scenarios | Proof ledger and frozen SCN expectations unchanged; bounded M3--M9 evidence retains exact classes | `mirrorea_canon/theory/11-metatheory-ledger.md`, `mirrorea_canon/spec/06-conformance.md` |

The active work separates semantic/runtime kernel, projection/compiler,
dispatch fabric, conformance evidence generator/verifier, release/profile
orchestration, and CLI facade. It does not extend the M10 release facade as the
new runtime architecture.

## 現在の停止線

Current blocker: close the first-review corrections, repeat independent review,
then integrate/commit/push and verify remote parity. Only then does SYS-1 become
active.

Next technical blocker: `mirrorea_canon/architecture/04-runtime-carriers.md`
OPEN-030 and the current M10 conformance facade's mixed dependency boundary.

After SYS-0 closes, SYS-1 may autonomously define the smallest typed **internal** carrier and
kernel seam, with positive/negative tests and review. It must keep receipt
non-authoritative, source/Core provenance explicit, effect/failure/redaction
typed, and conformance dependent on the kernel rather than the reverse.

Stop for owner input only if the North Star/guarantees must weaken, domain
vocabulary must become Core, a hidden multi-owner transaction becomes
unavoidable, a public API/ABI/wire must be frozen, real transport must be
selected/implemented now, production/publication or risky data/secret/paid
resource action is required, an irreversible semantic tie remains, or the
parent goal contradicts North Star. Official T1, deferred general OBLs, open
public contracts, later I3+, and unoptimized performance are not stop reasons.

## オーナーの確認・判断待ち

There is **no owner decision required for current SYS-0 correction/close or
the next SYS-1 work**. The following
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
| official lifecycle | `mirrorea_canon/plan/01-phases.md` |
| operating rules | `mirrorea_canon/plan/02-operating-model.md` |
| current roadmap | `plan/249-mirrorea-i2-systems-foundation-current-roadmap.md` |
| M10 closed baseline | `mirrorea_canon/adr/ADR-0025.md`, `plan/247-mir-theory-v0-i1plus-current-roadmap.md`, `docs/reports/2591-mir-theory-v0-i1plus-milestone-10-conformance-closeout.md` |
| SYS-0 evidence | `docs/reports/2592-mirrorea-i2-systems-foundation-sys0-baseline-goal-alignment.md` |
| proof status | `mirrorea_canon/theory/11-metatheory-ledger.md` |
| runnable evidence dashboard | `samples_progress.md` |

## 更新規約

Derived snapshots do not create Canon, lifecycle, proof, or conformance facts.
At every SYS close, synchronize Plan 249, this view, `progress.md`, `tasks.md`,
and the one milestone report from actual evidence. Change official Phase state
only through its pre-existing criteria and an explicit authorized acceptance
record.
