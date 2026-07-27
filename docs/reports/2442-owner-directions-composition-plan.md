# Report 2442 — Owner directions and composition research plan

- Date: 2026-07-28 05:35 JST
- Author / agent: Codex
- Scope: Record accepted direction packets without applying their prospective
  semantic amendments, then define the next autonomous composition and
  ergonomics research boundary.
- Decision levels touched: Canon L3 proposal records and LAB planning only.
  No L0/L1 rule, SCN, OBL, Gate, Phase, or implementation contract changed.

## Objective

Mirror the owner's accepted directions for P004, P008, P012, P013, Surface
fallback/`return`, and T2/I1 bootstrap into proposal records and current LAB
views. Establish the falsifier-first work that must precede a shared
proof-facing operational model, including when source detail may safely be
inferred rather than required from the Mir author.

## Scope and assumptions

The proposal records authorize only later bounded design/comparison work. Canon
theory/spec wording, scenarios, ledger, Gate/Phase, conformance, parser,
runtime, carrier, wire, and public API remain outside scope. The temporary
Oracle review is advisory and was cross-checked against local Canon source.

## Start state / dirty state

Started at `ce9fa464` (`docs: audit fixed control drift`) with a clean worktree.
The four fixed-control mismatches had already been scoped as governance-only;
no rebase, retry, or lifecycle movement was authorized.

## Documents consulted

- Canon hierarchy and task anchors: `mirrorea_canon/README.md`, `MAP.md`,
  `theory/01-mircore-v0.md`, `theory/05-authority.md`, `spec/02-surface-grammar.md`,
  `spec/05-runtime-semantics.md`, `spec/06-conformance.md`, SCN-02, SCN-08,
  ADR-0014, and the existing P004/P008/P012/P013 records.
- Lifecycle evidence: `mirrorea_canon/plan/00-gates.md`,
  `plan/01-phases.md`, `theory/11-metatheory-ledger.md`.
- LAB current views: `Documentation.md`, `docs/project-status.md`,
  `progress.md`, `tasks.md`, `samples_progress.md`, Plans 196--198, and
  `docs/reports/2441-fixed-control-drift-scoped-audit.md`.
- Oracle operating instructions: `/home/codex/.codex/docs/oracle-chatgpt-pro.md`
  and `.docs/oracle-chatgpt-pro-operations.md`.

## Actions taken

1. Recorded the accepted P004 A, P008 A, P012 V1/R1/SW1/conditional A2, and
   P013 M1 directions with explicit non-effects.
2. Added P015 for explicit scalar terminal/default closure and v0 `return`
   exclusion, and P016 for narrow T2, separate I1 readiness, explicit bootstrap,
   and C-static formal entry.
3. Regenerated Canon index metadata and changelogged the direction records.
4. Added Plan 199, which sequences C0--C7: exact domain/totality; SCN-02
   snapshot; request binding/replay; pending value control; served/admission
   facets; scalar terminal; and safe source inference/desugaring.
5. Updated reader-facing and current-state LAB documents. New numbered Plan 199
   was registered in both document validation and source-hierarchy scaffolds.

## Files changed

- Canon proposal/changelog/index: P004, P008, P012, P013, new P015/P016,
  `mirrorea_canon/CHANGELOG.md`, `mirrorea_canon/INDEX.json`.
- LAB plans: `plan/00-index.md`, Plans 196, 197, and new Plan 199.
- Current views: `Documentation.md`, `docs/project-status.md`, `progress.md`,
  `tasks.md`.
- Validation registration: `scripts/validate_docs.py`,
  `scripts/check_source_hierarchy.py`.
- This report: `docs/reports/2442-owner-directions-composition-plan.md`.

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- Temporary Oracle consultation through `ask-chatgpt-pro-temp` with a 60-minute
  browser timeout; completed after approximately 15 minutes.
