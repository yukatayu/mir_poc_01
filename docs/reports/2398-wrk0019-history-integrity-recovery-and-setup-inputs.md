# Report 2398 - WRK-0019 history-integrity recovery and setup inputs

- Date: 2026-07-23 18:19 JST
- Author / agent: Codex
- Scope: evidence-history recovery, status synchronization, and concise README
  setup-input guidance
- Decision levels touched: no Canon theory, ledger, Gate, Phase, grammar,
  implementation, or OBL decision

## Objective

Restore the unchanged working-annex provenance rule after an invalid evidence
commit, then document the actual Product Alpha Docker fixture inputs concisely.

## Scope and assumptions

The prior evidence command was valid, but its first retained commit changed
status documents outside WRK-0019's declared package. History-wide validation
therefore required reconstructing only those two commits after explicit user
approval. The README table documents a controlled fixture, not a public
credential or production setup surface.

## Start state / dirty state

Started from pushed invalid tip `8b3b0385`; evidence commit `45941f47` had
changed `Documentation.md`, `docs/project-status.md`, `progress.md`,
`samples_progress.md`, and `tasks.md` outside the registered evidence package.
The authoritative validator correctly rejected reachable history.

## Documents consulted

Read the WRK-0019 registration, ADR-0014, working-annex validator, existing
Product Alpha Docker Compose fixture and CLI transport implementation, root
README, report template, status snapshots, and the local Oracle operating
instructions. A temporary Oracle review independently compared freeze,
validator-exception, and history-reconstruction options.

## Actions taken

1. Confirmed the prior evidence commit violated its declared path boundary.
2. Obtained explicit approval for `--force-with-lease` recovery and reset local
   `main` to the last valid registration closeout.
3. Recreated only the exact sidecar, reran the registered command, and pushed
   a replacement evidence commit with only permitted paths.
4. Added the working-record result metadata in a separate control commit.
5. Synced status documents and added a concise README table for direct Docker
   Compose diagnostic inputs in this normal documentation commit.

## Files changed

- `README.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- this report

## Commands run

- Git history/path audit and remote-tip comparison
- exact registered WRK-0019 command with four SHA-256 guards
- `git push --force-with-lease=refs/heads/main:8b3b0385f73e5472fe764f52af77160c4df7a6ec`
- `make check` and `python3 scripts/validate_docs.py --authoritative-working-annex`

## Evidence / outputs / test results

Replacement evidence commit `249bc846` contains exactly the declared sidecar,
LAB memo/index, and direct report. Metadata commit `ad634ca9` links that commit
from WRK-0019. The rerun again passed every pinned digest, all 15 computational
matrix rows, both focused Rust tests, Product Alpha `check`, and the registered
`run-local` exit-2 `MirCompute` / `OutOfBounds` observation.

The old commits are no longer reachable from `main`; their identifiers are
retained here as the incident record. The validator itself was not changed.
The normal check and the authoritative working-annex audit both passed on the
reconstructed reachable history.

## What changed in understanding

For history-audited L3 evidence, documentation/status synchronization is a
separate normal commit even when it describes the result. A forward revert or
frozen record cannot repair a reachable invalid evidence commit; the invariant
requires a narrowly scoped history reconstruction.

The Docker fixture has no user password or account input. Its literal fixture
token is an accidental-use guard only, and normal CLI/helper paths inject all
Compose values themselves.

## Open questions

- No general direct P-COMP-03 carrier, public error contract, or workflow claim
  follows from WRK-0019.
- Core/result correspondence, global-step coverage, and outcome-totality
  placement remain independent formal boundaries.

## Suggested next prompt

Continue from the repaired checkpoint by selecting a distinct pre-registrable
research target, or request an owner/canon decision for a formal-interface
boundary. Do not use WRK-0019 as a coverage-widening or repair basis.

## Plan update status

`plan/` 更新不要: the evidence memo and index were correctly created in the
preceding strict evidence commit; this package does not change plan content.

## Documentation.md update status

`Documentation.md` 更新済み: it now distinguishes the completed one-sidecar
observation from a general computational workflow claim.

## docs/project-status.md update status

更新済み: the control view records the scoped result and refreshed timestamp.

## progress.md update status

`progress.md` 更新済み: the dated log records the integrity recovery and the
non-promotion boundary.

## tasks.md update status

`tasks.md` 更新済み: package 52 records evidence closeout and its lack of an
automatically selected successor.

## samples_progress.md update status

`samples_progress.md` 更新済み: the computational row now records the exact
sidecar observation without changing workflow classification.

## Reviewer findings and follow-up

The prior outcome review found only a stale timestamp, which this package
corrects. The temporary Oracle review advised the approved history
reconstruction because a frozen record or validator exception would preserve
an invalid reachable history. The final reviewer found one P2 README error:
`session` reads a stored session rather than creating it. The table now cites
the `run-local`-returned `session_path`; no other scope or factual defect was
found. The corrected diff, normal check, and authoritative audit passed.

## Skipped validations and reasons

No broad release workflow, Docker runtime, or public/product claim validation
was run: neither the strict evidence repair nor README input table implies a
new transport or product claim.

## Commit / push status

The strict replacement evidence commit `249bc846` was force-with-lease pushed,
then metadata commit `ad634ca9` and status/setup commit `e348b159` were
normally pushed. This report closeout follows as its own commit.

## Sub-agent session close status

The temporary Oracle consultation and final reviewer completed and are advisory
only. No sub-agent edited repository files in this package.
