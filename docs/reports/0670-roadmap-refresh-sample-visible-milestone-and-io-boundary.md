# Report 0670 — Roadmap Refresh for Sample-Visible Verification Milestone and Host-Facing I/O Boundary

- Date: 2026-04-13T00:42:16.294747Z
- Author / agent: Codex
- Scope: `tasks.md` / `progress.md` / repository memory refresh after approving a natural near-term roadmap toward sample-visible theorem and model-check validation, plus later positioning of host-facing I/O / adapter / FFI / engine integration
- Decision levels touched: L1 roadmap ordering mirror, L2 task sequencing mirror, L2 terminology guardrail for later host-facing boundary wording

## 1. Objective

Refresh the repository snapshot so that:

1. the near-term task order explicitly aims at a sample-visible theorem / model-check milestone before wider public-surface work,
2. the later I/O / adapter / FFI / engine line is positioned as a docs-first boundary task rather than immediate implementation work,
3. no document silently promotes unsettled terminology such as `host-facing port` to a settled normative term,
4. `tasks.md`, `progress.md`, `Documentation.md`, relevant `plan/`, and user-facing guidance remain mutually consistent.

## 2. Scope and assumptions

- No normative `specs/` text was changed in this task.
- This task updates mirrors, roadmap memory, and FAQ / guidance based on already-established architectural anchors.
- `host-facing port` is treated only as a **working label** for discussion convenience. The settled architectural anchors remain the Typed-Effect Wiring Platform, the `richer host interface` open line, and later adapter boundaries such as visualizer / host substrate / FFI / engine integration.
- Full strong type-system research remains a later workstream and is not pulled into the immediate task line by this refresh.

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/02-system-overview.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `faq_003.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `samples/current-l2/README.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- Prior package / audit reports: `0658`, `0659`, `0665`, `0667`, `0668`, `0669`

## 4. Actions taken

1. Recut the near-term task line in `tasks.md` so that the next natural sequence is:
   - shared-space admission / compile-time visibility reopen
   - shared-space authority / resource ownership reopen
   - model-check concrete carrier actualization comparison
   - model-check concrete carrier first actualization
   - source-sample emitted verification artifact wiring
   - sample-facing theorem / model-check evidence summary and bless/review flow
   - later docs-first I/O / host-facing boundary comparison
2. Rewrote `progress.md` to mirror that order, to separate current near-term milestone work from later public-surface and host-facing lines, and to add an explicit feature-family row for host-facing I/O / adapter integration.
3. Updated `Documentation.md`, `README.md`, `faq_003.md`, `samples/current-l2/README.md`, and `.docs/current-l2-source-sample-authoring-policy.md` so the human-facing explanation matches the new roadmap.
4. Updated repository-memory documents in `plan/01`, `08`, `09`, `10`, `11`, `12`, `13`, `17`, and `90` to remove stale ordering and preserve the same current / reserve split.
5. Added a source-traceability addendum in `plan/90` to anchor this roadmap refresh to the relevant higher-level specs, recent example packages, and this report.

### Files changed

- `.docs/current-l2-source-sample-authoring-policy.md`
- `Documentation.md`
- `README.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `faq_003.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `samples/current-l2/README.md`
- `tasks.md`
- `docs/reports/0670-roadmap-refresh-sample-visible-milestone-and-io-boundary.md`

## 5. Evidence / outputs / test results

### Resource check

```text
$ df -h .
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        99G   76G   19G  81% /
```

```text
$ free -h
               total        used        free      shared  buff/cache   available
Mem:           960Mi       750Mi        95Mi       2.0Mi       114Mi       209Mi
Swap:           19Gi       1.8Gi        17Gi
```

### Discord baseline

```text
$ python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
Task baseline recorded.
```

### Report creation

```text
$ python3 scripts/new_report.py --slug roadmap-refresh-sample-visible-milestone-and-io-boundary
Created docs/reports/0670-roadmap-refresh-sample-visible-milestone-and-io-boundary.md
```

