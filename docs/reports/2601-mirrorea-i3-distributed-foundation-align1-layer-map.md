# Report 2601 — ALIGN-1 project/product layer constitution

Identifier: `ALIGN-1` / Mirrorea I3 Distributed Foundation

## Objective

Give a context-free implementer one canonical map that distinguishes semantic
strata, project/product responsibility layers, and lifecycle phases, so future
Browser/Host and transport work cannot make an upper product or host mechanism
the source of Mir meaning.

Direct consumer: ALIGN-2 Browser/Host/package/View/provider responsibility and
trust contracts; after ALIGN-2, I3-0 uses the map to keep transport in PL-2/S4
realization rather than source semantics or a product API.

Blocker reduced: current Canon mixed owner-fixed semantic S0--S6 with legacy
LAB realization S0--S7 and had no separate PL-0--PL-6 responsibility map.

Acceptance use: classify a TLS adapter as PL-2/S4 over PL-0/S6, package
admission as PL-3/S6, World libraries as PL-5/S5, and Reversed Library as a
separate PL-6 consumer without inferring authority or phase acceptance.

## Scope and assumptions

This is an architecture-only milestone under PROPOSAL-037 / ADR-0034. The
owner-fixed PL-0--PL-6 decomposition is adopted without choosing detailed trust
tiers, BND-007 wording, FFI/resource contracts, transport, package format,
Shared-Space mechanisms or product APIs. Those details remain ALIGN-2 or later.

The formal theory chapters remain primarily scoped to S0--S5; S6 Host is the
current non-authoritative semantic/realization boundary, not a new theorem.
Legacy S7 Application is not a current semantic stratum.

## Start state / dirty state

- Pinned start revision: `ef19fef2cf0fde227b6d61d427fe3768c218a521`.
- `HEAD`, local `main`, and `origin/main` matched and the worktree was clean.
- ALIGN-0 was completed; Plan 250 was the sole roadmap and ALIGN-1 the sole
  active goal. Official I3 lifecycle was unentered, both transport candidates
  were UNSELECTED, and OPEN-032 was unresolved.
- No worktree was created. The existing root filesystem remained near its
  previously audited 89% use; no build or generated artifact tree was added.

## Documents consulted

Canon-first reading covered README, MAP, NORTH-STAR, DESIGN-CONSTITUTION,
architecture 01--05, plans 01/05, ADR-0034, and PROPOSAL-037. The exact ALIGN-1
contract in LAB Plan 250, current status readers, `.docs/progress-task-axes.md`,
and only the directly referenced Report 2600 were then inspected.

## Actions taken

- Added PROPOSAL-038 and ADR-0035 through the Canon forward-only process.
- Added `architecture/06-project-product-layers` with the independent
  S0--S6, PL-0--PL-6, and T0--T2/I1--I6 axes.
- Recorded responsibility, admitted input, output, prohibited flow, and actual
  maturity for every PL row.
- Fixed PL-4 as responsibility-only, PL-6 as a separate application/project,
  and PrismCascade/Typed-Effect Wiring Platform as satellites.
- Reconciled architecture/01, theory/00, GLOSSARY and MAP so current semantic
  S7 and optional `(S6 Host)` readings no longer exist.
- Renamed LAB feature maturity stages from S0--S6 to FM-0--FM-6.
- Synchronized reader/status/navigation pointers to ALIGN-1 completed,
  ALIGN-2 sole active, and I3-0 next/not active.
- Added a reader regression that first failed on the missing independent map,
  then passed after the HTML/status integration.

## Files changed

Normative architecture and decision records:

