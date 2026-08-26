# Current Task Map (LAB)

最終更新: 2026-08-27 01:09 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, or operational
state; canon wins.

## document role

This is the repository-wide current task-map snapshot. Canon holds normative
decisions, Plan 249 is the sole active execution roadmap, Plan 247 is the
closed M0--M10 baseline, and milestone reports hold evidence. Historical
plan “next” entries are not active tasks.

## current promoted package

“Promoted” here means the package selected by the owner-authorized ADR-0026
program and current roadmap. It is not Canon L2 promotion, Gate/Phase exit,
proof completion, or public-product acceptance.

**Active: SYS-3 per-locus projection and executable artifact generation.**
SYS-0--SYS-2 are completed/closed. ADR-0028 accepts source cut
`920d3fe050b8b909253f8511d9ad897272323ced` for deterministic ST, exactly-one-
owner OW1, acknowledged M9 successor visibility, and bounded ordering
evidence. SYS-4 in-process generated dispatch is next.

Sources: `mirrorea_canon/adr/ADR-0028.md`,
`mirrorea_canon/theory/11-metatheory-ledger.md`, and
`plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`.

Direct consumer: SYS-4 starts the generated artifacts without reparsing or
reconstructing source semantics; SYS-5 displays their causal correspondence.

Current SYS-3 direct blockers:

1. define the smallest deterministic `GlobalProjectionResult` and per-locus
   executable plan representation from checked Core plus logical topology;
2. derive complete communication/effect/observation/persistence plans without
   handwritten interface edges or fixture-name selection;
3. preserve owner/site/source span, authority/failure/effect rows, relation and
   fallback lineage, designated non-reexecution, cut/patch obligations, and
   the semantic parts of the SYS-2 ST/OW1 contract;
4. reject missing/extra edges, owner-moving operations, source/Core identity
   mismatch, and malformed/cyclic projection without partial artifacts; and
5. exercise one conservative finite relation-DAG pressure case without
   claiming an arbitrary-DAG theorem or freezing a public artifact ABI.

Completion signal: deterministic artifacts for at least three loci, visible
generated plans, positive and malformed cases, no manual interface fixture,
M10 behavior reproducible, one independent review, one Report 2595, validation,
commit/push, and remote parity. Only then does SYS-4 become active.

Official theory remains T1. Broad PHASE-I1 exit and official I2 entry/exit stay
unaccepted; OPEN-026/027 and the full carrier freeze remain exact residuals.

## ordered self-driven packages

Packages execute in the fixed order; only one SYS semantic frontier is active.

| Package | Capability / evidence | Current state | Macro position / rough estimate |
| --- | --- | --- | --- |
| SYS-0 | baseline, authority, one roadmap, goal alignment | **completed / closed**; Report 2592 | Macro 0 front; closed |
| SYS-1 | semantic runtime kernel; internal owner/designated carrier | **completed / closed** at `94e3707c...`; Report 2593 | Macro 1/3/7 front; closed |
| SYS-2 | ST/OW1 backend, M9 generation visibility, ten-edge finite model | **completed / closed** at `920d3fe0...`; OBL-058/059, Report 2594 | Macro 3/5/7 middle; closed |
| SYS-3 | checked Core → per-locus artifacts and generated plans | **active**; deterministic projection + preservation/falsifier evidence | Macro 6/7 front; heavy, multi-day |
| SYS-4 | independent in-process locus endpoints run SYS-3 artifacts | next; actual endpoint dispatch + ST/OW1 + save/patch/replay negatives | Macro 6/7 middle; heavy, multi-day |
| SYS-5 | four-locus headless toy + joined typed devtools | after SYS-4 | Macro 4/6/8 middle; heavy, multi-day |
| SYS-6 | finite I2 conformance/assurance and lifecycle closeout | after SYS-5; exact source→trace profile/evidence classes | Macro 0/5/6 close; heavy, multi-day |
| SYS-7 | inactive I3 goal and entry contract only | terminal after SYS-6; no transport implementation | Macro 0/6 reserve; small, sub-day |

