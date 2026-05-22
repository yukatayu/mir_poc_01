# tasks

最終更新: 2026-05-22 14:10 JST

## document role

This document is the repo-wide **current task map**. It is not normative source and is not append-only history.

- Normative source: `specs/`
- Repository memory: `plan/`
- Status snapshot: `progress.md`
- Runnable dashboard: `samples_progress.md`
- Execution evidence: `docs/reports/`

## current promoted package

Current promoted package:

```text
P-PROJ-03 boundary schemas
```

Next promoted package after this closeout:

```text
P-PROJ-04 server/client local split
```

Purpose:

- actualize `FS-06` packet and FFI schema preservation over the bounded projection IR floor.
- preserve `P-PROJ-02` closed with source-derived target manifests, source-owned capability/failure preservation, unassigned-place/save-load ownership rejection, and explicit client-write authority rejection evidence.
- keep `package.mir.json` as alpha compatibility/package artifact while source authority shifts to `.mir`.

## ordered self-driven packages

| Order | Package | Macro / stage | Goal | Close condition | Rough estimate |
|---:|---|---|---|---|---|
| 1 | `P-PROJ-03 boundary schemas` | `Macro 7`, `S4 -> S5` | packet/FFI schema preservation | positive/negative report covers effect/failure/capability/authority/provider-policy/rollback boundaries | medium |
| 2 | `P-PROJ-04 server/client local split` | `Macro 6/7`, `S4 -> S5` | local/Docker server/client roles | roles run from projection manifest plus write-escalation or undeclared-authority rejection row | heavy |
| 3 | `P-ENG-02 provider admission` | `Macro 7`, `S3 -> S5` | runtime provider manifest admission | accepted bounded provider, over-capability rejection, missing rollback/replay/cut rejection, disabled-native evidence, and explicit WASM inventory-only or sandbox-accepted evidence | medium |
| 4 | `P-ENG-03 renderer pose backend demo` | `Macro 7/8`, `S4 -> S5` | renderer receives pose snapshot | renderer stays non-semantic-owner | medium |
| 5 | `P-FSV1-01 source operational suite` | `Macro 8`, `S4 -> S5` | source-first WorldCore/MembershipChat/Sugoroku | check/run/devtools over source-first roots | heavy |
| 6 | `P-FSV1-02 portal/shard source samples` | `Macro 8`, `S4 -> S5` | source-first portal/shard/gradient | positive/negative portal/shard evidence | heavy |
| 7 | `P-FSV1-03 full V1 release check` | `Macro 0/7/8`, `S5 -> S6` | clean clone Full V1 workflow | release check, viewer, bundle, reports | heavy |
| 8 | `P-FSV1-99 final audit` | `Macro 0`, `S6` | claim/non-claim and docs cleanup | all validation recorded, report/commit/push done | medium |

## self-driven macro phase reading

| Macro | Reading | Closeout path |
|---|---|---|
| `Macro 0` | docs / reports / validator discipline | self-driven through every package close |
| `Macro 1` | semantics and invariant boundary | self-driven for source/typed IR/cut/PoseGraph wording; user gates only for final public commitments |
| `Macro 3` | compile-ready minimal actualization | `P-MIR-01..04`、`P-POSE-03..04`、`P-PROJ-02` closed; next main implementation path is boundary schemas |
| `Macro 6` | distributed fabric / runtime evolution | local/Docker split can be self-driven; WAN/federation remains user decision |
| `Macro 7` | projection/backend/provider/developer surface | self-driven for bounded alpha evidence; final ABI/SDK/distribution remains user decision |
| `Macro 8` | domain/application realization | source-first operational samples can be self-driven after language/runtime base |

## user decision gates

