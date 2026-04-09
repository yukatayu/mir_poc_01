# Report 0429 — review phase5 exchange-body threshold package

- Date: 2026-04-10 02:05 JST
- Author / agent: Codex GPT-5
- Scope: uncommitted Phase 5 exchange-body threshold package review across the requested scope files
- Decision levels touched: L2 review only; no normative change made

## 1. Objective

Review the uncommitted Phase 5 exchange-body threshold package and check:

- whether the current first choice is limited to adding only `exchange_rule_body_ref`
- whether runtime coupling / transport / failure body remain deferred
- whether mirror files consistently advance the next later reopen from exchange-body threshold to runtime-coupling threshold
- whether report `0428` has report-hygiene or evidence issues
- whether there is semantic inconsistency, provenance gap, or drift in the current Phase 5 theorem-line chain

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `specs/examples/154-current-l2-theorem-line-exchange-ready-adapter-validation-threshold.md`
- `specs/examples/155-current-l2-theorem-line-validation-ready-invocation-surface-threshold.md`
- `specs/examples/156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md`
- `docs/reports/0428-phase5-exchange-body-threshold-package.md`

## 3. Actions taken

1. Read the repository-required baseline documents in order, then the Phase 5 theorem-line documents and requested mirror files.
2. Inspected the uncommitted diff for the scope files.
3. Searched the scope for `exchange-body threshold`, `runtime-coupling threshold`, `exchange_rule_body_ref`, `consumer_invocation_surface_ref`, transport, and failure-body wording.
4. Ran local documentation validation and diff hygiene checks.
5. Compared report `0428` against repository reporting rules and current command outputs.

## 4. Files changed

- `docs/reports/0429-review-phase5-exchange-body-threshold-package.md`

plan/ 更新不要
progress.md 更新不要
tasks.md 更新不要

## 5. Commands run and exact outputs

```text
$ git status --short
 M Documentation.md
 M docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md
 M plan/11-roadmap-near-term.md
 M plan/12-open-problems-and-risks.md
 M plan/13-heavy-future-workstreams.md
 M plan/17-research-phases-and-autonomy-gates.md
 M plan/90-source-traceability.md
 M progress.md
 M specs/00-document-map.md
 M tasks.md
?? docs/reports/0428-phase5-exchange-body-threshold-package.md
?? specs/examples/156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md
```

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 429 numbered report(s).
```

```text
$ git diff --check
[no output]
```

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-10 02:05 JST
```

## 6. Evidence / findings

1. Medium: `progress.md` still contains a stale active Phase 5 snapshot that stops at `specs/examples/155...` and still says the next step is exchange-body threshold, while the same file now also says the next step is runtime-coupling threshold elsewhere. Evidence:
   - `progress.md:184` still ends with `specs/examples/155...` and says `next は exchange-body threshold をどこまで足すか`
   - `progress.md:200` says the next reopen is runtime-coupling threshold
   - `progress.md:24` and `progress.md:41` already reflect `specs/examples/156...`

2. Medium: the deferred set is not mirrored consistently. The normative spec for `156` keeps failure body deferred together with runtime coupling and transport, but several mirror summaries now mention only runtime coupling plus transport and drop failure body. That weakens provenance for the deferred boundary even if it does not fully promote failure semantics. Evidence:
   - `specs/examples/156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md:15`-`specs/examples/156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md:19`
   - `specs/examples/156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md:44`
   - `specs/examples/156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md:114`
   - `specs/examples/156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md:154`
   - mirrors that omit failure body: `specs/00-document-map.md:278`, `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md:157`, `plan/13-heavy-future-workstreams.md:189`, `tasks.md:177`, `plan/12-open-problems-and-risks.md:290`

3. Medium: report `0428` has hygiene and evidence drift. It updates `docs/research_abstract/...` in Actions Taken but does not list that document under Documents consulted, omits the template’s explicit `Files changed` and `Commands run` sections required by `AGENTS.md`, and records a stale validation count. Evidence:
   - missing consulted abstract despite Action 3 updating it: `docs/reports/0428-phase5-exchange-body-threshold-package.md:17`-`docs/reports/0428-phase5-exchange-body-threshold-package.md:45`
   - missing template sections versus repo rule: `AGENTS.md:24`-`AGENTS.md:28`, `docs/reports/TEMPLATE.md:11`-`docs/reports/TEMPLATE.md:16`, `docs/reports/0428-phase5-exchange-body-threshold-package.md:36`-`docs/reports/0428-phase5-exchange-body-threshold-package.md:58`
   - stale docs count: `docs/reports/0428-phase5-exchange-body-threshold-package.md:56` says `Found 427 numbered report(s).`, but `python3 scripts/validate_docs.py` now reports `Found 428 numbered report(s).`

4. Low: `progress.md`’s work log stops at the `155` package and does not append a `156` entry, even though the phase snapshot above it has been updated to include `156`. That leaves the task-close log incomplete under the repository’s progress policy. Evidence:
   - policy requires a new timestamped entry per non-trivial task: `AGENTS.md:104`-`AGENTS.md:107`
   - current log ends at the `155` entry: `progress.md:299`-`progress.md:300`

5. No finding on the primary semantic choice itself: the normative package is correctly limited to adding only `exchange_rule_body_ref`, and it still defers stronger runtime material. Evidence:
   - current question framing: `specs/examples/156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md:36`-`specs/examples/156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md:38`
   - chosen option: `specs/examples/156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md:121`-`specs/examples/156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md:156`
   - minimal bridge shape adds only `exchange_rule_body_ref`: `specs/examples/156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md:158`-`specs/examples/156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md:183`

## 7. Changes in understanding

- The core package judgment is semantically aligned: `156` itself preserves a narrow docs-first addition of `exchange_rule_body_ref` only.
- The main defects are mirror drift and report hygiene, not a wrong normative cut in `156`.
- `plan/17` and `plan/90` remain aligned with the intended theorem-line chain and source traceability.

## 8. Open questions

- Should every mirror summary for `156` explicitly list failure body together with runtime coupling and transport, or is a shorter summary acceptable if one canonical mirror keeps the full deferred set?
- Should historical work-log lines in `progress.md` remain as-is while only the latest line is appended, or should the Phase 5 row at `progress.md:184` be treated as the primary snapshot that must always match the latest theorem-line close?

## 9. Suggested next prompt

Fix the mirror/report drift from the Phase 5 exchange-body threshold package: update `progress.md` so the active Phase 5 snapshot and work log include `specs/examples/156...` and the next reopen is runtime-coupling threshold, then normalize the mirror summaries so failure body remains explicitly deferred wherever `156` is summarized, and clean up report `0428` to match the template and current validation evidence.
