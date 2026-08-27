# Current Task Map (LAB)

最終更新: 2026-08-27 21:06 JST

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

**Active: SYS-5 minimal typed devtools and local virtual-space vertical
slice.** SYS-0--SYS-4 are completed/closed. Accepted SYS-4 implementation/
evidence cut `22196f93b0112b8fd2987ec078021c8865b71651` runs only accepted
SYS-3 generated plans through explicit locus-local endpoints, with selected
ST/eligible-OW1 semantic correspondence, exact source/Core/artifact/carrier/
runtime provenance, one-consume designated retry, typed fail-closed observer
and fault paths, ST whole-fabric cut/restore, and one bounded checked patch.
This is crate-private finite runtime evidence, not a public API/ABI/wire or a
user workflow. SYS-6 finite I2 assurance/conformance is next.

Sources: `mirrorea_canon/adr/ADR-0026.md`,
`mirrorea_canon/adr/ADR-0030.md`,
`mirrorea_canon/spec/13-sys4-in-process-generated-dispatch.md`,
`mirrorea_canon/spec/12-sys3-per-locus-projection.md`,
`mirrorea_canon/architecture/04-runtime-carriers.md`, and
`plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`.

Direct consumer: SYS-6 consumes one exact source → checked Core → locus
artifact → generated communication → runtime occurrence chain, the
four-locus positive/negative workflow, and its observer-safe joined view as
source-first finite I2 conformance evidence.

Current SYS-5 direct blockers:

1. Compose `WorldAuthority`, `ParticipantA`, `ParticipantB`, and `ViewerC`
   from one ordinary source or small source module set through the accepted
   projector and SYS-4 endpoints. Do not use fixture-name plan selection,
   expected-result lookup, source reconstruction, handwritten routes, direct
   cross-locus store access, or a thick helper-call E2E wrapper.
2. Drive owner-side participant attack RMW, authoritative tick/frontier
   publication and named designated consume, a B-owned bird relation with
   A-primary/B-fallback, A leave, semantic invalidation/fallback, a ViewerC
   presentation-only sample gap, and fresh reacquire on that actual path.
3. Join source span → Core operation → locus artifact → generated edge →
   request/receive/serve or failure → owner/relation/designated/save/patch
   occurrence in one typed observer-safe view. Raw credentials, capability
   secrets, witness payloads, and private values must remain redacted.
4. Include one auth/policy attach/remove or revocation case, one optional
   verification residual/discharge example, ST save/restore, accepted and
   rejected bounded patch, and representative negative interactions without
   weakening the accepted SYS-4 boundary.
5. Provide a small reproducible source/build-project/run-fault/inspect command
   set and concise walkthrough, then pass focused/full regression, usability,
   semantic, and code-quality reviews. SYS-4 crate-private tests are inputs,
   not the SYS-5 user workflow or SYS-6 conformance profile.

Completion signal: one ordinary-source four-locus run exercises the required
positive and falsifier paths over generated endpoints, one observer-safe view
shows their causal line without manual joins, and a new user can reproduce the
workflow from the documented commands. The accepted internal contracts remain
unchanged, independent usability/semantics review finds no major
counterexample, exact validation/non-claims are recorded, and SYS-6 becomes the
sole active goal.

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
| SYS-4 | generated-plan-only independent in-process locus dispatch | **completed / closed** at `22196f93...`; bounded runtime-monitored evidence | Macro 6/7 middle; closed |
| SYS-5 | four-locus headless toy + joined typed devtools | **active** | Macro 4/6/8 middle; heavy, multi-day |
| SYS-6 | finite I2 conformance/assurance and lifecycle closeout | next after SYS-5; exact source→trace profile/evidence classes | Macro 0/5/6 close; heavy, multi-day |
| SYS-7 | inactive I3 goal and entry contract only | terminal after SYS-6; no transport implementation | Macro 0/6 reserve; small, sub-day |

Active SYS-5 execution order:

1. Fix RED contracts for ordinary-source composition, endpoint-only dispatch,
   domain-as-library vocabulary, joined provenance, observer redaction, and the
   primary fixture/expected-result/bypass falsifiers.
2. Compose the smallest actual four-locus source and run it through the
   accepted checker, projector, and SYS-4 runtime.
3. Connect attack RMW, designated publication/consume, relation/fallback,
   leave, presentation gap, and reacquire without opening a new semantic
   frontier.
4. Add the required auth/revocation, optional verification, ST save/restore,
   bounded accepted/rejected patch, and typed negative paths.
5. Build one joined typed causal view and the smallest reproducible command/
   walkthrough surface without freezing public layout or grammar.
6. Run focused and close-boundary validation, synchronize the single SYS-5
   report/status surfaces, obtain independent usability/semantics/code-quality
   review, pin the accepted cut, then activate SYS-6.

## self-driven macro phase reading

