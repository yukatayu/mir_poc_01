# Report 2435 - T0-T2 conditional autonomy roadmap

## Title and identifier

Report 2435 - T0-T2 conditional autonomy roadmap.

## Objective

Determine whether the project can autonomously proceed from its current state
through the end of T2, distinguish autonomous theory research from official
Gate/Phase movement, and leave a source-checked execution roadmap to the point
where implementation may be considered without treating current LAB evidence
as Canon completion.

## Scope and assumptions

- `mirrorea_canon/` remains the sole normative source.
- The source cut began at pushed commit
  `91f64b5f5d4443bc37b1699d301e5bdfd1a51510`; the Canon tree was
  `82c4c1363d37dafbe453a4431f156d107ca6cb51`.
- Official lifecycle remains T0. G0 exit and T1 entry are unrecorded, and all
  OBL-001 through OBL-028 entries remain `open`.
- Plan 196, snapshots, HTML, Oracle output, reviewer output, and this report
  are LAB. No Canon, Gate, Phase, SCN, OBL, proof status, implementation
  contract, sample status, or public claim is changed.
- "T2 end" and "I1 implementation authorization" are not assumed equivalent.
  The report explicitly audits that relation.

## Start state / dirty state

The worktree began clean at `91f64b5f`, matching `origin/main`. The Discord
task baseline was recorded before edits. Initial resource checks reported the
188 GiB root filesystem at 65% use with 64 GiB available, 15 GiB RAM with about
9.8-10 GiB available, and 15 GiB unused swap. No unrelated user change was
found or reverted.

## Documents consulted

- Canon entry and direction: `mirrorea_canon/README.md`, `MAP.md`,
  `NORTH-STAR.md`, and `CANON.md`.
- Canon lifecycle and governance: `plan/00-gates.md`, `plan/01-phases.md`,
  `plan/02-operating-model.md`, ADR-0013, ADR-0014, and `working/README.md`.
- Canon theory and proof status: theory 00-06 and 11, the scenario index, and
  relevant SCN records.
- Canon decision surfaces: PROPOSAL-003, 004, 008, 009, 012, and 013.
- LAB current views: `README.md`, `Documentation.md`,
  `docs/project-status.md`, `progress.md`, `tasks.md`,
  `samples_progress.md`, and `.docs/progress-task-axes.md`.
- LAB dependency evidence: Plans 155, 156, 158, 180-182, 187, 190-195,
  `plan/whole-theory-foundation-audit-20260725.md`, and Reports 2390,
  2432-2434.
- Oracle manuals: `/home/codex/.codex/docs/oracle-chatgpt-pro.md` and
  `.docs/oracle-chatgpt-pro-operations.md`.

## Actions taken

1. Reconstructed official T0, Gate wording, T1/T2 criteria, proof-ledger
   status, and ADR-0014 authority independently from LAB progress claims.
2. Identified the T0 `pass` / `derived-pass` profile contradiction, old
   artifact self-binding problem, deferred G0-D3, missing T1/T2 profiles,
   unresolved proof-facing semantic interfaces, and the gap between current
   T2 criteria and I1 readiness.
3. Consulted a planner, a read-only reviewer, and a temporary browser-backed
   GPT-5.6 Sol Pro Oracle. Each answer was treated as advisory and checked
   against repository sources.
4. Added Plan 196 with an execution dependency graph, owner checkpoints,
   package close conditions, ADR-0014 stop conditions, conservative L3
   preflight, T1/T2 proof-model route, and separate T2/I1 branches.
5. Rewrote current snapshots so current state is concise while historical
   detail remains in `plan/` and prior reports.
6. Updated the reader-facing HTML to show that narrow T2 may close
   independently and that I1 authorization requires a separate owner-defined
   relation unless an integrated profile is explicitly accepted.
7. Registered Plan 196 in both static documentation/source-hierarchy
   inventories.
8. Applied all substantive reviewer findings and obtained a no-blocking-finding
   narrow re-review.

## Files changed