### Interim diff snapshot

```text
$ git diff --stat
17 files changed, 316 insertions(+), 79 deletions(-)
```

### Sample inventory check

```text
$ python3 scripts/current_l2_source_sample_regression.py inventory
current L2 fixed-subset authored inventory
sample stem | authored status | expected static | expected runtime | formal hook | file state | note
----------------------------------------------------------------------------------------------------
e1-place-atomic-cut | source-authored | valid | explicit_failure | runtime_try_cut_cluster | present | first widened authored row runtime path
e2-try-fallback | source-authored | valid | success | runtime_try_cut_cluster | present | current authored runtime path
e3-option-admit-chain | source-authored | valid | success | not_reached_guarded | present | third widened authored row runtime path; formal hook stays guarded
e4-malformed-lineage | source-authored | malformed | not_evaluated | fixture_static_cluster | present | current authored static stop
e19-malformed-target-mismatch | source-authored | malformed | not_evaluated | fixture_static_cluster | present | stable-static edge-pair target-mismatch row
e21-try-atomic-cut-frontier | source-authored | valid | success | runtime_try_cut_cluster | present | second widened authored row runtime path
e22-try-atomic-cut-place-mismatch | source-authored | valid | success | runtime_try_cut_cluster | present | post-sextet first contrast-row runtime path
e23-malformed-try-fallback-missing-fallback-body | source-authored | malformed | not_evaluated | fixture_static_cluster | present | current authored static stop
```

### Runtime regression surface sanity check

```text
$ cargo test -p mir-runtime --test current_l2_source_sample_runner
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.32s
Running tests/current_l2_source_sample_runner.rs (...)

running 10 tests
...
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

### Final documentation validation

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 669 numbered report(s).
```

```text
$ git diff --check
(no output)
```

### Reviewer pass

- Initial reviewer pass found 3 doc-consistency issues:
  - premature recent-log wording in `progress.md`
  - placeholder evidence wording in this report
  - package-count drift between roadmap mirrors
- Those were fixed in the same task.
- Narrow re-review then found only the remaining placeholder evidence wording in this report.
- After this evidence block was filled, the final narrow reviewer response was `No findings remain.`

- The repository snapshot now consistently places the sample-visible theorem / model-check milestone **before** later public API / CLI reopening.
- Host-facing I/O / adapter work is represented as a **later docs-first boundary comparison**, not as an immediate implementation obligation.
- `host-facing port` is explicitly marked as a working label rather than a settled spec term.
- No normative `specs/` file was changed, so no settled semantics were rewritten as part of this task.

## 6. What changed in understanding

- The previous task ordering underrepresented the fact that the next natural research milestone is not another public-surface reopening, but a narrow sample-visible verification milestone.
- The repo already contains the right architectural anchors for later host-facing I/O and engine / FFI integration, but they live under broader concepts (`Typed-Effect Wiring Platform`, `richer host interface`, adapter boundaries). The refresh therefore needed to avoid overcommitting to a prematurely specific name.
- Public operational API / CLI work remains important, but it is better treated as a later gate after the next verification-visible sample milestone.

## 7. Open questions

- What should the eventual settled name be for the capability-scoped host-facing input/output boundary?
- How much of the host-facing boundary should be modeled as pure typed-effect routing versus explicit adapter contracts?
- At what point should visualizer / host substrate / FFI / engine adapters split into separate workstreams rather than one shared docs-first comparison line?
- When the model-check concrete carrier is actualized, which emitted artifact shape should be considered the minimal stable bridge for sample-facing bless/review flow?

## 8. Suggested next prompt

Proceed with the refreshed near-term line in order:

1. `shared-space admission / compile-time visibility reopen`
2. `shared-space authority / resource ownership reopen`
3. `model-check concrete carrier actualization comparison`

Then continue through the sample-visible theorem / model-check milestone and close with the usual document consistency audit.
