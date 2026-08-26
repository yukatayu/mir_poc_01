# Current Task Map (LAB)

最終更新: 2026-08-26 23:09 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins.

## document role

This is the repository-wide current task-map snapshot. Canon holds normative
decisions, Plan 249 holds the sole active execution sequence, Plan 247 holds
the closed M0--M10 baseline, and milestone reports hold evidence. Historical
plan “next” entries are not active tasks.

## current promoted package

The legacy word `promoted` in this heading is required by the documentation
validator. It means current LAB work package, not Canon L2, Gate, Phase, proof,
conformance, or public-product promotion.

**Active: SYS-2 concurrency, memory, and effect-handler refinement.** SYS-0 and
SYS-1 are completed/closed. ADR-0027 accepts source cut `94e3707c...` as the
crate-private ordinary-source/generic-OwnerEvent kernel and narrow owner/
designated-input carrier; SYS-3 per-locus projection/artifact generation is
next.

Sources: `mirrorea_canon/adr/ADR-0026.md`,
`mirrorea_canon/adr/ADR-0027.md`,
`mirrorea_canon/plan/01-phases.md`,
`mirrorea_canon/architecture/04-runtime-carriers.md`, and
`plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`.

Current SYS-2 direct blockers:

1. define ST and one-owner-worker (OW) execution profiles over the exact SYS-1
   owner/designated-input lifecycle;
2. map request→serve, send/publish→receive/observe, witness/grant/revoke→use,
   verdict→activation, cut→later transition, and relation epoch→sample to
   abstract happens-before and operation linearization;
3. close the immutable-M9-snapshot residual so revoke-after-enqueue/serve has
   explicit visibility and a removed edge yields a bounded counterexample;
4. show selected ST/OW semantic correspondence and owner-state data-race
   freedom without introducing Surface `memory_order_*`, lock-free goals, or a
   generic provider registry.

Completion signal: positive ST/OW execution, required litmus/model
counterexamples, exact runtime/model/proof classification, preserved SYS-1 and
M10 regressions, independent review, Report 2594, commit/push, and clean remote
parity. Only then does SYS-3 become active.

Official theory remains T1. OPEN-030 is resolved only for the ADR-0027 narrow
internal contract; OPEN-026/027 and full carrier freeze keep broad PHASE-I1 and
official I2 lifecycle entry/exit unaccepted.

## ordered self-driven packages

Rough estimates are effort bands, not elapsed-time commitments. Each semantic
frontier closes before the next opens.

| Order | Task package | Dependency / completion signal | Macro position / rough estimate |
| --- | --- | --- | --- |
| SYS-0 | baseline, owner authority, one roadmap, goal alignment | **completed / closed**; Report 2592 | Macro 0 front; closed |
| SYS-1 | semantic runtime kernel; owner/designated-input internal carrier | **completed / closed** at `94e3707c...`; runtime-monitored, Report 2593 | Macro 1/3/7 front; closed |
| SYS-2 | define/run ST and OW backend refinement | **active**; high-level edge mapping + litmus counterexamples + selected ST/OW agreement | Macro 3/5/7 middle; heavy |
| SYS-3 | generate per-locus artifacts and communication/effect/observation/persistence plans | **next**; consumes SYS-1/2 contracts | Macro 6/7 front; heavy |
| SYS-4 | run generated artifacts through independent in-process locus endpoints | SYS-3 artifacts; actual endpoint dispatch + ST/OW + save/patch/replay negatives | Macro 6/7 middle; heavy |
| SYS-5 | four-locus headless toy world and one joined typed devtools view | SYS-4 runtime; user walkthrough + semantic/usability/security review | Macro 4/8 middle; heavy |
| SYS-6 | finite I2 conformance/assurance and lifecycle closeout | SYS-3--5 accepted cuts; exact source→trace profile + evidence classes | Macro 0/5/6 close; heavy |
| SYS-7 | inactive I3 goal and entry contract only | accepted SYS-6 boundary; transport-neutral contract, no implementation | Macro 0/6 reserve; small |

Active SYS-2 execution order:

1. Pin the abstract ordering/refinement contract to the SYS-1 carrier and list
   each concrete ST/OW direct consumer.
2. Add the required litmus/model falsifiers before changing backend behavior,
   including revoke/use, publication/observation, patch/request, save/mutation,
   relation epoch/sample, two owner RMWs, and presentation-gap nonmutation.
3. Implement the smallest OW mailbox/worker profile while preserving ST as the
   deterministic reference; prefer safe channels/mutexes over lock-free work.
4. Compare allowed ST/OW observations, classify each claim accurately, rerun
   SYS-1/M10/workspace validation, and obtain independent review.
5. Close Report 2594, synchronize snapshots, commit/push, verify parity, then
   and only then activate SYS-3.

## self-driven macro phase reading

