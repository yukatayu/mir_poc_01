# Current Task Map (LAB)

最終更新: 2026-08-27 07:07 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, or operational
state; canon wins.

## document role

This is the repository-wide current task-map snapshot. Canon holds normative
decisions, Plan 249 is the sole active execution roadmap, Plan 247 is the
closed M0--M10 baseline, and milestone reports hold evidence. Historical plan
“next” entries are not active tasks.

## current promoted package

“Promoted” here means the package selected by the owner-authorized ADR-0026
program and current roadmap. It is not Canon L2 promotion, Gate/Phase exit,
proof completion, or public-product acceptance.

**Active: SYS-4 in-process generated dispatch.** SYS-0--SYS-3 are completed/
closed. Accepted SYS-3 source/evidence cut
`3013e7fe075a7605a1ffe01e0b14f4a0856eaeb9` derives the bounded non-final,
exactly-one source-named `designated consume E.result at C` path; topology
cannot invent the consumer. Candidate `ded622fe...` remains partial regression
history. SYS-5 minimal typed devtools/four-locus toy is next.

Sources: `mirrorea_canon/adr/ADR-0029.md`,
`mirrorea_canon/spec/12-sys3-per-locus-projection.md`,
`mirrorea_canon/theory/11-metatheory-ledger.md`, and
`plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`.

Direct consumer: SYS-5 composes the actual SYS-4 endpoint traces, owner state,
relation/fallback, designated result, save/patch, and typed failures into the
headless toy and joined devtools view.

Current SYS-4 direct blockers:

1. start each accepted `LocusProgram` with an independent local store, queues,
   generated carrier endpoint, authority view, relation/designated state, and
   local trace;
2. materialize only SYS-3 generated edges and bind request→dispatch→receive→
   serve/reply/failure occurrences without source reparse, fixture selection,
   handwritten routes, or direct cross-locus store access;
3. run identical selected artifacts under ST and eligible OW1 while preserving
   authority/failure/visibility and preventing worker/route metadata minting;
4. implement the source/Core-bound carrier-side idempotent return or compatible
   wrapper before exactly one accepted M8 consume; test first, same-consumer
   retry, and competing consumer while preserving legacy M8/M10 behavior;
5. add fail-closed endpoint negatives, deterministic replay, process-local
   whole-fabric cut/save/restore/patch, full regressions, independent reviews,
   a pinned cut, commit/push, and parity.

Completion signal: generated artifacts actually cross locus endpoints under ST
and OW1; trace correspondence and typed failures are source/Core/artifact-bound;
no direct cross-locus mutation or source reconstruction exists; retry returns
the retained decision with exactly one accepted M8 consume; replay/cut/save/
restore/patch evidence and independent reviews pass. Only then does SYS-5
become active.

Official theory remains T1. Broad PHASE-I1 exit and official I2 entry/exit stay
unaccepted; OPEN-026/027 and the full carrier freeze remain exact residuals.

## ordered self-driven packages

Packages execute in the fixed order; only one SYS semantic frontier is active.

| Package | Capability / evidence | Current state | Macro position / rough estimate |
| --- | --- | --- | --- |
| SYS-0 | baseline, authority, one roadmap, goal alignment | **completed / closed**; Report 2592 | Macro 0 front; closed |
| SYS-1 | semantic runtime kernel; internal owner/designated carrier | **completed / closed** at `94e3707c...`; Report 2593 | Macro 1/3/7 front; closed |
| SYS-2 | ST/OW1 backend, M9 generation visibility, ten-edge finite model | **completed / closed** at `920d3fe0...`; OBL-058/059, Report 2594 | Macro 3/5/7 middle; closed |
| SYS-3 | checked Core → per-locus artifacts and generated plans | **completed / closed** at `3013e7fe...`; OBL-060 static-only runtime-monitored, Report 2595 | Macro 3/6/7 front; closed |
| SYS-4 | independent in-process locus endpoints run SYS-3 artifacts | **active** | Macro 6/7 middle; heavy, multi-day |
| SYS-5 | four-locus headless toy + joined typed devtools | next after SYS-4 | Macro 4/6/8 middle; heavy, multi-day |
| SYS-6 | finite I2 conformance/assurance and lifecycle closeout | after SYS-5; exact source→trace profile/evidence classes | Macro 0/5/6 close; heavy, multi-day |
| SYS-7 | inactive I3 goal and entry contract only | terminal after SYS-6; no transport implementation | Macro 0/6 reserve; small, sub-day |

