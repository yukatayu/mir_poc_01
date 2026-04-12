# Report 0646 — feature maturity matrix for progress faq003

- Date: 2026-04-12 20:43:34 JST
- Author / agent: Codex
- Scope: Summarize the project's characteristic feature families and current maturity for later rewriting of `progress.md` and FAQ 003, using only the user-scoped documents plus the report template.
- Decision levels touched: No normative changes. Read-only synthesis across L0/L1/L2 judgments and snapshot status documents.

## 1. Objective

Produce a concise feature matrix that distinguishes:

- what is already decided,
- what is only docs/spec level,
- what is partially actualized or runnable today,
- what remains blocked or open,

and separately note snapshot-doc wording that is stale, compresses distinct maturity levels, or risks overclaiming.

## 2. Inputs consulted

- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `plan/04-core-semantics-current-l2.md`
- `plan/05-fallback-lease-and-chain-semantics.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/10-roadmap-overall.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`

## 3. Actions taken

1. Read the required baseline documents, including `specs/00` and `specs/01`, then the scoped snapshot documents to establish the current promoted line and current wording.
2. Read the scoped normative and plan documents to separate settled semantics from docs-only boundaries and retained-later work.
3. Grouped the material into feature families relevant to `progress.md` and FAQ 003.
4. Identified wording in snapshot docs that now understates actual runnable subsets or over-compresses separable subsystems.
5. Wrote this report. `plan/` 更新不要. `progress.md` 更新不要. `tasks.md` 更新不要.

## 4. Files changed

- Added `docs/reports/0646-feature-maturity-matrix-for-progress-faq003.md`

## 5. Commands run and exact outputs

```text
$ python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
Task baseline recorded.

$ date '+%Y-%m-%d %H:%M:%S %Z'
2026-04-12 20:43:34 JST

$ ls docs/reports/[0-9][0-9][0-9][0-9]-*.md | sort | tail -n 3
docs/reports/0643-phase6-post-task-document-consistency-audit.md
docs/reports/0644-phase-recut-faq003-and-snapshot-rewrite.md
docs/reports/0645-rust-python-implementation-split.md
```

Additional read commands:

- Multiple `nl -ba ... | sed -n ...` and `rg -n '^(#|##|###) '` commands were run against the scoped documents above; their outputs were the cited document contents used in sections 2 and 6.

## 6. Evidence / findings

### 6.1 Feature matrix

| Feature family | Current maturity | Strongest source | Major remaining gaps |
|---|---|---|---|
| Separable four-system architecture (`Mir` / `Mirrorea` / `PrismCascade` / Typed-Effect Wiring Platform) | decided | `specs/02-system-overview.md`, `specs/03-layer-model.md` | Snapshot docs still sometimes compress distinct subsystems into one maturity line, which obscures separability and different readiness levels. |
| Place-local causal execution model (`event DAG`, `place`, local `try`, place-local `atomic_cut`) | runnable for current fixed subset; decided semantically | `specs/04-mir-core.md`, reinforced by `plan/08-representative-programs-and-fixtures.md` | Exact restore-scope shape remains runtime/proof-boundary work; `durable_cut` / distributed finalization are intentionally out of Mir-0; final surface syntax is unresolved. |
| Guarded option chain / `lease` / monotone degradation / static evidence floor | partially actualized | `plan/05-fallback-lease-and-chain-semantics.md` with status support from `plan/04-core-semantics-current-l2.md` and `plan/08-representative-programs-and-fixtures.md` | Full parser syntax and full algebra remain open; `Approximate` / `Compensate` are not yet carried by the current PoC; syntax-backed authored-row widening is still missing `e3`. |
| Parser-free detached validation loop and helper stack | runnable; closeout fixed | `progress.md`, `plan/10-roadmap-overall.md`, `plan/17-research-phases-and-autonomy-gates.md` | Still compare-only / non-production; retention/path policy, detached serialization, exporter/public API boundaries remain later. |
| Parser / checker / runtime actualization | partially actualized | `plan/17-research-phases-and-autonomy-gates.md`, `plan/10-roadmap-overall.md`, `Documentation.md` | Actualization is intentionally narrow and non-production; final grammar, public parser/checker/runtime host surfaces, richer host interface, and later Phase 6 widenings remain open. |
| Source-backed sample corpus and executable verification ladder | runnable for a fixed subset | `plan/08-representative-programs-and-fixtures.md`, with current line mirrored in `progress.md` | Runnable scope is intentionally narrow: authored quintet only (`e1`, `e2`, `e21`, `e4`, `e23`); `e3` remains source-target-only / deferred; `.txt` paths and current runner shape do not imply final grammar or public API. |
| Dynamic attach/detach, route rebinding, overlay insertion, DAG-safe evolution, no-shadowing discipline | spec-only | `specs/05-mirrorea-fabric.md`, `specs/09-invariants-and-constraints.md` | Route proof representation, suspended-task behavior across route change / patch activation, concrete operational realization, and integration boundaries are still unresolved. |
| Shared-space / distributed membership / authority / consistency / fairness boundary | spec-only for finalization; docs-first boundary fixed; user-spec-gated beyond that | `plan/16-shared-space-membership-and-example-boundary.md`, `progress.md`, `plan/17-research-phases-and-autonomy-gates.md` | Final activation rule, authority / auth / identity / admission / consistency / fairness catalog, reconnect / late leave / in-flight action policy, and concrete upper-layer choice all require user specification. |
| Verification stack beyond the executable subset: contracts, theorem/model-check handoff, type/static-analysis line | partially actualized, but only at the boundary/pilot level | `plan/13-heavy-future-workstreams.md`, `progress.md`, `Documentation.md` | Contracts and static/runtime/formal-hook boundary work are real, but strong type system, concrete theorem/model-check tool binding, public checker API, and final external verifier contract remain later heavy workstreams. |
| Async-control / memory-order reconstruction above `atomic_cut` | open | `plan/12-open-problems-and-risks.md`, `plan/13-heavy-future-workstreams.md` | Only place-local cut semantics are source-backed today; higher-level authority/ordering families are docs-first only, and low-level memory-order vocabulary is intentionally deferred. |

