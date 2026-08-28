# Current Task Map (LAB)

最終更新: 2026-08-28 14:09 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB evidence, history, implementation, or operational
state; canon wins.

## document role

This is the repository-wide current task-map snapshot. Canon holds normative
decisions, Plan 249 is the sole active execution roadmap, Plan 247 is the
closed M0--M10 baseline, and milestone reports hold detailed evidence.
Historical plan “next” entries are not active tasks.

## current promoted package

“Promoted” here means selected by the owner-authorized ADR-0026 program and
current roadmap. It is not Canon L2 promotion, Gate/Phase exit, proof
completion, public-product acceptance, or public compatibility freeze.

**Active: SYS-6 I2 assurance, conformance, and lifecycle closeout.**
SYS-0--SYS-5 are completed/closed. Accepted SYS-5 implementation/evidence cut
`53a21e64b5a17e24b522f720db10b6e539c058e0` composes the checked projector and
generated endpoint runtime into one ordinary-source four-locus headless toy.
The provisional internal `project-loci`, `run-local`, and `inspect` commands
expose generated locus programs/communication and one observer-safe joined
source→Core→artifact→edge→occurrence view. The actual path includes owner
RMW, designated publication/consume, source-derived leave/fallback/fresh
reacquire, presentation-only gap, ST save/restore, accepted/rejected patch,
revocation/failure, and optional verification. OBL-062 classifies only this
finite evidence as `runtime-monitored`. Leave/fallback/fresh use the bounded
clone-prepared ST failure-atomic candidate, and post-leave cut/restore retains
the exact retired lineage; neither is a hidden transaction or durable cut.

Sources: `mirrorea_canon/adr/ADR-0026.md`,
`mirrorea_canon/adr/ADR-0031.md`,
`mirrorea_canon/spec/14-sys5-local-toy-devtools.md`,
`mirrorea_canon/spec/13-sys4-in-process-generated-dispatch.md`,
`mirrorea_canon/spec/12-sys3-per-locus-projection.md`, and
`plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`.

Direct consumer: SYS-7 consumes only an accepted SYS-6 I2 boundary to write an
inactive I3 goal/entry contract. SYS-7 does not activate or implement real
transport.

Current SYS-6 direct blockers:

1. Define one finite source-first I2 row inventory beginning at ordinary source
   and retaining checked Core, locus artifact, generated plan, runtime
   occurrence, and observer-safe result identity. Do not make M10 release/hash
   orchestration the runtime architecture.
2. Cover projection determinism, artifact-owner preservation, generated-
   communication completeness, actual dispatch, selected ST/OW semantic
   correspondence, owner data-race freedom, required visibility edges, no
   hidden/direct remote store, no source-free authority/state minting, and
   typed failure containment.
3. Cover relation projection coherence, semantic/presentation fallback
   separation, designated-result non-reexecution, save/patch no-stale/no-
   mutation, and observer-safe devtools using positive rows and representative
   falsifiers.
4. Add the smallest provisional `conform-i2` command/report with exact source
   and implementation cut, replay commands, row outcomes, evidence class,
   residual risk, and non-claim. Expected-result lookup or self-certified row
   omission must not pass.
5. Preserve M10 regression, complete the single SYS-6 report, and obtain
   independent assurance/lifecycle review. Move broad PHASE-I1 or official I2
   entry/exit only if their pre-existing Canon criteria are actually met;
   otherwise record exact residuals without weakening them.

Completion signal: the finite I2 profile rejects the primary falsifiers,
reproduces the accepted source→artifact→dispatch→trace capability, classifies
every claim accurately, and independent review finds no major counterexample.
SYS-7 then becomes the sole active goal.

Official theory remains T1. Broad PHASE-I1 exit and official I2 entry/exit stay
unaccepted; OPEN-026/027 and the full internal carrier freeze remain exact
broad-I1 residuals. SYS-5 close does not itself satisfy SYS-6 conformance or
lifecycle acceptance.

## ordered self-driven packages

Packages execute in the fixed order; only one SYS semantic frontier is active.