| Macro | Current state | Startability |
| --- | --- | --- |
| 0 repository memory/governance | ADR-0026/Plan 249 active; SYS-0--SYS-4 closed; SYS-5 active | SYS status sync startable |
| 1 semantic kernel | kernel/carrier + bounded ST/OW1 + source-derived projection/dispatch accepted | SYS-5 composes existing semantics; no new frontier |
| 2 parser-free historical evidence | retained; not current architecture | maintenance only |
| 3 source/checker/runtime | source-first M10 plus accepted kernel/backend/projector/endpoint runtime | SYS-5 workflow integration active |
| 4 executable samples | no joined I2 toy command/view yet | **active SYS-5** |
| 5 theorem/model-check bridge | OBL-058 bounded; OBL-059/060 and SYS-4 finite runtime evidence runtime-monitored | SYS-5 evidence now; SYS-6 assurance next |
| 6 generated/distributed fabric | generated artifacts cross actual in-process endpoints; real transport absent | SYS-5 integration active |
| 7 toolchain/backend | ST/eligible-OW1 endpoint runtime ready; user command/devtools absent | SYS-5 active |
| 8 upper application | four-locus toy is current sample/library consumer | SYS-5 active; no Core promotion |

## user decision gates

No owner decision is required to continue SYS-5--SYS-7 inside ADR-0026 unless
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
| four-locus source/module boundary | SYS-5 runtime and walkthrough | ordinary checked source reaches accepted projector/runtime; domain words remain sample/library terms; no fixture-name plan selection | use the smallest current Surface; no final grammar or domain Core promotion |
| relation/fallback/timing composition | SYS-5 causal view / SYS-6 relation profile | actual A-primary/B-fallback relation, A leave semantic invalidation, ViewerC presentation-only gap, and fresh reacquire preserve distinct lineage | accepted finite relation fragment only; no arbitrary DAG theorem |
| joined typed devtools schema | SYS-5 user workflow / SYS-6 correspondence | one observer-safe source/Core/artifact/edge/runtime/state view plus redaction and no-manual-join falsifiers | internal/provisional view; no public telemetry ABI or raw secret/payload leak |
| auth and optional verification example | SYS-5 negative workflow / SYS-6 containment profile | attach/remove or revocation plus one explicit residual/discharge path with typed failure/provenance | transport/provider remains non-authority; no general verifier composition claim |
| local command spelling | SYS-5 walkthrough | short build/project, run/fault, and inspect path over actual layers | align with existing CLI; do not freeze final public commands |
| SYS-6 handoff inventory | SYS-6 profile | exact source/artifact/trace cut, positive/falsifier matrix, evidence class, non-claims, and replay commands | preparation only; do not start SYS-6 before SYS-5 closes |
| broad-I1 carrier residual | lifecycle closeout | OPEN-026/027 and full internal freeze inventory | do not weaken exit criteria or infer acceptance from SYS-4/5 |

Do not open a WRK unless the active SYS-5 blocker cannot fit the active
milestone's single report and all direct-consumer/falsifier/adoption-discard
conditions in ADR-0026 are met. Do not create a second SYS-5 report for
registration, metadata, or snapshot synchronization.

## maintenance tasks

- Preserve `canon > LAB`, official lifecycle T1, and exact evidence classes.
- Preserve M10 cut, closed Plan 247, accepted SYS-1--SYS-4 cuts, and
  `ded622fe...` only as partial SYS-3 regression history.
- Preserve legacy direct M8 same-delivery `AlreadyConsumed` and accepted M10
  duplicate-delivery behavior. The SYS-4 wrapper is narrower: one exact
  source/Core semantic consume plus validated same-consumer return.
- Preserve SYS-4 non-claims: crate-private/internal carrier only, no public
  compatibility, durable/distributed cut, OW1 cut/patch, arbitrary patch,
  general theorem, SYS-5 user workflow, or SYS-6 conformance.
- Keep Plan 249 as the sole active roadmap and SYS-5 as the sole active
  semantic milestone. Do not start SYS-6 assurance before SYS-5 closes.
- Ordinary `.mir` source and checked Core are semantic authority; schedule,
  endpoint, viewer, and sample fixture cannot invent edges, authority, state,
  expected results, or fallback lineage.
- Surface gains no worker/mailbox/atomic/`memory_order_*` vocabulary. World,
  Avatar, Bird, and Viewer remain sample/library terms, never Core primitives.
- Transport/session/provider/worker/viewer identity and receipt remain
  non-authority. Observer output stays typed and redacted.
- Update `samples_progress.md` only when SYS-5 changes an actual runnable sample
  path, validation command, debug/view surface, or blocker.

## non-promoted references

- Active authority/roadmap: PROPOSAL-029, ADR-0026, Plan 249.
- SYS-1 contract: PROPOSAL-030, ADR-0027, Report 2593.
- SYS-2 contract: PROPOSAL-031, ADR-0028, OBL-058/059, Report 2594.
- SYS-3 contract: PROPOSAL-032, ADR-0029, spec/12, OBL-060, Report 2595.
- SYS-4 acceptance: PROPOSAL-033, ADR-0030, spec/13, cut `22196f93...`, and
  the milestone report; bounded runtime-monitored evidence only.
- Official lifecycle: `mirrorea_canon/plan/01-phases.md`.
- Proof/evidence status: `mirrorea_canon/theory/11-metatheory-ledger.md`.
- Runnable sample dashboard: `samples_progress.md` (SYS-5 updates it only when
  an actual workflow/view changes).
