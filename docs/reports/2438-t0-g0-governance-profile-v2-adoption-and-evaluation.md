# 2438 - T0/G0 governance-profile v2 adoption and evaluation

## Objective

Apply the owner's O0 decision through the normal Canon procedure, generate the
single permitted v2 artifact, and record its actual result without advancing
G0-D3, a Gate, a Phase, an OBL, conformance, or implementation authorization.

## Scope and assumptions

The exact owner decision authorizes v2, `pass` as the only success literal,
retention of v1 as nonconforming historical evidence, and one fresh v2
evaluation. It does not authorize re-pinning current controls, another
artifact, G0-D3 acceptance, G0 exit, T1 entry, or I1 authorization.

## Start state / dirty state

Started at clean, pushed `main` `360792e4`. The v1 artifact existed at
`plan/155-t0-g0-governance-profile-evaluation.json`; its source contract had
the known root-result vocabulary contradiction.

## Documents consulted

Canon `README.md`, `MAP.md`, ADR-0013/0014, `plan/01-phases.md`,
`meta/style-guide.md`, `CHANGELOG.md`, and the T0 proposal were read before
editing. LAB evidence included Plans 153--155, 196--197, the current status
snapshots, and the v1 artifact. A temporary GPT-5.6 Sol Pro Oracle consultation
and planner/reviewer sub-agents reviewed the amendment and artifact binding.

## Actions taken

Added PROPOSAL-014 and amended ADR-0013 in place. Version 2 replaces v1 as the
current T0 profile, pins the historical v1 file, uses `pass`/`pending`/`fail`,
and defines a one-parent direct-child artifact route. The v2 artifact was then
created once from exact `C2` Git blobs and committed alone. It derives `fail`:
the source-hierarchy control plus `CANON.md`, `README.md`, and `AGENTS.md` no
longer match their fixed pins. No pin was changed to obtain `pass`.

## Files changed

Canon: PROPOSAL-014, ADR-0013, the v2 phase profile, CHANGELOG, ADR index, and
generated Canon INDEX. LAB: Plan 198 JSON and explanatory note, historical T0
memory, status snapshots, HTML overview, required-path guards, and this report.

## Commands run

Ran Canon index generation/check, source hierarchy and documentation guards,
`make check`, focused documentation/index unit tests, `cargo check`, focused
diff checks, v1 blob preservation checks, and independent in-memory validators
against both the pre-commit source cut and committed v2 artifact. Also ran
`git fsck --no-reflogs --full`.

## Evidence / outputs / test results

`C2` is `0ee3fdec553de31252a37478fc4a31f507221258`; `B2` is direct child
`39c85e9f99ae45bcb097939a543b39dbb7bb9b81` and adds only the v2 JSON path.
The committed validator confirmed exact Git-blob hashes, check/evidence order,
duplicate-key rejection, root derivation, RFC 8785 digest
`b3748cb62569e25da4eaabf124eea97e97e98147185e433557575127b33895e4`, v1 byte
preservation, and absence of the v2 path from every pre-B2 ancestor. The checks
are `pass`, `fail`, `pass`; root is valid `fail`. `make check`, focused unit
tests, and `cargo check` passed. `git fsck` returned success but listed
pre-existing dangling objects; none were removed.

## What changed in understanding

O0 fixes the v1 profile-contract defect but does not make the present control
state pass. A valid `fail` is useful evidence of fixed-byte drift, yet cannot
be treated as a malformed artifact or as G0-D3 evidence. Re-basing controls is
a distinct owner/Canon choice rather than an implementation detail.

## Open questions

Whether to retain the historical pins and defer, scope the detected drift for
audit, or open a normal Canon rebase proposal is unresolved. Until a separately
authorized valid `pass` route exists, G0-D3 cannot be accepted. The later
T1/T2/I1 lifecycle and semantic decisions remain unchanged.

## Suggested next prompt

Review the four fixed-control drifts and decide whether to keep the current
defer, request a scoped drift audit, or authorize a normal Canon proposal for a
new control baseline. Do not authorize another artifact implicitly.

## plan/ update status

Updated: added Plan 198; indexed it; and corrected Plans 153--155 and 196--197
to distinguish historical v1, v2 `fail`, and the remaining decision boundary.

## Documentation.md update status

Updated: current-position and roadmap rows now state v2 `fail` and the absence
of G0-D3/T1/I1 effects.

## docs/project-status.md update status

Updated: the current status, stop line, owner-decision table, sources, and
timestamp now show the v2 artifact result and its limits.

## progress.md update status

Updated: logical-status, blockers, T0 milestone, owner/Canon items, and dated
recent log now distinguish the resolved v1 defect from the unresolved drift.

## tasks.md update status

Updated: the completed O0 package and current fixed-control-drift blocker
replace the stale profile-repair/fresh-evaluation tasks.

## samples_progress.md update status

Not updated: no runnable sample, command, debug surface, or sample blocker
changed.

## reviewer findings and follow-up

Planner and reviewer independently required an ADR-0013 amendment rather than
a new ADR, strict v1 preservation, a direct-child v2 artifact, fixed rather
than re-pinned controls, and an independent validator. Final review added the
v1 whole-file pin, all-ancestor path absence, required CHANGELOG adoption, and
removal of an unapproved object-order relaxation; all four were applied before
`C2`.

## skipped validations and reasons

No changed runtime behavior exists. Browser visual interaction for the static
HTML wording was not run; its text-only update was covered by focused diff
inspection and repository documentation validation. No cleanup was performed
for the pre-existing dangling Git objects reported by `git fsck`.

## commit / push status

Canon adoption `0ee3fdec` and sole artifact `39c85e9f` are pushed to
`origin/main`. This report and the synchronized LAB status files are included
in the final synchronization commit, which is pushed with this task close.

## sub-agent session close status

The planner and reviewer completed their read-only assignments; both sessions
were explicitly closed after their findings were integrated.