- `plan/196-t0-t2-implementation-entry-roadmap.md` (new)
- `plan/00-index.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/mirrorea-project-overview.html`
- `scripts/validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `docs/reports/2435-t0-t2-conditional-autonomy-roadmap.md` (new)

## Commands run

- Discord baseline command:
  `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`.
- Source/resource inspection with `rg`, `sed`, `nl`, `git`, `df -h`, `free -h`,
  `du`, and `wc`.
- Temporary Oracle consultation through `ask-chatgpt-pro-temp`.
- `python3 scripts/validate_docs.py`.
- `python3 scripts/check_source_hierarchy.py`.
- `python3 meta/build-index.py --check` from `mirrorea_canon/`.
- `python3 -m unittest scripts.tests.test_validate_docs`.
- Python HTML ID/link/fragment audit.
- Playwright 1.55.0 desktop and mobile full-page screenshots using installed
  Chrome.
- `git diff --check`, `make check`, final commit/push, and remote-parity
  checks.

## Evidence / outputs / test results

- Final documentation validation passed with 1589 numbered reports; the
  pre-report run found the previous 1588.
- Source hierarchy passed with 746 required paths present and none missing.
- Canon index check passed with 107 files indexed.
- The documentation validator regression suite passed all 87 tests in
  1245.653 seconds.
- Final `make check` passed Canon index, source hierarchy, documentation
  validation, and `cargo check`; Cargo completed the existing dev profile in
  0.10 seconds.
- HTML audit found seven IDs, 25 links, no duplicate IDs, no missing targets,
  and no missing fragments.
- Playwright produced nonblank full-page desktop and mobile renders at
  1440 x 10707 and 390 x 19286. Visual inspection found no incoherent overlap
  or clipped newly changed text.
- The first mobile screenshot command combined an emulated WebKit device with
  the Chrome channel and failed as an unsupported channel combination. The
  corrected Chromium/Chrome 390 x 844 viewport command passed; no product or
  repository behavior depended on the failed attempt.
- Using `npx` increased the rounded local npm cache reading from 1.3 GiB to
  1.4 GiB, approximately 0.1 GiB. No global package or repository dependency
  was installed, and generated screenshots remain under `/tmp`.

## What changed in understanding

The project cannot autonomously move its official lifecycle from T0 through
T2. The blockers are owner-reserved exact-contract, semantic, ledger, and
lifecycle decisions, not inability to run Lean or implementation experiments.

Autonomous research is still available in two forms:

1. before owner decisions, a conservative ADR-0014 eligibility preflight may
   select only non-duplicative literal-transcription or conditional-lemma
   candidates in existing lanes;
2. after an owner disposition, each follow-up package may proceed only if it
   independently satisfies ADR-0014. The disposition does not authorize Canon
   integration, ledger movement, production implementation, or a new lane.

Current Canon T2 is a proof-skeleton/G5 checkpoint, not automatically I1
authorization. An integrated route needs additional all-SCN/G0-G7 readiness
criteria and explicit OBL-003/027 evidence classes. A narrow T2 can close
independently, leaving I1 readiness and authorization to a later owner record.

## Open questions

- Whether to define T0 profile v2 with `pass`, retain v1 as nonconforming
  historical evidence, and authorize one fresh v2 artifact.
- Whether the fresh artifact digest is accepted for G0-D3 and a canonical T0
  exit record.
- Exact T1/T2 profiles, Gate-to-ledger status mapping, and proof-skeleton
  evidence class.
- Whether T2 and I1 authorization use an integrated profile or a narrow-T2
  plus later-I1 route.
- Owner dispositions for PROPOSAL-008, PROPOSAL-012, PROPOSAL-013, and the
  Surface/SCN closure set.
- The accepted shared proof-facing Core/Config/Step/WellFormed/elaboration/
  history model after those dispositions.

## Suggested next prompt

Authorize or reject the first owner checkpoint in Plan 196: define a T0
profile v2 using `pass`, retain the v1 artifact only as nonconforming historical
evidence, and allow one fresh v2 evaluation without yet accepting G0-D3 or
recording G0 exit. In parallel, allow the P0A eligibility preflight to search
for one conservative existing-lane literal/conditional L3 package.

## Plan update status

Updated `plan/00-index.md` and added
`plan/196-t0-t2-implementation-entry-roadmap.md`. Plan 196 is the detailed LAB
dependency and checkpoint map; it does not amend Canon.

## Documentation.md update status

Updated `Documentation.md` to point readers to Plan 196 and state the
conditional autonomy, ADR-0014 per-package limit, and separate T2/I1 routes.

## docs/project-status.md update status

更新済み: docs/project-status.md was updated in this package. It now presents
the official T0 state, T2/I1 distinction, stop lines, and ordered owner
decisions in 121 lines.

## progress.md update status

Updated `progress.md` as a concise three-axis, macro-phase, feature-maturity,
validation, and recent-log snapshot. Historical WRK detail remains in `plan/`
and prior reports.

## tasks.md update status

Rewrote `tasks.md` as the current package map. It separates autonomous
preflight, owner decisions, research-discovery work, T1/T2 packages, narrow
T2, and I1 authorization.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, command, debug surface,
evidence classification, workflow readiness, or sample blocker changed.

## Reviewer findings and follow-up

- Initial planning/review and Oracle advice agreed that official T2 cannot be
  reached autonomously and that current T2 does not define an auditable I1
  entry contract.
- Final read-only review found seven substantive issues: PROPOSAL-013
  comparison was ordered before owner disposition; T2 and I1 were coupled;
  G0-D3 had an unsupported reopen/dormant rule; post-disposition autonomy was
  too broad; OBL-008 was mislabeled as a statement; integrated I1 criteria
  were written as mandatory for narrow T2; and the task report was missing.
- All seven were corrected. The narrow re-review confirmed owner-first
  PROPOSAL-013, separate T2/I1 branches, exact G0-D3 defer wording,
  package-by-package ADR-0014 eligibility, distinct OBL-008 proof/status
  handling, and integrated-route-only all-SCN criteria, with no blocking
  finding.

## Skipped validations and reasons

- No Lean statement, runtime, distributed, or sample suite was rerun because
  no theory artifact, implementation, sample, active command, or expected
  output changed. Such runs would not validate the roadmap's authority and
  dependency claims.
- No Canon conformance, proof discharge, T0/T1/T2 exit, I1 authorization,
  production implementation, real federation, or public-product validation is
  claimed.
- Temporary screenshots and npm cache entries were not deleted because the
  repository cleanup policy requires explicit confirmation for cleanup.

## Commit / push status

The complete package, including Plan 196 and both inventory registrations, is
committed together with `git commit --no-gpg-sign`, pushed to `origin/main`,
and checked for `HEAD == origin/main` and a clean worktree.

## Sub-agent session close status

The planner and final reviewer completed and were closed. The temporary Oracle
consult completed successfully. No task-required sub-agent or Oracle process
remains running.
