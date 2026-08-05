# Project status

最終更新: 2026-08-05 16:34 JST

**Canon notice:** `mirrorea_canon/` is the normative source for direction,
theory, ADRs, conformance, and process. Everything outside `mirrorea_canon/`
is LAB; canon wins. This document is a LAB derived view.

## この文書の役割

これは人間向けの短い **派生ビュー** である。規範判断は
`mirrorea_canon/`、詳細な履歴は `plan/` と `docs/reports/` にある。この文書は
Gate/Phase、OBL、SCN、適合性、実装完了を決めない。

## 全体の進行チェックリスト

```text
M0 bootstrap → M1 Constitution → M2 T0/G0 semantic assertions
→ M3 evaluation/materialization → M4 maintained relation/projection
→ M5 shared model → M6 Surface → M7 checker/elaborator
→ M8 deterministic runtime → M9 auth/verification → M10 closeout
→ owner-defined post-program direction
```

M0--M10 は ADR-0015 の owner-approved bounded program として閉じた。各 milestone の
acceptance/non-effect は個別 report と Canon evidence に従う。

## 現在地

| 観点 | 状態 | 根拠 |
| --- | --- | --- |
| active frontier | **M0--M10 closed**。program 内に次の autonomous semantic milestone はない | `docs/reports/2591-mir-theory-v0-i1plus-milestone-10-conformance-closeout.md` |
| authority | ADR-0015 の bounded program は R5 で完了。次の public / distributed / product direction は owner-defined | `mirrorea_canon/adr/ADR-0015.md` |
| official lifecycle | `T1` remains. M10 does not claim broad PHASE-I1 exit, I2 activation, or public ABI/wire/carrier freeze | `mirrorea_canon/plan/01-phases.md`, `mirrorea_canon/adr/ADR-0025.md` |
| proof / scenarios | Proof ledger unchanged. General OBL-001..025 and OBL-027 remain `intentionally-deferred`; M3--M9 finite evidence keeps its exact recorded classes. SCN-01..10 finite M10 C-static/C-runtime profile is accepted; SCN-11/12 remain pressure scenarios | `mirrorea_canon/theory/11-metatheory-ledger.md`, `mirrorea_canon/spec/11-m10-i1plus-conformance.md` |
| M10 accepted evidence | R5 commit `23f5a8130334bf0c8516d51e9dcea38b92f50db1`, tree `d8a296fac7a94a37da92563d5feeeeaa96dbc682`; output SHA256 reproduced twice `083523518fdae0a111522f49b148c818ca0d5c21b4b7cc4f34dd476f10d172e7`; static 26/26, runtime 47/47, mismatch 0, missing 0, anchor true, waiver null; reviewer ACCEPT no P0/P1/P2 | Report 2591 |

M10 accepted the finite I1+ deterministic reference profile only. It preserves
the same-source path through M6/M7/M8/M9/runtime/projection and exact
correspondence verification, with profile/manifest
`fnv1a64:6a1cfac2a0950323`, verifier `fnv1a64:420308515cf98e18`, source
revision `fnv1a64:7bff6aa952a8ad53`, and execution
`fnv1a64:5b4d58cf1cd20428`.

## 現在の停止線

Source boundary: `mirrorea_canon/adr/ADR-0025.md` and
`plan/247-mir-theory-v0-i1plus-current-roadmap.md`.

- No autonomous package remains inside M0--M10.
- Post-program work requires a new owner direction, especially for OPEN-030 /
  carrier boundary, broad PHASE-I1 exit, public ABI/wire/carrier freeze, I2
  entry, product publication, or deployment.
- M10 does not claim C-distributed, sockets, final public grammar/API/ABI/wire,
  production runtime, general theorem discharge, I2+, or public-product
  completion.

## オーナーの確認・判断待ち

Authority boundary: `mirrorea_canon/adr/ADR-0015.md` and
`mirrorea_canon/adr/ADR-0025.md`.

| 条件 | 影響 | 現在の扱い |
| --- | --- | --- |
| Post-M0--M10 program direction | next roadmap and authority boundary | owner decision required |
| OPEN-030 / carrier boundary | public carrier / ABI / wire readiness | unresolved; do not freeze silently |
| PHASE-I1 exit / I2 activation | lifecycle and next implementation scope | not claimed by M10; owner-defined |
| production deployment / publication | irreversible external contract | owner-reserved |
| North Star / safety/privacy weakening or Core domain promotion | project guarantee | owner-reserved |

## 根拠と詳細

| 知りたいこと | 正本またはLAB evidence |
| --- | --- |
| Constitution/program authority | `mirrorea_canon/meta/proposals/PROPOSAL-018-mir-v0-i1plus-autonomous-execution.md`, `mirrorea_canon/adr/ADR-0015.md` |
| M10 conformance boundary | `mirrorea_canon/adr/ADR-0025.md`, `mirrorea_canon/spec/11-m10-i1plus-conformance.md`, `mirrorea_canon/spec/06-conformance.md` |
| official lifecycle / Gate | `mirrorea_canon/plan/00-gates.md`, `mirrorea_canon/plan/01-phases.md` |
| proof status | `mirrorea_canon/theory/11-metatheory-ledger.md` |
| M10 report | `docs/reports/2591-mir-theory-v0-i1plus-milestone-10-conformance-closeout.md` |
| runnable evidence dashboard | `samples_progress.md` |

## 更新規約

Derived snapshots must not create new Canon or owner decisions. Report 2591 is
the closeout evidence for M10; future roadmap edits should first establish a new
owner-defined post-program direction.
