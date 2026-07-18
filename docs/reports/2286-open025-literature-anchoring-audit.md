# 2286 - OPEN-025 literature anchoring audit

- Date: 2026-07-18 16:49 JST
- Author / agent: Codex with independent sub-agent review
- Scope: existing `OPEN-025` comparison-table research only
- Decision levels touched: no L0/L1 decision, Gate, Phase, SCN, or OBL status change; canon comparison rows updated under the explicit agent authority

## Objective

Execute the explicit `theory/12-literature.md` `OPEN-025` scan without
selecting a new Mir semantic carrier or overstating novelty. Add concise,
source-backed rows only where the primary or first-party sources support the
existing canon contrast.

## Scope and assumptions

The scope is the four families named by `OPEN-025`: session types for generated
protocols, distributed reactive programming, Matrix/Third Room, and Urbit.
This is not a systematic literature review, a novelty proof, an implementation
survey, or a decision about protocol, transport, authorization, witness,
reactive, persistence, or update semantics. External sources are evidence for
comparison, never normative authority over canon.

The criterion was: each row must be backed by the linked source and may only
repeat a Mir difference already stated in canon. The working hypothesis would
be falsified if a source already supplied theory/12's complete unified
composition, or if writing the difference required a new core concept. Such a
finding would have stopped this task before a table update.

## Start state / dirty state

`git status --short` was empty and `main...origin/main` was synchronized at
task start. The earlier T-RESEARCH-031/032 audits had closed the independent
metatheory-ledger source cuts, while `theory/12` still listed `OPEN-025`.

