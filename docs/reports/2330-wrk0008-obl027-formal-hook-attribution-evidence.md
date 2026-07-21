# Report 2330 - WRK-0008 OBL-027 formal-hook attribution evidence

- Date: 2026-07-22 05:27 JST
- Author / agent: Codex
- Scope: registered existing-current-L2 experiment and LAB evidence retention
- Decision levels touched: none; Canon and implementation are read-only

## Objective

Execute WRK-0008 exactly through its existing-lane route and record whether
the current formal hook distinguishes the same-Place `atomic_cut` frontier
needed for its own `rollback_cut_non_interference` label.

## Scope and assumptions

The committed WRK-0008 record, ADR-0014, Canon theory/04, theory/11, and
BND-003 bound this package. The result may classify a LAB formal-hook artifact
only. It cannot prove, disprove, refine, discharge, or move OBL-027, choose a
checker/prover carrier, or change source behavior.

The evidence commit is intentionally limited to the declared `plan` LAB root
and this direct numbered report. The subsequent manifest package will update
the working record and the reader-facing current snapshots after this evidence
commit has a stable Git identity.

## Start state / dirty state

Started from clean pushed `main` at
`31365085cb1826e423dddf5f43db340623832301`, the WRK-0008 preregistration
commit. No user changes were present, reverted, or overwritten.

## Documents consulted

- Canon README/MAP, ADR-0014, theory/04, theory/11, BND-003, and the
  committed WRK-0008 record.
- The four current-L2 source examples, sample dashboard, formal-hook support,
  detached-bundle support, smoke helper, focused tests, and regression helper.
- `plan/158`, `plan/00-index.md`, the report template, and current snapshots
  to determine the evidence/manifest split.

## Actions taken

1. Created a unique `/tmp` artifact root and ran the preregistered fail-fast
   command sequence without modifying fixtures, helpers, schemas, or tests.
2. Ran the focused formal-hook support test, source-sample runner test, four
   runtime formal-hook smokes, normalized JSON inspection, and the full
   current-L2 regression helper.
3. Compared the four detached payloads and formal-hook artifacts against the
   pre-registered same-Place / cut / rollback distinctions.
4. Retained the scoped finding and exact non-claims in `plan/wrk-0008...`.
5. Obtained and incorporated a read-only independent evidence challenge.

## Files changed

- `plan/wrk-0008-obl027-formal-hook-attribution.md`
- `plan/00-index.md`
- this report

## Commands run

- The exact WRK-0008 command chain with a unique
  `/tmp/mirrorea-wrk0008-formal-hook.XXXXXX` root.
- `jq -S` payload-core inspection for the four emitted detached bundles.
- Source/sample reads of the current-L2 helper and focused support/ladder tests.
- Resource inspection already recorded by the preregistration package; the
  resulting disposable artifact root used 456 KiB.

## Evidence / outputs / test results

The focused formal-hook suite passed 5/5 and the source-sample runner passed
2/2. The complete current-L2 regression helper completed 23/23 commands,
including the runtime and static formal-hook smokes and the existing theorem
stub/model-check carrier conformance previews.

The four runtime artifacts all emitted exactly one
`rollback_cut_non_interference` row with `runtime_try_cut_cluster` and only
fixture/runtime-cluster identity references. This held for `e1`, which has an
atomic cut but no rollback; `e2`, which has rollback but no atomic cut; `e21`,
which has both; and `e22`, whose source puts the cut in a nested Place. The
detached payloads contain event-kind strings and terminal outcome, but no
same-Place frontier or rollback-crossing relation. The helper guard accepts
either relevant event kind.

The reviewer also found a separate current-L2 interpreter path that refreshes
its rollback snapshot only at the matching Place and tests distinct `e21` /
`e22` final stores. That is relevant counter-evidence: this package does not
claim that the LAB runtime lacks locality. The formal-hook artifact, however,
does not retain that runtime relation.

The expected falsifier therefore did not occur. This evidence supports the
narrow conclusion that the existing output is a reachability/identity formal
hook, not a witness of the Canon same-Place no-cross relation.

## What changed in understanding

`formal_hook_status: reached(runtime_try_cut_cluster)` is not a semantic proof
indicator. It means the existing helper can construct its preview for a cluster
with the coarse event condition. The row's obligation-shaped name must not be
read as evidence that all data necessary for the named Canon relation was
checked or retained. The current interpreter's distinct local behavior and the
formal-hook artifact's insufficient attribution are separate facts.

## Open questions

- What proof/model-check carrier could represent a same-Place cut frontier
  without prematurely selecting Canon Core history or rollback semantics?
- Which future evidence relation would distinguish the four cases without
  promoting the current helper/schema to a public or normative interface?

## Suggested next prompt

Manifest WRK-0008's scoped evidence, synchronize the current LAB snapshots,
and then select the next standing-eligible research question without widening
the current-L2 helper.

## Plan update status

`plan/` 更新済み: added the non-numbered WRK-0008 LAB evidence memory and its
index entry. It is deliberately non-numbered, so it remains inside the
declared `plan` evidence root without modifying documentation guard source.

## Documentation.md update status

`Documentation.md` 更新不要: this evidence commit is deliberately restricted
to the declared `plan` root; the following manifest package will add the
reader-facing scoped result after this commit is available for append-only
ownership.

## docs/project-status.md update status

更新不要: the following manifest package will add the new L3 evidence without
changing the Canon lifecycle, Gate/Phase, or OBL status.

## progress.md update status

`progress.md` 更新不要: the following manifest package will update the LAB
research snapshot and dated recent log from the stable evidence commit.

## tasks.md update status

`tasks.md` 更新不要: the following manifest package will close this evidence
package and identify the next target-selection point.

## samples_progress.md update status

`samples_progress.md` 更新不要: the following manifest package will clarify the
existing formal-hook evidence classification without relabelling sample
workflow status.

## Reviewer findings and follow-up

Read-only reviewer `Nash` independently confirmed the two decisive points:
the helper accepts `rollback` or `atomic-cut` and emits the named row without
co-location/order/crossing data; the formal-hook schema has no Place/frontier
relation. It also identified the necessary qualification that the current-L2
interpreter has separate Place-sensitive rollback handling and `e21` / `e22`
store distinctions. The plan and this report now limit the conclusion to the
formal-hook row. No workspace edits were made by the reviewer.

## Skipped validations and reasons

No source edit occurred, so no new unit test, helper regression, or runtime
implementation validation was needed beyond the preregistered focused suites
and complete current-L2 regression. Workspace-wide test, Docker, Lean replay,
release workflows, and unrelated sample families do not answer the bounded
formal-hook attribution question. The post-edit documentation/canon validation
will run with the evidence-manifest package, after the working record references
this commit.

## Commit / push status

Pending at report write. This plan/report evidence package will be committed
and pushed before it is cited from WRK-0008.

## Sub-agent session close status

Read-only reviewer `019f865c-357d-7a20-8a13-babba9a19bf7` (`Nash`) completed
the evidence challenge without workspace edits and was closed after its
qualification was incorporated.