- `python3 meta/build-index.py` and `python3 meta/build-index.py --check` from
  `mirrorea_canon/`.
- `git diff --check`.
- `python3 scripts/validate_docs.py`.
- `python3 scripts/check_source_hierarchy.py`.

## Evidence / outputs / test results

- Canon index generation/check: `ok: 110 files indexed`.
- Documentation validation: passed after registering the new numbered plan.
- Source hierarchy check: passed after the same registration.
- `git diff --check`: passed.
- One initial combined command invoked root-level validation while its working
  directory was `mirrorea_canon/`; it failed only because `scripts/validate_docs.py`
  was not at that relative path. The root-level rerun above passed. No claim
  relies on the failed invocation.
- No sample/runtime/Lean command was run because this package changes neither
  executable behavior nor runnable sample taxonomy.

## What changed in understanding

The accepted directions remove the need to ask again whether to pursue their
families, but they do not produce a closed shared semantics. The main remaining
risk is composition, not an absence of a preferred direction: SCN-02 needs an
explicit read/evaluation/snapshot account; M1 needs semantic request identity
and a total validation-failure mapping; V1/R1/SW1/A2 need pending-control and
facet/causal rules; and SCN-08 needs a genuine scalar/terminal correspondence.

Ergonomic inference is viable only when the omitted fact is uniquely determined
by normative inputs and the elaborated result preserves reconstruction evidence.
This keeps ordinary source concise without allowing hidden authority, request,
snapshot, default, or history semantics.

## Open questions

- The exact v0 well-scoped source domain, totality obligation placement, and
  diagnostic coverage remain open.
- SCN-02 read snapshot/fusion and target read-mutate atomicity remain open.
- Semantic request/receipt identity, replay behavior, pending linearity,
  served/admission facets, scalar representation, and lifecycle profile wording
  need C0--C7 research before a shared model or Canon amendment.
- Fixed-control rebase and G0-D3 remain independent owner/Canon matters; the
  current v2 artifact is still valid `fail`.

## Suggested next prompt

Continue the autonomous C0--C2/C6 source-anchor and countermodel matrix from
Plan 199, stopping only for a minimum necessary Canon decision packet.

## Plan update status

更新済み: Plan 199 was added and Plans 196/197 plus the plan index now separate
recorded directions from the still-open composition research.

## Documentation.md update status

更新済み: the entry-point guide now links Plan 199 and distinguishes recorded
directions from uncreated Canon amendments/profile.

## docs/project-status.md update status

更新済み: the control view now presents C0--C7 as the active research stopline.

## progress.md update status

更新済み: three-axis, blocker, macro-phase, feature row, and recent-log views
now reflect selected directions and the non-closed composition boundary.

## tasks.md update status

更新済み: the current task map now places Plan 199 C0--C7 before the shared
formal model and classifies the remaining profile work separately.

## samples_progress.md update status

更新不要: runnable samples, commands, debug surfaces, and blockers did not
change in this documentation/governance package.

## Reviewer findings and follow-up

The temporary GPT-5.6 Sol Pro review found the chosen package directionally
coherent but not composition-closed. Its source-backed follow-up is Plan 199:
do not make a shared model or runtime claim before exact-domain coverage,
SCN-02 snapshot countermodels, request/replay binding, typed pending control,
served/admission facets, scalar terminal evidence, and inference equivalence
are addressed. No raw external transcript was committed or treated as Canon.

## Skipped validations and reasons

No executable sample, Rust, Lean, or runtime validation was run because no
executable source, sample taxonomy, toolchain, or generated operational
artifact changed. Browser/Oracle consultation was used for design review, not
as a validation substitute.

## Commit / push status

Pending at report write. The package will be committed with `--no-gpg-sign`,
pushed, and checked against `origin/main` before the next autonomous package.

## Sub-agent session close status

No callable sub-agent session was available in this environment. The independent
Oracle review completed; no external session state is required for the next
package.