| Macro | Current reading | Startability |
| --- | --- | --- |
| 0 repository memory/governance | ADR-0026/Plan 249 active; SYS-0/1 closed | SYS close sync startable |
| 1 semantic kernel | narrow kernel/carrier accepted runtime-monitored | consume in SYS-2; reopen only on listed falsifier |
| 2 parser-free validation | historical evidence only | maintenance; do not make architecture |
| 3 source/checker/runtime | source-first M10 + SYS-1 kernel; concurrency gap | **active SYS-2** |
| 4 executable sample expansion | existing roots unchanged | wait for SYS-5 |
| 5 theorem/model-check bridge | finite evidence retained; concurrency evidence current | **active SYS-2** |
| 6 generated/distributed fabric | no per-locus executable generation/dispatch yet | wait for SYS-3/4 |
| 7 toolchain/backend | kernel available; ST/OW backend current | **active SYS-2**, then SYS-3--5 |
| 8 applications | four-locus toy is a later domain consumer | wait for SYS-5; no Core promotion |

## user decision gates

No owner decision is required to continue SYS-2--SYS-7 inside ADR-0026 unless
one of the complete reserved conditions below is reached.

| Item | Impact | Main options | Current recommendation |
| --- | --- | --- | --- |
| public API/ABI/wire freeze | irreversible external compatibility | separately authorize/freeze; or keep internal/provisional | keep internal throughout this program |
| real transport | future I3 architecture | later compare at most two candidates; or defer | SYS-7 entry contract only; no selection/implementation now |
| production/publication | external state and user risk | separately authorize/deploy; or defer | defer; owner-reserved |
| North Star/guarantee change | project meaning/safety | explicit owner change; or preserve | preserve; stop if change is required |
| domain vocabulary as Core | architecture separability | promote explicitly; or keep library/sample | keep World/Avatar/Bird outside Core |
| hidden multi-owner transaction | authority/atomicity semantics | introduce explicitly; or preserve visible operations | preserve; stop if unavoidable |
| user data, secret, or paid-resource danger | safety/external cost | separately authorize; or avoid | avoid and stop |
| irreversible semantic tie | incompatible observable semantics with no later migration | owner chooses A/B | stop with decision bundle |
| reproducible parent-goal/North-Star conflict | program feasibility | change North Star/goal; or stop program | stop with counterexample evidence |

## research discovery items

These are bounded findings inside the named milestone, not user decisions or
parallel semantic frontiers.

| Item | Direct consumer / blocker reduced | Acceptance use | Stop/discard condition |
| --- | --- | --- | --- |
| ST/OW happens-before map | SYS-3/4 backend contract | exact high-level edge→backend guarantee table | no general memory model or Surface atomics |
| immutable-M9 revocation visibility | SYS-2 owner/designated-input use | revoke-after-enqueue/serve litmus and fail-closed result | reopen SYS-1 only if its immutable boundary cannot be conservatively refined |
| OW primitive choice | SYS-3/4 executable artifacts | selected safe mailbox/worker profile | compare at most current design + one viable alternative; no lock-free work |
| finite relation-DAG extension boundary | SYS-4/5 | one three-step/shared-ancestor pressure case | defer to SYS-3; no arbitrary DAG theorem |
| provisional CLI spelling | SYS-5/6 user workflow | smallest existing-convention command set | no public compatibility commitment |
| finite I2 evidence decomposition | SYS-6 acceptance | exact lean/model/runtime/deferred classes | no bounded-to-general proof widening |

No new WRK is justified. A future WRK still needs a named direct consumer,
current blocker, reason the milestone report cannot hold it, falsifier, and
adoption/discard rule.

## maintenance tasks

- Preserve `canon > LAB`, official lifecycle T1, and exact evidence classes.
- Preserve M10 cut, closed Plan 247, and accepted SYS-1 cut; do not reinterpret
  specialized M10 runners as kernel evidence.
- Keep Plan 249 as the only active roadmap and one SYS semantic frontier.
- Maintain one report per milestone; snapshot/metadata sync stays in it.
- At each close, update Plan 249, `Documentation.md`, `docs/project-status.md`,
  `progress.md`, `tasks.md`, and inspect `samples_progress.md` update need.
- Keep public contract, real transport, production, browser renderer, and I3
  implementation out of SYS-0--SYS-7.
- Check resources before heavy builds and preserve other shared-worktree edits.
- Do not commit secrets, force-push, rewrite history, or delete user work.

## non-promoted references

- Active authority/roadmap: PROPOSAL-029, ADR-0026, Plan 249.
- SYS-1 accepted internal contract: PROPOSAL-030, ADR-0027,
  `mirrorea_canon/architecture/04-runtime-carriers.md`,
  `mirrorea_canon/spec/05-runtime-semantics.md`.
- Official lifecycle: `mirrorea_canon/plan/01-phases.md`.
- Closed baseline: ADR-0025, Plan 247, Report 2591.
- Milestone evidence: Report 2592 (SYS-0), Report 2593 (SYS-1).
- Proof status: `mirrorea_canon/theory/11-metatheory-ledger.md`.
- Runnable sample dashboard: `samples_progress.md` (unchanged by SYS-1).

No new Gate/Phase exit, broad PHASE-I1/I2 acceptance, general OBL discharge,
public contract, transport/product deployment, or sample-workflow completion is
claimed by this task map.
