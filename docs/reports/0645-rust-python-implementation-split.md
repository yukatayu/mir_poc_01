# Report 0645 — rust python implementation split

- Date: 2026-04-12T11:45:46.896785Z
- Author / agent: Codex (GPT-5)
- Scope: Summarize the current Rust vs Python implementation split from the required local docs and relevant `crates/` / `scripts/` entry points only.
- Decision levels touched: Read-only summary. No normative change. Main claims rely on current L2 / PoC L2 readings plus L2 implementation-direction docs.

## 1. Objective

Summarize:

- what currently lives in Rust
- what currently lives in Python
- why that split exists
- whether the repo points toward Rust-heavy convergence or a durable mixed-tool split
- any stale or contradictory wording found while checking docs against code

## 2. Scope and assumptions

- Read in the repo-required order before narrowing to task-specific files: `README.md`, `Documentation.md`, `progress.md`, `specs/00..03`, `specs/09`, `plan/00-index.md`, then `plan/07`, `plan/09`, `plan/10`.
- Used `specs/11-roadmap-and-workstreams.md` and `specs/12-decision-register.md` as the strongest normative/roadmap anchors for long-term implementation direction.
- Read only relevant Rust and Python entry points after the docs pass.
- Did not run cargo tests or Python validation helpers; this task was a doc/code reading pass, not an implementation or regression task.

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/00-index.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `Cargo.toml`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-runtime/src/lib.rs`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-semantics/examples/current_l2_emit_detached_bundle.rs`
- `crates/mir-semantics/examples/current_l2_emit_detached_aggregate.rs`
- `crates/mir-semantics/examples/current_l2_emit_static_gate.rs`
- `crates/mir-semantics/examples/current_l2_emit_formal_hook.rs`
- `crates/mir-semantics/examples/current_l2_emit_proof_notebook_review_unit.rs`
- representative placeholder crate entry points:
  - `crates/mirrorea-core/src/lib.rs`
  - `crates/prism-core/src/lib.rs`
  - `crates/engine-abi/src/lib.rs`
- `scripts/current_l2_detached_loop.py`
- `scripts/current_l2_source_sample_regression.py`
- `scripts/current_l2_same_lineage_checker.py`
- `scripts/current_l2_try_rollback_structural_checker.py`
- `scripts/current_l2_diff_detached_artifacts.py`
- `scripts/current_l2_scaffold_fixture.py`
- `scripts/new_report.py`
- `scripts/validate_docs.py`

## 4. Actions taken

- Read the required baseline docs and status snapshots.
- Cross-checked roadmap/decision wording against current code entry points.
- Mapped current responsibilities into Rust core logic vs Python repo-local helpers.
- Identified wording that now reads broader or narrower than the actual codebase.
- Created this report.

## 5. Files changed

- Added `docs/reports/0645-rust-python-implementation-split.md`
- `plan/` 更新不要
- `progress.md` 更新不要
- `tasks.md` 更新不要

## 6. Commands run and exact outputs

Representative commands:

```text
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
Output:
Task baseline recorded.

python3 scripts/new_report.py --slug rust-python-implementation-split
Output:
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0645-rust-python-implementation-split.md
```

Read/inspection commands also included:

- `sed -n ...` over the required docs and selected entry points
- `nl -ba ...` to capture line-anchored evidence
- `find crates ...` and `find scripts ...` to enumerate relevant entry points
- `rg -n ...` over the consulted docs for Rust/Python wording and helper boundaries

## 7. Evidence / findings

### Current Rust responsibilities

1. current L2 parser carrier and parser-boundary evidence live in Rust.
   - Paths: `crates/mir-ast/src/lib.rs`, `crates/mir-ast/src/current_l2.rs`
   - Strongest source: `plan/09-helper-stack-and-responsibility-map.md:218-221`
   - Code confirmation: `crates/mir-ast/src/lib.rs:6-17`, `crates/mir-ast/src/current_l2.rs:1-4`, `45-261`, `263+`
   - Reading: Rust currently owns the narrow parser-side carrier for stage 1/2 and some stage 3 helper-local surfaces. This is explicitly non-production and intentionally narrow.

