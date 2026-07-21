# Report 2287 — Delegated theory research governance

- Date: 2026-07-21
- Author / agent: Codex (canon steward), with independent exploration and final review requested
- Scope: owner-authorized governance amendment and project-visibility recut
- Decision levels touched: L0 process amendment; L2/L3 working-theory operating route. No theory content, OBL, Gate, Phase, SCN, conformance, implementation, or public-completion state changed.

## Objective

Turn the owner's approved L2/L3 theory-research delegation into a bounded canon
process and a concise, source-linked LAB view of the whole project. The route
must permit active reversible theory work without treating experiments, Lean
compilation, or dashboard text as settled proof or lifecycle progress.

## Scope and assumptions

The owner authorization recorded on 2026-07-21 is the decision input. Canon
remains the sole normative source; LAB remains the location for candidates,
evidence, and history. The work only establishes governance and observability.
It deliberately does not select a theory candidate, alter a primitive or an
external contract, change any scenario or exit condition, or promote a sample.

## Start state / dirty state

Started after clean, pushed commit `450a844d` (`docs: plan delegated theory
research governance`), which introduced `plan/157` and registered it with the
documentation validators. The worktree was otherwise clean. The task created
only documentation and Mermaid-source changes; no user changes were reverted.

## Documents consulted

- Canon entry path: `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, and
  `mirrorea_canon/NORTH-STAR.md`.
- Authority and process: ADR-0012, ADR-0013, the adopted owner direction,
  `plan/01-phases.md`, `plan/02-operating-model.md`, `plan/03-risks.md`, and
  `meta/{agent-instructions,source-hierarchy,style-guide}.md`.
- Theory and topology: `theory/11-metatheory-ledger.md`,
  `architecture/01-strata.md`, and `architecture/07-satellites.md`.
- LAB memory and dashboards: `plan/00-index.md`, `plan/154-project-control-cockpit.md`,
  `plan/156-t0-t2-research-autonomy-envelope.md`, `plan/157-delegated-theory-research-governance.md`,
  `Documentation.md`, `docs/project-status.md`, `progress.md`, `tasks.md`,
  `samples_progress.md`, and the existing diagrams.
- Relevant recent evidence: reports 2284 through 2286.

## Actions taken

1. Recorded the owner disposition in `PROPOSAL-005` and made ADR-0014 the
   effective L0 process rule. It reserves L0/L1, primitives, contracts,
   SCN/Gate/Phase, every proof-ledger change, and public claims to the owner.
2. Defined autonomous existing-LAB candidate research plus the stricter canon
   route: an exact owner-maintained editable target, an authority cut, a
   steward-rebased frozen final diff/evidence cut, independent review, and
   reviewed rollback or escalation.
3. Reconciled active canon operating documents. Lean transcription and
   conditional lemmas remain LAB evidence; every `theory/11` identity or status
   movement remains owner-controlled.
4. Preserved the moratorium: the route permits only scoped research artifacts
   in existing lanes and cannot create a lane, helper family, schema, CI
   surface, Make target, or production implementation.
5. Marked plan 156 as historical pre-delegation evidence while retaining its
   investigative content. Plan 157 is the current LAB lifecycle/ratchet plan.
6. Recut the human-facing views: concise control/status snapshots, a canonical
   layer stack, and a workflow that cannot directly advance a Gate or Phase.
   Removed the unused broad `relations.mmd` diagram because its generic arrows
   obscured the canon's separable boundaries.
7. Found and corrected two integration defects before review: the plan cited a
   non-existent ADR filename, and front-matter dependencies were rearranged so
   PROPOSAL-005 is the decision record and ADR-0014 is the one-way effective
   rule rather than mutually depending on later operational documents.
8. Incorporated independent-review findings before integration: a file-level
   L2/L3 label is not an editable surface; no canon demotion can bypass review;
   every rebase invalidates approval; and the proof ledger cannot be delegated.
   The initial editable-target table is deliberately empty, so the next canon
   update needs a new owner row while LAB research remains autonomous.

## Files changed

- Canon governance: `mirrorea_canon/adr/ADR-0014.md`,
  `mirrorea_canon/meta/proposals/PROPOSAL-005-l2-l3-theory-research-delegation.md`,
  ADR-0012, changelog, ADR index, root/MAP, operating plan, risk plan, and
  relevant meta rules. `theory/11` was reviewed and left unchanged.
- LAB governance and memory: `AGENTS.md`, `CANON.md`, `plan/00-index.md`,
  `plan/154-project-control-cockpit.md`, `plan/156-t0-t2-research-autonomy-envelope.md`,
  and `plan/157-delegated-theory-research-governance.md`.
- Reader views: `README.md`, `Documentation.md`, `progress.md`, `tasks.md`,
  `docs/diagrams/layer-stack.mmd`, and `docs/diagrams/workflow.mmd`; deleted
  unused `docs/diagrams/relations.mmd`.
- `docs/project-status.md`
- Generated canon metadata: `mirrorea_canon/INDEX.json`.

## Commands run

- `df -h .` and `free -h` before validation: 35 GiB disk free and about 10 GiB
  memory available; no heavy build was introduced.
- `python3 meta/build-index.py` and `python3 meta/build-index.py --check` from
  `mirrorea_canon/`.
- `python3 scripts/check_source_hierarchy.py`.
- `python3 scripts/validate_docs.py`.
- Focused registry tests:
  `python3 -m unittest scripts.tests.test_validate_docs.ValidateDocsTests.test_all_repo_numbered_plan_files_are_registered scripts.tests.test_validate_docs.ValidateDocsTests.test_numbered_plan_required_scaffold_matches_source_hierarchy`.
- Stale-policy searches with `rg`, `git diff --check`, and `make check`.

## Evidence / outputs / test results

- Canon index generated and checked successfully: `ok: 76 files indexed`.
- Source hierarchy passed: `required: 705, present: 705`.
- Documentation validation passed after current-position rows were given explicit
  canon source paths; it found 1,441 numbered reports.
- The two focused plan-registration tests passed.
- `make check` passed, including source hierarchy, documentation validation,
  and `cargo check` (finished in 0.15 seconds).
- Final integration rerun passed: `git diff --check`, canon index, source
  hierarchy, documentation validation (1,441 reports), and `make check`
  including `cargo check` (finished in 0.04 seconds).

## What changed in understanding

The project can make active theory progress in LAB without asking the owner to
re-decide every local reversible hypothesis. A canon update is deliberately
stricter: an L2/L3 label alone is never authority, and an exact owner-listed
surface plus review of the final rebased diff is required. The five linked views
separate topology, process, concise status, current task order, and detailed
history; none of them is a second source of authority.

## Open questions

- The owner must add the first exact editable-target row before any delegated
  canon update. Until then, candidate selection and falsification are LAB-only.
- G0-D3, the OBL-001 concrete-evidence bridge, PROPOSAL-003, and PROPOSAL-004
  remain owner-reserved as listed in `tasks.md`.
- The current browser-backed Oracle session has no authenticated session in
  this environment. No duplicate consultation was attempted; local/sub-agent
  review is the evidence used for this governance package.

## Suggested next prompt

"Add one narrow ADR-0014 editable-target row for a specified bounded canon
claim, or leave the target table empty and ask for LAB-only candidate triage.
For a listed row, require a frozen rebased diff and independent review before
any canon update."

## Plan update status

`plan/` 更新済み: added plan 157, indexed it, updated plan 154, and marked plan
156 historical pre-delegation evidence without rewriting its findings.

## Documentation.md update status

`Documentation.md` 更新済み: the concise reader entry points now point to the
five-view model and the delegated research boundary.

## docs/project-status.md update status

更新済み: governance, current lifecycle, owner-reserved stop lines, sources,
and the three required status axes are explicitly separated.

## progress.md update status

`progress.md` 更新済み: refreshed the macro phase map, readiness axes,
research/autonomy boundary, feature rows, and timestamped recent log.

## tasks.md update status

`tasks.md` 更新済み: replaced the stale task shape with the current autonomous
candidate packages, owner gates, research-discovery items, and maintenance
requirements.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample root, command, debug
surface, blocker, or workflow-readiness evidence changed.

## Reviewer findings and follow-up

Two exploratory sub-agents independently mapped stale policy references and
designed the visibility recut. The first final reviewer found five valid
blocking issues: no safe editable surface, a review/demotion bypass, a rebase
race, proof-ledger laundering, and a premature dashboard completion claim. A
planner independently recommended an owner-maintained exact table in ADR-0014,
rebase-before-review, all-ledger reservation, and no new global dependency-cycle
rule. The table/sequence/ledger/dashboard corrections are applied.

The narrow re-review found no remaining blocking control issue. It found two
valid mirror/listing defects: `progress.md` and `Documentation.md` omitted the
active-owner-row condition, and the change lists incorrectly named an unchanged
`theory/11`; both were corrected. The residual limitation is intentional: the
validators check structure and references, not governance semantics or Mermaid
rendering.

## Skipped validations and reasons

No new runtime, Lean theorem, or sample execution was required because the
package changes governance and views only. `make check` still ran the project
documentation checks and `cargo check`. Oracle consultation was skipped only
because the documented browser wrapper has no authenticated session; retrying
would duplicate a known unavailable request.

## Commit / push status

Governance package commit `8090274e` (`docs: establish delegated theory
research governance`) passed the recorded validation set and was pushed to
`origin/main`. This closeout status synchronization is committed and pushed
immediately after the package commit.

## Sub-agent session close status

The two exploratory sub-agents and remediation planner completed and were
closed. The initial reviewer completed, was resumed for the narrow re-review,
and completed again; it is closed after this report update.
