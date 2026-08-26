# Current Task Map (LAB)

最終更新: 2026-08-26 20:13 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins.

## document role

This is the repository-wide current task-map snapshot. Canon holds normative
decisions, Plan 249 holds the active long-term execution sequence, Plan 247
holds the closed M0--M10 baseline, and milestone reports hold work evidence.
Historical plan “next” entries are not active tasks.

## current promoted package

The legacy word `promoted` in this heading is required by the documentation
validator. It means the current LAB work package, not Canon L2, Gate, Phase,
proof, conformance, or public-product promotion.

**Active: SYS-1 runtime kernel / conformance separation and internal carrier
boundary.** ADR-0026 authorizes the bounded Mirrorea I2 Systems Foundation
program and Plan 249 is its sole current roadmap. SYS-0 baseline/goal alignment
is completed/closed at accepted integration cut `350e04b4...`; SYS-2
concurrency/memory/effect-handler refinement is next.

Sources: `mirrorea_canon/adr/ADR-0026.md`,
`mirrorea_canon/plan/01-phases.md`,
`mirrorea_canon/architecture/04-runtime-carriers.md`, and
`plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`.

Current SYS-1 direct blockers:

1. extract semantic state/occurrence production from the M10
   conformance/release/profile facade without changing accepted behavior;
2. fix the smallest non-public request/result/receipt carrier with complete
   source/Core provenance and no authority/state minting;
3. expose typed effect request → admitted handler → result/failure ordering
   without collapsing transport, auth, projection, or persistence;
4. record whether broad PHASE-I1 carrier criteria are met or the exact residual.

Completion signal: positive and primary-falsifier tests, preserved M10
regression, exact internal/public and lifecycle non-claims, independent review,
Report 2593, commit/push, and remote parity; only then does SYS-2 become active.

Official theory remains T1. Broad PHASE-I1 exit and official I2 lifecycle
entry/exit are not implied by SYS-0 or by starting SYS-1.

## ordered self-driven packages

Rough estimates are effort bands, not elapsed-time commitments. Every package
closes before the next semantic frontier opens.

| Order | Task package | Dependency / completion signal | Macro position / rough estimate |
| --- | --- | --- | --- |
| SYS-0 | baseline, owner authority, one roadmap, goal alignment | **completed / closed**; accepted integration cut + Report 2592 | Macro 0 front; closed |
| SYS-1 | extract semantic runtime kernel; internal request/result/receipt and effect seam | **active**; dependency inversion + carrier falsifiers + review | Macro 1/3/7 front; medium-heavy |
| SYS-2 | define/run ST and OW backend refinement | **next** after SYS-1; high-level edge mapping + litmus counterexamples + ST/OW agreement | Macro 3/5/7 middle; heavy |
| SYS-3 | generate per-locus artifacts and communication/effect/observation/persistence plans | SYS-1/2 contracts; deterministic 3+ locus projection + malformed negatives | Macro 6/7 front; heavy |
| SYS-4 | run generated artifacts through independent in-process locus endpoints | SYS-3 artifacts; actual endpoint dispatch + ST/OW + save/patch/replay negatives | Macro 6/7 middle; heavy |
| SYS-5 | four-locus headless toy world and one joined typed devtools view | SYS-4 runtime; user walkthrough + semantic/usability/security review | Macro 4/8 middle; heavy |
| SYS-6 | finite I2 conformance/assurance and lifecycle closeout | SYS-3--5 accepted cuts; exact source→trace profile + evidence classes + independent review | Macro 0/5/6 close; heavy |
| SYS-7 | inactive I3 goal and entry contract only | accepted SYS-6 boundary; reviewed transport-neutral contract, no implementation | Macro 0/6 reserve; small |

Active SYS-1 execution order:

1. Map the M10 facade's semantic-state ownership/import direction and identify
   the smallest extractable kernel boundary.
2. Write failing tests for kernel independence, complete carrier identity,
   source-free mutation, authority minting, receipt non-authority,
   stale/duplicate/mismatched result, and effect-handler ordering.
3. Extract the minimal typed kernel and internal carrier without public naming
   or release/profile dependencies.
4. Adapt the conformance shell to consume the kernel, rerun M10/focused/full
   changed-layer validation, and conduct one independent falsification review.
5. Close Report 2593 (the one future SYS-1 report), synchronize snapshots,
   commit/push, verify remote parity, then start SYS-2.

## self-driven macro phase reading