2. parser-free semantics, fixture execution, host-plan harness, bundle/batch/profile selection, and current L2 public helper stack live in Rust.
   - Paths: `crates/mir-semantics/src/lib.rs`, `crates/mir-semantics/src/harness.rs`
   - Strongest source: `plan/07-parser-free-poc-stack.md:42-57`, `59-90`; `plan/09-helper-stack-and-responsibility-map.md:222-229`
   - Code confirmation: `crates/mir-semantics/src/lib.rs:4-8`, `21-33`, `35+`; `crates/mir-semantics/src/harness.rs:14-31`, `87-99`, `101-220`
   - Reading: Rust owns the actual parser-free interpreter kernel and the fixture/batch execution stack that machine-checks current L2 semantics.

3. syntax-backed sample lowering and the checker/runtime bridge also live in Rust.
   - Paths: `crates/mir-runtime/src/lib.rs`, `crates/mir-runtime/src/current_l2.rs`
   - Strongest source: `plan/09-helper-stack-and-responsibility-map.md:230-238`
   - Code confirmation: `crates/mir-runtime/src/lib.rs:6-9`; `crates/mir-runtime/src/current_l2.rs:1-6`, `26-94`, `96-236`
   - Reading: Rust owns the thin orchestration layer that lowers fixed-subset source text to semantic `Program`, checks parser-bridge consistency, and runs the runtime skeleton.

4. detached artifact emission and theorem-side first-cut emitters are in Rust, but kept outside stable public APIs.
   - Paths:
     - `crates/mir-semantics/examples/current_l2_emit_detached_bundle.rs`
     - `crates/mir-semantics/examples/current_l2_emit_detached_aggregate.rs`
     - `crates/mir-semantics/examples/current_l2_emit_static_gate.rs`
     - `crates/mir-semantics/examples/current_l2_emit_formal_hook.rs`
     - `crates/mir-semantics/examples/current_l2_emit_proof_notebook_review_unit.rs`
   - Strongest source: `plan/09-helper-stack-and-responsibility-map.md:176-217`, `243-245`
   - Code confirmation: each example is a thin CLI that reads inputs, reuses Rust library behavior, and writes JSON artifacts.
   - Reading: the compile-ready path is already Rust-led even for export/verification emitters.

5. Most non-Mir crates are still Rust boundary skeletons, not active implementations.
   - Paths: `crates/mirrorea-core/src/lib.rs`, `crates/prism-core/src/lib.rs`, `crates/engine-abi/src/lib.rs` and peers listed in `Cargo.toml`
   - Strongest source: the crate entry-point docs themselves
   - Code confirmation:
     - `crates/mirrorea-core/src/lib.rs:4-8`
     - `crates/prism-core/src/lib.rs:4-8`
     - `crates/engine-abi/src/lib.rs:4-8`
   - Reading: Rust is already the chosen container for future subsystem boundaries, but only the Mir current L2 line has meaningful implementation depth today.

### Current Python responsibilities

1. Python is the repo-local orchestration layer around Rust emitters and compare helpers.
   - Path: `scripts/current_l2_detached_loop.py`
   - Strongest source: `plan/09-helper-stack-and-responsibility-map.md:176-245`
   - Code confirmation: `scripts/current_l2_detached_loop.py:22-66`, `158-216`
   - Reading: Python invokes `cargo run` for Rust emitters, manages artifact paths/run labels, and wires diff/checker helpers into an operational detached loop.

2. Python holds non-production compare/checker utilities for detached artifacts and fixture expectations.
   - Paths:
     - `scripts/current_l2_same_lineage_checker.py`
     - `scripts/current_l2_try_rollback_structural_checker.py`
     - `scripts/current_l2_diff_detached_artifacts.py`
   - Strongest source: `Documentation.md` item 110 plus the script entry points themselves
   - Code confirmation:
     - `scripts/current_l2_same_lineage_checker.py:8-17`
     - `scripts/current_l2_try_rollback_structural_checker.py:21-31`, `114-179`
     - `scripts/current_l2_diff_detached_artifacts.py:67-105`
   - Reading: Python is used for narrow comparison/reporting helpers, not for semantics execution.

