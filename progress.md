# progress

最終更新: 2026-08-04 23:26 JST

**Canon notice:** `mirrorea_canon/` is normative. Everything outside
`mirrorea_canon/` is LAB; canon wins. This file is a concise LAB snapshot and
creates no Canon, Gate, Phase, proof, or conformance decision.

## document role

Current execution is defined only by
`plan/247-mir-theory-v0-i1plus-current-roadmap.md`. `docs/project-status.md`
is the concise human control view; other `plan/` files and reports are
repository memory/evidence, not parallel queues.

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
| Logical specification | M0--M8 are evidence-complete; official lifecycle is T1; M9 typed auth/verification extension is active | **着手可能**: transform the M8 contract without creating authority or redefining base semantics |
| User-facing specification | M8 source-first runtime route is closed; M9 adds no final public syntax/grammar/ABI | **後段依存**: M9 then M10 |
| Implementation / operation | M8 finite deterministic runtime evidence is closed; M9 owns explicit contract update, activation, and provenance/invalidation | **着手可能**: one M8-admitted-program extension route |

Sources: `mirrorea_canon/adr/ADR-0015.md`,
`mirrorea_canon/plan/01-phases.md`, and
`plan/247-mir-theory-v0-i1plus-current-roadmap.md`.

M1 adopts `root/design-constitution` and ADR-0016, separating authority origin
from evaluation site, semantic from presentation fallback, and a pre-M6 grammar
candidate from final Surface. M2 then accepted the reproduced v3 semantic
assertion `pass` digest through ADR-0017, moving only the lifecycle to T1. The
M3 established a finite `EvalPlan` calculus with owner RMW, exact release-admitted
receipt, and designated publication/explicit consumption evidence. M4 then closed
the finite owner-held relation / C-local coherent projection calculus, including
semantic/presentation fallback separation, fresh reacquire, privacy and split-frame
rejection. M5 then closed one finite concrete `SurfaceFragment → Core | Diagnostic`
model with shared Config / Step / WellFormed / Trace / Projection, 13 focused Rust
tests, and exact finite OBL-040--047 Lean evidence. M6 then closed bounded
ordinary Surface grammar, source spans, and total M5-aligned classification with
3 AST parser tests, 11 classifier tests, and exact finite OBL-048 Lean evidence.
M7 then closed one ordinary `.mir` route through M6 full classification retention,
finite checking, typed Core/effects/obligations/residuals/source map. Eight AST tests,
11 M6 classifier tests, 22 M7 pipeline tests, full `mir-ast` / `mir-semantics`
suites, format/clippy, and OBL-049's 16 trusted Lean theorems are bounded evidence.
The 10-row M7 fixture matrix is not frozen SCN-01..10 official conformance. M8 then
closed the deterministic single-process runtime over one checked artifact: 53 focused
tests, full `mir-runtime` and `mir-semantics` all-target suites, format/clippy, and 28
axiom-free `--trust=0` Lean theorem checks. OBL-050--056 are exact finite
`lean-proved` evidence; OBL-057 is runtime-monitored for bounded validation correspondence. This is a source/runtime
fixture and trace/replay/cut/save/patch evidence cut, not official SCN conformance,
public ABI/wire, sockets, production deployment, or a general proof. M9 is now the
direct consumer and M10 retains official conformance/closeout.

## milestone map

| Milestone | Aim | Position | Startability |
| --- | --- | --- | --- |
| M0 | governance, agent config, one roadmap | closed | Report 2581 / push parity |
| M1 | concise Constitution | closed; payload `aa0771ec` pushed | `root/design-constitution`, ADR-0016, Report 2582 |
| M2 | semantic-assertion T0/G0 closeout | closed; T1 entry accepted | `plan/248` reproduced pass / ADR-0017 |
| M3 | evaluation/materialization calculus | closed; finite theory/Lean/Rust evidence | ADR-0018/theory-13, Report 2584, closeout commit/push parity |
| M4 | maintained relation/late projection | closed; finite theory/Lean/Rust evidence | ADR-0019/theory-14/SCN-12, Report 2585, closeout evidence |
| M5 | shared model/metatheory | closed; finite shared model and exact evidence | ADR-0020/theory-15, Report 2586 |
| M6 | Surface | closed; finite grammar/AST/span/classification evidence | ADR-0021/spec-01--04, Report 2587 |
| M7 | checker/elaborator | closed; finite checked source-to-Core boundary | Report 2588 / OBL-049; 10-row fixture matrix is not SCN conformance |
| M8 | deterministic runtime | closed; finite checked-artifact runtime evidence | 53 focused tests; full target suites; exact OBL-050--056 Lean evidence; OBL-057 runtime-monitored for bounded validation correspondence |
| M9 | auth/verification | active | MembershipAuth, CapabilityAuth, non-transparent ContractUpdate, revocation/removal, refinement/model/Lean evidence, provenance/invalidation |
| M10 | conformance/closeout | next | after M9; first official fresh SCN-01..10 release profile |