| Macro | Current reading | Startability |
| --- | --- | --- |
| 0 repository memory/governance | ADR-0026/Plan 249 active; Plan 247 closed | SYS milestone sync startable |
| 1 semantic kernel | accepted finite semantics; reusable kernel boundary missing | **active SYS-1** |
| 2 parser-free validation | historical evidence only | maintenance; do not make architecture |
| 3 source/checker/runtime | source-first M10 baseline; kernel/concurrency work next | SYS-1 then SYS-2 |
| 4 executable sample expansion | existing roots unchanged | wait for SYS-5 |
| 5 theorem/model-check bridge | exact finite evidence retained | SYS-2/6 in order |
| 6 generated/distributed fabric | no per-locus executable generation/dispatch yet | wait for SYS-3/4 |
| 7 toolchain/backend | projector/runtime/devtools boundaries planned | SYS-1 active, then SYS-2--5 |
| 8 applications | four-locus toy is a later domain consumer | wait for SYS-5; no Core promotion |

## user decision gates

No owner decision is required to continue SYS-1--SYS-7
inside ADR-0026 unless one of the complete reserved conditions below is reached.

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

These are self-driven bounded findings inside their named milestones, not user
decision requests and not reasons to open parallel semantic frontiers.

| Item | Direct consumer / blocker reduced | Acceptance use | Stop/discard condition |
| --- | --- | --- | --- |
| smallest internal reply/receipt carrier | SYS-2/3; OPEN-030 and kernel seam | positive lifecycle + no-mint/stale/duplicate negatives | discard fields without direct consumer; stop before public freeze |
| broad-I1 exact residual | lifecycle record; avoids false acceptance | accept only actual carrier criteria or record residual | never weaken criteria to match milestone name |
| OW primitive/profile | SYS-3/4; threaded owner execution | finite trace refinement and counterexample detection | no lock-free/general memory rabbit hole |
| finite relation-DAG extension boundary | SYS-4/5; avoids permanent two-anchor Core limit | one three-step/shared-ancestor pressure case | no arbitrary DAG theorem without direct consumer |
| provisional CLI spelling | SYS-5/6 user workflow | smallest existing-convention command set | no public compatibility commitment |
| finite I2 evidence decomposition | SYS-6 acceptance | exact lean/model/runtime/deferred classification | no bounded-to-general proof widening |

New WRK admission still requires a named direct consumer, current blocker,
reason the milestone report cannot hold it, falsifier, and adoption/discard
rule. No new WRK is currently justified.

## maintenance tasks

- Preserve `canon > LAB` and keep official lifecycle in Canon plan/01.
- Preserve M10 cut and closed Plan 247; do not rewrite historical acceptance.
- Keep Plan 249 as the only active roadmap and one SYS semantic frontier.
- Maintain one report per milestone; metadata/snapshot sync stays in that
  report.
- At each close, update Plan 249, `Documentation.md`, `docs/project-status.md`,
  `progress.md`, `tasks.md`, and inspect `samples_progress.md` update need.
- Keep public contract, real transport, production, browser renderer, and I3
  implementation out of SYS-0--SYS-7.
- Check resources before heavy builds and use verified external workdir policy.
- Do not commit secrets, force-push, rewrite history, or overwrite other
  writers' shared-worktree changes.

## non-promoted references

- Active authority/roadmap:
  `mirrorea_canon/meta/proposals/PROPOSAL-029-mirrorea-i2-systems-foundation.md`,
  `mirrorea_canon/adr/ADR-0026.md`,
  `plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`.
- Official lifecycle/operating rules:
  `mirrorea_canon/plan/01-phases.md`,
  `mirrorea_canon/plan/02-operating-model.md`.
- Current SYS-1 technical blocker:
  `mirrorea_canon/architecture/04-runtime-carriers.md`,
  `mirrorea_canon/spec/05-runtime-semantics.md`.
- Closed baseline:
  `mirrorea_canon/adr/ADR-0025.md`,
  `plan/247-mir-theory-v0-i1plus-current-roadmap.md`,
  `docs/reports/2591-mir-theory-v0-i1plus-milestone-10-conformance-closeout.md`.
- SYS-0 evidence:
  `docs/reports/2592-mirrorea-i2-systems-foundation-sys0-baseline-goal-alignment.md`.
- Proof status: `mirrorea_canon/theory/11-metatheory-ledger.md`.
- Runnable sample dashboard: `samples_progress.md` (unchanged by SYS-0).

No new Gate/Phase exit, broad PHASE-I1/I2 acceptance, general OBL discharge,
public contract, transport/product deployment, or sample-workflow completion is
claimed by this task map.
