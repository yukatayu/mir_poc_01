# plan/158 - Standing bounded autonomy and first research ratchet

## Purpose

This is the current LAB execution plan for the owner's 2026-07-21 direction:
agents choose and advance theory targets, formal experiments, and bounded
implementation validations autonomously, while keeping macro direction,
settled semantics, and public commitments under the canon process. It replaces
`plan/157` as the active operating plan. `plan/157` remains historical evidence
of the stricter exact-target-table design; `plan/156` remains pre-delegation
research history.

The governing canon is ADR-0014, `working/README.md`, and
`plan/02-operating-model.md`. This plan is LAB memory; it neither grants
authority nor changes a Gate, Phase, SCN, OBL, conformance result, or public
implementation state.

## Autonomous envelope

### What agents decide and do

Within the standing eligibility predicate, an agent may:

- select a narrow research question attached to existing canon anchors;
- open an L3 `working/WRK-####` pre-registration without waiting for a target
  approval;
- compare alternatives, seek counterexamples, consult literature, transcribe a
  conditional Lean statement, and run existing-lane experiments;
- add or change bounded non-production source/tests inside an existing,
  documented LAB lane when the result remains evidence rather than a product or
  conformance claim;
- revise an L3 record by addendum, or create a forward successor after a
  falsifier; and
- preserve a candidate for a future L2 route only after the frozen-material
  independent review and an owner-authenticated trust anchor exist; and

This is deliberate research, not permission to choose the conclusion first. A
record must be committed with its alternative, falsifier, non-effects, and
rollback/reopen trigger before its results are relied on.

### What stops and escalates

Do not choose or silently narrow an L0/L1 direction or interpretation; core,
authority, ownership, effect, failure, or judgment primitive; any external
contract; SCN/Gate/Phase or lifecycle action; any `theory/11` movement; proof
or OBL discharge; public claim; new evidence lane/helper/schema/CI/Make
surface; or production implementation. If a candidate needs one, record an
`escalated` bundle with alternatives and evidence. The presence of an existing
LAB implementation is never evidence that the required semantics were decided.

## WRK protocol

### Before evidence

1. Re-read the relevant canon sources and classify the candidate against the
   ADR-0014 reserved boundary.
2. Create `working/WRK-####-short-topic.md` with exact read-only anchors,
   pinned base revision/blob hashes, permitted LAB locations, and non-effects.
3. State the narrow proposition, current reading, at least one alternative,
   expected falsifier, rollback/reopen trigger, and reproducible experiment
   plan. Commit this pre-registration before interpreting results.
4. Choose only existing documented commands and lanes. Check resources before a
   heavy build. Do not create a new runner, helper family, schema, CI, Make
   target, public API, or top-level sample family.
5. Treat the committed WRK identity/path and three pre-registration sections as
   immutable. Keep the registration commit to the WRK plus exact operational
   metadata. A retained source/test result is evidence only after its full Git
   commit is appended to `Evidence commits:`; that commit must stay within the
   already-declared `Permitted LAB locations`.

### During and after evidence

1. Run positive and negative cases, recording commands, tool version, input
   identity, retained artifact hashes when applicable, and what the result does
   not prove.
2. Keep source and generated evidence in LAB. Update the WRK record only with
   dated addenda/results and append-only evidence-commit ownership; preserve
   pre-registration text. An unlisted independent commit is not WRK evidence.
3. Run the local validation appropriate to the touched existing lane plus the
   documentation/canon checks. A falsifier immediately stops reliance.
4. Do not promote this run's pilot to L2: the owner-authenticated trust anchor
   is unresolved, so L2 promotion is fail-closed. Preserve any successful
   evidence as L3 or escalate it. A future authorized L2 route must freeze the
   integration base, canon/LAB SHA-256 snapshots, normalized working-record
   SHA-256, evidence digests, and rollback before distinct review.
5. If the candidate remains open or is falsified, retain the L3/falsified
   outcome and choose the next eligible target. Do not manufacture an L2 result
   merely to close a package.

## Current bounded run

The current self-driving run stops at the following **research checkpoint**, not
at final project completion:

| Package | Work | Completion evidence | Stop condition |
| --- | --- | --- | --- |
| A | Adopt standing governance | canon/LAB mirror, validators, independent review, commit/push | complete in this task package |
| B | First bounded pilot | committed WRK pre-registration, existing-lane positive and negative evidence, report, commit/push | escalate only if a reserved boundary is reached |
| C | Pilot checkpoint | review of outcome, dashboard/plan/report synchronization, next target class or escalation bundle, commit/push | stop this run and report the meaningful state |

Packages B and C intentionally do **not** promise a theory theorem, a ledger
move, a Gate transition, or runnable distributed product. Their job is to prove
or falsify the research ratchet itself on one carefully bounded target. If the
first candidate is ineligible, select another eligible target rather than ask
for routine approval. If no eligible target survives a documented triage, close
package C with an escalation bundle and the evidence for that fact.

## Target selection order

Choose the first pilot from current LAB evidence using this order:

1. a narrow existing-theory clause with explicit source anchors and a cheap
   falsifier, preferably a continuation/reproduction of a `T-RESEARCH` result;
2. an existing Lean statement or countermodel whose carrier is explicitly
   experiment-local and can test a claimed sufficiency/necessity boundary;
3. an existing executable sample/test whose result checks an already documented
   invariant without widening its public or conformance surface.

Exclude PROPOSAL-003, PROPOSAL-004, G0-D3, the OBL-001 concrete-evidence
bridge, every `theory/11` action, grammar closure, contract selection, runtime
or transport design, and any target requiring a new helper or runner. The
researcher must record why the selected candidate is smaller and less
committing than rejected alternatives.

## Acceptance criteria for this run

1. There is one authoritative bounded-autonomy route with no active policy text
   still requiring a routine owner-listed target row.
2. A new agent can distinguish autonomous L3 research, reviewed L2 working
   state, LAB evidence, and owner-reserved decisions from the root documents.
3. The first pilot commits its adverse test before the conclusion and runs the
   relevant existing implementation/formal validation rather than only writing
   prose.
4. The pilot outcome is reproducible, scoped, and either retained/falsified as
   L3 or escalated with a concrete boundary reason. L2 promotion is outside
   this run until owner-authenticated trust configuration exists.
5. `docs/project-status.md`, `progress.md`, `tasks.md`, `Documentation.md`, and
   a new report say what changed and what did not. `samples_progress.md` is
   updated only if sample status/commands actually change.
6. The documentation validator rejects a malformed WRK front matter, identity,
   required pre-registration/review field, section, or reliance marker, and
   `meta/build-index.py --check` rejects a stale index through `make check`.
   It also rejects a renamed/reidentified historical WRK record, an invalid
   registration, or a listed evidence commit outside its declared LAB lane.
   The future L2 route additionally resolves its frozen Git base and exact
   canon/LAB source digests, record SHA-256, and distinct signed admission
   trace; the current unresolved trust anchor rejects active L2 records.

## Non-claims

This plan does not make T0 complete, enter T1, discharge OBL-020/021/001,
prove a theorem, select a final carrier or grammar, promote a sample to
conformance, create a production runtime, or make a public product claim. It
sets a finite autonomous research horizon through the first reviewed pilot
checkpoint.