Resource audit before validation: root filesystem 188G total, 169G used, 11G
available (95%); repository 7.1G; memory 15Gi total with 7.7Gi available; swap
15Gi total with 13Gi free. No heavy build artifact was introduced.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, and `CANON.md`
- `mirrorea_canon/theory/12-literature.md`
- `mirrorea_canon/plan/02-operating-model.md`
- `mirrorea_canon/meta/agent-instructions.md`
- `mirrorea_canon/plan/03-risks.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `docs/reports/2284-proof-status-lean-evidence-integrity-audit.md`
- `docs/reports/2285-cross-boundary-theory-claim-integrity-audit.md`
- `Documentation.md`, `docs/project-status.md`, `progress.md`, `tasks.md`, and `samples_progress.md`
- [Hu and Yoshida, endpoint API generation](https://www.doc.ic.ac.uk/~rhu/scribble/index.html)
- [Distributed REScala](https://www.rescala-lang.com/assets/pdf/2014%20Distributed%20REScala.pdf)
- [Matrix specification](https://spec.matrix.org/latest/)
- [Third Room documentation](https://thirdroom.io/docs/guides/)
- [Urbit Arvo documentation](https://docs.urbit.org/urbit-os/kernel/arvo)

## Actions taken

1. Rechecked the prior stop condition with two read-only sub-agents. Both
   agreed that no new OBL-020 proof-facing semantic package may be invented,
   and that G0-D3 does not prohibit unrelated bounded work.
2. Tried the required temporary Oracle route. It failed before prompt
   submission because the browser profile had no usable session cookie for the
   configured model. It was not retried without new failure evidence.
3. Identified `OPEN-025` as a separately explicit, existing canon work item.
   `plan/02` and `meta/agent-instructions` explicitly permit literature
   research and comparison-table updates.
4. Read the five source records and added four rows that distinguish existing
   Mir directions from the compared systems without adopting their models.
5. Synchronized the LAB plan and human-facing current-status views. The
   metatheory-ledger exhaustion statement is retained with its existing narrow
   scope.

## Files changed

- `mirrorea_canon/theory/12-literature.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2286-open025-literature-anchoring-audit.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .` -> task baseline recorded.
- `ask-chatgpt-pro-temp ...` -> failed before prompt submission because the browser profile had no matching model session cookie; no retry.
- `df -h .`, `free -h`, and `du -sh . target .git` -> resource state recorded above.
- `python3 mirrorea_canon/meta/build-index.py --check` from repository root -> failed because that script expects the canon root.
- `python3 meta/build-index.py --check` from `mirrorea_canon/` -> `ok: 74 files indexed`.
- `git diff --check` -> passed.
- `python3 scripts/validate_docs.py` -> passed; documentation scaffold complete and 1440 numbered reports found.
- `make check` -> passed; source hierarchy 704/704, documentation scaffold complete, and cached `cargo check` passed.

## Evidence / outputs / test results

- Scribble/MPST endpoint API generation starts from a multiparty protocol and
  generates endpoint APIs; this supports a contrast with Mir's existing
  checked-state, non-choreography direction.
- Distributed REScala supplies a distributed reactive propagation algorithm
  with glitch freedom; this supports the existing S0 boundary on occurrences
  and generated request/publication machinery, not a rejection of reactive
  implementations outside S0.
- Matrix specifies federated room event DAGs and event authorization; Third
  Room documents a Matrix-based shared 3D client. These support a shared-space
  and federation comparison, not a claim that Mir is a Matrix replacement.
- Arvo documents deterministic state from an event log, snapshotting, and
  interpreter source-code updates. This supports the existing no-eval,
  frontier-bound patch contrast.
- Canon index verification passed: `ok: 74 files indexed`.
- Documentation validation and `make check` passed after the final report and
  wording corrections.

## What changed in understanding

The prior phrase "independent source cuts are exhausted" is correct only for
the current metatheory-ledger family. `OPEN-025` is a separate canon literature
lane with an explicit permission route, so it can advance without bypassing
the OBL-020 semantic boundary. The bounded scan did not reveal a source that
already supplies theory/12's complete unified composition. This is negative
evidence within four sources only, not a novelty result.

## Open questions

- `OPEN-025` remains open; the rows do not make the scan exhaustive or
  establish novelty.
- An OBL-020 proof-facing carrier, transition relation, component frame, and
  proof interface remain unselected. The next owner-facing organizational
  question is still PROPOSAL-003 A/B/C.
- G0-D3 remains deferred for any official T1 entry, and PROPOSAL-004 remains a
  separate Surface grammar decision.

## Suggested next prompt

"Continue only an existing canon-authorized non-semantic research lane, or
record an owner disposition for PROPOSAL-003; do not infer a proof-facing Mir
relation from the literature rows."

## Plan update status

`plan/` 更新済み: `plan/156-t0-t2-research-autonomy-envelope.md` now records
T-RESEARCH-033, its source route, falsifier, result, and non-claims.

## Documentation.md update status

`Documentation.md` 更新不要: its entry-point and source-hierarchy guidance did
not change.

## docs/project-status.md update status

更新済み: `docs/project-status.md` に `OPEN-025` の簡潔な状況行を追加した。

## progress.md update status

`progress.md` 更新済み: current research summary and dated recent log now
separate the literature result from the closed metatheory-ledger source cuts.

## tasks.md update status

`tasks.md` 更新済み: T-RESEARCH-033 is recorded as complete and non-semantic;
the current-package summary preserves the remaining owner-controlled stops.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, validation command, debug
surface, or sample blocker changed.

## Reviewer findings and follow-up

The two preliminary read-only reviews confirmed the OBL-020 boundary and the
separate nature of unrelated existing-lane work. Final reviewer `Curie` found
no authority/process problem, but required two wording narrowings: Matrix is a
federated room-event DAG rather than a "transport DAG", and ADR-0002 does not
by itself establish that Mir rejects reactive source graphs. The table and LAB
summary now use the narrower, source-backed wording. Curie also preferred
"multiparty protocols" for the Scribble row; that correction was applied.
Second reviewer `Pauli` found no remaining issue in the corrected comparison
claims, authority boundary, status views, or report.

## Skipped validations and reasons

No runtime, Lean, or sample command changed; this is a documentation and canon
comparison-table update. The first canon index invocation used the wrong
working directory; the canonical invocation passed and is recorded above. No
validation remains skipped.

## Commit / push status

Validated package committed with `git commit --no-gpg-sign` as
`f37a4189` (`Anchor OPEN-025 literature comparisons`) and pushed to
`origin/main` before task close. This report-status recording change will be
committed and pushed separately after its documentation validation.

## Sub-agent session close status

Read-only planners `Kepler` (`019f7429-3ffa-78d1-8e67-4c1c2764a10b`) and
`Newton` (`019f7429-5e63-7f62-be64-280c6649b6d1`) completed and were closed.
Final row reviewer `Curie` (`019f7433-80cd-72d3-865d-c21cdf4eb60a`) completed;
the session was closed. Second reviewer `Pauli`
(`019f7437-e8fd-7661-be8f-90d8a7a921e4`) completed without findings and was
closed.