3. Python owns source-sample regression orchestration and sample inventory policy glue.
   - Path: `scripts/current_l2_source_sample_regression.py`
   - Strongest source: `plan/09-helper-stack-and-responsibility-map.md:239-242`
   - Code confirmation: `scripts/current_l2_source_sample_regression.py:45-94`, `192-240`
   - Reading: Python tracks the fixed-subset sample inventory and assembles mixed cargo/Python regression command bundles. The docs explicitly call it a repo-local helper and not a public CLI.

4. Python also owns repository workflow helpers.
   - Paths:
     - `scripts/current_l2_scaffold_fixture.py`
     - `scripts/new_report.py`
     - `scripts/validate_docs.py`
   - Strongest source: `README.md:51`, `61`; script entry points
   - Code confirmation:
     - `scripts/current_l2_scaffold_fixture.py:64-103`
     - `scripts/new_report.py:36-52`
     - `scripts/validate_docs.py:28-43`
   - Reading: Python is used for authoring scaffolds, report generation, and documentation scaffolding checks.

### Expected long-term split

1. Strongest explicit direction: core implementation should continue converging toward Rust.
   - Strongest sources:
     - `README.md:64-73`
     - `specs/11-roadmap-and-workstreams.md:81-86`
     - `specs/12-decision-register.md:7-12`
   - Reading: the repo explicitly recommends Rust for core runtime, graph processing, and tooling backend. This is a strong direction, though not an architectural law.

2. Strongest roadmap reading: the near-term promoted line keeps widening Rust parser/checker/runtime/formal-hook code, not Python semantics.
   - Strongest sources:
     - `plan/10-roadmap-overall.md:44-58`, `92-101`, `144-148`
     - `progress.md:23-30`, `36-38`, `46-50`
   - Reading: the active implementation frontier is Phase 6 parser/checker/runtime/formal-hook and source-sample actualization in Rust.

3. Inference: the docs do not imply full elimination of Python helpers.
   - Strongest sources:
     - `README.md:77-78`
     - `plan/09-helper-stack-and-responsibility-map.md:176-245`
     - `plan/10-roadmap-overall.md:87-101`
   - Reading: Python remains the repo-local operational glue for validation loops, diffing, scaffolding, and reporting. The docs repeatedly describe these as non-production helpers rather than temporary semantics implementations scheduled for Rust migration.

4. Best current synthesis:
   - Decided/explicit: Rust-heavy convergence for semantics/runtime/parser/kernel paths.
   - Inference from docs and code: durable mixed-tool split for repo-local helper/orchestration tasks is more likely than a total Rust-only repository, unless a future task explicitly promotes those helpers into public/stable tooling surfaces.

### Potential doc drift

1. `README.md:62` says `crates/` is an "intentional minimal Rust workspace skeleton".
   - This now reads stale or over-broad.
   - Why: `mir-ast`, `mir-semantics`, and `mir-runtime` contain non-trivial Phase 6 parser/checker/runtime/formal-hook/sample-runner code. The wording still fits many crates, but not the workspace as a whole.

2. `README.md:61` says `scripts/` contains "report creation and validation support scripts".
   - This is now too narrow.
   - Why: the Python scripts also orchestrate detached artifact emission, compare helpers, source-sample regression, and fixture scaffolding.

3. No hard contradiction found on long-term direction.
   - The docs are broadly consistent that Rust is preferred for core implementation while Python remains in local validation/reporting workflows.
   - The issue is mostly stale shorthand in `README.md`, not a split-brain architectural statement.

## 8. What changed in understanding

- Before reading code, the repo-level wording could be read as "Rust skeleton plus Python helpers."
- After reading the current entry points, the stronger reading is:
  - Rust already contains the real current L2 implementation path.
  - Python mainly wraps, compares, scaffolds, or validates around that Rust path.
- The repo therefore is not "Python-first with a future Rust rewrite"; it is already "Rust core with Python operational glue."

## 9. Open questions

- OPEN QUESTION: should `README.md` be tightened so `crates/` and `scripts/` describe the current Phase 6 reality more precisely?
- OPEN QUESTION: if the project later promotes validation helpers into a public/stable tool surface, should any Python helpers be re-homed into Rust CLIs, or is repo-local mixed tooling the intended steady state?

## 10. Suggested next prompt

Audit `README.md` wording around `crates/` and `scripts/` against the current Phase 6 codebase, then propose a minimal doc-only patch that keeps the repo introduction concise but no longer understates the existing Rust implementation path or the current Python helper scope.
