# Report 2160 - G1 OBL-024 replay vocabulary preflight

Date: 2026-07-04 08:29 JST
Author: Codex
Scope: Package 22 / docs-first replay vocabulary boundary for OBL-024
Decision levels: L2/L3 LAB repository memory only; no canon status movement

## Objective

Separate current report-local `trace_local_replay` anchors from future
proof-level replay relations for OBL-024, so later work does not accidentally
treat helper-local LAB projection evidence as final replay semantics, a replay
engine, final Diagnostic ABI, or proof discharge.

## Scope and assumptions

- Scope is documentation only.
- No production code, expected JSON, Lean statement file, canon file, or repair
  output is changed.
- Current executable evidence remains the non-final E-ROW projection carrier
  from `plan/110`, guarded by `plan/111`.
- OBL-024 remains open in the canon proof-status ledger.
- OBL-024 diagnostic soundness remains separate from OBL-025 repair
  completeness.
- Oracle advice is advisory and was checked against repo evidence before being
  mirrored into `plan/112`.

## Start state / dirty state

- Started from pushed clean `main` at `847978b8`.
- `origin/main` also pointed to `847978b8`.
- Discord task baseline was recorded with
  `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
  before Package 22 file edits.
- Worktree became dirty only through docs-first repository-memory/status
  updates and this report.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `.docs/oracle-chatgpt-pro-operations.md`
- `/home/codex/.codex/docs/oracle-chatgpt-pro.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `plan/00-index.md`
- `plan/81-g1-obl024-statement-shape-inventory.md`
- `plan/90-source-traceability.md`
- `plan/109-g1-obl024-lean-statement-draft.md`
- `plan/110-g1-obl024-executable-projection-carrier.md`
- `plan/111-g1-obl024-projection-rust-fixture-guards.md`
- Oracle consult `repository-context-mirrorea-canon-is` advisory result.

## Actions taken

1. Read the repo-local Oracle operations notes and global Oracle manual.
2. Started an Oracle consultation for a second opinion on the replay vocabulary
   boundary.
3. Added `plan/112-g1-obl024-replay-vocabulary-preflight.md`.
4. In `plan/112`, separated:
   - carrier projection;
   - report-local replay anchor;
   - future proof-level replay relation.
5. Added wording guards for qualified replay terms, bridge rule, trace-local
   exactness, and mixed branch boundaries.
6. Preserved unresolved status for final Diagnostic ABI, final replay ABI,
   request/branch/association/replay IDs, diagnostic ordering, whole-judgment
   versus rule-local replay, global root-cause uniqueness, and OBL-025 repair
   coverage.
7. Synchronized `plan/00-index.md`, `plan/90-source-traceability.md`,
   `plan/109`, `plan/110`, `plan/111`, `README.md`, `Documentation.md`,
   `progress.md`, and `tasks.md`.
8. After read-only reviewer feedback, re-read the canon entry-point documents
   named by `plan/90` and corrected this report's provenance record.

## Files changed

- `plan/112-g1-obl024-replay-vocabulary-preflight.md`
- `plan/00-index.md`
- `plan/90-source-traceability.md`
- `plan/109-g1-obl024-lean-statement-draft.md`
- `plan/110-g1-obl024-executable-projection-carrier.md`
- `plan/111-g1-obl024-projection-rust-fixture-guards.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2160-g1-obl024-replay-vocabulary-preflight.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
sed -n '1,240p' /home/codex/.codex/docs/oracle-chatgpt-pro.md
sed -n '1,220p' .docs/oracle-chatgpt-pro-operations.md
ask-chatgpt-pro -p "<OBL-024 replay vocabulary review prompt>" --file mirrorea_canon/theory/10-diagnostics.md --file mirrorea_canon/spec/07-diagnostics-format.md --file mirrorea_canon/theory/11-metatheory-ledger.md --file plan/81-g1-obl024-statement-shape-inventory.md --file plan/109-g1-obl024-lean-statement-draft.md --file plan/110-g1-obl024-executable-projection-carrier.md --file plan/111-g1-obl024-projection-rust-fixture-guards.md --file docs/reports/2159-g1-obl024-projection-rust-fixture-guards.md
sed -n '1,220p' mirrorea_canon/README.md
sed -n '1,260p' mirrorea_canon/MAP.md
date '+%Y-%m-%d %H:%M %Z'
python3 scripts/validate_docs.py
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
git diff --check
bash -lc 'ENDPOINT_SEGMENT=webhooks; ENDPOINT_PATTERN="discord[.]com/api|api/${ENDPOINT_SEGMENT}"; mapfile -t files < <(git ls-files --modified --others --exclude-standard); if ((${#files[@]} == 0)); then printf "no changed files\n"; exit 0; fi; if rg -n --pcre2 "$ENDPOINT_PATTERN" "${files[@]}"; then exit 2; else rc=$?; if [ "$rc" -eq 1 ]; then printf "no endpoint matches in changed files\n"; exit 0; fi; exit "$rc"; fi'
```

