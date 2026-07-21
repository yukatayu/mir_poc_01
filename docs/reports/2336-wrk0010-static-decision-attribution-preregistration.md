# WRK-0010 static-decision attribution preregistration (R-2336)

## Objective

Pre-register a bounded L3 audit of whether existing current-L2 static-gate
decision payload is attributed by existing static formal-hook artifacts.

## Scope and assumptions

The record is an ADR-0014 L3 existing-lane experiment. It compares only
literal artifact fields for e4/e5/e12/e14; no diagnostic meaning, defect,
mapping, carrier, theorem/OBL, helper/schema, lifecycle, or public claim is in
scope.

## Start state / dirty state

Started from clean pushed `main` at `c072aa9c7585ed456ff438e61e330839df12020f`.

## Documents consulted

- Canon README, MAP, ADR-0014, architecture/02, theory/11, and working README.
- `plan/158`, `plan/168`, and manifested WRK-0009 LAB evidence.
- `samples/current-l2/README.md` and e4/e5/e12/e14 source samples.
- Existing static-gate/formal-hook support and regression command source.

## Actions taken

1. Used independent planner and temporary Oracle triage to rank remaining
   candidates; both selected static decision attribution before e21/e22 test
   coverage.
2. Pinned existing inputs, exact output paths, adverse outcomes, and stop line.
3. Added L3 working record and MAP entry; regenerated Canon index.

## Files changed

- `mirrorea_canon/working/WRK-0010-static-formal-hook-decision-attribution.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/INDEX.json`
- this report

## Commands run

- Clean `main`/tracking-state check.
- Canon/LAB source reading and exact artifact-layout inspection.
- SHA-256 pinning of Canon anchors and declared LAB inputs.
- `(cd mirrorea_canon && python3 meta/build-index.py)`.
- Focused validators, commit, push, and post-commit `make check` remain pending.

## Evidence / outputs / test results

No WRK-0010 evidence command ran. The record is valid only after its
registration commit. Existing source inspection shows a non-valid static gate
is accepted by the formal-hook constructor and emits fixed obligation rows;
this is status-quo input, not an experiment result.

## What changed in understanding

After WRK-0009, static decision attribution is the strongest non-duplicative
candidate. e21/e22 final-store assertion directness remains a distinct reserve
candidate; it is not combined with this record.

## Open questions

- Do the four selected formal hooks preserve every predeclared static field?
- Is there only partial attribution, or none?
- Does the method remain executable without an unregistered surface?

## Suggested next prompt

Commit this pre-registration, synchronize its selection snapshot, then execute
only the registered existing-lane command from clean pushed main.

## Plan update status

`plan/` 更新不要: a separate post-registration snapshot will retain selection
rationale without mixing it into this registration-only commit.

## Documentation.md update status

`Documentation.md` 更新不要: reader-facing state is synchronized separately
after this immutable registration receives a commit identity.

## docs/project-status.md update status

更新不要: current status remains WRK-0009 manifested until WRK-0010 is
registered and snapshot synchronization runs separately.

## progress.md update status

`progress.md` 更新不要: the next research target is not current snapshot state
until its registration commit exists.

## tasks.md update status

`tasks.md` 更新不要: task-map synchronization follows the registration commit.

## samples_progress.md update status

`samples_progress.md` 更新不要: no sample execution or workflow changed.

## Reviewer findings and follow-up

Planner and Oracle independently recommended this target and rejected a
WRK-0009 mapping/repair successor. Focused reviewer `Ampere` found and this
draft corrects input pinning, field-presence projection, static-gate
preconditions, and Canon-index command context; a narrow re-review will run
before commit.

## Skipped validations and reasons

No lane command runs before pre-registration is committed. No implementation
test is applicable because no implementation changes are proposed.

## Commit / push status

Pending at report write; commit with `git commit --no-gpg-sign`, push, and
verify `HEAD == origin/main` before evidence execution.

## Sub-agent session close status

Planner `Aristotle` completed and was closed. Oracle temporary consultation
completed; its distilled advice is recorded here, not the external transcript.
