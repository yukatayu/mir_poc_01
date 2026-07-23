# Report 2405 - WRK-0020 option admit falsifier

## Title and identifier

Report 2405 - WRK-0020 option admit falsifier.

## Objective

Execute the exact post-push WRK-0020 command once, classify its first registered
falsifier without repairing it, and freeze the record before any semantic claim.

## Scope and assumptions

- The registration at `fec0a0da` is immutable. No command quoting, source
  assertion, input, or carrier interpretation is revised in this package.
- The failure is classified only as a command-level inability to establish the
  registered text facts. It is not evidence that the named Canon/LAB texts
  agree or disagree.
- Canon remains normative; the plan memo is LAB failure evidence only.

## Start state / dirty state

The worktree was clean at `fec0a0da`, with `main` equal to `origin/main` after
WRK-0020 registration and its successful `make check` history validation.

## Documents consulted

- Canon: `ADR-0014`, `working/README`, WRK-0020, `theory/01`, `theory/06`,
  `spec/02`, and `spec/04`.
- LAB: `plan/07`, current-L2 e3, `progress.md`, `tasks.md`, and Report 2404.
- Process: `AGENTS.md` and the systematic-debugging procedure.

## Actions taken

1. Executed the record's exact command only after its registration was pushed.
2. Read the shell and Python failure output and traced it to the unescaped
   backticks inside the double-quoted Python command argument.
3. Did not alter, retry, or repair the registered command.
4. Set WRK-0020 reliance to `frozen` and retained the command-level failure in
   a permitted LAB memo.

## Files changed

- `mirrorea_canon/working/WRK-0020-option-admit-carrier-literal-audit.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/INDEX.json`
- `plan/wrk-0020-option-admit-carrier-literal-audit.md`
- `plan/00-index.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2404-wrk0020-option-admit-registration.md`
- `docs/reports/2405-wrk0020-option-admit-falsifier.md`

## Commands run

- `make check` after `fec0a0da` registration
- the exact registered WRK-0020 command, extracted from the working record and
  evaluated once after push
- root-cause source inspection of the quoted command and its shell/Python
  failure output
- `make check` at frozen evidence commit `19ee70e6`, which passed Canon index,
  source-hierarchy, documentation, and Cargo checks
- `sha256sum plan/wrk-0020-option-admit-carrier-literal-audit.md`, which
  produced `6db9d08c49d616352c715ecfe65c2e365af864958a649fffd4cb02b58ad5b2cf`
- follow-up manifest metadata update, diff review, commit, and push at
  `3d281755`
- clean detached worktree audit at `3d281755`: `make docs`
- final report closeout diff review, commit, push, and main-worktree
  `make check`

## Evidence / outputs / test results

The exact command emitted `OptionDecl.admit: command not found` from Bash,
followed by Python `AssertionError`. The backticks in the text assertion were
command-substituted by the outer double quotes, so the Python program did not
receive the registered literal. The wrapper's later `printf` was unconditional
and does not change the failed command result.

This satisfies WRK-0020's `cannot establish the exact selected text facts`
falsifier. No positive source audit, parser test, runtime test, or Lean test
was run or claimed.

Frozen evidence is retained only as
`LAB:plan/wrk-0020-option-admit-carrier-literal-audit.md` at
`19ee70e6bf791f748e3c0f7348e97e9b480aad08` with SHA-256
`6db9d08c49d616352c715ecfe65c2e365af864958a649fffd4cb02b58ad5b2cf`.

The detached audit of manifest commit `3d281755` passed `make docs`: Canon
index check reported 102 files, source hierarchy reported all 733 required
paths present, and documentation validation exited successfully. This validates
the record structure and documentation graph, not the failed source-audit
claim.

## What changed in understanding

The candidate's first evidence is a reproducible pre-registration failure, not
a semantics result. The project guardrail worked: the record freezes instead of
allowing a convenient command repair to become evidence.

The earlier post-PROPOSAL-010 source screen is only a source-cut result for its
reviewed runtime/Surface, current-L2, and Product Alpha families. It did not
review this later-named Canon literal chain, so it cannot be read as a ban on
WRK-0020 or another distinct future L3 audit.

## Open questions

- Is a distinct, independently pre-registered Option/admit source audit useful
  after this frozen record, or would it duplicate this failure evidence?
- Does the independent `G_e` row-kind literal audit offer a more useful next
  bounded research package?

## Suggested next prompt

Review the frozen WRK-0020 artifact as a failed command boundary, then select
only a distinct new L3 question rather than repairing its command.

## Plan update status

`plan/` 更新済み: the unnumbered WRK-0020 memo records the exact failure,
root-cause boundary, and forbidden repair; `plan/00-index.md` now links it.

## Documentation.md update status

`Documentation.md` 更新不要: no reader-facing workflow or capability changed.

## docs/project-status.md update status

更新済み: WRK-0020 is now frozen at its command-level falsifier, with no
source-consistency or Canon conclusion.

## progress.md update status

更新済み: the current snapshot and dated log distinguish the frozen command
failure from a semantic result and qualify the earlier source-cut screen.

## tasks.md update status

更新済み: the task map closes WRK-0020 and forbids a repair/retry route; it also
corrects the historical source-cut screen so it does not misstate a prohibition
on this distinct audit.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, command, debug surface, or
sample evidence classification changed.

## Reviewer findings and follow-up

The registration-diff reviewer found no blocking issue before execution and
confirmed that the record should freeze rather than repair on a falsifier. The
failure itself needs no semantic reviewer: it is directly shown by the shell
and Python error output. The focused frozen-falsifier diff review found no
blocking issue. The clean detached `make docs` audit of the manifest commit
also passed; it validates document/record structure only, not source facts.

## Skipped validations and reasons

The exact audit command is deliberately not retried after its registered
falsifier. No modified substitute command, parser/runtime/Lean command, or
semantic source comparison is run because each would be a repair or a new
experiment outside this record.

## Commit / push status

Frozen evidence commit `19ee70e6bf791f748e3c0f7348e97e9b480aad08` was pushed
with `--no-gpg-sign`. Manifest commit `3d281755` was also pushed with
`--no-gpg-sign`; it appends only the exact commit/hash to the working-record
manifest and corrects the source-cut status wording. This report closeout will
be committed and pushed immediately with `--no-gpg-sign`.

## Sub-agent session close status

The registration-diff reviewer completed read-only work and was closed after
manifest integration, reporting no blocking finding. No sub-agent made
repository edits or interpreted the failed command as a semantic result.
