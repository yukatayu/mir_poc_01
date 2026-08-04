# progress

最終更新: 2026-08-04 13:24 JST

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
| Logical specification | M0--M4 are evidence-complete; official lifecycle is T1; M5 shared formal model/metatheory is active | **着手可能**: one non-opaque shared model and correspondence matrix |
| User-facing specification | No M0 syntax change. M6 owns Surface v0 after M5 shared model | **後段依存**: M1--M5 |
| Implementation / operation | No M0 runtime change. M8 owns deterministic I1+ runtime after M7 | **後段依存**: M1--M7 |

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
rejection. Its direct consumer is M5's shared model; official conformance and M8
runtime state remain unchanged.

## milestone map

| Milestone | Aim | Position | Startability |
| --- | --- | --- | --- |
| M0 | governance, agent config, one roadmap | closed | Report 2581 / push parity |
| M1 | concise Constitution | closed; payload `aa0771ec` pushed | `root/design-constitution`, ADR-0016, Report 2582 |
| M2 | semantic-assertion T0/G0 closeout | closed; T1 entry accepted | `plan/248` reproduced pass / ADR-0017 |
| M3 | evaluation/materialization calculus | closed; finite theory/Lean/Rust evidence | ADR-0018/theory-13, Report 2584, closeout commit/push parity |
| M4 | maintained relation/late projection | closed; finite theory/Lean/Rust evidence | ADR-0019/theory-14/SCN-12, Report 2585, closeout evidence |
| M5 | shared model/metatheory | active | M1--M4 common model, correspondence, assurance mapping |
| M6/M7 | Surface, checker/elaborator | later | after M5 |
| M8/M9 | runtime, auth/verification | later | after M7/M8 |
| M10 | conformance/closeout | later | after M9 |

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

Provider/rendering/backend remain typed later boundaries. M4 closes only a finite
consumer-local projection fragment; M8 owns deterministic runtime behavior.

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
| M5 research | non-opaque shared model, correspondence, exact assurance mapping | reject opaque/stub model or untracked rule family |
| M6--M10 | Surface, toolchain, runtime, extensions, closeout | sequentially after M5 acceptance |

## macro phase map

| Macro | Focus | Current position | Weight | Self-drive |
| --- | --- | --- | --- | --- |
| 0 | governance/repository memory | M0 closed | medium | maintenance only |
| 1 | semantics/shared model | M1--M4 accepted; M5 is the active integration cut | heavy | yes |
| 2 | parser-free evidence | historical maintenance | medium | not current semantic frontier |
| 3 | source/checker/runtime | M6--M8 | heavy | after shared model |
| 4 | executable samples | historical evidence | medium | maintenance only |
| 5 | theorem/model-check | finite M3/M4 evidence; M5 shared correspondence then M10 | heavy | active shared-model input |
| 6 | distributed fabric | beyond I1+ | heavy | deferred |
| 7 | toolchain/backend | M7--M9 bounded support | heavy | after M6 |
| 8 | applications | domain consumers | heavy | deferred |

## feature maturity rows

| Feature | Evidence status | Remaining gate | Startability |
| --- | --- | --- | --- |
| multi-node/fabric | historical LAB only | beyond deterministic I1+ | deferred |
| contracts/theorem/model-check | finite M3 OBL-029--034 and M4 OBL-035--039 evidence; general ledger deferred | M5 shared statements | active |
| dynamic attach/detach/DAG evolution | bounded LAB evidence | M8 patch then M10 | later |
| atomic_cut/ordering | Canon + LAB evidence | M5 model | later |
| executable sample corpus | runnable historical roots | source-first conformance profile | M10 consumer |
| Mirrorea | separate fabric layer | later I2+ | deferred |
| PrismCascade | separate performance kernel | not I1+ Core | deferred |
| Typed-Effect platform | separate adapter boundary | M9 extension evidence | later |
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
