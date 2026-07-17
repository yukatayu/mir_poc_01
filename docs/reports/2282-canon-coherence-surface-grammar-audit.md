# Report 2282 - Canon coherence and Surface grammar audit

## Objective

Audit the normative canon across source hierarchy, references, Surface/Core
boundaries, grammar, scenarios, and conformance documents. Identify only
contradictions or formalization gaps that affect a future exact claim, without
inventing syntax, changing canon, or conflating LAB evidence with a decision.

## Scope and assumptions

`mirrorea_canon/` remains normative. This is a bounded LAB audit of its current
text, not a canon amendment, parser implementation, proof, conformance result,
or Gate/Phase action. A missing formal production is reported as a clarification
boundary, not silently filled from sample style or common language conventions.

## Start state / dirty state

The worktree was clean at `6b5736e4` (`Map T0-T2 formalization decisions`) and
matched its upstream. The task baseline was recorded with the Discord helper
before the audit. No tracked source or generated artifact had changed.

## Documents consulted

- Canon entry, map, glossary, style guide, source hierarchy, operating model,
  gate/phase plan, all architecture and spec chapters, theory/00--11, ADR-0007,
  ADR-0008, ADR-0010, and scenarios/SCN-01--10
- LAB `plan/156-t0-t2-research-autonomy-envelope.md`, `Documentation.md`,
  `docs/project-status.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`
- Oracle manual and `.docs/oracle-chatgpt-pro-operations.md`

## Actions taken

- Checked the canon index against the filesystem and all declared dependency
  targets. Mutual `depends_on` cycles were retained as allowed knowledge
  dependencies under the style guide.
- Cross-checked canonical ADR, scenario, obligation, theorem, concept,
  boundary, open-item, and diagnostic identifiers against their registries.
- Read lexical, Surface grammar, static semantics, Core IR, theory, ADR, and
  scenario text together to distinguish a Core companion notation from Surface
  syntax.
- Traced every undefined Surface EBNF nonterminal and the named-keyspace path
  against the surrounding static and theoretical requirements.
- Recorded the result as a narrow owner/canon clarification boundary only; no
  canon file was edited.

## Files changed

- `docs/reports/2282-canon-coherence-surface-grammar-audit.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`

## Commands run

- focused `sed`, `nl`, and `rg` reads across canon, scenarios, and LAB
  snapshots
- Python read-only index, dependency, and identifier-registry checks
- `df -h .`, `free -h`, `du -sh .`, `lsblk -f`, and `findmnt -T .`
- `python3 meta/build-index.py --check` from `mirrorea_canon/`
- final `make check`, focused unit suites, diff, and Git checks listed below

## Evidence / outputs / test results

- `INDEX.json` lists exactly 73 Markdown paths, matching the 73 actual canon
  Markdown files; no indexed file is missing and no actual file is unindexed.
  All `depends_on` targets resolve. The style guide explicitly allows mutual
  knowledge dependencies, so the observed dependency cycles are not findings.
- Exact registry checks found no unknown identifier: 13 ADR, 10 SCN, 28 OBL, 6
  THM, 40 CON, 9 BND, and 30 OPEN IDs. The 30 spec/07 diagnostic IDs cover all
  exact diagnostic references. Five declared diagnostics currently have no use;
  that is catalog coverage, not an inconsistency.
- ADR-0008, spec/01, and spec/04 agree that `atomic_cut` is Core companion
  notation and rejected in Surface v0. Theory/01's `[CUT]` rule maps that Core
  notation to `cut(L)`, consistently with ADR-0007's non-fence rule. This is a
  positive cross-boundary result.
- spec/02 leaves `ModulePath`, `DottedName`, `TypePath`, `PlacePath`,
  `RolePath`, `Param`, `FailName`, and `Expr` without a production. Its prose
  precedence order does not make the `Expr` EBNF closed. `Keyspace` also admits
  `Ident`, while spec/03 and theory/01 require declared finite keyspaces and
  `Item` has no keyspace declaration form. A parser/checker cannot resolve
  these choices by implementation convenience.
- Resource audit before validation: root filesystem has 21 GiB available
  (89% used); the repository is 7.1 GiB, including a 7.0 GiB `target/`; memory
  has about 9.1 GiB available. No heavy artifact was created and no cleanup was
  performed.

## What changed in understanding

The canon is structurally coherent at the document, dependency, ID, and
Surface/Core-boundary levels. The remaining issue is not a broad theory
contradiction or `atomic_cut` ambiguity. It is one small, material language
specification boundary: Surface v0 cannot yet be treated as a closed EBNF or an
exact parser/checker contract. The needed action is a bounded grammar-closure
decision, not a new runtime mechanism or a broad redesign.

## Open questions

- Does Surface v0 retain named custom keyspaces, or restrict the fragment to
  built-in keyspaces until a later grammar revision?
- If retained, what minimal declaration form and name-resolution scope applies
  to a custom keyspace?
- What exact production form closes paths, parameters, failures, calls, record
  literals, and expression precedence without adding unneeded core vocabulary?

## Suggested next prompt

Choose the Surface v0 grammar-closure direction: retain declared custom
keyspaces with one minimal declaration form, restrict v0 to built-in keyspaces,
or explicitly defer exact grammar closure. Then prepare the corresponding canon
proposal and complete the EBNF without changing Core semantics.

## Plan update status

Updated: `plan/156-t0-t2-research-autonomy-envelope.md` records the audit,
positive checks, non-finding, bounded grammar issue, and required canon action.

## Documentation.md update status

`Documentation.md` update unnecessary: reader entry points and runnable
instructions did not change.

## docs/project-status.md update status

更新済み: the human status view now separates the coherent Core companion
boundary from the unselected exact Surface grammar closure.

## progress.md update status

Updated: the current research snapshot and dated recent log record
T-RESEARCH-029 as a decision-ready clarification boundary.

## tasks.md update status

Updated: the task map contains the owner/canon grammar-closure choice and
states that a LAB parser must not select it.

## samples_progress.md update status

`samples_progress.md` update unnecessary: no runnable sample, command, or
evidence classification changed.

## Reviewer findings and follow-up

Local cross-source review found one material clarification boundary and no
`atomic_cut` contradiction. A fresh Oracle review was not started: the recent
browser session failed before prompt submission because the reusable profile
had no usable login cookies, and the operations policy prohibits duplicate
requests without changed failure evidence. The repository default for a later
one-off review is confirmed as `ask-chatgpt-pro-temp`. No local sub-agent
service was available.

## Skipped validations and reasons

No parser conformance test can validate an unclosed normative EBNF, and no
runtime, distributed, or product behavior changed. Repository regression and
documentation checks are run for the LAB documentation changes; they do not
establish a grammar decision, conformance level, theorem, or Gate exit.

## Commit / push status

The final package commit uses `--no-gpg-sign` and is pushed immediately after
the final validation and status review.

## Sub-agent session close status

No local sub-agent service was available; no session was opened or requires
closure.