Active SYS-4 execution order:

1. Define RED contracts for locus ownership, endpoint-only dispatch, no source
   reparse/direct store/handwritten edge, and source→occurrence correspondence.
2. Implement the smallest ST locus-runtime shell over accepted artifacts.
3. Reuse the same artifact/endpoint abstraction under eligible OW1.
4. Add the source/Core-bound designated-consume retry wrapper and endpoint
   first/retry/competing tests without changing legacy M8/M10 semantics.
5. Add typed failures, replay, whole-fabric cut/save/restore/patch, preserved
   regressions, independent reviews, the future single SYS-4 report (Report
   2596 only if the numbering convention remains available; do not create it
   now), commit/push, and parity.

## self-driven macro phase reading

| Macro | Current state | Startability |
| --- | --- | --- |
| 0 repository memory/governance | ADR-0026/Plan 249 active; SYS-0--SYS-3 closed; SYS-4 active | SYS status sync startable |
| 1 semantic kernel | kernel/carrier + bounded ST/OW1 + static projection accepted | **SYS-4 runtime refinement active** |
| 2 parser-free historical evidence | retained; not current architecture | maintenance only |
| 3 source/checker/runtime | source-first M10 + local backend + accepted designated consume projection | **SYS-4 active** |
| 4 executable samples | no I2 sample yet | wait for SYS-4 then SYS-5 |
| 5 theorem/model-check bridge | OBL-058 bounded + OBL-059 runtime + OBL-060 static finite runtime-monitored | SYS-4 adds runtime evidence only |
| 6 generated/distributed fabric | complete finite generated artifacts; dispatch absent | **active SYS-4** |
| 7 toolchain/backend | ST/OW1/projector ready; endpoint execution pending | **active SYS-4** |
| 8 upper application | four-locus toy remains future consumer | wait for SYS-5 |

## user decision gates

No owner decision is required to continue SYS-4--SYS-7 inside ADR-0026 unless
an owner-reserved stop condition becomes real.

| Overview | Impact | Major options | Current recommendation / view |
| --- | --- | --- | --- |
| North Star or safety/privacy/redaction/no-stale guarantee change | whole project semantics | preserve; or explicitly weaken | preserve; stop if weakening is required |
| domain vocabulary as Core primitive | Core architecture | keep library/sample; or promote | keep library/sample; stop if promotion is unavoidable |
| hidden multi-owner transaction | authority/atomicity | explicit operations; or hidden transaction | preserve explicit operations; stop if hidden transaction is unavoidable |
| public API/ABI/wire freeze | external compatibility | keep internal/provisional; or freeze | keep provisional; stop before irreversible freeze |
| real transport selection/implementation | I3 architecture | defer; or choose now | defer to future owner program; SYS-7 only writes inactive entry contract |
| production/publication/paid resources | external state/risk | remain local; or deploy | remain local; stop for owner authority |
| irreversible observable semantic tie | migration compatibility | Constitution orders; or owner decides | use priority order; stop only if tied and non-migratable |
| reproducible North-Star contradiction | parent program validity | revise program; or revise North Star | return decision bundle; do not weaken silently |

Official T1, deferred general OBLs, open final grammar/public ABI, incomplete
I3+, and unoptimized performance are not blockers requiring owner input.

## research discovery items

