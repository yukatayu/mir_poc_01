# Report 2361 — WRK-0014 same-carrier variance evidence

- Date: 2026-07-22 18:11 JST
- Author / agent: Codex
- Scope: standalone conditional Lean transfer-law evidence for WRK-0014
- Decision levels touched: L3 only; no L0/L1 decision

## Objective

Execute the registered same-carrier relation-variance experiment without
introducing a Canon carrier, then retain only the evidence delta allowed by
WRK-0014.

## Scope and assumptions

The record was committed and pushed before the red check or source creation.
The retained evidence is confined to the declared `samples/lean` LAB location
and a direct report. Its relation parameters share carriers by type; it does
not address a representation-changing model.

## Start state / dirty state

`main...origin/main` was clean at `5382860d`, including a passing
post-registration `make docs`. The registered red check confirmed that the
target Lean file did not exist before this evidence package.

## Documents consulted

Read WRK-0014, ADR-0014, working/README, theory/01, theory/03, theory/11,
plan/171, existing WRK-0005/0006/0007 artifacts, and the OBL-020 Lean lane.
An independent governance review assessed the exact retention boundary.

## Actions taken

Added a standalone three-theorem Lean artifact and its declared explanation.
Compiled the artifact and ran the registered lexical audit. A numbered
`plan/172` draft was deliberately not retained after documentation validation
showed that it would require excluded validator-source changes.

## Files changed

- `samples/lean/lab-statements/obl020/SameCarrierVarianceBoundary.lean`
- `samples/lean/lab-statements/obl020/SameCarrierVarianceBoundary.md`
- this report

## Commands run

- `test ! -e samples/lean/lab-statements/obl020/SameCarrierVarianceBoundary.lean`
- `lean --version`
- `lean samples/lean/lab-statements/obl020/SameCarrierVarianceBoundary.lean`
- the pre-registered required-name and forbidden-token Python audit
- `make docs` with the disposable numbered-plan draft
- source inspections with `nl`, `sed`, and a token-specific Python probe

## Evidence / outputs / test results

The red check passed. Lean 4.29.1 compiled all three final lemmas and the
registered audit found all three required theorem names with no forbidden
token. During source construction, Lean rejected the reserved tactic-binder
name `scoped`; after renaming it, the lexical audit found `Result` inside a
local variable name. Renaming that variable yielded the final compiling source
without changing its theorem signatures, premises, or conclusions.

The otherwise passing documentation check rejected the numbered plan draft
because `plan/172-...` is absent from `validate_docs.py`'s required registry.
Adding that registry entry would change source outside the declared evidence
locations and execution cut. The draft and its index/README edits are not
retained. This is a retention boundary, not a mathematical counterexample.

## What changed in understanding

The three correspondence gaps share a precise variance rule. To transfer a
universal safety/coherence result, the intended relation must be inside the
model relation. To transfer outcome existence, model witnesses must be inside
the intended relation. This makes future bridge obligations explicit without
proposing any actual bridge.

## Open questions

- What future evidence can establish actual inclusion or witness realization?
- Can THM-001 be formalized directly over Core `c` without selecting a Result
  carrier?
- What owner disposition resolves PROPOSAL-008's totality placement?

## Suggested next prompt

Manifest the exact source-and-report evidence commit in WRK-0014, independently
review it, and use its variance result only as a guard on later proof-facing
proposals.

## Plan update status

`plan/` 更新不要: the pre-registered numbered plan artifact was not admitted.
Changing validator registration to retain it is excluded; plan/171 remains the
prior LAB correspondence checkpoint.

## Documentation.md update status

`Documentation.md` 更新不要: the current reading map will be synchronized by
the later append-only evidence manifest.

## docs/project-status.md update status

更新不要: the evidence has not yet been manifested in WRK-0014; the next
manifest package will update the current control view.

## progress.md update status

`progress.md` 更新不要: current status remains the registered record until
its exact evidence commit is manifested.

## tasks.md update status

`tasks.md` 更新不要: task 36 already states the command and manifest/freeze
boundary; its final evidence status belongs to the manifest package.

## samples_progress.md update status

`samples_progress.md` 更新不要: this is compile-check-only theory evidence,
not a runnable sample workflow or dashboard change.

## Reviewer findings and follow-up

Independent reviewer Carver confirmed that adding either validator registry is
outside WRK-0014's declared locations and exact execution cut. It also confirmed
that `may add only` permits omitting the numbered plan and README edits, so the
Lean source, declared explanation, and direct report remain admissible. Carver
made no edits and was closed.

## Skipped validations and reasons

The attempted `make docs` with the numbered plan failed for the recorded
retention reason. After that draft was removed, `make docs` passed (94 Canon
files, 721 required hierarchy paths, 1,515 numbered reports). Working-record
manifestation, broad Lean synchronization, Cargo, and release checks are
deferred to the next manifest package. No validator/source-hierarchy source
change was attempted.

## Commit / push status

Pending at report write. This evidence package will be committed with
`--no-gpg-sign` and pushed before its WRK manifest is written.

## Sub-agent session close status

Reviewer Carver completed the governance review and was closed. No sub-agent
edited the evidence artifact.