### 6.2 High-signal rewrite notes for `progress.md` / FAQ 003

1. The project is no longer well described as "implementation前段階" without qualification.
   - Accurate replacement: architecture-and-semantics-first, with a non-production but runnable current-L2 subset.
2. "Verification" needs decomposition in prose.
   - Current reality is not "types/theorem/model-check all advanced equally".
   - What is strong now is: contracts, static gate, parser-free regression, tool-neutral formal hook, proof-notebook review-unit pilot, fixed-subset sample runner.
   - What is still later is: strong type system, concrete prover/model-check tool binding, public checker surface.
3. "Distributed" also needs decomposition.
   - `place` and local rollback/cut semantics are much more mature than shared-space membership / activation / fairness / consensus-like realization.
4. Sample executability should be stated as a fixed-subset milestone, not as final-language readiness.
   - The current source corpus is intentionally narrow and does not imply final grammar.

### 6.3 Snapshot doc drift / misleading wording

1. `README.md`
   - `crates/` is described as an intentionally minimal Rust workspace skeleton.
   - That is now stale or at least materially understated: the repo contains non-production parser/checker/runtime/formal-hook/source-runner actualization, not just an empty skeleton.
2. `Documentation.md`
   - The phrases "実装前段階 / アーキテクチャ重視段階" and "いくつかの実装 skeleton" are directionally true but now misleading unless they also state that a runnable fixed-subset current-L2 path exists.
3. `progress.md`
   - The high rough percentages are accurate only for the current-L2 / fixed-subset / non-production line; they are easy to overread as whole-project maturity if that scope is not repeated.
4. `progress.md`
   - The single row `Mirrorea / Typed-Effect / Prism / 上位アプリ` hides an important architectural distinction.
   - Mirrorea has a stronger docs-first boundary than PrismCascade or upper-layer application realization, so a single blended row weakens separability.
5. `progress.md`
   - The shared-space row can be read as "mostly immature" unless it explicitly distinguishes "docs-first boundary fixed" from "operational realization and final catalog still open / user-spec-required".

## 7. Changes in understanding

- The clearest maturity split is not "spec vs implementation" but:
  - decided semantics,
  - docs-first fixed boundaries,
  - non-production actualization,
  - fixed-subset runnable path,
  - user-spec-gated future layers.
- The strongest current differentiator is the combination of:
  - strict semantic boundary discipline,
  - executable parser-free / fixed-subset verification ratchet,
  - and explicit refusal to collapse higher-layer distributed concerns into Mir core.
- FAQ 003 and `progress.md` should stop describing the project as if all verification-related or distributed-related features were maturing at the same rate.

## 8. Open questions

- How should FAQ 003 balance "current runnable subset exists" against "final parser / public surface is still later" without overstating readiness?
- Should `progress.md` split the current blended upper-layer row into at least:
  - Mirrorea / shared-space docs boundary,
  - Typed-Effect boundary,
  - PrismCascade,
  - upper-layer apps?
- When rewriting snapshot docs, should the current line be explained primarily by phase numbering or by feature-family readiness?

## 9. Suggested next prompt

Rewrite `progress.md` and FAQ 003 using report `0646` as the source summary. Keep the feature-family split explicit, scope all high percentages to the current-L2 fixed-subset non-production line, and avoid collapsing Mirrorea / Typed-Effect / Prism / upper-layer maturity into one row.
