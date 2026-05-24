# tasks

最終更新: 2026-05-24 20:42 JST

## document role

This document is the repo-wide **current task map**. It is not normative source
and is not append-only history.

- Normative source: `specs/`
- Repository memory: `plan/`
- Status snapshot: `progress.md`
- Runnable dashboard: `samples_progress.md`
- Execution evidence: `docs/reports/`

## current promoted package

No current promoted Surface package after `P-SURF-99` closeout.

Current holding state:

- Surface alpha `P-SURF-01..08` evidence rows remain runnable through
  `scripts/surface_mir_samples.py`.
- `P-SURF-99` reran full Surface validation and Product Alpha compatibility
  anchors.
- P-SURF-08 devtools diagnostics remain static source/Core evidence, not final
  viewer / telemetry ABI or runtime devtools completion.

## ordered self-driven packages

| Order | Package | Objective | Close condition |
|---:|---|---|---|
| 1 | `P-SURF-01 surface brace parser` | parse `S { ... }`, role-instance blocks, `state`, and `when`; reject `S[ ... ]` | closed with `SURF-01..09`, parser test, sample helper, authoring check, and release check |
| 2 | `P-SURF-02 indexed state` | represent `S { state player[p: Participant]: Player }` as S-owned indexed state | closed with `IDX-01..05`, semantic checker test, sample helper, authoring check, and release check |
| 3 | `P-SURF-03 Surface-to-Core elaboration` | lower cross-locus read/write to explicit Core IR | closed with `ELAB-01/02/04/05/06/07/08`, elaboration test, sample helper, authoring check, and release check |
| 4 | `P-SURF-04 auto communication` | generate MessageEnvelope / publish / observe / failure-row obligations | closed with generated MessageEnvelope rows, visible field publish/observe rows, `VisibilityDenied` failure-row containment, private/non-visible field rejection, and `ELAB-03/09/10` |
| 5 | `P-SURF-05 role admission` | implement role claim, admission request, capability grant, spoof/stale rejection | closed with `ROLE-01..04`, role claim / join admission / grant-backed accepted write / witness rows, missing-grant write rejection, stale membership rejection, and hash metadata non-safety-proof |
| 6 | `P-SURF-06 source patch hot-plug` | implement parse/typecheck/elaborate/admit/activation-cut patch pipeline | closed with CLI `check-source` / `parse-source` / `elaborate-source` / `patch-source` / `export-core-ir`, `PATCH-01..04`, HotPlugRequest / HotPlugVerdict / activation_cut rows, no-direct-eval evidence, and rejection-without-mutation rows |
| 7 | `P-SURF-07 source operational suite` | create Surface source WorldCore / MembershipChat / Sugoroku / related roots | closed with six source roots, `operational-matrix.json`, and `E2E-SURF-01..12` positive/negative rows |
| 8 | `P-SURF-08 devtools and diagnostics` | show Surface source, Core IR, generated communication, semantic indexed-state map, admission, redacted patch lifecycle | closed with `samples/full-system-v1-surface/devtools/`, `DEV-01..02`, required panels, diagnostics, redaction gates, and source-span evidence |
| 9 | `P-SURF-99 final audit` | rerun validation and compatibility anchors | closed with full validation, docs/report cleanup, non-claim audit |

## self-driven macro phase reading

| Macro | Reading | Closeout path |
|---|---|---|
| `Macro 0` | docs / reports / validator discipline | self-driven through every Surface package close |
| `Macro 1` | semantics and invariant boundary | self-driven for source authority, place syntax, indexed state, admission, patch pipeline |
| `Macro 3` | compile-ready minimal actualization | `P-SURF-01..08` and P-SURF-99 audit closed; maintenance only until a new package is promoted |
| `Macro 4` | executable sample expansion | `P-SURF-07` created operational roots; `P-SURF-08` added static diagnostics; P-SURF-99 audit closed |
| `Macro 6` | distributed fabric / runtime evolution | local/Docker alpha can be self-driven; WAN/federation remains user decision |
| `Macro 7` | toolchain / backend / developer surface | Surface CLI/devtools can be self-driven within alpha scope |
| `Macro 8` | domain/application realization | Surface operational suite can be self-driven after language/runtime base |

## user decision gates

| Gate | Affects | Main options | Current recommendation |
|---|---|---|---|
| final public grammar | final language/API | freeze Surface alpha / revise before public / keep package compatibility longer | do not freeze in Surface alpha; keep grammar explicitly alpha |
| final ABI / SDK | external developers | Rust library ABI / CLI-only / hosted API / engine SDK | defer until Surface parser/elaboration/runtime evidence exists |
| broader distribution | product delivery | developer-built bundle / release archive / installer / hosted service | keep current developer-built binary + generated host bundle |
| final shared-space catalog breadth | product scope | bounded showcase / broader room catalog / Reversed Library path | keep bounded showcase; decide separately |
| production WAN/federation | runtime/network | local/Docker only / WAN federation / hosted fabric | keep out of Surface alpha unless explicitly promoted |
| distributed durable save/load R3/R4 | persistence | R0/R2 only / R3 durable / R4 distributed replay | keep R3/R4 later |
| native/WASM execution | provider boundary | disabled/inventory / sandboxed WASM / bounded native | keep default disabled/inventory |
| final engine adapter ABI | engine/provider line | internal provider manifest / public SDK / engine-specific ABI | defer; no Unity/Unreal/VRM compatibility claim |

