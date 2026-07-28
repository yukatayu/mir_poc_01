# Report 2471 - WRK-0031 review-field contract repair

- Date: 2026-07-28 10:07 JST
- Author / agent: Codex
- Scope: Repair the strict L3 review-field format detected by post-link
  documentation validation; no evidence or semantic claim changes.
- Decision levels touched: WRK metadata formatting and generated index only.

## Objective

Restore the WRK annex contract after `make docs` rejected the C0-C review-field
value as noncanonical for an L3 record.

## Scope and assumptions

The L3 review field must be exactly `not-required-for-L3` unless it carries the
distinct frozen-base approval format. The Oracle scope-control note remains an
advisory activity record in Report 2469, not Canon working-record metadata.

## Start state / dirty state

Started clean at pushed index-sync commit
`b10d849e1d844cf6e270cc79a6e164c13e0c2244`, equal to `origin/main`.

## Documents consulted

- ADR-0014, `working/README.md`, WRK-0031, and `scripts/validate_docs.py`.
- Successful L3 working records and the failed post-index `make docs` output.

## Actions taken

1. Reproduced and read the exact WRK contract failure.
2. Compared the field with the validator's exact-match branch and successful
   L3 records.
3. Restored the exact L3 marker and scheduled index regeneration and full docs
   validation.

## Files changed

- `mirrorea_canon/working/WRK-0031-c0c-source-local-diagnostic-reference-audit.md`
- `mirrorea_canon/INDEX.json`
- `docs/reports/2471-wrk0031-review-field-contract-repair.md`

## Commands run

- `make docs` after the index-sync commit (failed before this repair).
- Focused validator and working-record reads.
- `cd mirrorea_canon && python3 meta/build-index.py` and its `--check` form.
- `git diff --check` before commit.

## Evidence / outputs / test results

The validator rejected `not-required-for-L3; ...` because L3 records accept
only the exact marker or a frozen-base approval. The repaired field is exactly
`Independent review: not-required-for-L3`. Full `make docs` will run after the
durable repair commit; the prior failure is not claimed as a success.

## What changed in understanding

The review-field value is machine-validated control metadata, not a prose
comment field. Advisory-review narrative belongs in the report trail unless a
future reviewed record uses the prescribed frozen-base binding.

## Open questions

- The C3/C5/C4 temporary Oracle portfolio review is still pending and must be
  checked against local Canon sources before it affects research sequencing.

## Suggested next prompt

Finish the repaired documentation validation, then assess the C3/C5/C4
portfolio review and open only a bounded L3 package that passes ADR-0014.

## plan/ update status

更新不要: no plan result or research order changed.

## Documentation.md update status

更新不要: reader navigation is unchanged.

## docs/project-status.md update status

更新不要: no workflow state changed.

## progress.md update status

更新不要: no readiness or research boundary changed.

## tasks.md update status

更新不要: the C3/C5/C4 portfolio screen remains next.

## samples_progress.md update status

更新不要: no sample, runner, validation command, or dashboard evidence changed.

## Reviewer findings and follow-up

Local validator review identified a single formatting cause. No semantic review
is required for this repair. The temporary Oracle portfolio review is separate
and remains advisory until its conclusions are checked locally.

## Skipped validations and reasons

No runtime, parser, Lean, or sample execution is relevant to a metadata-format
repair. Full `make docs` is deferred until this repair is committed so the
validated state is the pushed state.

## Commit / push status

Pending at report write. This repair will be self-reviewed, committed with
`--no-gpg-sign`, pushed, and compared with `origin/main`.

## Sub-agent session close status

No callable sub-agent session is available. The temporary Oracle portfolio
review remains in progress.
