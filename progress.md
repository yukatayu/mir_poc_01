# progress

最終更新: 2026-08-05 16:34 JST

**Canon notice:** `mirrorea_canon/` is normative. Everything outside
`mirrorea_canon/` is LAB; canon wins. This file is a concise LAB snapshot and
creates no Canon, Gate, Phase, proof, or conformance decision.

## document role

`docs/project-status.md` is the concise human control view. Plan 247 remains
historical/current-program repository memory for ADR-0015, but M0--M10 is now
closed; no new queue is selected in this file.

## project axis

```text
正しい理論に基づき、正しく hot-plug でき、Place をまたいで
実行・通信・検証・可視化できる仮想空間システム
```

Mir, Mirrorea, PrismCascade, and Typed-Effect Wiring Platform stay separable.
World/Game/Avatar remain user or library vocabulary, never Mir Core primitives.

## final ideal

```text
ordinary .mir → parse → static check → elaborate → typed Core/obligations
→ deterministic multi-locus runtime → trace/cut/save-load/patch → projection
→ designated value delivery or consumer-local relation evaluation → diagnostics
```

Communication is derived from checked meaning; authority is not transport;
observation is a typed information effect; patches use a checked activation cut.

## current milestone position

| Axis | Current status | Startability |
| --- | --- | --- |
| Logical specification | M0--M10 closed inside ADR-0015. Official lifecycle remains T1; M10 does not claim broad PHASE-I1 exit | **要仕様確認**: next semantic/program direction is owner-defined |
| User-facing specification | No final public syntax/grammar/API/ABI/wire or product contract is frozen | **要仕様確認**: public surface/carrier scope needs owner direction |
| Implementation / operation | I1+ finite deterministic reference profile accepted at R5; no I2 or production activation | **後段依存**: wait for post-program direction, OPEN-030/carrier boundary, or I2 scope |

Sources: `mirrorea_canon/adr/ADR-0015.md`,
`mirrorea_canon/adr/ADR-0025.md`,
`mirrorea_canon/spec/11-m10-i1plus-conformance.md`, and Report 2591.
The closed execution record is
`plan/247-mir-theory-v0-i1plus-current-roadmap.md`.

M0--M9 closed governance, Constitution, T0/G0 semantic assertions,
evaluation/materialization, maintained relation/projection, shared model,
Surface, checker/elaborator, deterministic runtime, and auth/verification in
their exact finite scopes. M10 then accepted the source-first finite
conformance profile for frozen SCN-01..10: static 26/26, runtime 47/47,
mismatch 0, missing 0, anchor true, waiver null, fresh-clone output SHA256
`083523518fdae0a111522f49b148c818ca0d5c21b4b7cc4f34dd476f10d172e7`
reproduced twice at commit `23f5a8130334bf0c8516d51e9dcea38b92f50db1`.

## milestone map

| Milestone | Aim | Position | Evidence |
| --- | --- | --- | --- |
| M0 | governance, agent config, one roadmap | closed | Report 2581 / push parity |
| M1 | concise Constitution | closed | `root/design-constitution`, ADR-0016, Report 2582 |
| M2 | semantic-assertion T0/G0 closeout | closed; T1 entry accepted | `plan/248` reproduced pass / ADR-0017 |
| M3 | evaluation/materialization calculus | closed | ADR-0018/theory-13, finite Lean/Rust evidence |
| M4 | maintained relation/late projection | closed | ADR-0019/theory-14/SCN-12, finite Lean/Rust evidence |
| M5 | shared model/metatheory | closed | ADR-0020/theory-15, exact OBL-040..047 evidence |
| M6 | Surface | closed | ADR-0021/spec-01--04, OBL-048 evidence |
| M7 | checker/elaborator | closed | Report 2588 / OBL-049; fixture matrix is not public conformance |
| M8 | deterministic runtime | closed | OBL-050--056 Lean evidence; OBL-057 runtime-monitored |
| M9 | auth/verification | closed | OBL-026 exact Lean proof; OBL-028 bounded model |
| M10 | conformance/closeout | accepted / closed | R5 static 26/26, runtime 47/47, reviewer ACCEPT no P0/P1/P2 |

## line snapshots

### Product Alpha line

Historical runnable LAB evidence only; it does not establish final product API
or public distribution.

### Operational Suite line

Historical bounded operational roots remain evidence. Their commands and
classification are unchanged by M10 closeout.

### Mir Language line

M6/M7 provide the bounded v0 source-to-Core route used by the accepted M10
reference profile. This is not a final public grammar or diagnostic ABI.

### PoseGraph line

Existing pose evidence is not Mir Core. Renderer/IK/LOD behavior remains outside
M10.

### Projection/Backend line

