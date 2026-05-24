# tasks

最終更新: 2026-05-24 15:38 JST

## document role

This document is the repo-wide **current task map**. It is not normative source
and is not append-only history.

- Normative source: `specs/`
- Repository memory: `plan/`
- Status snapshot: `progress.md`
- Runnable dashboard: `samples_progress.md`
- Execution evidence: `docs/reports/`

## current promoted package

Current promoted package after `P-SURF-01` closeout:

```text
P-SURF-02 indexed-state semantics
```

Purpose:

- implement semantic checks for `S { state player[p: Participant]: Player }`.
- record owner = enclosing place block, keyspace = declared key type, and value
  type = declared state value.
- keep key lookup separate from authority: a Participant key does not grant
  write/read capability.
- add representative positive and negative rows for owner/keyspace/access/stale
  semantics while preserving the parser floor from `P-SURF-01`.

## ordered self-driven packages

| Order | Package | Objective | Close condition |
|---:|---|---|---|
| 1 | `P-SURF-01 surface brace parser` | parse `S { ... }`, role-instance blocks, `state`, and `when`; reject `S[ ... ]` | closed with `SURF-01..09`, parser test, sample helper, authoring check, and release check |
| 2 | `P-SURF-02 indexed state` | represent `S { state player[p: Participant]: Player }` as S-owned indexed state | owner/keyspace/access/stale-key rows pass |
| 3 | `P-SURF-03 Surface-to-Core elaboration` | lower cross-locus read/write to explicit Core IR | Core IR source spans and obligations are visible |
| 4 | `P-SURF-04 auto communication` | generate MessageEnvelope / publish / observe / failure-row obligations | generated edges are explicit and private field auto-publish is blocked |
| 5 | `P-SURF-05 role admission` | implement role claim, admission request, capability grant, spoof/stale rejection | role claim does not grant authority; grants and stale rejects are observable |
| 6 | `P-SURF-06 source patch hot-plug` | implement parse/typecheck/elaborate/admit/activation-cut patch pipeline | rejected patches do not mutate runtime; accepted patch emits activation cut |
| 7 | `P-SURF-07 source operational suite` | create Surface source WorldCore / MembershipChat / Sugoroku / related roots | roots have positive/negative evidence and remain distinct from Product Alpha roots |
| 8 | `P-SURF-08 devtools and diagnostics` | show Surface source, Core IR, generated communication, indexed state, admission, patch lifecycle | observer-safe panels and diagnostics preserve source/Core links |
| 9 | `P-SURF-99 final audit` | rerun validation and compatibility anchors | full validation, docs/report cleanup, non-claim audit |

## self-driven macro phase reading

| Macro | Reading | Closeout path |
|---|---|---|
| `Macro 0` | docs / reports / validator discipline | self-driven through every Surface package close |
| `Macro 1` | semantics and invariant boundary | self-driven for source authority, place syntax, indexed state, admission, patch pipeline |
| `Macro 3` | compile-ready minimal actualization | `P-SURF-01..06` can proceed package-by-package with focused rows |
| `Macro 4` | executable sample expansion | `P-SURF-07` creates roots only after parser/elaboration/indexed-state floors exist |
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
| indexed-state runtime carrier | `P-SURF-02` | plain map / membership-aware partial map / distributed table | use membership-aware owner-locus partial map first |
| elaboration IR shape | `P-SURF-03` | direct Core transitions / intermediate elaboration report / both | produce Core IR plus source-linked elaboration report |
| auto publish policy | `P-SURF-04` | publish all writes / visible-fields-only / explicit-only | visible-fields-only; private field blocked |
| admission witness metadata | `P-SURF-05` | principal only / role + principal / optional package/runtime hash | role + principal required; package/runtime hash optional report metadata |
| source patch compatibility | `P-SURF-06` | check-only / check+diff / full migration planner | check+Core diff+activation cut first; migration planner later |
| Surface sample root shape | `P-SURF-07` | reuse `full-system-v1/` / new `full-system-v1-surface/` / product-alpha root | use planned `samples/full-system-v1-surface/` to avoid mixing roots |

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
- `S[ ... ]` remains rejected and must not be introduced as a compatibility sugar.
- `package.mir.json` remains alpha compatibility / package artifact, not semantic source authority.
- Direct LLVM/native backend remains later than Surface parser, elaboration, typed IR, projection IR, and preservation tests.