Active SYS-3 execution order:

1. Inventory the exact checked-Core operation/dependency/topology inputs and
   pin one internal projection result with no public compatibility promise.
2. Write determinism, no-hidden-edge, owner-preservation, source-span, and
   malformed/cyclic projection falsifiers before production generation.
3. Generate per-locus programs plus communication/effect/observation/
   persistence/source-map/diagnostic plans for at least three loci.
4. Add the smallest finite DAG pressure case and project-then-evaluate
   coherence evidence for the accepted relation fragment.
5. Rerun SYS-2/SYS-1/M10/runtime/workspace validation, obtain independent
   review, synchronize status/report, commit/push, and verify parity.

## self-driven macro phase reading

| Macro | Current state | Startability |
| --- | --- | --- |
| 0 repository memory/governance | ADR-0026/Plan 249 active; SYS-0--SYS-2 closed | SYS close sync startable |
| 1 semantic kernel | kernel/carrier + bounded ST/OW1 accepted | consume only; reopen on named falsifier |
| 2 parser-free historical evidence | retained; not current architecture | maintenance only |
| 3 source/checker/runtime | source-first M10 + executable local backend | projection active; dispatch later |
| 4 executable samples | no I2 sample yet | wait for SYS-4 then SYS-5 |
| 5 theorem/model-check bridge | OBL-058 bounded + OBL-059 runtime | projection evidence may add exact finite rows only |
| 6 generated/distributed fabric | no executable per-locus generation/dispatch yet | **active SYS-3** |
| 7 toolchain/backend | ST/OW1 internal prerequisite ready | **active SYS-3**, then SYS-4/5 |
| 8 upper application | four-locus toy remains future consumer | wait for SYS-5 |

## user decision gates

No owner decision is required to continue SYS-3--SYS-7 inside ADR-0026 unless
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
| projection IR granularity | SYS-4 artifact loader | smallest current design vs one viable alternative; determinism and identity tests | no public ABI/field-name freeze |
| communication completeness | SYS-4 dispatch | accepted Core operation inventory + omitted/extra edge falsifiers | no hand-authored schema or hidden edge |
| effect/observation/persistence plan split | SYS-4/5 | typed plan presence and source/Core provenance tests | do not collapse transport/auth/projection/persistence |
| finite relation-DAG extension | SYS-3/5 | three-step fallback or shared-ancestor pressure case | no arbitrary DAG theorem |
| broad-I1 carrier residual | lifecycle closeout | OPEN-026/027 and full internal freeze inventory | do not weaken exit criteria |

Do not open a WRK unless the active blocker cannot fit Report 2595 and all
direct-consumer/falsifier/adoption-discard conditions in ADR-0026 are met.

## maintenance tasks

- Preserve `canon > LAB`, official lifecycle T1, and exact evidence classes.
- Preserve M10 cut, closed Plan 247, SYS-1 cut, and SYS-2 cut; do not use their
  release/profile hashes as an I2 runtime architecture.
- Keep Plan 249 as the sole active roadmap and one SYS semantic frontier.
- Ordinary `.mir` source and checked Core are semantic authority; topology may
  place loci but cannot invent message interfaces or semantic edges.
- Surface gains no worker/mailbox/atomic/`memory_order_*` vocabulary.
- World/Avatar/Bird remain sample/library terms, never Core primitives.
- Transport/session/provider/worker identity and receipt remain non-authority.
- `samples_progress.md` stays unchanged until a runnable path, command, debug
  surface, or blocker actually changes.

## non-promoted references

- Active authority/roadmap: PROPOSAL-029, ADR-0026, Plan 249.
- SYS-1 contract: PROPOSAL-030, ADR-0027, Report 2593.
- SYS-2 contract: PROPOSAL-031, ADR-0028, OBL-058/059, Report 2594.
- Official lifecycle: `mirrorea_canon/plan/01-phases.md`.
- Proof/evidence status: `mirrorea_canon/theory/11-metatheory-ledger.md`.
- Runnable sample dashboard: `samples_progress.md` (unchanged by SYS-2).