## line snapshots

### Product Alpha line

Historical runnable LAB evidence only; it does not establish the source-first
M0--M10 program, official conformance, or a final product API.

### Operational Suite line

Historical bounded operational roots remain evidence. Their commands and
classification are unchanged in M0.

### Mir Language line

Existing parser/checker/elaboration evidence is LAB history. M6/M7 replace it
with the authoritative v0 source-to-Core route after the shared model closes.

### PoseGraph line

Existing pose evidence is not Mir Core. M4 closed a finite relation/projection
fragment without making renderer/IK/LOD behavior authoritative.

### Projection/Backend line

Provider/rendering/backend remain typed later boundaries. M4/M8 close finite
consumer-local projection and deterministic runtime fragments; M9 adds only typed
auth/verification extension behavior.

### Engine/Provider line

External providers remain adapters; names, sessions, packages, and transport
do not grant authority.

## validation floor

| Changed layer | Required M0 command |
| --- | --- |
| agent configuration | `python3 scripts/validate_agent_configs.py` and focused pytest |
| Canon metadata | `cd mirrorea_canon && python3 meta/build-index.py --check` |
| hierarchy/docs | `make docs` |
| documentation validator | `python3 -m unittest scripts.tests.test_validate_docs` |
| diff / secret guard | `git diff --check` and validator secret scan |

No Cargo/Lean/runtime/model/sample suite is claimed for M0 because those layers
do not change; their validations are assigned to the milestone that changes them.

## non-claims

No G0/T1 exit, OBL discharge, final proof, SCN/C-static/C-runtime pass, final
grammar/API/ABI/wire, production deployment, WAN/federation, or public-product
completion is claimed.

## user decision items vs research-discovery items

| Kind | Item | Current handling |
| --- | --- | --- |
| Owner-reserved | North Star/safety weakening, Core domain promotion, final public contract, deployment, user-data/secret risk | stop only if triggered; none observed |
| M2 research | semantic assertions/profile/fresh evaluation | preserve v1/v2 history; no premature exit |
| M3 research | eval policy and RMW inference | reject hidden communication/authority/transaction |
| M4 relation/projection | closed finite owner-held relation and C-local projection evidence | no general DAG/naturality or runtime claim |
| M5 shared model | closed finite shared model, correspondence, exact assurance mapping | general theorem/runtime remains deferred |
| M6 Surface | closed finite grammar, source spans, total M5-aligned classification | no final grammar/runtime/general theorem claim |
| M7 | closed one source-first checker/elaborator preserving M6 meaning | exact finite evidence only; no official SCN conformance |
| M8 | closed deterministic runtime over one checked artifact | exact finite evidence only; no official SCN conformance or general runtime theorem |
| M9 | typed auth/verification contract extension | reject authority grant conflation, untyped layer mutation, stale capability after removal/revocation, hidden residual success, or base-semantics redefinition |
| M10 | release conformance/closeout | fresh official profile only after M9 acceptance |

## macro phase map

| Macro | Focus | Current position | Weight | Self-drive |
| --- | --- | --- | --- | --- |
| 0 | governance/repository memory | M0 closed | medium | maintenance only |
| 1 | semantics/shared model | M1--M8 accepted; M9 is the active contract-transformer cut | heavy | yes |
| 2 | parser-free evidence | historical maintenance | medium | not current semantic frontier |
| 3 | source/checker/runtime | M8 closed; M9 extends only its typed runtime contract | heavy | M9 active |
| 4 | executable samples | historical evidence | medium | maintenance only |
| 5 | theorem/model-check | finite M3--M8 evidence; OBL-050--056 Lean-proved and OBL-057 runtime-monitored for bounded validation correspondence; general proof remains deferred | heavy | M9 extension evidence |
| 6 | distributed fabric | beyond I1+ | heavy | deferred |
| 7 | toolchain/backend | M9 bounded auth/verification support | heavy | M9 active |
| 8 | applications | domain consumers | heavy | deferred |

## feature maturity rows

