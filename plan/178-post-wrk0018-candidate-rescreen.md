# 178 - Post-WRK-0018 candidate re-screen

## Role and authority

This is LAB repository memory for the candidate screen after WRK-0018 froze.
Canon remains normative.  It changes neither ADR-0014, `working/README.md`,
the validator policy, theory/07, BND-008, the ledger, a Gate, a Phase, a
scenario, grammar, contract, implementation, or public readiness.

## Evidence cut and selection rule

The source cut is `fadc60a4ee296d20b598b2332f67478011196e76`.  A selected
candidate still requires an existing documented LAB lane, a bounded falsifier
and rollback, a non-duplicative question, and two outcomes that change a named
immediate downstream LAB decision without selecting a reserved interface.

WRK-0018 is frozen history, not an input to a successor.  Its first marked-tail
compiler failure is decisive; its later green tail was discarded and the IFC
foundation was restored.  This screen neither repairs nor replays it.

## Independent input and reconciliation

The temporary Oracle review `mirorea-candidate-rescreen-20260723` found no
qualified next package from the attached Canon/plan/status evidence.  An
independent planner instead proposed a conditional Product Alpha
observer-export attribution split.  The planner's proposal was useful as a
Phase-0 challenge, but it is not selected because the local source and
repository-memory checks below fail its live-decision and non-duplication tests.
These reviews are advisory; their raw transcripts are not repository state.

## Candidate screen

| Candidate | Disposition | Evidence and reason |
| --- | --- | --- |
| Product Alpha observer-export attribution split | not selected | `build_observer_safe_export` literally projects `visible_event_ids` from `event_dag.nodes` and `visible_host_io_events` from `host_io_history`, but `comp-02-pure-add-one` already has a direct `run-local` route, expected host-I/O/event order, and existing runtime source reading. `plan/50` already records typed host-I/O request/response observation in the same event DAG. Re-running the known output would not decide a current consumer. Treating only event IDs as a future provenance input would introduce an unselected input-class/interface rather than resolve an existing decision. |
| WRK-0018 telemetry successor | rejected | Any same dependency model, corrected tail, or result reuse is a frozen-route repair/replay. |
| THM-002 history maximum | rejected | Existing T-RESEARCH-005/011 already delimit its state-local and trace-formalization boundary. |
| OBL correspondence bridges | reserve | Direct `c`, relation/coverage mapping, or outcome-totality placement remains an owner/canon proof-interface boundary. |
| P-SURF-05, SCN-08, axiom-profile, P-COMP-03 variants | rejected or reserve | They remain respectively lane-catalog/grammar owner questions, known evidence without a consumer, or retained carrier/phase evidence without a distinct result branch. |

## Product Alpha Phase-0 finding

The proposed literal split itself is real source shape, but it is not a new
autonomous research question at this cut.

- `scripts/mir_computational_samples.py` already runs `comp-02-pure-add-one`
  through Product Alpha `run-local` and checks two host-I/O rows, the Mir
  compute row, and the ordered `host_input_received -> mir_compute_step ->
  host_output_emitted` sequence.
- `samples/product-alpha1/computational/expected/add-one-pure-mir.expected.json`
  already fixes that direct runtime contract.
- `plan/50-product-alpha1-public-boundary-roadmap.md` already states that
  typed host-I/O `AddOne` is represented as request/response observation in the
  same event DAG.
- The suggested positive branch only reserves an undefined future provenance
  input; the suggested adverse branch is either a known baseline failure or
  source/contract repair.  Neither is a named immediate downstream decision.

The absence of a row-level source reference in a summary export must not be
turned into a Canon `H` judgment, a BND-008 compliance question, a telemetry
semantics choice, or an export-ABI design decision.  Those are reserved.  No
runtime command ran during this screen, because running the candidate command
before a valid pre-registration would itself compromise a later experiment.

## Disposition and reopen boundary

**No standing-eligible L3 candidate is selected at this cut.**  No WRK, source
edit, helper, schema, test, runtime command, or output artifact is created.

Reopen only when all of the following are present:

1. a new literal mismatch not already fixed by existing source, expected output,
   plan, or prior report;
2. a named current consumer that must make a binary retain/reject decision from
   the result; and
3. a pre-registrable question whose adverse branch does not merely request a
   source/ABI/semantic repair or reserved interpretation.

An owner/canon decision can separately reopen the existing proof-interface,
lane-catalog, grammar/scenario, or outcome-totality boundaries.  This screen
does not request or infer such a decision.

## Non-claims

This disposition does not say that Product Alpha's event DAG is Canon `H`, that
any exported summary is or is not BND-008 compliant, that an observer export
has a final provenance ABI, or that THM-005/OBL-017/018 is advanced.  It does
not change a runtime, source grammar, Core primitive, effect, label,
declassification, authority, redaction, retention, contract, conformance,
Gate, Phase, theorem/ledger, sample workflow, or public completion claim.