These are resolved inside the active package from evidence; they are not owner
decision requests and do not open separate semantic frontiers.

| Item | Direct consumer | Evidence needed | Boundary |
| --- | --- | --- | --- |
| designated consumer Core/projection | SYS-4 dispatch | accepted at `3013e7fe...`: explicit E-CONSUME source/Core edge, one consumer, delivery/source-map/observation/persistence, static retry identity/conflict and no-inference falsifiers | closed finite contract; no final grammar/multi-consumer/runtime claim; legacy M8 is not retry-return evidence |
| designated retry endpoint refinement | SYS-5 toy/devtools and SYS-6 conformance | carrier-side idempotent return or compatible wrapper, exactly one accepted M8 consume, actual positive/retry/competing-consumer tests, preserved M8/M10 duplicate-delivery baseline | deferred until SYS-4; not SYS-3 evidence and not network exactly-once |
| locus-runtime ownership | SYS-5 toy world | one local store/queue/view per locus; no global mutable map bypass | deferred until SYS-4 |
| generated endpoint materialization | SYS-5 devtools/SYS-6 conformance | exact carrier→endpoint→occurrence binding plus missing/extra route falsifiers | deferred until SYS-4 |
| ST/OW1 artifact correspondence | SYS-5/SYS-6 | same selected semantic result and permitted observation over identical artifacts | deferred until SYS-4; no general scheduler theorem |
| whole-fabric local cut/patch | SYS-5 save/patch path | artifact/carrier/queue/relation/designated/authority-ref inclusion and stale rejects | deferred until SYS-4; no durable/distributed persistence |
| broad-I1 carrier residual | lifecycle closeout | OPEN-026/027 and full internal freeze inventory | do not weaken exit criteria |

Do not open a WRK unless the active SYS-4 blocker cannot fit the active
milestone's single report and all direct-consumer/falsifier/adoption-discard
conditions in ADR-0026 are met. Under the current numbering convention that
future SYS-4 report would be Report 2596, but do not create it before material
SYS-4 work begins and the one-report admission condition applies.

## maintenance tasks

- Preserve `canon > LAB`, official lifecycle T1, and exact evidence classes.
- Preserve M10 cut and closed Plan 247/SYS-1/SYS-2 cuts; retain `ded622fe...`
  only as partial SYS-3 regression evidence and do not use these
  release/profile/internal names as a public I2 runtime architecture.
- Preserve legacy M8 same-delivery `AlreadyConsumed` and accepted M10 duplicate-
  delivery behavior. Do not reinterpret either as theory/13 idempotent-return
  evidence; that endpoint refinement is the active SYS-4 work.
- Keep Plan 249 as the sole active roadmap and one SYS semantic frontier.
- Ordinary `.mir` source and checked Core are semantic authority; schedules and
  endpoints cannot invent message edges or semantic state.
- Surface gains no worker/mailbox/atomic/`memory_order_*` vocabulary. The only
  current addition is the bounded non-final `designated consume E.result at C`
  semantic source fact; it is not final/public grammar.
- World/Avatar/Bird remain sample/library terms, never Core primitives.
- Transport/session/provider/worker identity and receipt remain non-authority.
- `samples_progress.md` stays unchanged until a runnable path, command, debug
  surface, or blocker actually changes.

## non-promoted references

- Active authority/roadmap: PROPOSAL-029, ADR-0026, Plan 249.
- SYS-1 contract: PROPOSAL-030, ADR-0027, Report 2593.
- SYS-2 contract: PROPOSAL-031, ADR-0028, OBL-058/059, Report 2594.
- SYS-3 contract: PROPOSAL-032, ADR-0029, spec/12, OBL-060, Report 2595.
- Official lifecycle: `mirrorea_canon/plan/01-phases.md`.
- Proof/evidence status: `mirrorea_canon/theory/11-metatheory-ledger.md`.
- Runnable sample dashboard: `samples_progress.md` (unchanged by SYS-3).