## research discovery items

| Item | Impact | Main options | Current recommendation |
|---|---|---|---|
| brace disambiguation | `P-SURF-01` | namespace-only / context-only / combined namespace + context | use combined namespace + context with ambiguous diagnostic |
| role-instance block parse | `P-SURF-01` | role path only / arbitrary indexed expression block | alpha accepts declared role path only |
| indexed-state runtime carrier | `P-SURF-02` / later runtime packages | plain map / membership-aware partial map / distributed table | `P-SURF-02` fixed checker semantics; use membership-aware owner-locus partial map first when runtime carrier is added |
| elaboration IR shape | `P-SURF-03` / `P-SURF-04` | direct Core transitions / intermediate elaboration report / both | closed with Core IR plus source-linked elaboration and generated communication rows |
| auto publish policy | `P-SURF-04` | publish all writes / visible-fields-only / explicit-only | closed narrow alpha: visible-fields-only; private/non-visible fields blocked; TypeMismatch discharge remains later |
| admission witness metadata | `P-SURF-05` | principal only / role + principal / optional package/runtime hash | closed narrow alpha: role + principal required; package/runtime hash optional report metadata and not safety proof |
| source patch compatibility | `P-SURF-06` | check-only / check+diff / full migration planner | closed narrow alpha: check+Core diff+HotPlugRequest/HotPlugVerdict+activation_cut; full migration planner later |
| Surface sample root shape | `P-SURF-07` | reuse `full-system-v1/` / new `full-system-v1-surface/` / product-alpha root | closed with `samples/full-system-v1-surface/` top-level operational roots distinct from Product Alpha roots |
| Surface diagnostics shape | `P-SURF-08` | static helper bundle / CLI export / runtime devtools integration | closed with static observer-safe report bundle first; final viewer/telemetry ABI later |

## maintenance tasks

| Task | Objective | Validation | Stop line |
|---|---|---|---|
| docs freshness audit | keep README, Documentation, progress, tasks, samples dashboard, indexes aligned | `python3 scripts/validate_docs.py`, `python3 scripts/check_source_hierarchy.py`, `git diff --check` | snapshot docs must not create new normative decisions |
| product compatibility audit | preserve Product Alpha and operational suite while Surface advances | product release check, operational suite helper, minimal pattern verifier | do not reinterpret alpha workflow as final product |
| sample taxonomy audit | keep Surface planned roots distinct from active roots | source hierarchy and relevant helper checks | do not create or mark `samples/full-system-v1-surface/` workflow-ready until implementation rows exist |
| validator scaffold update | add required docs only when they exist | `python3 -m unittest scripts.tests.test_validate_docs` | validators check presence and heading shape, not semantic correctness |
| report discipline | write a new report for every non-trivial package | `python3 scripts/validate_docs.py` | never overwrite previous report |

## non-promoted references

- Product Alpha line remains bounded alpha workflow, not final product.
- Operational suite remains bounded local/Docker workflow, not production shared-space catalog completion.
- Full System V1 release-check closure remains bounded local/source-first evidence, not final grammar / final ABI / final server-client compiler.
- `samples/full-system-v1-surface/syntax/` is P-SURF-01 parser evidence only,
  not a Surface runtime or operational suite.
- `samples/full-system-v1-surface/indexed-state/` is P-SURF-02 semantic
  checker evidence only, not a Surface runtime, elaboration, or operational
  suite.
- `samples/full-system-v1-surface/elaboration/` is P-SURF-03/P-SURF-04
  elaboration and generated communication evidence only, not runtime
  MessageEnvelope dispatch, role admission, source patch activation, or an
  operational suite.
- `samples/full-system-v1-surface/role-admission/` is P-SURF-05 report-level
  admission/grant evidence only, not production identity, hardware attestation,
  WAN admission, runtime membership lifecycle, source patch activation, or an
  operational suite.
- `samples/full-system-v1-surface/source-patch/` is P-SURF-06 source patch
  hot-plug pipeline evidence only, not a final hot-plug ABI, distributed
  durable migration planner, production patch registry, or arbitrary
  native/WASM execution route.
- `samples/full-system-v1-surface/world-core/`, `membership-chat/`,
  `sugoroku-world/`, `portal-worldlink/`, `two-shard-hard-boundary/`, and
  `gradient-observation/` are P-SURF-07 source operational evidence only, not a
  final operational runtime/transport or final shared-space catalog.
- `S[ ... ]` remains rejected and must not be introduced as a compatibility sugar.
- `package.mir.json` remains alpha compatibility / package artifact, not semantic source authority.
- Direct LLVM/native backend remains later than Surface parser, elaboration, typed IR, projection IR, and preservation tests.
