# 2088 — P-MIR-04 effectful integration

## Objective

Close `P-MIR-04 effectful integration` by widening the Full System V1 source-first runtime from the pure subset into a bounded transition/effect lane, adding positive and negative evidence rows, synchronizing status/docs, and recording package-close validation.

## Scope and assumptions

- Scope is limited to `P-MIR-04` from `sub-agent-pro/full-system-completion-001/19-codex-package-sequence.md`.
- The semantic owner remains the source-first Mir lane in `crates/mir-semantics`; `package.mir.json` remains Product Alpha-1 compatibility/package artifact rather than the authority for this package.
- This package implements a bounded local effect runtime only:
  - transition execution from textual `.mir`
  - host read/write boundary evidence
  - bounded `publish` / `observe` / witness / handoff / local cut interaction
  - observer-safe effect-session summaries
- This package does not claim final effect grammar, final public effect ABI, Rust-level language completion, distributed cut/save execution, provider execution, LLVM/native codegen, final server/client split, or final public SDK.
- When semantics were still open, the implementation took the narrow side: unsupported or unprovable behavior remains explicit rejection, and cut/save semantics stay local evidence rather than distributed claims.

## Start state / dirty state

- Branch: `main`
- Start point: after `P-MIR-03` closeout (`6aacc36a`) had been committed and pushed
- Initial local state for this package was not clean:
  - the tree already contained in-scope P-MIR-04 runtime, sample, and doc edits
  - those edits were treated as package work and were not reverted
- During closeout, one major anchor (`product_alpha1_release_check`) first failed because `cargo fmt --check` was still red. The package was kept open, formatted, and revalidated before close.

## Documents consulted

- Core repo docs:
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
- Full System V1 specs:
  - `specs/33-full-system-v1-scope.md`
  - `specs/34-textual-mir-alpha-grammar.md`
  - `specs/35-mir-typed-ir-and-interpreter.md`
  - `specs/36-projection-ir-and-boundary-preservation.md`
  - `specs/37-posegraph-runtime-semantics.md`
  - `specs/38-engine-provider-admission.md`
- Full System V1 plans:
  - `plan/58-full-system-v1-roadmap.md`
  - `plan/59-textual-mir-roadmap.md`
  - `plan/60-computational-runtime-roadmap.md`
  - `plan/61-posegraph-runtime-roadmap.md`
  - `plan/62-projection-backend-roadmap.md`
  - `plan/63-engine-provider-roadmap.md`
- Handoff package:
  - `sub-agent-pro/full-system-completion-001/*.md`
- Additional policy/status doc required by `AGENTS.md` for roadmap/status work:
  - `.docs/progress-task-axes.md`

## Actions taken

1. Widened the runtime entry surface so the source-first lane can execute either a function or a transition directly from textual `.mir`.
2. Fixed checker lowering for `Bind` contract clauses so post-effect `ensure` can refer to the newly bound name rather than evaluating in the pre-bind environment.
3. Expanded the interpreter from the pure subset into a bounded effect session with explicit runtime state for:
   - host input/output
   - published and observed channels
   - witness issuance and lookup
   - handoff references
   - accepted cuts and local cut flags
4. Implemented bounded effect execution for the current floor:
   - `read_int`
   - `write_int`
   - `publish_*`
   - `observe_*`
   - `issue_*_witness`
   - `handoff_*`
   - `seal_places`
   - `quiesce_messages`
   - `atomic_cut`
   - `rollback_cut`
   - `load_cut`
5. Added explicit runtime rejection reasons for unsupported or invalid effectful situations, including:
   - missing publication
   - missing live witness
   - `R2` precondition failure
   - rollback across cut rejection
   - stale-state non-resurrection
   - unsupported runtime effect dispatch
6. Extended the public runtime report to expose `entry_kind` and an observer-safe `effect_session` summary instead of leaking raw internal state.
7. Added effectful checker/runtime tests in `mir-semantics` and `mir-runtime` for:
   - successful host boundary and bounded Sugoroku-like transition execution
   - missing publication rejection
   - missing witness rejection
   - cut precondition rejection
   - rollback rejection
   - stale load rejection
