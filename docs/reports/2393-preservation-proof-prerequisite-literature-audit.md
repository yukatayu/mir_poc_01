# Report 2393 - Preservation-proof prerequisite literature audit

- Date: 2026-07-23 05:14 JST
- Author / agent: Codex
- Scope: literature-backed proof-readiness catalog and active Lean-consumer map
- Decision levels touched: none; LAB research and repository-memory update

## Objective

Separate general preservation-proof prerequisites from Mir-specific normative
choices, and determine whether the active Lean corpus contains a fresh
non-reserved L3 source locus.

## Scope and assumptions

Canon remains normative. This task reads OBL-020 as the existing global target
and does not change its scope or add a queue, event, scheduler, patch, or label
primitive. The current LAB priority screen is explicitly not an ADR-0014
eligibility condition.

## Start state / dirty state

Started clean and synchronized at `412b1c7f`. Root storage remained at 97%
use with 6.9 GiB free, so no broad build, runtime, generated-artifact, or
networked execution was started.

## Documents consulted

Read Canon README, MAP, ADR-0014, theory/01, 02, 04, 07, 08, 11, and 12;
spec/05; working/README; WRK-0006, WRK-0014, and WRK-0018; plans 126, 161,
163, 176, 180, post-WRK-0013 portfolio review, and remaining-ledger
revalidation. Read all active Lean foundations and statement-draft import
relationships. Consulted primary literature on asynchronous session queues,
mechanised subject reduction, causal order, and event structures. Ran a
temporary Oracle final review with the relevant Canon excerpts and package
files attached.

## Actions taken

1. Mapped each active Lean foundation/draft to its actual importer or consumer,
   Canon anchor, and prior WRK coverage.
2. Compared Canon request queues, occurrence DAG, owner seriality, and patch
   activation with standard preservation-proof requirements.
3. Distinguished established Canon facts from unselected proof-interface
   placement choices.
4. Screened potential source loci without opening a duplicate L3 record.

## Files changed

- `plan/181-preservation-proof-prerequisite-literature-audit.md`
- `plan/00-index.md`
- `Documentation.md`
- `scripts/validate_docs.py`
- `progress.md`
- this report

## Commands run

- targeted Canon/LAB source reads and import/declaration searches
- `rg` consumer and prior-WRK coverage searches
- primary-source review through browser-backed web access
- read-only code-mapper and literature-research sub-agent reviews
- `ask-chatgpt-pro-temp` final authority/factual-boundary review
- `git diff --check` and `make docs`

## Evidence / outputs / test results

The asynchronous-session literature supports a standard proof obligation that
runtime queues remain admissible across send/dequeue/serve. The comparison does
not prescribe Mir's representation. Canon already states request validation,
but its future proof carrier or legal-step premise remains unselected.

Likewise, `H` is already a Canon acyclic occurrence DAG, but a proof needs an
explicit safe-insertion argument for every occurrence-adding step. This does
not establish a missing Canon rule. Successful `[E-PATCH]` remains an OBL-020
case; THM-006/OBL-019 cover only rejected/deferred no-mutation.

Primary sources: [Honda, Yoshida, and Carbone, *Multiparty Asynchronous
Session Types*](https://mrg.cs.ox.ac.uk/publications/multiparty-asynchronous-session-types-jacm/jacm.pdf);
[Tirore, Bengtson, and Carbone, *Multiparty Asynchronous Session Types: A
Mechanised Proof of Subject Reduction*](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ECOOP.2025.31);
[Lamport, *Time, Clocks, and the Ordering of Events in a Distributed
System*](https://www.microsoft.com/en-us/research/publication/time-clocks-ordering-events-distributed-system/);
and [Winskel, *Event Structures*](https://www.cl.cam.ac.uk/techreports/UCAM-CL-TR-95.html).

The active Lean consumer map found no new source locus with a non-duplicative,
non-reserved branch under the current LAB priority screen. OBL-020 coverage,
OBL-001/Core, OBL-021/totality, diagnostics/repairs, frozen IFC work, and
helper-local foundations all have recorded boundaries or require a reserved
mapping. No new source, theorem, or outcome command was created.

`make docs` passed after registering the new numbered plan in the established
validator list. The run checked the 97-file Canon index, the 730-path source
hierarchy, and the documentation scaffold with 1,547 numbered reports.

## What changed in understanding

The theory does not require new Core vocabulary before formalization. It does
require the later proof package to make queue admissibility, occurrence
insertion, dispatch ordering, and successful patch activation explicit enough
to prove the existing global claim. This is a proof completeness catalog, not
a specification amendment.

## Open questions

- Where should request/queue admissibility reside in the future proof model?
- What exact safe-insertion formulation proves `H` remains acyclic?
- Which patch lifecycle/compatibility properties require proof-interface
  representation beyond the five present well-formedness clauses?
- The owner decisions for G0-D3, OBL-001, and PROPOSAL-008 remain unchanged.

## Suggested next prompt

When a new source locus has a concrete non-reserved consumer, re-screen it
against the prerequisite catalog before a fresh L3 pre-registration. Otherwise
advance the existing owner/canon proof-interface decisions rather than adding
unconnected toy lemmas.

## Plan update status

更新済み: plan 181 records the literature comparison, prerequisite catalog,
and non-claims; plan index now links it.

## Documentation.md update status

更新済み: the concise current-candidate list now links the new LAB audit.

## docs/project-status.md update status

更新不要: no Canon lifecycle, proof status, implementation status, or compact
status classification changed.

## progress.md update status

更新済み: recent log records the proof-prerequisite catalog and no-candidate
screen without claiming a new Core requirement.

## tasks.md update status

`tasks.md` 更新不要: no self-driven executable package or owner-decision item
changed; the catalog refines a later proof package only.

## samples_progress.md update status

`samples_progress.md` 更新不要: no sample source, runnable command, or
dashboard status changed.

## Reviewer findings and follow-up

The code mapper found no active Lean source locus with a new direct consumer;
all nearest routes are already covered, frozen, or reserved. The literature
researcher confirmed that queue admissibility and safe causal insertion are
ordinary proof obligations while their exact representation remains
calculus-specific. Both reviews support a catalog, not a Canon change. The
temporary Oracle final review found no must-fix factual, authority, or process
issue. It cautioned only that future text must keep proof-interface
prerequisites distinct from missing Canon invariants and must not present the
literature comparison as a completeness theorem.

## Skipped validations and reasons

No Lean, Cargo, runtime, distributed, or generated-artifact command was run:
this package adds only LAB memory and links, and the root filesystem remains
capacity-constrained. Documentation validation did run and passed; those
execution classes remain intentionally out of scope for this documentation-only
package.

## Commit / push status

Pending at report write. The plan, index, progress log, and report will be
validated, committed with `--no-gpg-sign`, and pushed.

## Sub-agent session close status

The code mapper and literature researcher completed without edits; their
findings are integrated and no follow-up was requested.