Final docs/source/leak validation passed.

## Evidence / outputs / test results

- Oracle returned from session `repository-context-mirrorea-canon-is` with model
  selection evidence `requested=Pro; resolved=Pro Extended`.
- Oracle recommended the docs-first package as the right next ratchet, with
  narrow scope and no code / expected JSON / Lean / canon edits.
- Oracle's key wording advice was mirrored into `plan/112`:
  - use qualified terms such as `report-local replay anchor` and
    `proof-level replay relation`;
  - treat `trace_local_replay` as serialized LAB consistency evidence only;
  - keep proof-level replay, replay witness, diagnostic-to-rejection relation,
    carrier projection, ordering, and final IDs unresolved.
- `python3 scripts/validate_docs.py` passed:
  `Documentation scaffold looks complete. Found 1312 numbered report(s).`
- `python3 -m unittest scripts.tests.test_validate_docs` passed: 20 tests.
- `python3 scripts/check_source_hierarchy.py` passed: 602 required paths, 602
  present, 0 missing.
- `git diff --check` passed.
- Endpoint scan over changed files passed: no endpoint matches in changed files.

## What changed in understanding

The next risk after `plan/110` and `plan/111` is not a missing executable field
or missing fixture guard. It is semantic drift: later prose could treat the
current `trace_local_replay` helper as if it were proof-level replay semantics.
`plan/112` adds a vocabulary boundary to prevent that drift while keeping the
current E-ROW evidence useful for future proof planning.

## Open questions

- Final Diagnostic JSON / ABI field names.
- Final request id, branch id, association key, and replay id semantics.
- Whether proof-level replay should be whole-judgment, rule-local, or both.
- Whether `NoEarlierTraceLocalFailure` belongs in OBL-024 or a later ordering
  theorem.
- How diagnostic ordering / equality interacts with OBL-021 determinism.
- Multi-span declaration-site / use-site blame semantics.
- Whether the first formal OBL-024 target is E-ROW-only or all diagnostic
  families.
- Whether mixed-row branches should ever get branch-local replay witnesses.

## Suggested next prompt

Continue with an OBL-024 Lean statement refinement that separates report-local
anchor vocabulary from future proof-level replay relation vocabulary, or switch
to another G1 proof-obligation ratchet if the next priority is broader theorem
coverage rather than diagnostic replay.

## Plan update status

Updated. Added `plan/112-g1-obl024-replay-vocabulary-preflight.md` and linked it
from `plan/00-index.md`, `plan/90-source-traceability.md`, `plan/109`,
`plan/110`, and `plan/111`.

## Documentation.md update status

Updated. `Documentation.md` now records `plan/112` as replay vocabulary
preflight and explicitly keeps proof-level replay / replay ABI open.

## progress.md update status

Updated. `progress.md` now records the Package 22 vocabulary boundary in current
G1 notes, feature maturity row, and recent log.

## tasks.md update status

Updated. `tasks.md` now records `plan/112` as docs-first replay vocabulary that
does not change production behavior, expected JSON, Lean files, repair output,
proof status, conformance, G1 exit, or canon.

## samples_progress.md update status

samples_progress.md 更新不要. No runnable sample path, validation command,
debug surface, or sample evidence status changed in this docs-only package.

## Reviewer findings and follow-up

Completed. Fermat read-only reviewer found one report provenance mismatch:
`plan/90` named `mirrorea_canon/README.md` and `mirrorea_canon/MAP.md` for
`plan/112`, but the draft report did not list those files in Documents
consulted or command log. I re-read both canon entry-point files and updated
this report. Reviewer found no prohibited overclaims; `samples_progress.md
更新不要` was accepted as defensible for this docs-only package.

## Skipped validations and reasons

- Rust/Python sample tests are not required for this docs-only package because
  no code, expected JSON, helper behavior, or sample data changed. They remain
  covered by Package 21.
- No Lean validation is required because no Lean files or manifest entries
  changed.
- No canon proof validation was run because the package does not edit canon or
  discharge OBL-024.

## Commit / push status

Pending. Package changes are not committed yet at this report draft stage.

## Sub-agent session close status

Fermat read-only reviewer `019f2a54-7e4d-70d1-80e9-301abf088f50` completed and
was closed. Oracle was used directly as an advisory browser consult.
