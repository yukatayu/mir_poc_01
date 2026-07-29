# Report 2521 — WRK-0040 P017 X1 preregistration

- Date: 2026-07-29
- Author / agent: codex
- Scope: Record the first ADR-0014 L3 preregistration enabled by the owner's
  P017 X1 disposition. This is a registration cut only; no outcome source,
  Lean execution, or positive semantic model is included.
- Decision levels touched: L3 only (`working/WRK-0040`). No L0/L1 decision,
  theory ledger, implementation contract, Gate, Phase, or public claim changed.

## Objective

Open one reproducible, reversible existing-LAB-lane investigation that can
detect five distinct anti-collapse failures in the bounded P017 X1 V1/R1
cross-locus-read scope without choosing the relation's eventual representation
or the language/runtime surface.

## Scope and assumptions

The owner recorded P017 X1 on 2026-07-29. ADR-0014 permits an L3
pre-registration only after an independent standing-eligibility cut. The
registration uses the existing `plan/` Lean evidence lane and permits only a
single later Markdown-held, temporary-extracted Lean artifact plus direct
reports and operational metadata. It excludes Core, Config, SaveObject,
request identity, transition/causal relation, restore function, receipt
rejection, consumption representation, authority algorithm, observer
projection, grammar, runtime, transport, theory/11, OBL, scenario, Gate,
Phase, implementation, and public behavior.

## Start state / dirty state

`HEAD` and `origin/main` both resolved to
`0da3869b1307409ae7260b360c7b1ce0a1d60c2d`; the worktree was clean. The
previous package had recorded the owner's X1 disposition but no WRK-0040 or
new X1 outcome artifact existed.

## Documents consulted

Canon entry points: `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`,
`mirrorea_canon/adr/ADR-0014.md`, `mirrorea_canon/working/README.md`,
`mirrorea_canon/theory/01-mircore-v0.md`,
`mirrorea_canon/theory/04-ordering-and-cuts.md`,
`mirrorea_canon/theory/05-authority.md`,
`mirrorea_canon/theory/07-observation.md`, and
`mirrorea_canon/meta/proposals/PROPOSAL-017-c2b-c3-relation-state-envelope.md`.
LAB evidence: Plans 217, 220, and 221; the earlier WRK-0039 record and source;
`Documentation.md`; `docs/project-status.md`; `progress.md`; `tasks.md`; the
report template; and the current Oracle operating notes.

## Actions taken

Registered `WRK-0040` as a predicate-only finite countermodel. It accepts two
supplied distinct occurrence witnesses, two supplied post-load witnesses, and
supplied restore correspondences without asserting cross-load equality. Its
only registered detector classes are: effective-state sharing from incidental
equality (`SEP`); owner-service/receipt-use collapse (`PHASE`); a second
accepted consumption after load (`ONE`); owner success/provenance without
authoritative grounds (`AUTH`); and raw observation without a separately
authorized projection witness (`OBS`).

The record explicitly rejects importing WRK-0039's artifact-local request
constructors, phase table, receipt roles, state functions, or restore function.
It fixes the pre-execution falsifiers and rollback: any need for an identity,
field, function, transition, rejection treatment, causal edge, schema, helper,
or runtime stops the line rather than being repaired into a positive model.

An independent temporary Oracle review completed with GPT-5.6 Sol / Pro effort.
It recommended this `countermodel` class, the six-row neutral-control-plus-five-
mutant matrix, and the prohibition on treating fixture labels as semantic
identities. The review is advisory only; its useful constraints are recorded
in the WRK and this report, not treated as a normative source.

## Files changed

- `mirrorea_canon/working/WRK-0040-p017-x1-coupled-anti-collapse-countermodel.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/INDEX.json`
- `docs/reports/2521-wrk0040-p017-x1-preregistration.md`

## Commands run

- Read the relevant Canon, LAB plans, prior WRK evidence, report template, and
  project snapshots.
- Checked `git status --short`, `git rev-parse HEAD`, `git rev-parse
  origin/main`, and the timestamp before editing.
- Ran `cd mirrorea_canon && python3 meta/build-index.py`.
- Planned before-commit validation: `python3 meta/build-index.py --check`,
  `make docs`, `git diff --check`, focused WRK wording scans, and a secret
  scan. No registered outcome command was run before the registration commit.

## Evidence / outputs / test results

`meta/build-index.py` completed successfully with `ok: 128 files indexed`.
No Lean source exists yet, so no Lean command, countermodel result, or theorem
claim is available. Documentation and diff validation are run immediately
before the registration commit and recorded in the closeout amendment to this
report.

## What changed in understanding

P017 X1 is sufficient to investigate a negative detector, but not to select a
positive relation-state semantics. The smallest useful test is therefore not a
state machine: it is a finite predicate matrix that distinguishes a neutral
control from deliberately seeded collapses. This preserves the ability to find
that the needed detector itself depends on a still-reserved decision.

## Open questions

The eventual positive relation carrier, administrative pending binding,
receipt/rejection semantics, exact accepted-consumption representation,
save/load relation, authority mechanism, and observer projection remain open.
No outcome has yet shown whether the registered detector can avoid all those
choices.

## Suggested next prompt

Continue the registered WRK-0040 experiment: first materialize its one
predicate-only finite artifact in the declared LAB lane, run the exact
pre-registered checks, and freeze or retain only the resulting bounded
evidence.

## Plan update status

`plan/` update deferred to the immediately following LAB snapshot commit. The
ADR-0014 registration commit is intentionally restricted to the WRK and exact
operational metadata; Plan 221 will then record that registration exists before
any outcome source is added.

## Documentation.md update status

`Documentation.md` update deferred to the immediately following LAB snapshot
commit for the same registration-cut isolation reason. The reader-facing state
does not yet claim an executed result.

## docs/project-status.md update status

`更新` deferred to the immediately following LAB snapshot commit. The concise
status will say that the X1 countermodel is pre-registered and unexecuted.

## progress.md update status

`progress.md` update deferred to the immediately following LAB snapshot commit.
The progress log will record the registration with its actual timestamp.

## tasks.md update status

`tasks.md` update deferred to the immediately following LAB snapshot commit.
The next autonomous package remains registered execution, not a positive
semantic adoption.

## samples_progress.md update status

`samples_progress.md` 更新不要: no active sample, validation command, debug
surface, or runnable sample dashboard row has changed. The future artifact is
LAB theory evidence, not an active sample.

## Reviewer findings and follow-up

The advisory Oracle review found that reusing WRK-0039's state machine would
silently select a relation representation. This record instead uses supplied
witnesses and predicates only. No callable sub-agent facility is available in
this environment; the main agent performed the source-hierarchy and scope
review.

## Skipped validations and reasons

Lean compilation, axiom reporting, source scans, and the six-row matrix are
intentionally skipped until after this WRK registration is committed and
pushed, as required by `working/README.md`. No heavy build was needed.

## Commit / push status

Pending at report write. The registration commit and push are performed only
after the planned documentation/diff validation passes; its exact SHA is added
in the closeout amendment.

## Sub-agent session close status

No callable sub-agent session was available. The independent Oracle session
`p017-x1-first-l3-review` completed before this registration; it is advisory
and requires no repository session close action.