8. Added new Full System V1 computational sample roots and updated `runtime-matrix.json` so the executable matrix now covers both pure and bounded effectful runtime rows.
9. Regenerated `expected/run.json` projections for runtime rows to reflect the widened report shape (`entry_kind` plus `effect_session`).
10. Fixed an import regression discovered during sample regeneration: a newly added effectful support module originally reused `Shared.AddOne`, which made `mir-03-imports-positive` fail by ambiguous import resolution. The helper module was renamed to `EffectfulSupport.AddOne`.
11. Updated `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, relevant plan files, and Full System V1 sample/script READMEs so `P-MIR-04` is closed and `P-POSE-03 runtime PoseGraph` is promoted.
12. Incorporated package-close reviewer findings by:
   - aligning bind `require` / `ensure` evaluation with post-bind scope for the current alpha floor
   - separating function arguments and non-consuming transitions from synthetic host-input queue state
   - keeping `write_int` outside the transport `NoInFlight` bit so host output remains a typed boundary rather than message backlog
13. Added regression tests for the reviewer-found gaps and regenerated runtime expected JSON after the semantic fix.

## Files changed

- Rust source/tests:
  - `crates/mir-semantics/src/full_system_v1/checker.rs`
  - `crates/mir-semantics/src/full_system_v1/interpreter.rs`
  - `crates/mir-semantics/src/full_system_v1/mod.rs`
  - `crates/mir-semantics/tests/typed_ir_interpreter.rs`
  - `crates/mir-runtime/tests/full_system_v1_session.rs`
- Scripts/tests:
  - `scripts/full_system_v1_samples.py`
  - `scripts/tests/test_full_system_v1_samples.py`
- Full System V1 computational samples/matrix:
  - `samples/full-system-v1/computational/README.md`
  - `samples/full-system-v1/computational/runtime-matrix.json`
  - `samples/full-system-v1/computational/host-boundary-positive/README.md`
  - `samples/full-system-v1/computational/host-boundary-positive/expected/run.json`
  - updated `expected/run.json` for existing runtime rows
  - new sample roots:
    - `samples/full-system-v1/computational/effectful-sugoroku-positive/`
    - `samples/full-system-v1/computational/observe-before-publish-negative/`
    - `samples/full-system-v1/computational/handoff-missing-witness-negative/`
    - `samples/full-system-v1/computational/atomic-cut-r2-precondition-negative/`
    - `samples/full-system-v1/computational/atomic-cut-rollback-negative/`
    - `samples/full-system-v1/computational/atomic-cut-stale-load-negative/`
- Snapshot/docs/repository memory:
  - `README.md`
  - `Documentation.md`
  - `samples/README.md`
  - `scripts/README.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `plan/58-full-system-v1-roadmap.md`
  - `plan/60-computational-runtime-roadmap.md`
  - `docs/hands_on/full_system_v1_roadmap_01.md`
  - `docs/research_abstract/full_system_v1_roadmap_01.md`

## Commands run

```bash
git status --short
git diff --name-only
git diff --stat
cargo fmt
cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture
cargo test -p mir-runtime --test full_system_v1_session -- --nocapture
python3 -m unittest scripts.tests.test_full_system_v1_samples
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/textual_mir_samples.py check-all --format json
python3 scripts/full_system_v1_samples.py check-all --format json
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
python3 scripts/minimal_alpha1_patterns.py check-all --format json
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-CJCQmQ
python3 scripts/product_alpha1_release_check.py --format json check-all --out "$(mktemp -d /tmp/mirrorea-alpha1-release-XXXXXX)"
python3 scripts/operational_product_samples.py check-all --format json
date '+%Y-%m-%d %H:%M %Z'
```

## Evidence / outputs / test results

- Package tests after implementation:
  - `cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture`: passed, 11 tests
  - `cargo test -p mir-runtime --test full_system_v1_session -- --nocapture`: passed, 4 tests
  - `python3 -m unittest scripts.tests.test_full_system_v1_samples`: passed, 10 tests
  - `python3 -m unittest scripts.tests.test_validate_docs`: passed, 17 tests
- Source-first helpers:
  - `python3 scripts/textual_mir_samples.py check-all --format json`: passed all 10 parse rows
  - `python3 scripts/full_system_v1_samples.py check-all --format json`: passed all 29 rows
    - checker lane: 12 rows
    - runtime lane: 17 rows
      - 8 positive
      - 9 negative