| Package | Capability / evidence | Current state | Macro position / rough estimate |
| --- | --- | --- | --- |
| SYS-0 | baseline, authority, one roadmap, goal alignment | **completed / closed**; Report 2592 | Macro 0 front; closed |
| SYS-1 | semantic runtime kernel; internal owner/designated carrier | **completed / closed** at `94e3707c...`; Report 2593 | Macro 1/3/7 front; closed |
| SYS-2 | ST/OW1 backend, M9 generation visibility, ten-edge finite model | **completed / closed** at `920d3fe0...`; OBL-058/059, Report 2594 | Macro 3/5/7 middle; closed |
| SYS-3 | checked Core → per-locus artifacts and generated plans | **completed / closed** at `3013e7fe...`; OBL-060, Report 2595 | Macro 3/6/7 front; closed |
| SYS-4 | generated-plan-only independent in-process locus dispatch | **completed / closed** at `22196f93...`; OBL-061, Report 2596 | Macro 6/7 middle; closed |
| SYS-5 | four-locus headless toy + joined typed devtools | **completed / closed** at `53a21e64...`; OBL-062, Report 2597 | Macro 4/6/8 middle; closed |
| SYS-6 | finite I2 conformance/assurance and lifecycle closeout | **active** | Macro 0/5/6 close; heavy, multi-day |
| SYS-7 | inactive I3 goal and entry contract only | next/terminal after SYS-6; no transport implementation | Macro 0/6 reserve; small, sub-day |

Active SYS-6 execution order:

1. Freeze the finite profile inputs and source/Core/artifact/edge/occurrence row
   identities from accepted SYS-3--SYS-5 cuts.
2. Write RED falsifiers for omitted communication, moved owner, direct remote
   store, source-free mint, ST/OW mismatch, relation/fallback/designated drift,
   stale save/patch mutation, and observer leakage.
3. Implement the minimal conformance producer/verifier separation and
   provisional `conform-i2` command without changing runtime meaning.
4. Run positive/falsifier rows, selected ST/OW correspondence, M10 regression,
   and exact evidence classification checks.
5. Audit lifecycle criteria and record either evidence-backed acceptance or
   exact residuals; do not weaken broad-I1/I2 criteria to fit the milestone.
6. Synchronize the one SYS-6 report/status package, obtain independent review,
   pin/push the accepted cut, and activate SYS-7 only after SYS-6 closes.

## self-driven macro phase reading

| Macro | Current state | Startability |
| --- | --- | --- |
| 0 repository memory/governance | ADR-0026/Plan 249 active; SYS-0--SYS-5 closed; SYS-6 active | status/lifecycle inventory startable |
| 1 semantic kernel | finite semantics, internal carrier/backend, projection, dispatch, and toy consumer accepted | assurance only; no new semantic frontier |
| 2 parser-free historical evidence | retained; not current architecture | maintenance only |
| 3 source/checker/runtime | source-first M10 plus kernel/backend/projector/endpoint/toy path accepted | SYS-6 conformance active |
| 4 executable samples | four-locus headless toy and project/run/inspect accepted | regression/assurance; public product later |
| 5 theorem/model-check bridge | OBL-058 bounded; OBL-059--062 runtime-monitored | evidence classification active |
| 6 generated/distributed fabric | generated artifacts execute across in-process endpoints and drive the toy | SYS-6 assurance; real transport deferred |
| 7 toolchain/backend | ST/eligible-OW1 plus provisional local commands | `conform-i2` active |
| 8 upper application | toy is an accepted sample/library consumer | assure only; no Core promotion |

## user decision gates

No owner decision is required to continue SYS-6--SYS-7 inside ADR-0026 unless
an owner-reserved stop condition becomes real.

| Overview | Impact | Major options | Current recommendation / view |
| --- | --- | --- | --- |
| North Star or safety/privacy/redaction/no-stale guarantee change | whole project semantics | preserve; or explicitly weaken | preserve; stop if weakening is required |
| domain vocabulary as Core primitive | Core architecture | keep library/sample; or promote | keep library/sample; stop if promotion is unavoidable |
| hidden multi-owner transaction | authority/atomicity | explicit operations; or hidden transaction | preserve explicit operations; stop if hidden transaction is unavoidable |
| public API/ABI/wire/devtools freeze | external compatibility | keep internal/provisional; or freeze | keep provisional; stop before irreversible freeze |
| real transport selection/implementation | I3 architecture | defer; or choose now | defer to future owner program; SYS-7 only writes inactive entry contract |
| production/publication/paid resources | external state/risk | remain local; or deploy | remain local; stop for owner authority |
| irreversible observable semantic tie | migration compatibility | Constitution orders; or owner decides | use priority order; stop only if tied and non-migratable |
| reproducible North-Star contradiction | parent program validity | revise program; or revise North Star | return decision bundle; do not weaken silently |

Official T1, deferred general OBLs, open final grammar/public ABI, incomplete
I3+, and unoptimized performance are not blockers requiring owner input.

## research discovery items

These are resolved inside active SYS-6 from evidence; they are not owner
decision requests and do not open separate semantic frontiers.

