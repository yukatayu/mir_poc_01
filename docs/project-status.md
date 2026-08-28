# Project status

最終更新: 2026-08-28 14:09 JST

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
→ [x] SYS-4 in-process generated dispatch (completed / closed)
→ [x] SYS-5 typed devtools + four-locus toy world (completed / closed)
→ [~] SYS-6 finite I2 assurance/lifecycle closeout (active)
→ [ ] SYS-7 inactive I3 entry contract only (next)
```

ADR-0026 authorizes the bounded SYS-0--SYS-7 program. The list above is an
implementation-program roadmap, not an official Phase acceptance checklist.

## 現在地

| 観点 | 状態 | 根拠 |
| --- | --- | --- |
| active frontier | **SYS-6 active** — accepted SYS-3--SYS-5 cutsをordinary-source-firstのfinite I2 assurance/conformance profileとして独立に検査する; **SYS-7 is next** | `mirrorea_canon/adr/ADR-0031.md`, `plan/249-mirrorea-i2-systems-foundation-current-roadmap.md` |
| authority | ADR-0026 / PROPOSAL-029 permit evidence-gated SYS-0--SYS-7 work. ADR-0015 / Plan 247 remain closed M0--M10 history | `mirrorea_canon/meta/proposals/PROPOSAL-029-mirrorea-i2-systems-foundation.md`, `mirrorea_canon/adr/ADR-0015.md` |
| official lifecycle | Theory remains **T1**. Broad PHASE-I1 exit, I2 lifecycle entry, and I2 exit are not accepted by program activation or SYS-0--SYS-5 completion | `mirrorea_canon/plan/01-phases.md` |
| accepted baseline | M10 R5 cut `23f5a8130334bf0c8516d51e9dcea38b92f50db1`; static 26/26, runtime 47/47, mismatch/missing 0, anchor true, waiver null | `mirrorea_canon/adr/ADR-0025.md`, `docs/reports/2591-mir-theory-v0-i1plus-milestone-10-conformance-closeout.md` |
| SYS-1 evidence | source cut `94e3707c...`; crate-private kernel on ordinary source/generic OwnerEvent; focused 13/13 and preserved M10/workspace validation; semantics/code-quality ACCEPT | `mirrorea_canon/adr/ADR-0027.md`, `docs/reports/2593-mirrorea-i2-systems-foundation-sys1-runtime-kernel-carrier.md` |
| SYS-2 evidence | source cut `920d3fe0...`; selected ST/OW1 result agreement, actual M8 LP/reads-from, same-seam M9 ack-before-publish; 27/27 and four review lanes ACCEPT | `mirrorea_canon/adr/ADR-0028.md`, `docs/reports/2594-mirrorea-i2-systems-foundation-sys2-concurrency-refinement.md` |
| SYS-3 evidence | accepted cut `3013e7fe...`; source-bound exactly-one E-CONSUME consumer/artifact/delivery, 9/13/25/8/27/7/2/67 focused rows, full runtime/workspace, scoped Clippy, and final semantic/code-quality ACCEPT; `ded622fe...` remains partial history | `mirrorea_canon/adr/ADR-0029.md`, `docs/reports/2595-mirrorea-i2-systems-foundation-sys3-per-locus-projection.md` |
| SYS-4 evidence | accepted cut `22196f93...`; generated-plan-only staged endpoints, locus-partitioned ST and eligible OW1 correspondence, one-consume designated retry, typed fail-closed observer/fault paths, ST whole-fabric cut/restore, bounded checked patch; focused 99/99, runtime 179/179, M10 2/4/67 | `mirrorea_canon/adr/ADR-0030.md`, `mirrorea_canon/spec/13-sys4-in-process-generated-dispatch.md` |
| SYS-5 evidence | accepted cut `53a21e64...`; ordinary-source four-locus local toy, actual generated dispatch, source-derived leave/fallback/fresh reacquire, observer-safe joined causal view, ST save/restore, accepted/rejected patch, revocation, and optional verification; focused 10/27/28/8/17/12/3/4, `mir-runtime --all-targets`, M10 2/4/67, and three independent reviews ACCEPT | `mirrorea_canon/adr/ADR-0031.md`, `mirrorea_canon/spec/14-sys5-local-toy-devtools.md`, `docs/reports/2597-mirrorea-i2-systems-foundation-sys5-minimal-typed-devtools-local-toy.md` |
| proof/scenarios | Frozen SCN expectations and earlier proof status stay unchanged; OBL-058 is `model-checked-bounded`, OBL-059--062 are bounded `runtime-monitored`; no new Lean/general theorem | `mirrorea_canon/theory/11-metatheory-ledger.md`, `mirrorea_canon/spec/06-conformance.md` |

SYS-5 now supplies the headless user-visible direct consumer of the accepted
SYS-3/SYS-4 projector and endpoint runtime. The provisional internal
`project-loci`, `run-local`, and `inspect` commands operate on the ordinary
source at `samples/clean-near-end/mirrorea-i2-local-toy/main.mir`. They expose
four locus programs, generated communication, and one deterministic joined
report from source span/Core/artifact through request, receive, serve/failure,
owner/relation/designated state, cut/restore, and patch lifecycle. No manual
evidence join, fixture-name plan selection, expected-result lookup, or direct
cross-locus store mutation is accepted.

The actual local path includes owner-side attack RMW; designated publication
and named consume; B-owned relation with explicit A-primary/B-fallback anchor
loci; A leave and duplicate-leave rejection; semantic fallback; a ViewerC
presentation-only gap; and fresh membership/capability/witness reacquire. It
also includes ST save/restore, one accepted designated patch, one rejected
owner-RMW patch with no semantic mutation, capability revocation/failure, and
one optional verification example. Leave/fallback/fresh changes use a bounded
clone-prepared ST failure-atomic candidate, and post-leave cut/restore retains
the exact retired lineage. Observer output excludes raw credential, capability
secret, witness payload, private values, and raw M8/M9 identity.

This is still a finite internal profile. Production relation projection remains
the current checked two-anchor shape; explicit anchor-locus syntax is bounded
and non-final. The deeper/shared finite DAG case remains test-only extension
pressure. SYS-5 does not claim OW1 whole-workflow cut/patch, patch/reacquire
commutation, durable or distributed persistence, public grammar/API/ABI/wire,
browser/View product, a general theorem, or I2 conformance/lifecycle acceptance.
Reopen SYS-5 only for an accepted falsifier: inferred anchor placement,
caller-minted lifecycle authority, M8 mutation before M9 retirement, missing
exact leave→fresh join, partial failed-candidate mutation, invalid post-leave
restore, invented causal join/secret leak, filename/expected-result semantics,
or inability of SYS-6 to consume the finite rows conservatively.

## 現在の停止線

Current technical blocker: SYS-6 must convert the accepted SYS-3--SYS-5 cuts
into one finite source-first I2 conformance profile and provisional
`conform-i2` report. Rows must independently detect missing generated
communication, owner movement, direct remote-store access, source-free
authority/state minting, ST/selected-OW semantic disagreement, relation or
fallback drift, designated re-execution, stale save/patch mutation, and unsafe
observation. Each row must name its exact evidence class, source/implementation
cut, replay command, non-claim, and failure reason. Conformance orchestration
must consume the runtime/projector evidence rather than control its meaning.

Boundary sources: `mirrorea_canon/adr/ADR-0031.md`,
`mirrorea_canon/spec/14-sys5-local-toy-devtools.md`,
`mirrorea_canon/adr/ADR-0030.md`,
`mirrorea_canon/spec/13-sys4-in-process-generated-dispatch.md`,
`mirrorea_canon/spec/12-sys3-per-locus-projection.md`,
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

There is **no owner decision required for current SYS-6 implementation**. The
following remain reserved future checkpoints rather than current blockers:

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
| SYS-4 generated-dispatch acceptance | `mirrorea_canon/meta/proposals/PROPOSAL-033-sys4-in-process-generated-dispatch.md`, `mirrorea_canon/adr/ADR-0030.md`, `mirrorea_canon/spec/13-sys4-in-process-generated-dispatch.md`; cut `22196f93...` |
| SYS-5 local toy/devtools acceptance | `mirrorea_canon/meta/proposals/PROPOSAL-034-sys5-local-toy-devtools.md`, `mirrorea_canon/adr/ADR-0031.md`, `mirrorea_canon/spec/14-sys5-local-toy-devtools.md`; cut `53a21e64...` |
| official lifecycle | `mirrorea_canon/plan/01-phases.md` |
| current roadmap | `plan/249-mirrorea-i2-systems-foundation-current-roadmap.md` |
| M10 closed baseline | `mirrorea_canon/adr/ADR-0025.md`, `plan/247-mir-theory-v0-i1plus-current-roadmap.md`, `docs/reports/2591-mir-theory-v0-i1plus-milestone-10-conformance-closeout.md` |
| SYS-0--SYS-5 milestone reports | Reports 2592--2597; SYS-5 is fixed by PROPOSAL-034 / ADR-0031 / spec/14 at cut `53a21e64...` and Report 2597 is its single closeout evidence mirror |
| proof status | `mirrorea_canon/theory/11-metatheory-ledger.md` |
| runnable evidence dashboard | `samples_progress.md` |

## 更新規約

Derived snapshots do not create Canon, lifecycle, proof, or conformance facts.
At every SYS close, synchronize Plan 249, this view, `progress.md`, `tasks.md`,
and the one milestone report from actual evidence. Change official Phase state
only through its pre-existing criteria and an explicit authorized acceptance
record.
