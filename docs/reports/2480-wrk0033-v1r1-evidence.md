# Report 2480 - WRK-0033 V1/R1 presentation-refinement evidence

**Identifier:** `LAB-REPORT-2480`
**Date:** 2026-07-28 12:26 JST
**Status:** validated evidence package; commit/push pending

## Objective

Execute exactly the finite conditional-lemma route pre-registered by WRK-0033:
compare an administrative binding presentation with a one-slot machine
presentation, then show the finite distinctions created by weakening matching,
single-use, or failure exclusion.

## Scope and assumptions

The source is retained only in `plan/wrk-0033-v1r1-presentation-refinement.md`
and materialized to a disposable `/tmp` file for Lean. It uses opaque LAB
labels. It does not define Mir syntax or semantics, and it does not create a
sample, helper, schema, module, manifest, CI target, or public interface.

## Start state / dirty state

The start point was clean at `32e7d9a8` on `main`, equal to `origin/main`.
WRK-0033 was committed and pushed with no outcome artifact or evidence commit.

## Documents consulted

- `mirrorea_canon/adr/ADR-0014.md`
- `mirrorea_canon/working/README.md`
- `mirrorea_canon/working/WRK-0033-v1r1-presentation-refinement.md`
- `mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md`
- `plan/187-mircore-value-flow-and-occurrence-decision-packet.md`
- `plan/202-v1-r1-presentation-refinement-candidate-selection.md`
- `docs/reports/2478-v1-r1-presentation-refinement-selection.md`
- `docs/reports/2479-wrk0033-v1r1-registration.md`

## Actions taken

1. Retained the exact finite Lean source as a fenced LAB artifact.
2. Kept administrative and machine states distinct and connected only by an
   explicit refinement mapping.
3. Added the positive presentation theorem, one-shot/failure checks, and the
   three registered adversarial distinctions.
4. Ran the pre-registered source/hash/query/Lean/diff validation sequence.

## Files changed

- `plan/wrk-0033-v1r1-presentation-refinement.md`
- `plan/00-index.md`
- this report

## Commands run

- all eight registered nonempty-input checks
- registered `sha256sum` over Canon and LAB inputs
- registered source-local `rg` query
- registered `awk` extraction to
  `/tmp/mir-wrk0033-v1r1-presentation-refinement.lean`
- `lean --trust=0` on the extracted 133-line source
- focused forbidden-token audit of the extracted source
- `git diff --check`
- final documentation validation recorded below

## Evidence / outputs / test results

All registered outcome commands passed at 2026-07-28 12:26 JST. The eight
pinned source hashes exactly matched WRK-0033. The source query returned P012's
V1 machine-presentation boundary, R1 matching-receipt boundary, and Plan 187's
comparison obligations. Lean 4.29.1 completed `--trust=0` without output on
the 133-line extracted source, whose SHA-256 was
`7436c62eb3406f1e91ba7d3546ec979dfd7f2484557a941607b9f9082cac39ec`.
The focused audit found no `sorry`, `admit`, `axiom`, `unsafe`, `partial`,
`implemented_by`, `Classical`, or `Choice` token, and `git diff --check`
passed.

`presentation_refinement` exhaustively compares the six finite administrative
states with the four finite reply labels through `toMachine`. The one-shot and
failure theorems pass for both presentations. The three adversarial theorems
separately distinguish a swapped reply, duplicate reply, and failure followed
by an attempted success when matching, single-use, or failure exclusion is
weakened. This is conditional finite evidence only.

The final `make docs` pass checked 120 Canon files, all 752 required hierarchy
paths, and 1634 numbered reports. Its first run exposed a stale `progress.md`
last-updated header left by the prior Plan 202 snapshot update; the header-only
repair is intentionally held for the separate snapshot-link commit and the
final validation passed with it present.

## What changed in understanding

Within the explicit finite boundary, an administrative binding and one-slot
machine state can carry the same observable waiting/success/failure result.
That result depends on matching, single-use, and failure exclusion. The three
adversarial distinctions make each dependency visible, so no one of these
facts becomes eligible for ergonomic omission merely because the two positive
presentations agree.

The first uncommitted authoring attempt had a cross-inductive proof-tactic
error and a Markdown-escaping error in conjunction notation. They were fixed
before any evidence commit, did not alter WRK-0033's pre-registration, and did
not meet its semantic falsifier conditions. The final retained source is the
one that passed every registered command.

## Open questions

- Whether a later Canon design can supply the missing correlation, pending
  carrier, payload, failure, persistence, and source-elaboration relations.
- Whether a future ergonomic elaboration can reconstruct the corresponding fact
  and unique basis without widening this finite boundary.

## Suggested next prompt

Link this retained LAB artifact forward in WRK-0033 metadata, synchronize the
current LAB snapshots, and leave C3 proper and C7 inference deferred.

## Plan update status

更新済み: `plan/wrk-0033-v1r1-presentation-refinement.md` と `plan/00-index.md`
に限定 evidence と長期 index entry を追加した。

## Documentation.md update status

更新不要: evidence commit を WRK-0033 の declared LAB locations と operational
metadata boundary 内に保つため。次の snapshot-link package で reader-facing
pointer を更新する。

## docs/project-status.md update status

更新不要: evidence commit を declared LAB locations と operational metadata
boundary 内に保つため。次の snapshot-link package で current status を更新する。

## progress.md update status

更新不要: current progress snapshot の意味論的更新は metadata link 後に別 package
として行う。現在の header repair はその後続 package にのみ含める。

## tasks.md update status

更新不要: current task map の outcome link は metadata link と同じ snapshot package
で更新する。

## samples_progress.md update status

更新不要: evidence は runnable sample、active root、validation command、debug
surface のいずれも変更しない。

## Reviewer findings and follow-up

No additional Oracle consultation was needed after the pre-registration: the
model is the narrow route already selected after advisory review. No callable
sub-agent interface was available.

## Skipped validations and reasons

No sample or executable build validation applies. Documentation validation is
run before this evidence package is committed. No frozen falsifier occurred.

## Commit / push status

Pending commit, push, and branch-equality check after final documentation
validation.

## Sub-agent session close status

No callable sub-agent session was opened.
