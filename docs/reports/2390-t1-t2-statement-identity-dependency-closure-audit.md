# Report 2390 - T1/T2 statement identity and dependency closure audit

- Date: 2026-07-23 04:00 JST
- Author / agent: Codex
- Scope: Canon/LAB statement and evidence audit for T1/T2 exit-critical rows
- Decision levels touched: none; LAB dependency classification only

## Objective

Identify the exact statement, model, and proof boundaries needed for T1/T2
without selecting a reserved semantic interface or treating existing Lean
evidence as proof completion.

## Scope and assumptions

Canon is normative. This read-only audit uses the current source cut
`b9320fa7a57baa8327caf93787268444ea818f09`, its active Lean corpus, retained
LAB evidence, independent reviewers, and a temporary Oracle consultation.
It does not create a WRK, amend an OBL, or narrow ADR-0014 eligibility.

## Start state / dirty state

Started clean and synchronized with `origin/main`. Root storage had about 6.9
GiB free and no mounted external workdir; memory had about 9.1 GiB available.
No Cargo build or generated-artifact command was started.

## Documents consulted

Read Canon README/MAP, style-guide, ADR-0014, plans 00/01/02, theory 01 through
11 as relevant, architecture boundaries/carriers, the working annex, active
Lean README and statement/model files, plans 156 through 179, current snapshots,
report template, source-hierarchy validators, and Reports 2274 through 2280,
2330, and 2389.

## Actions taken

1. Mapped every T1/T2 exit-critical OBL to its Canon domain, current evidence,
   necessary relation, and first boundary.
2. Replayed the active Lean evidence with a module-rooted external `.olean`
   layout and isolated the portable import procedure.
3. Compared independent planner, evidence-inventory, and semantic-review
   outputs with Canon source; rejected the alleged `depends_on` cycle because
   Canon explicitly permits mutual knowledge dependencies.
4. Used a temporary Oracle review to distinguish statements, models,
   countermodels, conditional lemmas, proof skeletons, and runtime evidence.
5. Recorded the resulting dependency map and synchronized reader-facing LAB
   snapshots without changing samples or Canon.

## Files changed

- `plan/00-index.md`
- `plan/180-t1-t2-statement-identity-dependency-closure-audit.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- this report

## Commands run

- `df -h .` and `free -h`
- Git state and upstream checks
- `lean --version`; direct Lean compilation of the active clean suite
- explicit module-rooted Lean replays for OBL-001, OBL-020, and OBL-021 models
- direct `.lean` placeholder scan
- Canon/LAB source and validator searches
- `git diff --check`
- `make docs` (after two validator findings were diagnosed and corrected)
- `python3 mirrorea_canon/meta/build-index.py --check` from the repository
  root (failed: Canon root not found), then
  `cd mirrorea_canon && python3 meta/build-index.py --check`
- temporary Oracle session `mirorea-theory-solidification-20260723`
- independent planner, evidence-inventory, and semantic-review sub-agents

## Evidence / outputs / test results

Lean 4.29.1 passed all four foundations, five direct statement drafts, sixteen
clean-suite stubs, and the OBL-001/020/021 import-bearing models when parent
`.olean` files were emitted under a temporary module-root hierarchy and exposed
through `LEAN_PATH`. The same models fail when the parent `.olean` is placed at
the temporary directory root: the import `samples.lean...` cannot resolve. Both
`lake env lean` and direct `lean` reproduce that failure, so the finding is a
portable replay-procedure correction, not a theorem result. No placeholder was
found in active `.lean` source.

The independent evidence inventory confirms every OBL-001 through OBL-028 is
still `open` in the sole proof-status ledger. The semantic reviewer found real
future boundaries around OBL-003, OBL-020, hot-plug frontier scope, and
observation provenance; its dependency-cycle claim was rejected against the
Canon style-guide. Oracle independently recommends theory hygiene and premise
auditing before another experiment. Raw advisory transcripts are not committed.

Final documentation validation passed: Canon index `97`, source hierarchy
`730/730`, and `1544` numbered reports. `git diff --check` also passed. The
final independent review found that Canon already defines chain and save/load
semantics; the corrected audit now identifies only the missing Canon-aligned
formal/Lean relations, restores the full ledger order, and narrows its scope to
listed OBL rows. The direct build-index invocation from repository root is not
a valid command form because the script locates the Canon root from its working
directory; its corrected Canon-directory invocation also passed.

## What changed in understanding

The immediate problem is not lack of runnable Lean files. It is the missing
identity relation between Canon objects and bounded LAB carriers. The next
formalization must expose, rather than assume, Core-write coverage, complete
step coverage, outcome totality/projection, and the save/load history relation.
This identifies the shortest path to useful owner/canon decisions without
forcing a premature Core expansion.

## Open questions

- Will the owner/canon route select direct Core `c` or a result-to-Core write
  enumeration bridge for OBL-001?
- Where does outcome totality belong relative to OBL-021 and BND-001?
- How should the canonical T1/T2 phase profiles be recorded after the required
  statement boundaries are resolved?
- What G2 syntax/SCN reconciliation should resolve OPEN-005 and SCN-08?

## Suggested next prompt

Review the T1/T2 dependency map and then prepare the smallest owner/canon
decision packet for G0-D3, OBL-001's proof-facing identity, and PROPOSAL-008;
do not open a formal source model before those boundaries are chosen.

## Plan update status

`plan/` 更新済み: plan 180 is the long-lived T1/T2 statement-identity and
dependency map; `plan/00-index.md` points to it.

## Documentation.md update status

`Documentation.md` 更新済み: the current research-reading list includes the
new dependency audit.

## docs/project-status.md update status

更新済み: the compact current-state view distinguishes the completed LAB audit
from Canon lifecycle or proof-status movement.

## progress.md update status

`progress.md` 更新済み: Macro 1, the theorem/model-check maturity row, and the
dated recent log now identify the T1/T2 statement-identity map.

## tasks.md update status

`tasks.md` 更新済み: task 51 records the completed audit and its owner/canon
reopen boundary.

## samples_progress.md update status

`samples_progress.md` 更新不要: sample source, validation commands, dashboard
rows, and workflow classifications did not change.

## Reviewer findings and follow-up

The planner, evidence-inventory agent, semantic reviewer, and final diff
reviewer completed without edits and are closed. The final reviewer found an
overstatement of missing Canon chain/save-load semantics, an incomplete ledger
order, an overbroad scope claim, a stale task-map timestamp, and weak validation
recording. Its narrow re-review found one remaining overstatement of the Canon
`Consistent(K)` definition. All findings were verified against Canon/validators
and corrected before the final validation. The Oracle consultation completed;
its raw output remains advisory only.

## Skipped validations and reasons

`current_l2_lean_sample_sync.py` was not run because it rewrites generated
sample output and invokes Cargo. Full current-L2 regression was not run because
it writes build artifacts, does not validate this documentation audit, and the
root filesystem has only about 6.9 GiB free. No runtime command was needed to
validate a statement-identity classification.

## Commit / push status

Pending at report write. The synchronized audit package passed final
documentation validation and focused review, and will be committed with
`--no-gpg-sign` and pushed.

## Sub-agent session close status

All three read-only sub-agents completed without edits and are closed. The
temporary Oracle session completed; its raw transcript remains advisory only.
