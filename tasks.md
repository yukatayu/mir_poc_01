# tasks

最終更新: 2026-05-22 18:53 JST

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
P-FSV1-99 final audit
```

Next promoted package after this closeout:

```text
none
```

Purpose:

- lock the accepted `scripts/full_system_v1_release_check.py` lane into the repo snapshot with final claim/non-claim cleanup, report closure, and final validation recording.
- preserve `P-FSV1-03` closed with bounded release-check outputs: static `bundle.json` / `index.html`, per-command JSON reports, compatibility-floor replay, and representative Full V1 CLI surfaces.
- preserve the already-closed 12 executable source-first operational rows across `world-core/`, `membership-chat/`, `sugoroku-world/`, `portal-worldlink/`, `two-shard-hard-boundary/`, and `gradient-observation/` without widening claims past bounded local evidence.
- keep `package.mir.json` as alpha compatibility/package artifact while source authority stays on `.mir`.

## ordered self-driven packages

| Order | Package | Macro / stage | Goal | Close condition | Rough estimate |
|---:|---|---|---|---|---|
| 1 | `P-FSV1-99 final audit` | `Macro 0`, `S6` | claim/non-claim and docs cleanup | all validation recorded, report/commit/push done | medium |

## self-driven macro phase reading

| Macro | Reading | Closeout path |
|---|---|---|
| `Macro 0` | docs / reports / validator discipline | self-driven through every package close |
| `Macro 1` | semantics and invariant boundary | self-driven for source/typed IR/cut/PoseGraph wording; user gates only for final public commitments |
| `Macro 3` | compile-ready minimal actualization | `P-MIR-01..04`、`P-POSE-03..04`、`P-PROJ-02..04`、`P-ENG-02..03`、`P-FSV1-01..03` closed; current work is final audit closure |
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
| native/WASM execution | provider boundary | disabled/inventory / sandboxed WASM / bounded native | keep default disabled/inventory; current bounded evidence admits viewer-diagnostic and WASM inventory-only rows only |
| final engine adapter ABI | engine/provider line | internal provider manifest / public SDK / engine-specific ABI | defer; no Unity/Unreal/VRM compatibility claim |

## research discovery items

| Item | Impact | Main options | Current recommendation |
|---|---|---|---|
| alpha grammar shape | later reopen only | minimal C-like syntax / reuse current companion notation / package-derived migration syntax | keep the current minimal parser surface and widen only when a later non-audit package proves it necessary |
| typed IR representation | `P-MIR-02` | crate-local IR / new `mir-ir` crate / reuse existing product schema structs | start crate-local and split only if coupling becomes too high |
| interpreter rejection model | `P-MIR-03` | static reject / runtime reject / residual obligation | keep explicit static vs runtime split and failure row `ρ` |
| projection IR granularity | later reopen only | summary manifest / typed projection IR / deployment planner | typed projection IR is actualized; preserve that floor and keep planner later |
| packet/FFI payload schema granularity | later reopen only | payload-shape-only / effect-tag-rich schema / full transport protocol | keep the current payload-shape plus effect/failure/capability/authority preservation floor and defer transport semantics |
| local role-split carrier shape | later reopen only | same-binary local roles / paired local processes / Docker-pinned manifest roles | same-binary manifest-driven local roles are actualized; keep the accepted release-check above that floor |
| renderer/provider pose snapshot seam | later reopen only | preserve typed pose snapshot surface / add later planner normalization | bounded renderer pose delivery is actualized; preserve non-semantic-owner discipline while audit consumes the same seam |
| PoseGraph/projection preservation seam | later reopen only | project raw runtime state / preserve typed boundary manifests / add later planner normalization | preserve typed manifests and boundary evidence first, then close the audit without widening semantic ownership |

## maintenance tasks

| Task | Objective | Validation | Stop line |
|---|---|---|---|
| docs freshness audit | keep README, Documentation, progress, tasks, samples dashboard, indexes aligned | `python3 scripts/validate_docs.py`, `python3 scripts/check_source_hierarchy.py`, `git diff --check` | snapshot docs must not create new normative decisions |
| alpha/product regression audit | preserve Product Alpha and operational suite while Full V1 advances | product release check, operational suite helper, minimal pattern verifier | do not reinterpret alpha workflow as final product |
| sample taxonomy audit | keep active, planned, generated, archive roots distinct | source hierarchy and relevant helper checks | `samples/full-system-v1/computational/`、`world-core/`、`membership-chat/`、`sugoroku-world/`、`portal-worldlink/`、`two-shard-hard-boundary/`、`gradient-observation/`、`avatar-pose/`、`projection/`、`server-client/`、`provider-adapter/` remain evidence-closed as roots, while `scripts/full_system_v1_release_check.py` now provides the line-level bounded release-check workflow |
| validator scaffold update | add required docs only when they exist | `python3 -m unittest scripts.tests.test_validate_docs` | validators check presence, not semantic correctness |
| report discipline | write a new report for every non-trivial package | `python3 scripts/validate_docs.py` | never overwrite previous report |

## non-promoted references

- Product Alpha line remains bounded alpha workflow, not final product.
- Operational suite remains bounded local/Docker workflow, not production shared-space catalog completion.
- Existing `samples/product-alpha1/computational/` rows remain first-floor evidence, not Rust-level language completion.
- Existing `samples/product-alpha1/posegraph/` rows remain helper evidence, while `samples/full-system-v1/avatar-pose/` now carries bounded source-first runtime PoseGraph plus pose save/devtools evidence; distributed durable pose save/load and final devtools family remain later.
- `samples/product-alpha1/projection/` and `samples/product-alpha1/engine-adapter/` remain inventory-only comparison evidence, while `samples/full-system-v1/projection/` carries bounded source-first projection IR plus boundary-schema evidence, `samples/full-system-v1/server-client/` carries bounded same-binary local role-run evidence, and `samples/full-system-v1/provider-adapter/` now carries bounded provider-admission plus renderer-pose-backend evidence.
- `P-FSV1-03` release-check closure is bounded local evidence with static bundle/viewer/report outputs; it is not a final public viewer/devtools family, final installer/archive, or final public product workflow.
- Direct LLVM/native backend remains later than typed IR, projection IR, and preservation tests.
