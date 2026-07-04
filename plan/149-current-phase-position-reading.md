# plan/149 - current phase position reading

## status

LAB repository-memory / phase-reading clarification.

This note records how to answer "where are we in the whole plan?" without
confusing canon lifecycle state with LAB evidence accumulated ahead of canon
acceptance. It does not edit canon and does not move any gate, phase, OBL,
proof, conformance, runtime, sample, or workflow status.

## purpose

The project has two status surfaces that are easy to mix:

- `mirrorea_canon/plan/01-phases.md` is the implementation-state canon.
- LAB `progress.md`, `tasks.md`, `plan/`, samples, helpers, and reports record
  implementation evidence, historical work, and current management memory.

When asked for the whole-project phase, answer from canon first, then explain
what LAB evidence has already been prepared.

## canon phase reading

Canon defines nine lifecycle phases:

1. `T0` vocabulary and decision
2. `T1` computational system
3. `T2` proof skeleton
4. `I1` reference implementation
5. `I2` multi-locus
6. `I3` real transport
7. `I4` persistence and patch
8. `I5` projection and view
9. `I6` distributed persistence and federation

The current canon position is still `T0/G0 rebaseline`.

Counting phases as human-visible stages, this is "phase 1 of 9." Counting by
the canon label, it is `T0`. Both phrasings are acceptable if the distinction
is made explicitly.

## rough percent reading

Percent is not a gate metric in this repository. Gate exit requires the canon
criteria and human/canon acceptance path, not a percentage.

If a rough conversational estimate is unavoidable:

- within `T0`, the repo appears to be in a late pre-exit position, roughly
  around the last third of T0 work;
- this is not G0 exit;
- this is not T1 entry;
- across the whole canon lifecycle, the project is still in the first canon
  stage, even though LAB evidence for later stages is substantial.

Do not write a precise percentage into normative docs. Prefer gate language:
`T0/G0 rebaseline, late pre-exit, G0 not exited`.

## LAB evidence reading

LAB evidence is ahead of canon lifecycle state in several ways:

- Product Alpha and Operational Product Suite remain bounded alpha floors.
- Full System V1 is closed through bounded release-check / final audit.
- Surface Mir alpha is closed through `P-SURF-99` as bounded source-authority
  evidence.
- G1 ordinary-assignment preparation has a long LAB trail:
  `plan/71..78`, `plan/117..148`, and related samples / tests / reports.
- OBL-001 / OBL-020 / OBL-021 / OBL-024 / OBL-025 statement-shape drafts and
  sync guards are compile-check / guard evidence, not proof completion.

This means the repo has useful groundwork for G1/T1, but it has not crossed the
canon T0 -> T1 boundary.

## answer template

For user-facing answers:

> Canonically, this is still T0/G0 rebaseline: the first of the nine canon
> phases, or T0 by label. Within T0 it is late pre-exit rather than early
> discovery, but G0 exit is unclaimed. LAB evidence for Product Alpha,
> Full System V1, Surface Mir alpha, and G1 ordinary-assignment preparation is
> substantial and reusable; it should not be read as canon T1/I-phase entry.

If asked "how much of this phase?", say:

> Roughly late T0, around the last third, but percentage is not a project gate.
> The real gate is G0 exit by canon acceptance.

## non-claims

This note does not:

- edit canon;
- claim G0 exit, G1 exit, T1 entry, or any I-phase entry;
- select OBL-020 or OBL-001 review-facing extraction;
- choose requested OBL status;
- move the canon ledger;
- discharge proof obligations;
- claim C-static / C-runtime / C-distributed conformance;
- change runtime, sample, product, or workflow readiness;
- freeze public grammar, ABI, SDK, transport, storage, or distribution status.

## next use

Use this note when a future agent must answer the phase-position question or
update `progress.md` / `tasks.md` without overreading LAB work as canon phase
movement.