| Gate | Affects | Main options | Current recommendation |
|---|---|---|---|
| final public grammar | final language/API | freeze alpha grammar / revise before public / keep package compatibility longer | do not freeze in Full V1; keep alpha grammar explicit |
| final ABI / SDK | external developers | Rust library ABI / CLI-only / hosted API / engine SDK | defer until source/typed IR/projection evidence exists |
| broader distribution | product delivery | developer-built bundle / release archive / installer / hosted service | keep current developer-built binary + generated host bundle until user choice |
| final shared-space catalog breadth | product scope | bounded showcase / broader room catalog / Reversed Library path | keep bounded showcase; decide final catalog separately |
| production WAN/federation | runtime/network | local/Docker only / WAN federation / hosted fabric | keep out of Full V1 unless explicitly promoted |
| distributed durable save/load R3/R4 | persistence | R0/R2 only / R3 durable / R4 distributed replay | keep R3/R4 later |
| native/WASM execution | provider boundary | disabled/inventory / sandboxed WASM / bounded native | default disabled/inventory until provider admission package proves safety |
| final engine adapter ABI | engine/provider line | internal provider manifest / public SDK / engine-specific ABI | defer; no Unity/Unreal/VRM compatibility claim |

## research discovery items

| Item | Impact | Main options | Current recommendation |
|---|---|---|---|
| alpha grammar shape | `P-MIR-02` and later sample widening | minimal C-like syntax / reuse current companion notation / package-derived migration syntax | keep the current minimal parser surface and widen only when checker/runtime packages need it |
| typed IR representation | `P-MIR-02` | crate-local IR / new `mir-ir` crate / reuse existing product schema structs | start crate-local and split only if coupling becomes too high |
| interpreter rejection model | `P-MIR-03` | static reject / runtime reject / residual obligation | keep explicit static vs runtime split and failure row `ρ` |
| projection IR granularity | `P-PROJ-03` and later | summary manifest / typed projection IR / deployment planner | typed projection IR is now actualized; preserve that floor and keep planner later |
| packet/FFI payload schema granularity | `P-PROJ-03` | payload-shape-only / effect-tag-rich schema / full transport protocol | start with payload shape plus effect/failure/capability/authority preservation and keep transport later |
| provider admission policy | `P-ENG-02` | inventory-only / accepted renderer row / sandboxed WASM first | accepted renderer/diagnostic row plus negative provider rows; native disabled |
| PoseGraph/projection preservation seam | `P-PROJ-03..04` | project raw runtime state / preserve typed boundary manifests / add later planner normalization | preserve typed manifests and boundary evidence first, then add packet/FFI payload schemas before role-run |

## maintenance tasks

| Task | Objective | Validation | Stop line |
|---|---|---|---|
| docs freshness audit | keep README, Documentation, progress, tasks, samples dashboard, indexes aligned | `python3 scripts/validate_docs.py`, `python3 scripts/check_source_hierarchy.py`, `git diff --check` | snapshot docs must not create new normative decisions |
| alpha/product regression audit | preserve Product Alpha and operational suite while Full V1 advances | product release check, operational suite helper, minimal pattern verifier | do not reinterpret alpha workflow as final product |
| sample taxonomy audit | keep active, planned, generated, archive roots distinct | source hierarchy and relevant helper checks | only `samples/full-system-v1/computational/`、`avatar-pose/`、`projection/` are evidence-closed; wider Full System V1 roots must not be workflow-ready before evidence |
| validator scaffold update | add required docs only when they exist | `python3 -m unittest scripts.tests.test_validate_docs` | validators check presence, not semantic correctness |
| report discipline | write a new report for every non-trivial package | `python3 scripts/validate_docs.py` | never overwrite previous report |

## non-promoted references

- Product Alpha line remains bounded alpha workflow, not final product.
- Operational suite remains bounded local/Docker workflow, not production shared-space catalog completion.
- Existing `samples/product-alpha1/computational/` rows remain first-floor evidence, not Rust-level language completion.
- Existing `samples/product-alpha1/posegraph/` rows remain helper evidence, while `samples/full-system-v1/avatar-pose/` now carries bounded source-first runtime PoseGraph plus pose save/devtools evidence; distributed durable pose save/load and final devtools family remain later.
- `samples/product-alpha1/projection/` remains inventory-only comparison evidence, while `samples/full-system-v1/projection/` now carries bounded source-first projection IR evidence only; packet/FFI payload schemas, executable role split, and provider admission remain later.
- Direct LLVM/native backend remains later than typed IR, projection IR, and preservation tests.