| Item | Direct consumer | Evidence needed | Boundary |
| --- | --- | --- | --- |
| finite I2 row inventory | SYS-6 profile | every accepted Core operation/failure family maps to artifact/plan/runtime/trace rows or an explicit not-applicable reason | finite profile only; no general completeness theorem |
| ST/OW selected correspondence | SYS-6 assurance | reuse accepted SYS-2/SYS-4 backend evidence with exact shared observable domain and fresh falsifier controls | no arbitrary scheduler/fairness or multi-owner OW claim |
| projection/communication completeness | SYS-6 assurance | deterministic artifact identity, owner preservation, and generated-edge coverage for accepted Core operations | accepted finite fragment only; no public artifact schema |
| relation/fallback/designated assurance | SYS-6 assurance | project/evaluate coherence, semantic/presentation separation, fresh lineage, and no designated re-execution rows | current two-anchor/profile scope; arbitrary DAG deferred |
| save/patch assurance | SYS-6 assurance | no stale resurrection and rejected-patch no-mutation rows over accepted ST cut/patch | no OW1 cut/patch, durability, migration, or lifecycle commutation theorem |
| observer-safe conformance output | SYS-6 user command | one joined report with exact cuts/classes and negative leak controls | internal/provisional; no public telemetry ABI |
| broad-I1/I2 lifecycle inventory | SYS-6 closeout | compare fresh evidence to existing Canon criteria and retain exact residuals | no criteria weakening or inference from program labels |
| SYS-7 handoff | SYS-7 | accepted I2 boundary, transport-as-non-authority, failure/order requirements, candidate limit two | preparation only; no I3 activation or transport work |

Do not open a WRK unless the active SYS-6 blocker cannot fit the active
milestone's single report and all direct-consumer/falsifier/adoption-discard
conditions in ADR-0026 are met. Do not create a second SYS-6 report for
registration, metadata, or snapshot synchronization.

## maintenance tasks

- Preserve `canon > LAB`, official lifecycle T1, and exact evidence classes.
- Preserve M10 cut, closed Plan 247, accepted SYS-1--SYS-5 cuts, and
  `ded622fe...` only as partial SYS-3 regression history.
- Preserve legacy direct M8 same-delivery `AlreadyConsumed`; the SYS-4 wrapper
  remains the narrower one-consume/same-consumer validated-return contract.
- Preserve SYS-5 non-claims: internal/provisional commands and report only; no
  public compatibility, browser/View product, OW1 whole-workflow cut/patch,
  durable/distributed state, arbitrary relation/lifecycle theorem, SYS-6
  conformance acceptance, or lifecycle movement.
- Reopen SYS-5 only for inferred anchor placement, caller-minted lifecycle
  authority, M8-before-M9 mutation, missing exact leave→fresh join, partial
  failed-candidate mutation, invalid post-leave restore, invented/leaking
  causal output, filename/expected-result semantics, or an unusable SYS-6
  direct-consumer boundary.
- Keep Plan 249 as the sole current roadmap and SYS-6 as the sole active
  semantic milestone. Do not start SYS-7 before SYS-6 closes.
- Ordinary `.mir` source and checked Core remain semantic authority; schedule,
  endpoint, viewer, profile, release identity, and sample fixture cannot invent
  edges, authority, state, expected results, or fallback lineage.
- Surface gains no worker/mailbox/atomic/`memory_order_*` vocabulary. World,
  Avatar, Bird, and Viewer remain sample/library terms, never Core primitives.
- Transport/session/provider/worker/viewer/profile identity and receipt remain
  non-authority. Observer and conformance output stays typed and redacted.
- Update `samples_progress.md` only when SYS-6 changes an actual runnable
  command, profile validation anchor, debug/view surface, or blocker.

## non-promoted references

- Active authority/roadmap: PROPOSAL-029, ADR-0026, Plan 249.
- SYS-1 contract: PROPOSAL-030, ADR-0027, Report 2593.
- SYS-2 contract: PROPOSAL-031, ADR-0028, OBL-058/059, Report 2594.
- SYS-3 contract: PROPOSAL-032, ADR-0029, spec/12, OBL-060, Report 2595.
- SYS-4 contract: PROPOSAL-033, ADR-0030, spec/13, OBL-061, Report 2596,
  accepted cut `22196f93...`.
- SYS-5 contract: PROPOSAL-034, ADR-0031, spec/14, OBL-062, Report 2597,
  accepted cut `53a21e64...`.
- Official lifecycle: `mirrorea_canon/plan/01-phases.md`.
- Proof/evidence status: `mirrorea_canon/theory/11-metatheory-ledger.md`.
- Runnable sample dashboard: `samples_progress.md`.
