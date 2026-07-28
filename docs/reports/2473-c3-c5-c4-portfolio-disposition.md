# Report 2473 - C3/C5/C4 portfolio disposition

- Date: 2026-07-28 10:17 JST
- Author / agent: Codex
- Scope: Re-screen the remaining recorded-direction families after C0-C and
  select only a nonsemantic C5-PRE candidate for later L3 pre-registration.
- Decision levels touched: LAB plan/snapshot sequencing and validation-source
  registration only; no Canon semantic decision.

## Objective

Determine whether C3, C5, or C4 can supply one non-duplicate ADR-0014
standing-eligible L3 package without selecting a carrier, identity, rule,
history schema, or implementation contract.

## Scope and assumptions

The owner-recorded P012/P013 directions are not current Core semantics. The
user's ergonomic-inference direction remains restrictive: an omitted fact may
be inferred only after unique semantic determination and reconstruction from
an elaborated artifact. This package does not make such an inference.

## Start state / dirty state

Started clean at pushed documentation-validation closure commit
`ac2fa06acd2c2513eb228e528cbb6ea973514ded`, equal to `origin/main`. The full
`make docs` validation at that cut passed after earlier index, WRK-metadata, and
report-heading repairs were separately committed.

## Documents consulted

- Canon README/MAP, ADR-0014, P012, P013, theory/01, theory/04, theory/05,
  theory/08, spec/05, and current working records.
- Plans 186, 187, 199, 200, WRK-0024, WRK-0027, WRK-0028, WRK-0030, WRK-0031,
  current snapshots, and the report template.
- Temporary Oracle review `c3-c5-c4-portfolio-screen-20260728`, assessed as
  advisory and checked against the local source above.

## Actions taken

1. Read the C3/C5/C4 direction and stop boundaries, including M1's explicit
   non-selection of request-instance and occurrence identity.
2. Compared the proposed C5 guard with Plan 186/R0 and searched current working
   records, plans, and reports for a duplicate separate-issuance guard audit.
3. Excluded theory/08 patch admission from the ordinary-admission corpus.
4. Selected only C5-PRE for a future pinned L3 pre-registration; synchronized
   Plan 199/200/201, current views, and validation source registration.

## Files changed

- `plan/201-c5-a2-issuance-guard-candidate-selection.md`
- `plan/00-index.md`
- `plan/199-selected-semantic-composition-and-inference-boundary.md`
- `plan/200-reanchored-semantic-composition-research-plan.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `scripts/validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `docs/reports/2473-c3-c5-c4-portfolio-disposition.md`

## Commands run

- Focused Canon/Plan reads and literal source queries for admission, issuance,
  schedule, observation, failure, and identity language.
- Non-duplication searches in current `working/`, `plan/`, and reports.
- Oracle temporary consultation with eleven local files attached.
- `git diff --check` before commit; full `make docs` after commit is pending.

## Evidence / outputs / test results

Local source confirms that C3's D3/D4 and C4's D9 need a pending/correlation or
served-write/transition identity, respectively. P012 gives C5 a narrow,
literal guard: separately failing, observable, or schedulable issuance must
stop the conditional-A2 direction. No current working record contains the
proposed ordinary-admission guard audit. The source inventory also confirms
that patch admission is not evidence about ordinary admission issuance.

## What changed in understanding

The remaining portfolio is not three symmetric research lanes. C3 and C4
already cross a semantic boundary at their first useful discriminator. C5-PRE
can still improve the evidence base, but only by recording source wording that
could require an A1 successor assessment; it cannot support A2 compatibility
or atomicity.

## Open questions

- Does the pinned C5-PRE source query find a literal ordinary-admission marker
  for a separately failing, observable, or schedulable issuance phase?
- If so, what ordinary Canon/A1 successor proposal should evaluate it without
  conflating it with patch admission or selecting a history carrier?

## Suggested next prompt

Pre-register WRK-0032 C5-PRE at the current cut, then execute only its literal
ordinary-admission issuance-guard audit and retain a nonsemantic source matrix.

## Plan update status

更新済み: Plan 201 records the C5-PRE eligibility preflight, corpus boundary,
falsifiers, and execution order; Plans 199/200 and the index reflect the new
portfolio disposition.

## Documentation.md update status

更新済み: the concise entry point now links the current C5-PRE selection plan.

## docs/project-status.md update status

更新済み: the control view distinguishes C5-PRE from the deferred C3/C4/C5
semantic models and records the ordinary-admission corpus boundary.

## progress.md update status

更新済み: current logical-specification, research row, timestamp, and recent
log now point to C5-PRE pre-registration.

## tasks.md update status

更新済み: package 5 now names C5-PRE as the current autonomous task and keeps
C3/C4/C5 proper on the Canon-design boundary.

## samples_progress.md update status

更新不要: no runnable sample, runner, validation command, or dashboard evidence
changed.

## Reviewer findings and follow-up

The temporary Oracle review ranked C5-PRE above C4 and C3, but its attachment
omitted some sources and showed pre-link WRK-0031 metadata. Local review filled
those gaps: ADR-0014, theory/04, theory/08, spec/05, the current WRK-0031
evidence link, and duplicate searches were checked before adoption. The advice
was retained only as a LAB sequencing input, never as normative evidence.

## Skipped validations and reasons

No Lean, parser, runtime, or sample run is relevant to a documentation and
source-query candidate selection. Full `make docs` runs after the commit so it
checks the new numbered plan, current snapshots, and report together.

## Commit / push status

Pending at report write. This selection package will be self-reviewed,
committed with `--no-gpg-sign`, pushed, and compared with `origin/main` before
WRK-0032 is created.

## Sub-agent session close status

No callable sub-agent session is available. The temporary Oracle session
completed and is no longer needed; its distilled, locally checked findings are
recorded above.