Projection is observer-safe typed evidence in the finite M10 profile; provider
and backend public contracts remain later boundaries.

### Engine/Provider line

External providers remain adapters; names, sessions, packages, and transport do
not grant authority.

## validation floor

| Changed layer | Required command family |
| --- | --- |
| Canon metadata | `cd mirrorea_canon && python3 meta/build-index.py --check` |
| hierarchy/docs | `python3 scripts/check_source_hierarchy.py`, `python3 scripts/validate_docs.py` |
| M10 runtime/conformance | `cargo test -p mir-runtime --test m10_conformance -- --nocapture` and related M10/runtime/workspace suites |
| Lean inventory | seven M3--M9 `lean --trust=0` foundation checks plus generated axiom audit |
| diff / secret guard | `git diff --check` and validator secret scan |

## non-claims

No broad PHASE-I1 exit, I2 activation, C-distributed, socket/transport delivery,
final grammar/API/ABI/wire, public carrier freeze, production deployment,
public-product completion, or general OBL discharge is claimed.

## decisions taken

| Decision | Status |
| --- | --- |
| Accept R5 finite I1+ deterministic reference profile | done |
| Treat R1/R2/R3/R4 as rejected/corrected history | done |
| Keep proof ledger unchanged; M10 adds no general theorem | done |
| Treat post-M0--M10 direction as owner-defined | done |

## user decision items vs research-discovery items

| Kind | Item | Current handling |
| --- | --- | --- |
| Owner decision | post-program roadmap and authority boundary | required before new autonomous semantic milestone |
| Owner decision | OPEN-030 / carrier boundary and public ABI/wire/carrier freeze | unresolved; do not infer from M10 |
| Owner decision | broad PHASE-I1 exit or I2 activation | not claimed; requires explicit direction |
| Research discovery | future carrier/theorem/runtime decomposition inside a new program | wait for owner-defined direct consumer |

## macro phase map

| Macro | Focus | Current position | Weight | Self-drive |
| --- | --- | --- | --- | --- |
| 0 | governance/repository memory | M0--M10 closeout snapshots synchronized | medium | maintenance only |
| 1 | semantics/shared model | finite v0/I1+ line accepted through M10 | heavy | no new line selected |
| 2 | parser-free evidence | historical LAB evidence retained | medium | maintenance only |
| 3 | source/checker/runtime | finite I1+ reference profile accepted | heavy | post-program direction required |
| 4 | executable samples | historical active roots retained | medium | maintenance only |
| 5 | theorem/model-check | finite M3--M9 proof/model evidence retained; ledger unchanged | heavy | post-program direction required |
| 6 | distributed fabric | outside M0--M10 | heavy | owner-defined future |
| 7 | toolchain/backend | bounded reference support accepted; public interface open | heavy | owner-defined future |
| 8 | applications | domain consumers remain LAB | heavy | deferred |

## feature maturity rows

| Feature | Evidence status | Remaining gate | Startability |
| --- | --- | --- | --- |
| multi-node/fabric | historical LAB only | I2/I3 owner direction | 要仕様確認 |
| contracts/theorem/model-check | finite M3--M9 evidence retained; proof ledger unchanged | post-program proof policy | 要仕様確認 |
| dynamic attach/detach/DAG evolution | finite M8/M9/M10 evidence | public/runtime widening decision | 後段依存 |
| atomic_cut/ordering | finite runtime/profile evidence only | higher-level ordering/memory-order family | 要仕様確認 |
| executable sample corpus | runnable historical roots plus M10 conformance command | public sample/catalog decision | 着手可能 for maintenance only |
| Mirrorea | separate fabric layer | later I2+ | 要仕様確認 |
| PrismCascade | separate performance kernel | not I1+ Core | deferred |
| Typed-Effect platform | separate adapter boundary | public platform contract | 要仕様確認 |
| domain applications | LAB consumers | no Core promotion | deferred |

## recent log

- 2026-08-05 16:34 JST: closeout planner review corrected two stale
  owner-boundary/current-frontier references across Plan 247, reader entry
  points, and `AGENTS.md`; final re-review ACCEPT found no remaining P0/P1/P2.
- 2026-08-05 15:53 JST: M10 accepted/closed at R5 commit
  `23f5a8130334bf0c8516d51e9dcea38b92f50db1`: fresh same-source finite I1+
  reference conformance passed static 26/26 and runtime 47/47 with mismatch 0,
  missing 0, anchor true, waiver null; reviewer ACCEPT had no P0/P1/P2. The
  M0--M10 program is closed; broad PHASE-I1 exit, I2 activation, public
  ABI/wire/carrier freeze, production, and general theorem discharge remain
  unclaimed pending owner direction.
