# Report 2444 — WRK-0024 SCN-02 snapshot countermodel

- Date: 2026-07-28 05:44 JST
- Author / agent: Codex
- Scope: Execute the already committed/pushed WRK-0024 finite countermodel and
  retain only its bounded LAB evidence.
- Decision levels touched: L3 evidence only. No Canon rule or lifecycle state
  was changed.

## Objective

Test whether owner-serial application of already-computed writes alone entails
the atomic read-dependent result intuitively expected from two concurrent
SCN-02 attacks.

## Scope and assumptions

The Lean model abstracts away Mir carriers and uses only submitted integer
writes versus owner-side integer damage applications. It is a countermodel of
an implication, not a model of the selected Mir semantics.

## Start state / dirty state

Started at clean, pushed registration commit `2a08b2f2`. The registered scratch
file path was absent, then created outside the repository under `/tmp`.

## Documents consulted

- WRK-0024, ADR-0014, theory/01, theory/03, spec/05, SCN-02, P012, and Plan 199.
- The existing `samples/lean/lab-statements/` lane inventory and prior Plans
  187/192/193 as LAB comparison evidence.

## Actions taken

1. Ran the registered pre-source marker after the registration push.
2. Created the minimal scratch Lean model outside the repository.
3. Checked its three named theorems with Lean `--trust=0`, checked for
   disallowed proof shortcuts, and reread the pinned source anchors.
4. Retained a prose reproduction artifact and synchronized the current LAB
   plan/status views without selecting a repair.

## Files changed

- `plan/wrk-0024-scn02-read-write-snapshot-ambiguity.md`
- `plan/00-index.md`
- `plan/199-selected-semantic-composition-and-inference-boundary.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2444-wrk0024-scn02-snapshot-countermodel.md`

## Commands run

- `lean --version`
- Registered marker check, `/tmp` directory creation, and resource check.
- `lean --trust=0 /tmp/mirrorea-wrk0024-scn02-snapshot/Scn02SnapshotAmbiguity.lean`
- Registered required-name/forbidden-token Python check.
- Registered `rg` source-anchor audit, `sha256sum`, and `git diff --check`.

## Evidence / outputs / test results

- Lean 4.29.1 compiled the scratch model with `--trust=0`.
- The shortcut scan passed. Scratch SHA-256:
  `9c02e90a8accaf156dffd4ee14c9fc10052a8d6f16b2ec6e82fca85b99b15cac`.
- `ownerSerialFinal(10, [7, 6]) = 6` and `atomicDamageFinal(10, [3, 4]) = 3`.
- The checked non-implication is that serial processing of submitted writes does
  not alone entail an atomic read-dependent update.

## What changed in understanding

SCN-02 needs a source-visible or elaborated relation that identifies where the
target read is evaluated and how its result stays valid until mutation. Owner
seriality solves simultaneous mutation at one store, but cannot by itself bind
an earlier remote read to that mutation. The current sources intentionally do
not yet choose the required relation.

## Open questions

- Whether to evaluate at owner service, bind a versioned snapshot, or reject a
  stale dependent update is a reserved follow-up decision.
- Its composition with V1/R1 pending control, M1 semantic request/replay
  identity, SW1 service facets, and failure-row behavior remains open.
- C0, C2, and C6 are the next independent Plan 199 source-anchor packages.

## Suggested next prompt

Continue Plan 199 C0/C2/C6, beginning with exact total-domain and M1
request/replay source-anchor comparisons; do not select a C1 repair.

## Plan update status

更新済み: Plan 199 now records the C1 non-implication and links its reproduction
artifact; the plan index registers that artifact.

## Documentation.md update status

更新不要: the reader-facing orientation did not need additional detail beyond
the existing Plan 199 link.

## docs/project-status.md update status

更新済み: current status now distinguishes the C1 countermodel from a Canon
semantic decision.

## progress.md update status

更新済み: current blockers and the recent log now capture the bounded C1 result.

## tasks.md update status

更新済み: the active autonomous package now names C0/C2/C6 while preserving C1
as a decision boundary.

## samples_progress.md update status

更新不要: no committed runnable sample, command, debug surface, or workflow
status changed. The Lean file is disposable scratch evidence, not a sample.

## Reviewer findings and follow-up

The prior Oracle review identified this exact snapshot gap. Local result review
confirms the Lean model proves only the explicitly stated non-implication. It
does not establish a Canon trace or a preferred repair. A later design package
must compare any repair against V1/R1/M1/SW1/A2 and the failure/DAG/save-load
boundaries.

## Skipped validations and reasons

No runtime/sample/Rust validation was run because the repository contains no
changed executable artifact. The scratch Lean model is intentionally outside
the repository and does not alter the active Lean sample corpus.

## Commit / push status

Pending at report write. The evidence artifact will be committed/pushed first;
a following metadata commit will append that exact evidence commit to WRK-0024
without rewriting its pre-registration.

## Sub-agent session close status

No callable sub-agent session was available. The temporary Oracle consultation
remains advisory; its concern was independently reproduced by the registered
local countermodel.