- Runtime matrix now includes new effectful rows:
  - `mir-04-host-boundary-positive`
  - `mir-04-effectful-sugoroku-positive`
  - `mir-04-observe-before-publish-negative`
  - `mir-04-handoff-missing-witness-negative`
  - `mir-04-atomic-cut-r2-precondition-negative`
  - `mir-04-atomic-cut-rollback-negative`
  - `mir-04-atomic-cut-stale-load-negative`
- Doc/source validators:
  - `python3 scripts/check_source_hierarchy.py`: passed
  - `python3 scripts/validate_docs.py`: passed
  - `cargo fmt --check`: initially failed inside a major anchor, then passed after `cargo fmt`
  - `git diff --check`: passed
- Existing major anchors:
  - `python3 scripts/minimal_alpha1_patterns.py check-all --format json`: accepted
  - `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-CJCQmQ`: failed because `validation:cargo-fmt` was red before formatting
  - `python3 scripts/product_alpha1_release_check.py --format json check-all --out "$(mktemp -d /tmp/mirrorea-alpha1-release-XXXXXX)"`: accepted after formatting
  - `python3 scripts/operational_product_samples.py check-all --format json`: accepted

## What changed in understanding

- The first useful effectful runtime floor is not a broad effect system. A bounded local lane that keeps effect names explicit and rejection reasons concrete is enough to connect textual Mir to real transition/runtime evidence without over-claiming a final language surface.
- `Bind` contract semantics need post-bind scope to be credible in this alpha floor. The reviewer-found mismatch showed that checker/runtime agreement matters more than preserving an implied pre-effect meaning that the current source surface does not yet model explicitly.
- The runtime/devtools surface must summarize effect-session state rather than expose raw internals. Observer-safe summaries are enough for sample comparison and closeout evidence.
- Function arguments and typed host-input are different carriers. Treating every runtime entry as latent host input polluted pure rows and non-consuming transitions with fake boundary state.
- Host output should not be silently folded into transport backlog. `NoInFlight` remains about message/cut progress, not every external effect.
- Import hygiene matters even in narrow evidence packages. The ambiguous `Shared.AddOne` regression showed that helper sample modules can silently perturb earlier rows if module-path ownership is not kept explicit.

## Open questions

- `P-POSE-03` still needs the runtime-owned PoseGraph lane so transform/anchor semantics are not just checker or doc inventory.
- The current effect dispatch names and summary shape are implementation evidence, not a frozen public syntax or ABI.
- Distributed or durable cut/save semantics beyond bounded local rejection evidence remain later work and must not be inferred from this package.

## Suggested next prompt

```text
P-POSE-03 runtime PoseGraph
```

## Plan update status

Updated:

- `plan/58-full-system-v1-roadmap.md`
- `plan/60-computational-runtime-roadmap.md`

## Documentation.md update status

Updated for `P-MIR-04` closeout and `P-POSE-03` promotion.

## progress.md update status

Updated to show:

- `P-MIR-04` closed
- current promoted package `P-POSE-03`
- next promoted package after current closeout `P-POSE-04`
- recent reviewer-fix log entry at `2026-05-22 12:19 JST`

## tasks.md update status

Updated to promote `P-POSE-03 runtime PoseGraph` as the current package and `P-POSE-04 pose save/devtools` as the next promoted closeout target.

## samples_progress.md update status

Updated to keep `samples/full-system-v1/computational/` marked as parser+checker+bounded-effectful-runtime evidence and to append the `P-MIR-04` reviewer-fix closeout log entry.

## Reviewer findings and follow-up

- Sub-agent reviewer `Raman` completed a read-only package-close review and reported three concrete issues:
  - checker/runtime mismatch for bind contract scope
  - fake host-input state leaking into pure rows and non-consuming transitions
  - host output incorrectly toggling the transport `NoInFlight` bit
- Follow-up completed in this package:
  - added failing regression tests first
  - fixed runtime behavior in `crates/mir-semantics/src/full_system_v1/interpreter.rs`
  - regenerated runtime expected JSON
  - reran package floor and major anchors successfully
- No further reviewer findings remained open at package close.

## Skipped validations and reasons

- None. Required package-close validations were executed.

## Commit / push status

- Pending until package-close commit and push are executed.

## Sub-agent session close status

- Reviewer `Raman` (`019e4da6-35b8-7b81-ba8d-80fb03d29fd3`) completed and was closed after findings were integrated.