- `mirrorea_canon/architecture/01-strata.md`
- `mirrorea_canon/architecture/06-project-product-layers.md`
- `mirrorea_canon/architecture/README.md`
- `mirrorea_canon/meta/proposals/PROPOSAL-038-align1-project-product-layer-map.md`
- `mirrorea_canon/adr/ADR-0035.md`, `mirrorea_canon/adr/README.md`
- `mirrorea_canon/GLOSSARY.md`, `mirrorea_canon/theory/00-overview.md`
- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`,
  `mirrorea_canon/CHANGELOG.md`, `mirrorea_canon/INDEX.json`
- Canon plan/meta current pointers required by the accepted transition.

LAB/current readers and evidence:

- `plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md`,
  `plan/00-index.md`
- `.docs/progress-task-axes.md`, `AGENTS.md`, `CANON.md`, root `README.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/mirrorea-project-overview.html`
- `scripts/tests/test_mirrorea_project_overview_html.py`
- this sole ALIGN-1 report

No Rust, Lean, model, SCN, OBL, sample, runtime or generated evidence artifact
was changed.

## Commands run

- Repository and size checks: `git status`, revision/parity checks, `wc -c`,
  focused `rg` contradiction/stale-pointer scans and `git diff --check`.
- Canon index: generation and `--check` from `mirrorea_canon/`.
- Index, hierarchy, reader and agent-config unit/validation commands.
- `python3 scripts/validate_docs.py` and `make docs` at final close.
- Scoped secret/private-key/webhook pattern scan over the tracked diff.

## Evidence / outputs / test results

- Canon index generation/check: 196 files indexed.
- Canon index unit tests: 5 passed.
- Source hierarchy: 800 required, 800 present, 0 missing.
- HTML reader regression: the new three-axis assertion first produced one
  intended failure while the existing eight tests passed; integrated result is
  10 passed, 0 failed after adding the Canon MAP active-frontier regression.
- Agent configuration validation passed; its unit tests passed 9/9; strict
  Codex config/help exited 0.
- `architecture/06` is 9,470 bytes and `plan/01-phases.md` is 15,000 bytes.
- `git diff --check` and scoped secret-pattern scan passed.
- The first full docs validation correctly rejected stale snapshot header
  timestamps; those headers were synchronized before the final run.
- Final `python3 scripts/validate_docs.py`: exit 0; documentation scaffold
  complete; 1,755 numbered reports found.
- Final `make docs`: exit 0 after agent config, 196-file Canon index check,
  800/800 source hierarchy and the same full docs validation.
- Independent review is recorded below from its actual completion result; no
  pending review is counted as acceptance.

## What changed in understanding

The repository previously used “layer” for three different questions. The
accepted cut now makes them orthogonal: S says where meaning/boundary lives, PL
says which project/product responsibility consumes and produces information,
and T/I says how much of that responsibility has been accepted. The decisive
anti-collapse facts are `S6 Host != PL-0`, `S5 Domain != PL-5`, and
`I2 != PL-2`.

This also exposes a clean continuation: ALIGN-2 can define the Browser/Host
trust edges without moving domain semantics into PL-3 or transport work into a
product/public contract.

## Open questions

- Detailed Browser/Host/package/View/provider trust tiers, BND-007
  clarification, raw FFI separation and resource contracts remain ALIGN-2.
- OPEN-032 and both reliable-stream candidates remain unresolved/unselected.
- Shared-Space addressing, linking, discovery, publication, federation,
  governance and product UX remain UNRESOLVED for a future owner program.
- Reversed Library concrete product design remains owner-reserved.

None is an ALIGN-1 blocker.

## Suggested next prompt

Continue autonomously with ALIGN-2: bind the future package admission,
Browser-to-fabric, View/renderer, typed input/effect, privileged plugin and
resource/sandbox responsibility edges to ADR-0035 without freezing a concrete
API, ABI, package or sandbox technology.

## Plan update status

更新済み: Plan 250 remains the sole roadmap; ALIGN-1 is completed, ALIGN-2 is
sole active, and I3-0 is next/not active. `plan/00-index.md` mirrors the new
Canon architecture without becoming normative.

## Documentation.md update status

更新済み: the three independent axes, PL-4/PL-6/satellite boundaries, current
ALIGN-2 frontier and unchanged lifecycle/public non-claims are synchronized.

## docs/project-status.md update status

更新済み: ALIGN-1 completion, ALIGN-2 active state and the accepted three-axis
responsibility cut are synchronized.

## progress.md update status

更新済み: the concise current snapshot includes the Canon three-axis map,
feature/subsystem status, startability and a timestamped ALIGN-1 close log.

## tasks.md update status

更新済み: ALIGN-2 is the sole active package and owns the detailed trust/FFI/
resource boundary work; I3-0 remains dependency-gated.

## samples_progress.md update status

更新不要: ALIGN-1 changes no runnable sample, command, debug surface,
validation workflow or sample blocker.

## Reviewer findings and follow-up

The pre-edit Canon-first planner returned GO with no P0. Its P1 findings were
all incorporated: reconcile every S-axis conflict, rename the LAB maturity
scale, make the three axes explicitly many-to-many, and keep ALIGN-2 contract
detail out of ALIGN-1. Its P2 size/navigation/reader observations were also
addressed without attempting the unrelated existing MAP size debt.

The read-only code mapper independently localized the semantic-number drift to
architecture/01, theory/00, GLOSSARY and MAP and confirmed no production module
uses S/PL identifiers as an interface. Final independent review found no P0.
It found one P1 stale Canon MAP pointer that still called ALIGN-1 active; the
pointer was corrected to ALIGN-0/1 closed and ALIGN-2 sole active, a direct
Canon MAP regression assertion was added, the Canon index was regenerated,
and 10/10 reader tests passed. Post-fix exact-diff review found no remaining
P0/P1. Its only P2 was the report's obsolete 9/9 count, corrected here to
10/10. Final disposition: ACCEPT.

## Skipped validations and reasons

Rust format/Clippy/workspace tests, accepted I2 runtime suites, Lean, bounded
models, transport tests, fuzzing and multi-process tests were not rerun because
this architecture-only milestone changes no production, formal, scenario,
sample or runtime surface. ALIGN-0 had freshly preserved the focused I2/M10
floor. None of these skipped commands is reported as passing for ALIGN-1.

## Commit / push status

Final validation and independent ACCEPT are complete. The milestone integration
commit cannot embed its own future hash in this report; the parent commits with
`--no-gpg-sign`, pushes to `origin/main`, verifies `HEAD == main == origin/main`
and a clean worktree, then reports the accepted cut at the milestone checkpoint.

## Sub-agent session close status

Completed bounded sessions: pre-edit Canon planner, Canon architecture mapper,
HTML test author, and derived status writer. The final independent reviewer is
closed before commit; every advisory result is checked by the parent against
the exact repository diff.
