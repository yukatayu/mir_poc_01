# plan/159 - WRK evidence-commit integrity recut

## Purpose

This LAB implementation plan repairs the validator design behind the standing
L3 route. It does not change Mir theory, a Gate, a Phase, `theory/11`, a
contract, or L2 activation. Its governing direction remains ADR-0014 and the
owner-approved bounded-autonomy route in `plan/158`.

The plan follows an independent GPT-5.6 Sol Pro advisory review and an
independent planner review. Both found that assigning every descendant commit to
every active WRK is unsound: it serializes independent candidates and mishandles
merges. Their shared recommendation is a per-record append-only evidence-commit
list plus a graph-wide WRK identity audit and a clean validation worktree.

## Status

- **Settled direction:** L3 research is reversible; L2 remains fail-closed
  until a separate owner-authenticated trust anchor exists.
- **Working protocol refinement:** an L3 record owns retained source/test
  evidence through an append-only `Evidence commits:` field in its existing
  Results and review section.
- **Non-claim:** this is an evidence-integrity mechanism. It neither proves
  that an experiment was never run before registration nor authenticates
  external tools, services, rewritten-away history, helper intent, or L2
  authority.

## Target invariant

For every `WRK-####`, derive and retain:

```text
record ID + immutable path + pre-registration projection + registration commit
+ exact declared existing LAB locations + append-only evidence commits
```

The validator must establish all of the following within `HEAD`-reachable Git
history:

1. The registration commit has one parent, introduces one L3 record at its
   final path, and changes no retained research content.
2. Every descendant tree keeps the ID, path, and three pre-registration
   sections unchanged. A temporary rename, deletion, malformed ID, duplicate
   origin, or restored-later rewrite remains an error.
3. `LAB inputs` predate registration. Each listed evidence commit is a full,
   reachable commit strictly after registration, appears in exactly one WRK,
   and changes only that WRK's permitted existing LAB locations plus exact
   operational metadata.
4. A merge contributes only its combined local result, not paths merely
   imported unchanged from another parent. A mixed-WRK resolution must be
   split or escalated.
5. Authoritative validation runs in a clean disposable worktree. Dirty tracked,
   untracked, ignored, deleted, or unmerged research paths are not evidence.
   Known disposable build outputs remain outside the repository evidence set,
   not implicit accepted inputs.

## Exact operational metadata

Lane accounting may classify only these as operational metadata, never as
evidence or authorization:

- exact `docs/project-status.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`;
- exact canon `MAP.md`, `INDEX.json`, and `CHANGELOG.md` paths already governed
  by the canon process; and
- direct numbered Markdown reports in `docs/reports/`.

`working/README.md`, `review-keys.json`, prefix lookalikes, nested reports,
non-Markdown files, another WRK, helpers, schemas, CI, and Make files are not
operational metadata for this purpose.

## Implementation order

### 1. Freeze adversarial tests

- Keep the red tests for current-record deletion, transient malformed identity,
  ignored helper, and pre-registration side-branch merge. An unmanifested
  annex-policy change is deliberately not attributed to a WRK.
- Rewrite the former descendant-wide-lane tests around a manifested evidence
  commit: an unlisted independent commit does not belong to a WRK; a listed
  out-of-lane commit fails.
- Add graph tests for a metadata-only registration, two independent WRKs,
  duplicated evidence ownership, pre-registration evidence, append-only list
  removal, and merge-local conflict resolution.

### 2. Replace per-record descendant scans

- Remove the current `_working_record_lane_errors` descendant-wide traversal.
- Add one parent-before-child graph pass over all reachable direct entries under
  `mirrorea_canon/working/`, including malformed entries and branches whose
  final tree restores an earlier state.
- Cache tree-state parsing by Git tree object so the audit remains proportional
  to reachable commits and changed annex trees rather than active records times
  descendants.

### 3. Bind retained evidence

- Add `Evidence commits:` to the existing Results and review field set and
  document its empty L3 value and append-only full-SHA-1 form.
- Verify location, reachability, registration ancestry, global exclusivity,
  local commit delta, snapshot ancestry, and exact operational metadata.
- Use combined merge diffs for merge-local results; do not use `diff-tree -m`
  as evidence ownership.

### 4. Separate clean-state validation

- Add an authoritative validation mode which requires the exact committed
  worktree and reports every dirty/ignored/untracked/nonmetadata path.
- Run it from a disposable detached worktree for an L3 pilot closeout. A normal
  developer worktree may receive diagnostics but is not an authoritative
  evidence pass.

### 5. Synchronize and verify

- Update ADR-0014 derivative instructions, `working/README.md`, plan/158,
  source hierarchy guidance, dashboards, report, and canonical index through
  the existing proposal/changelog process.
- Run focused red/green tests, the full documentation suite, index generation
  and freshness, source hierarchy, `make docs`, `make check`, Rust check, diff
  check, independent review, commit, and push.

## Remaining reserved or unresolved matters

- L2 trust-anchor activation and reviewer identity remain owner/canon work.
- A Git validator cannot determine whether an arbitrary source file is truly a
  helper family or whether an unlisted experiment influenced a conclusion.
- External dependencies, network responses, compiler provenance, and history
  removed from the reachable graph remain outside this evidence boundary.
- L0/L1, contracts, SCN/Gate/Phase, ledger movement, proof/OBL discharge, and
  public claims remain unchanged and reserved.
