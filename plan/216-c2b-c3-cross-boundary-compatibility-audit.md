# Plan 216: C2-B/C3 Cross-Boundary Compatibility Audit

## Role

This LAB record audits the corrected Plan 215 decision envelope against the
existing canonical theory and planning boundaries. It does not select a C2-B
or C3 candidate, change canon, or claim I1 implementation readiness.

## Authority cut and inputs

The audited repository cut is commit `8b201d0ecc061d698d63f9fc02deb1d2d69fc81c`.
Inputs are LAB evidence unless explicitly identified as canon:

| Input | Pinned digest / identity | Use in this audit |
| --- | --- | --- |
| Plan 215 before this correction | `ea7ad39d4d06437325e83b483c3503145018e4832022d96809c14f3965bd2398` | Original bounded decision envelope |
| `theory/01` | `35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12` | Step, occurrence, ownership, and DAG discipline |
| `theory/02` | `40c49504e86162fb065d0f5850c4039d88d08af30da7d12dc2e073c43a107257` | Row containment and static Diagnostic boundary |
| `theory/03` | `2d703895da4f75bf57848275db6ae03e0abe7d56f62a11ef364af8fe22677641` | Semantic strata and non-hidden carriers |
| `theory/04`, `spec/04`, `spec/05` | `70bde483330d3745a8694b15cd75f447b6610513ae66cb1ad4ec1faed274a264` / `50c23acf01deedbe5bdb78baeba58053e28c940d8202b6d25bfd1f03546fd950` / `25749e3b171659fa59e3de6ff49126e15331ef52cf3ba5337ece4c46e72ca06c` | Admissible cut/load and typed saved-state boundary |
| `theory/05`, P013 | `e06dc5ef0539eb5b87bce71b34d3e8d2ab0638603642e0d9f89581f29d25e6c4` / `4e0ecf7475f20eec85c09d50201d2d2cc29848d480e8382935fe489b43877213` | Request-local M1 versus transport/correlation recovery |
| `theory/06` | `3da20d43a0a87ec8417a4519700777adea141f499e2627f433927ce975a086c8` | Fallback scope and monotone lineage |
| P008, P012, `OPEN-010`, `OPEN-011` | `777a6b2e043ae0313c402c836341bdedf9e12758f480c44fef8391715d34f3dc` / `09ea4d6957c320b4d0647806714a1643101c2022b2893ac76ec7de3bf1db73d5` / current Canon questions | Elaboration, continuation, reply/receipt open boundary |
| Plan 197 | `95c5e4276c04495f97eaad21aa812192e87de2a81be5f48dba6dd4f71fb5bb2d` | Prior cross-layer planning context |
| Oracle broad-boundary review | `19cae3668c56cfbd1fe54ea49e3a67c684b85974536d818ed228e76ed838003f` | Advisory independent review |

## Compatibility result

| Boundary | Required compatibility condition for a future candidate | Plan 215 correction |
| --- | --- | --- |
| `theory/01` | No hidden local read; explicit zero/one step and owner seriality; DAG-safe evolution; no duplicate occurrence claim | D1--D3 must state these facts or leave the candidate open |
| `theory/02` | Effect/error rows contain the operation; malformed or underdeclared programs are static Diagnostic, not runtime Reject | D2 requires typed result/failure rows and branch partition |
| `theory/03` / `spec/04` | M1, validation, request, pending state, result, receipt, provenance, and redacted history have an explicit semantic stratum; no implicit span, core graph, authority, or ownership carrier | D2--D3 now require stratum mapping and permit a new carrier only through the normal decision route |
| `theory/04` / `spec/05` | Save/load is only for an admissible full state, preserving channel/prefix and excluding stale membership, witness, lease, and capability resurrection | D3 now requires a full admissible `SaveObject`, fresh reacquisition, and branch-exclusive load state |
| `theory/05` / P013 | M1 is request-local semantic context, never ambient, correlation-only, transport recovery, or a fact reconstructed from a selected object | D2 separates M1 from M2 and requires authoritative lineage facts |
| `theory/06` | Fallback either remains out of scope or preserves monotone lineage explicitly | D2--D3 must declare fallback scope before a candidate is evaluated |
| P008 | User-friendly surface notation may omit only administration uniquely and correctly elaborated into the complete semantic record | Ergonomics are deferred as model-relative elaboration, not payload/locus/session inference |
| P012 / `OPEN-010` / `OPEN-011` | Delimited continuation constraints, exact reply/receipt carrier, and requester-failure receive occurrence remain open boundaries | A candidate must be parametric or escalate; it cannot silently choose one |
| Runtime model | Conformance is trace-set preservation; no liveness claim or deterministic scheduler claim is introduced | The packet explicitly excludes liveness and schedule selection |

## Corrected candidate envelope

1. A, B, and C are non-exhaustive LAB views, not a ranking and not a proof that
   C requires A or B to fail.
2. A future candidate must supply a semantic-stratum map before a syntax or
   public API claim. If the existing strata cannot carry a required fact,
   creating a carrier is an explicit future Canon decision rather than an
   implementation convenience.
3. M1 is carried by the request or a direct semantic projection from it. No
   payload, locus, span, session, transport correlation, selected object, or
   ambient host state may silently serve as its substitute.
4. A load test must start from the complete admissible cut and test distinct
   success, failure, raw-result, receipt, provenance, and redacted-history
   branches. It must not assume a direct edge, one resume occurrence, or a
   deterministic runtime schedule.
5. Surface ergonomics may later infer administrative spelling only when the
   elaborator can construct all required facts uniquely, checkably, and without
   hidden authority or graph edges. This is deliberately stricter than ordinary
   syntactic sugar.

## Remaining boundary and non-effects

This audit does not settle a concrete carrier, syntax, storage representation,
reply/receipt identity, fallback semantics, continuation model, formal proof,
or implementation contract. It does not promote an L3/L2/L1/L0 decision and
does not change the status of I1. The next research package is a bounded,
non-normative candidate sketch plus a stratum/obligation matrix, subject to
these conditions.
