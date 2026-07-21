# Report 2329 - WRK-0008 OBL-027 formal-hook attribution preregistration

- Date: 2026-07-22 05:05 JST
- Author / agent: Codex
- Scope: reversible L3 registration for an existing current-L2 formal-hook audit
- Decision levels touched: L3 working annex only; all Canon theory and LAB implementation remain read-only

## Objective

Pre-register a narrow, falsifiable audit of whether the existing current-L2
runtime try/cut formal hook has enough attribution to support its own
`rollback_cut_non_interference` label, before executing or interpreting the
registered evidence commands.

## Scope and assumptions

`mirrorea_canon/` remains the normative source. `atomic_cut` and OBL-027 in
Canon theory/04 and theory/11 are read-only. This package asks only about the
existing LAB hook's evidence attribution. It neither proves nor challenges
OBL-027, chooses a BND-003 carrier, or changes helper behavior.

ADR-0014 permits this L3 registration because it uses an existing documented
current-L2 lane, declares an alternative, falsifier, rollback condition, and
non-effects first, and introduces no helper, schema, fixture, runner, CI/Make,
or public surface.

## Start state / dirty state

Started from clean pushed `main` at `057cff585d42c7974e865c19d33ea0555aa917d5`.
No user changes were present, reverted, or overwritten. The prior P-COMP-03
documentation package was already committed and pushed.

## Documents consulted

- Canon README, MAP, ADR-0014, theory/04, theory/11, BND-003, and
  `working/README.md`.
- LAB `plan/158`, the current-L2 sample README, the formal-hook and detached
  bundle support sources, the existing smoke helper, and their tests.
- `docs/reports/TEMPLATE.md`, `Documentation.md`, `docs/project-status.md`,
  `progress.md`, `tasks.md`, and `samples_progress.md` for update eligibility.
- Repo-local Oracle operating guidance. A prior temporary advisory consult was
  considered only for candidate selection; no Oracle conclusion is treated as
  evidence for this record.

## Actions taken

1. Re-read the authority boundary and pinned the Canon/LAB inputs at the clean
   starting commit.
2. Selected the existing runtime try/cut formal-hook route instead of a new
   helper or carrier design.
3. Recorded the competing interpretation, expected falsifier, exact existing
   commands, non-claims, and rollback trigger in WRK-0008.
4. Registered WRK-0008 in the Canon map and regenerated the mechanical index.
5. Corrected the Canon anchor ID and narrowed retained LAB snapshots to the
   validator's allowed `plan` and active-sample roots.
6. Replaced a fixed-directory cleanup and failure-masking command separators
   with a unique `mktemp` directory and fail-fast `&&` sequence.
7. Did not run the registered evidence commands or interpret their outcomes.

## Files changed

- `mirrorea_canon/working/WRK-0008-obl027-formal-hook-attribution.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/INDEX.json` (mechanically regenerated)
- this report

## Commands run

- Canon/LAB source reads and pinned `git show ... | sha256sum` commands.
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-runtime --help`
- `python3 scripts/current_l2_source_sample_regression.py regression --help`
- `df -h .`, `free -h`, `lsblk -f`, `findmnt -T .`, and size checks.
- `python3 meta/build-index.py` and `python3 meta/build-index.py --check`
  from `mirrorea_canon/`.
- `python3 scripts/check_source_hierarchy.py`, `python3 scripts/validate_docs.py`,
  `git diff --check`, and `python3 -m unittest scripts.tests.test_validate_docs`.

## Evidence / outputs / test results

The repository was clean before registration. The root filesystem had about
13 GiB free, with the existing `target/` directory using about 2.2 GiB; no
heavy build or repo-root generated artifact was created.

The smoke helper's argument surface supports a caller-selected artifact root,
so the pre-registered experiment will use a disposable `/tmp` directory. The
actual support tests, smoke commands, JSON artifacts, and regression command
have not yet run. Therefore this report contains no outcome about the formal
hook's adequacy.

The initial document validator found two record-shape errors: the BND file's
Canon ID is `arch/02-boundary-contracts`, not its filesystem directory name,
and retained LAB snapshots may use only the validator's permitted `plan` and
active-sample roots. Both were corrected. Its remaining pre-commit failure is
the expected requirement that a WRK record must already be committed at HEAD.
The full validator unit suite consequently reported 29 assertion failures,
all caused by that same expected process guard; it will be rerun after the
registration commit.

## What changed in understanding

The current research boundary is narrower than OBL-027: it asks whether a LAB
evidence reference is justified by the facts it retains. This isolates a
possible attribution gap without converting an executable helper result into a
theorem, a carrier decision, or a product claim.

## Open questions

- Do the four registered runtime examples produce a discriminating formal-hook
  artifact, or only the same symbolic label?
- If not, what future owner-authorized proof/model-check carrier would be
  appropriate remains unresolved and out of scope here.

## Suggested next prompt

Continue the registered WRK-0008 existing-lane commands, inspect the emitted
artifacts against the falsifier, and record only the resulting scoped evidence.

## Plan update status

`plan/` 更新不要: this is a pre-registration only. No result, current plan
ordering, or roadmap status has changed.

## Documentation.md update status

`Documentation.md` 更新不要: no user-facing operational capability or current
status changed.

## docs/project-status.md update status

更新不要: no workflow readiness or evidence classification changed before the
registered commands run.

## progress.md update status

`progress.md` 更新不要: the current LAB snapshot must not present an
unexecuted L3 question as progress evidence.

## tasks.md update status

`tasks.md` 更新不要: the existing autonomous-research package remains current;
this registration has not yet created an outcome or a new blocker.

## samples_progress.md update status

`samples_progress.md` 更新不要: no sample, validation command, debug surface,
or runnable-state classification changed.

## Reviewer findings and follow-up

L3 registration does not require independent-review admission. Read-only
reviewer `Helmholtz` requested changes: command separators could mask an early
failure, and recursive deletion of a fixed `/tmp` directory could remove
unrelated data. Both were corrected with an `&&` chain and `mktemp -d`.
It also requested explicit pre-commit validation commands, now listed above.
Its narrow re-review found no remaining issue in the four-file preregistration
scope. No outcome claim was made by the reviewer.

## Skipped validations and reasons

The registered current-L2 evidence commands, full regression, workspace-wide
build, Docker flow, Lean replay, and release workflows are deliberately not
run before registration. Running the target commands before the committed
falsifier would violate the WRK pre-registration protocol. The unrelated heavy
workflows do not validate a documentation-only L3 registration. The full
documentation unit suite was run but cannot pass until the WRK is committed;
it will be rerun post-commit rather than treated as a successful validation.

## Commit / push status

Pending at report write. The registration will be validated, committed with
`git commit --no-gpg-sign`, and pushed before evidence is used.

## Sub-agent session close status

Read-only reviewer `019f864a-91ed-7b51-be59-0c3000a66d8e` (`Helmholtz`)
completed its preregistration review. It made no workspace edits and will be
closed after its findings are incorporated and rechecked.
