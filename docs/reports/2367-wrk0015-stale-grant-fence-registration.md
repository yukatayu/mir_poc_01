# Report 2367 - WRK-0015 stale-fence registration preflight

- Date: 2026-07-22 21:12 JST
- Author / agent: Codex
- Scope: validate whether the selected P-SURF-05 candidate can enter L3
- Decision levels touched: none; no WRK was created

## Objective

Determine whether the selected stale-fence second-admission experiment has an
existing permitted LAB lane and can therefore be pre-registered before any
fresh outcome command.

## Scope and assumptions

No outcome command may run during this preflight. Canon authority, revocation,
rejoin, membership lifecycle, OBL-028, defect diagnosis, repair, and workflow
claims are excluded.

## Start state / dirty state

`main...origin/main` was clean at `fae1c98f`. The preceding selection had
explicitly excluded its unregistered preliminary command from any WRK evidence.

## Documents consulted

Read Canon README/MAP, ADR-0014, working-annex requirements, the documented
LAB-root allowlist in `scripts/validate_docs.py`, the P-SURF-05 README, pinned
checker/test/sample inputs, the selection memo, and current LAB snapshots.

## Actions taken

Prepared a registration preflight, then checked its pinned inputs against the
working-annex allowlist. The checker/test/sample inputs are outside the current
permitted roots and the target-literal search found no matching input inside one. Removed the
uncommitted draft rather than widening the policy or disguising the input.
Recorded the stop and reopen condition in LAB memory. No outcome command ran.

## Files changed

- `plan/wrk-0015-stale-grant-fence-registration-preflight.md`
- `plan/00-index.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- this report

## Commands run

- targeted `sed`, `rg`, `git show`, and `sha256sum` input/root inspection
- temporary registration preflight through `python3 scripts/validate_docs.py`
- allowed-root search across `plan` and all current permitted sample roots
- `make docs`, source-hierarchy, Canon-index, and whitespace validation

## Evidence / outputs / test results

No result-producing Cargo command has run. The preflight validator rejected
the temporary draft because its required `crates/mir-semantics` and
`samples/full-system-v1-surface` inputs are not permitted LAB locations. The
target-literal allowed-root search returned no matching input; it does not rule
out every semantically similar input. Therefore no WRK, evidence artifact, or
evidence commit exists.

## What changed in understanding

An existing LAB source is not automatically an existing *permitted* WRK lane.
The annex's input-location rule prevents a local source experiment from
silently broadening the governance/evidence surface.

## Open questions

- Does a matching documented input later appear in a permitted LAB root?
- Would the owner ever explicitly reopen the evidence-lane policy? No such
  decision is requested or assumed by this preflight.

## Suggested next prompt

Select the next standing-eligible candidate from an existing permitted LAB
lane. Do not execute this candidate or expand the allowlist autonomously.

## Plan update status

`plan/` 更新済み: the new preflight memo records the exact lane-policy stop,
rejected workarounds, and reopen condition; `plan/00-index.md` links it.

## Documentation.md update status

更新済み: the reader map distinguishes selection from the unregistrable
preflight stop; it does not present a WRK record.

## docs/project-status.md update status

更新済み: the human control view states that no WRK exists and that policy is
not widened for this candidate.

## progress.md update status

更新済み: the logical and macro snapshots distinguish candidate selection from
an unregistrable preflight stop, with no lifecycle or evidence promotion.

## tasks.md update status

更新済み: task 39 is closed as a preflight stop and names only sound reopen
conditions.

## samples_progress.md update status

`samples_progress.md` 更新不要: no committed sample, validation command,
debug surface, or runnable workflow classification changed.

## Reviewer findings and follow-up

The independent reviewer correctly required a checker-report observation rather
than direct fence-persistence language and a broad falsifier. The subsequent
validator finding is more basic: the exact source/test/sample inputs cannot be
declared in the current permitted LAB roots. Oracle session
`wrk0015-lab-lane-governance-20260722` independently concluded that this is an
admission-policy block, not a frozen WRK or experimental falsifier; its advice
is non-normative and cannot widen policy. The final reviewer did not return
after two long waits, so no reviewer finding is inferred from it. The next
package must not reuse the excluded preliminary command output.

## Skipped validations and reasons

Cargo and the broad suite are intentionally skipped: no valid pre-registration
exists, so running the command would create excluded output. No source/runtime
change occurred, so unrelated heavy builds are not justified.

## Commit / push status

Pending at report write. This preflight/package will be committed with
`--no-gpg-sign` and pushed; it contains no WRK registration or outcome.

## Sub-agent session close status

The initial read-only registration reviewer and the Oracle sub-agent completed
without edits and are closed. The final read-only reviewer did not answer after
two long waits and was closed without a result; local validation and the
separate Oracle conclusion are recorded above.