| Feature | Evidence status | Remaining gate | Startability |
| --- | --- | --- | --- |
| multi-node/fabric | historical LAB only | beyond deterministic I1+ | deferred |
| contracts/theorem/model-check | finite M3 OBL-029--034, M4 OBL-035--039, M5 OBL-040--047, M6 OBL-048, M7 OBL-049, M8 OBL-050--056 Lean evidence, and OBL-057 runtime-monitored bounded validation correspondence; general ledger deferred | M9 auth/verification extension evidence | active |
| dynamic attach/detach/DAG evolution | M8 bounded patch evidence | M9 layer activation/removal then M10 | active |
| atomic_cut/ordering | Canon + LAB evidence | M5 model | later |
| executable sample corpus | runnable historical roots | source-first conformance profile | M10 consumer |
| Mirrorea | separate fabric layer | later I2+ | deferred |
| PrismCascade | separate performance kernel | not I1+ Core | deferred |
| Typed-Effect platform | separate adapter boundary | M9 typed contract-transformer evidence | active |
| domain applications | LAB consumers | no Core promotion | deferred |

## recent log

- 2026-08-03 19:20 JST: M0 bootstrapped ADR-0015 governance, Codex role
  validation, a single Plan 247 roadmap, and derived snapshots; official T0,
  proof, scenario, conformance, and implementation state remain unchanged.
- 2026-08-03 19:44 JST: M0 completed independent review, approval-policy
  regression coverage, focused validation, commit/push, and parity; M1
  Constitution is now the sole active semantic milestone.
- 2026-08-04 09:48 JST: M1 adopted the concise `root/design-constitution` and
  ADR-0016, corrected SCN-02 owner-side RMW and fallback prose conflicts, and
  passed independent review/one correction cycle; payload `aa0771ec` pushed
  with remote parity, so M2 is the sole active semantic milestone.
- 2026-08-04 10:36 JST: M2 accepted the reproduced semantic-assertion v3 pass
  digest in `plan/248`, then G0-D3, G0 exit, and T1 entry under ADR-0017;
  v1/v2 remain historical evidence and M3 is now the sole active milestone.
- 2026-08-04 12:09 JST: M3 closed the finite evaluation/materialization
  calculus: release-admitted causal receipts, serial owner RMW, designated
  publish/consume separation, trusted Lean compilation, bounded target-set
  enumeration, focused Rust traces, and independent re-review passed; M4 is
  now the sole active semantic milestone.
- 2026-08-04 13:24 JST: M4 closed ADR-0019/theory-14/SCN-12's finite maintained
  relation and late-projection fragment. OBL-035..039 compiled with `--trust=0`;
  13 focused Rust tests, the full `mir-semantics` suite, and test-target clippy
  passed. Oracle advice remained advisory; the reviewer corrected authority after
  five initial P1/P2 findings and found no final P0/P1. M5 is now sole active.
- 2026-08-04 15:15 JST: M5 closed ADR-0020/theory-15's finite shared
  `SurfaceFragment → Core | Diagnostic` model with shared Config / Step /
  WellFormed / Trace / Projection. Thirteen focused Rust tests and exact finite
  OBL-040..047 Lean evidence passed final independent review; M6 Surface is now
  the sole active semantic milestone, without a grammar, final ABI/wire, M8
  runtime, general theorem, conformance, or I1 completion claim.
- 2026-08-04 16:50 JST: M6 closed ADR-0021/spec-01--04's bounded ordinary
  Surface grammar, span-rich AST, and total M5-aligned classification. Three
  AST parser tests, 11 classifier tests, and exact finite OBL-048 Lean evidence
  passed without stubs; M7 checker/elaborator is now the sole active semantic
  milestone, without runtime, final grammar/ABI/wire, general theorem,
  conformance, or I1 completion claim.
- 2026-08-04 18:56 JST: M7 closed one ordinary `.mir` source-first
  parse/classification/check/elaboration route with typed Core/effects/obligations/
  residuals/source map. Eight AST tests, 11 M6 classifier tests, 22 M7 tests, full
  `mir-ast` / `mir-semantics` suites, format/clippy, and 16 exact OBL-049 Lean
  theorems passed. The 10-row fixture matrix is not SCN official conformance; M8
  deterministic runtime is now the sole active milestone.
- 2026-08-04 23:26 JST: M8 closed the finite deterministic runtime over a checked
  source artifact. Fifty-three focused tests, full `mir-runtime` / `mir-semantics`
  all-target suites, format/clippy, and 28 axiom-free `--trust=0` Lean theorem checks
  passed; OBL-050--056 are exact `lean-proved` evidence and OBL-057 is runtime-monitored
  for bounded validation correspondence. No official SCN conformance, public ABI/wire, sockets, production, or
  general proof is claimed. M9 auth/verification is now the sole active milestone.
