# Report 2250 - P111 T0/G0 governance-profile proposal

- Date: 2026-07-15
- Author / agent: Codex with ChatGPT Pro Oracle advisory review
- Scope: owner-input recording and a non-effective T0/G0 profile design memo
- Decision levels touched: no ADR, status, maturity, Gate, Phase, SCN, or OBL decision changed

## Objective

Record the owner's selected response to G0-EXIT-001 as a reviewable canonical
proposal, keep T0 governance evidence separate from SCN conformance, and leave
G0/T0 exit authority with the owner/canon process.

## Scope and assumptions

`mirrorea_canon/` remains normative. The owner confirmed the core/domain,
domain-handler, grant-lineage, and `.mir` source-authority directions; selected
the T0-specific governance-profile route; and waived a separate
semantic/historical LAB-demotion audit at this checkpoint. Those inputs do not
silently accept the exact five-ADR/GLOSSARY/LAB-demotion evidence set for
G0-D1, adopt a profile, or approve G0 exit.

## Start state / dirty state

Started from clean `main...origin/main` at `4005b8c7`. P109 was the controlling
G0 decision packet, P110 was the closed control-cockpit package, canon stated
T0, and no T0 `mir-conform` profile existed.

## Documents consulted

- Canon: `README.md`, `MAP.md`, plans 00-02, `spec/06-conformance.md`,
  `architecture/03-toolchain.md`, `architecture/02-boundary-contracts.md`,
  `meta/agent-instructions.md`, `meta/source-hierarchy.md`,
  `meta/style-guide.md`, `CHANGELOG.md`, ADR-0001/0002/0005/0009/0012, and
  `GLOSSARY.md`.
- LAB: `CANON.md`, `Documentation.md`, `progress.md`, `tasks.md`,
  `samples_progress.md`, plans 00, 149, 153, and 154, project status, and
  reports 2247-2249.
- Operations: the Oracle manual and `.docs/oracle-chatgpt-pro-operations.md`.

## Actions taken

- Ran the required Discord task baseline without sending a message.
- Recorded the owner inputs in `PROPOSAL-002`, explicitly separating them from
  G0-D1 acceptance, profile adoption, profile evaluation, and G0-D3.
- Proposed a T0/G0 phase-governance profile that is outside C-static,
  C-runtime, and C-distributed conformance.
- Added artifact-binding, acceptance-only pass semantics, and a moratorium-safe
  producer precondition after adversarial review.
- Added `plan/155`, registered it in the plan index and required-scaffold
  guards, and synchronized current LAB/status views.
- Regenerated `mirrorea_canon/INDEX.json`.

## Files changed

- `mirrorea_canon/meta/proposals/PROPOSAL-002-t0-g0-governance-profile.md`
- `mirrorea_canon/INDEX.json`
- `plan/155-t0-g0-governance-profile-proposal.md`
- `plan/00-index.md`
- `plan/153-g0-closeout-evidence-and-exit-decision-packet.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `scripts/validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `docs/reports/2250-p111-t0-g0-governance-profile-proposal.md`

## Commands run

- Read the governing canon/LAB documents and current Git state.
- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `python3 meta/build-index.py` and `python3 meta/build-index.py --check` from
  `mirrorea_canon/`.
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `python3 -m unittest scripts.tests.test_validate_docs -v`
- `make check`
- `git diff --check` and focused diff inspection.
- Oracle sessions `g0-owner-confirmati-20260715`,
  `review-the-attached-drafted-proposal`, and its narrow retry.

## Evidence / outputs / test results

The canon index reports 71 files. Documentation validation completed with
1,404 numbered reports after this report was added. Source hierarchy reported
703 required paths, all present. The focused documentation suite passed 52
tests, and `make check` passed source hierarchy, documentation, and Cargo
check. The final Oracle re-review found the six initial governance defects
resolved and judged the proposal safe to file as an L3-open, non-effective
draft.

At task start, the repository occupied 7.1 GiB and the root filesystem had
32 GiB free. This package adds only text/index metadata; no compiler, runtime,
cache, generated runtime artifact, or external workdir allocation was added.

## What changed in understanding

The owner has selected the profile mechanism and audit scope, but that is not
the same as G0-D1's exact acceptance of ADR effectivity, GLOSSARY readiness,
and LAB-demotion completeness. A valid future profile result must be bound to
the effective profile definition, evaluated revision, owner records,
LAB-demotion evidence revision, and result digest; it cannot itself move a
Gate. The profile's normative definition belongs in phase/governance canon,
not inside the SCN conformance definition.

## Open questions

- Does the owner accept or defer the exact five ADRs as effective for G0, the
  named GLOSSARY baseline as prepared, and current LAB-demotion evidence as
  complete (G0-D1)?
- Does the owner adopt, reject, or revise `PROPOSAL-002`, including its
  plan-level normative location and moratorium-compatible producer route?
- After an adopted profile is evaluated, does the owner approve or defer G0
  exit and name its effective canonical ADR/ledger record (G0-D3)?

## Suggested next prompt

Review `PROPOSAL-002` and decide its adoption/revision together with an exact
G0-D1 acceptance or deferral. Do not request G0-D3 until the adopted profile's
producer path and exact artifact binding are defined.

## Plan update status

`plan/` 更新済み: added `plan/155`, updated `plan/153`, and indexed the new
LAB routing note.

## Documentation.md update status

`Documentation.md` 更新済み: added the concise P111/proposal route.

## docs/project-status.md update status

更新済み: `docs/project-status.md` now distinguishes recorded D2/D4 inputs
from unresolved G0-D1/G0-D3 and cites the proposal.

## progress.md update status

`progress.md` 更新済み: current P111 proposal status and a timestamped log
entry record the non-effective draft.

## tasks.md update status

`tasks.md` 更新済み: P111 is closed as a design memo and no successor is
promoted before the remaining owner/canon decisions.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, validation command, debug
surface, or workflow classification changed.

## Reviewer findings and follow-up

Oracle's first adversarial review found six defects: expanded owner inputs,
an accidental normative phrase, an internally inconsistent illustrative JSON,
the wrong normative location, incorrect L1/L2/moratorium treatment, and a
defer-to-pass path. All were corrected. The first narrow re-review failed
before evaluation because Oracle's manual-login browser session timed out; the
single permitted retry completed and found no remaining blocker. Oracle output
is advisory; no external transcript or browser state was committed.

## Skipped validations and reasons

No `mir-conform` implementation/result, runnable sample suite, Lean proof,
runtime, transport, benchmark, or product workflow was run. P111 is a
governance/design-memo package, and T0/moratorium constraints prohibit claiming
or introducing those layers here.

## Commit / push status

Pending at report write. The proposal package and this report will be committed
with `--no-gpg-sign`, pushed to `origin/main`, and checked for a clean tracking
branch before closeout.

## Sub-agent session close status

No separate in-session sub-agent tool was available. Oracle advisory sessions
completed as recorded above; the failed narrow re-review was retried once and
the retry completed.
