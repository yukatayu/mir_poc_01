# 2093 — P-PROJ-04 server/client local split

## Objective

Close `P-PROJ-04 server/client local split` by actualizing a bounded same-binary local role-run lane over accepted Full System V1 projection manifests, proving one accepted row plus one undeclared-entry rejection row, synchronizing sample/helper/runtime/CLI/doc surfaces, and promoting the roadmap snapshot to `P-ENG-02 provider admission`.

## Scope and assumptions

- Scope is limited to `P-PROJ-04` from `sub-agent-pro/full-system-completion-001/19-codex-package-sequence.md`.
- Semantic source of truth remains `.mir` source plus typed IR; `package.mir.json` remains alpha compatibility/package artifact only.
- This package is allowed to actualize:
  - same-binary local role-run over accepted target manifests
  - target-scoped undeclared-entry rejection before runtime execution
  - a separate `samples/full-system-v1/server-client/` evidence root
  - runtime/example/CLI/helper/generated-report evidence for the role-run lane
  - snapshot/roadmap promotion to `P-ENG-02`
- This package does not claim final packet/FFI transport semantics, final server/client binary split, Docker/deployment planner completion, provider admission, LLVM/native codegen, final public ABI/SDK, WAN/federation, distributed durable save/load R3/R4, Unity/Unreal/WASM/native provider execution, or arbitrary native package execution.
- Safe-side narrowing used in this package:
  - client execution is proven with a client-owned transition that does not depend on still-deferred transport semantics
  - role selection remains manifest/entry constrained; no runtime widening of target authority is admitted
  - projection IR and role-run evidence stay separated as `FS-06` and `FS-07` sample roots

## Start state / dirty state

- Branch: `main`
- Start point: after `P-PROJ-03` closeout (`0afcc1bf`) had been committed and pushed.
- Initial local state for this package was not clean:
  - the tree already contained in-scope `P-PROJ-04` draft edits in `crates/mir-runtime/src/lib.rs`
  - untracked in-scope draft files already existed for `crates/mir-runtime/src/full_system_v1_local_split.rs` and `crates/mir-runtime/examples/mir_full_system_v1_local_split.rs`
- Those edits were treated as package work and were not reverted.

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

1. Finished and exported `crates/mir-runtime::full_system_v1_local_split` as a bounded same-binary role-run wrapper over accepted projection manifests.
2. Kept `mirrorea-alpha project-full-v1` pure projection/report generation and added a separate `mirrorea-alpha run-full-v1-split` CLI surface for `FS-07`.
3. Added a runtime/example lane:
  - `crates/mir-runtime/examples/mir_full_system_v1_local_split.rs`
  - target filter via `--target`
  - target-scoped entry override via `--entry`
  - bounded input injection via `--input`
4. Added role-run diagnostics:
  - unknown target rejection
  - `entry_transition_not_admitted`
  - target-local surfacing of runtime rejection codes when a launched target session fails
5. Created a separate `samples/full-system-v1/server-client/` root with one shared source/request and two executable rows:
  - `proj-04-local-role-split-positive`
  - `proj-04-client-entry-override-negative`
6. Widened `scripts/projection_v1_samples.py` from `FS-06`-only checking into a combined projection/backend helper that now validates:
  - 4 projection rows under `samples/full-system-v1/projection/`
  - 2 local role-run rows under `samples/full-system-v1/server-client/`
7. Added/updated regression coverage:
  - `crates/mir-runtime/tests/projection_ir.rs`
  - `crates/mirrorea-cli/tests/full_system_v1_cli.rs`
  - `scripts/tests/test_projection_v1_samples.py`
8. Updated `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, sample/script READMEs, hands-on/summary docs, and relevant plan files so `P-PROJ-04` is closed and `P-ENG-02 provider admission` is promoted.

## Files changed

- Rust source/runtime exports:
  - `crates/mir-runtime/src/lib.rs`
  - `crates/mir-runtime/src/full_system_v1_local_split.rs`
  - `crates/mir-runtime/examples/mir_full_system_v1_local_split.rs`
  - `crates/mirrorea-cli/src/main.rs`
- Rust tests:
  - `crates/mir-runtime/tests/projection_ir.rs`
  - `crates/mirrorea-cli/tests/full_system_v1_cli.rs`
- Scripts/tests:
  - `scripts/projection_v1_samples.py`
  - `scripts/tests/test_projection_v1_samples.py`
- Samples:
  - `samples/full-system-v1/server-client/README.md`
  - `samples/full-system-v1/server-client/matrix.json`
  - `samples/full-system-v1/server-client/role-split-positive/*`
  - `samples/full-system-v1/projection/README.md`
- Snapshot/docs/repository memory:
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `samples/README.md`
  - `samples/full-system-v1/README.md`
  - `scripts/README.md`
  - `plan/58-full-system-v1-roadmap.md`
  - `plan/62-projection-backend-roadmap.md`
  - `docs/hands_on/full_system_v1_roadmap_01.md`
  - `docs/research_abstract/full_system_v1_roadmap_01.md`

## Commands run

```bash
git status --short
date '+%Y-%m-%d %H:%M JST'
df -h .
free -h
cargo test -p mir-runtime --test projection_ir -- --nocapture
cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture
python3 -m unittest scripts.tests.test_projection_v1_samples
python3 scripts/projection_v1_samples.py matrix --format json
python3 scripts/projection_v1_samples.py check-all --format json
python3 scripts/projection_boundary_samples.py check-all --format json
cargo fmt
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
python3 scripts/minimal_alpha1_patterns.py check-all --format json
out_dir=$(mktemp -d /tmp/mirrorea-alpha1-release-XXXXXX) && python3 scripts/product_alpha1_release_check.py --format json check-all --out "$out_dir"
python3 scripts/operational_product_samples.py check-all --format json
git add <package files>
git commit --no-gpg-sign -m "P-PROJ-04: server/client local split"
git push
```

## Evidence / outputs / test results

- Package-specific role-run evidence:
  - `cargo test -p mir-runtime --test projection_ir -- --nocapture`: passed, 9 tests
  - `cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture`: passed, 4 tests
  - `python3 -m unittest scripts.tests.test_projection_v1_samples`: passed, 11 tests
  - `python3 scripts/projection_v1_samples.py check-all --format json`: passed, 6 executable rows matched expected summaries/artifacts
  - `python3 scripts/projection_boundary_samples.py check-all --format json`: accepted and preserved the older product-alpha inventory scaffold
- Docs/source validators:
  - `python3 -m unittest scripts.tests.test_validate_docs`: passed, 17 tests
  - `python3 scripts/check_source_hierarchy.py`: passed
  - `python3 scripts/validate_docs.py`: passed
  - `cargo fmt --check`: failed once before formatting, then passed after `cargo fmt`
  - `git diff --check`: passed
- Existing major anchors:
  - `python3 scripts/minimal_alpha1_patterns.py check-all --format json`: accepted
  - `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-mrJ623`: accepted
  - `python3 scripts/operational_product_samples.py check-all --format json`: accepted after rerun once formatting drift was cleared
- Resource snapshot used for the long-running package closeout:
  - `df -h .`: `/dev/vda2` 99G total, 58G used, 37G available, 62%
  - `free -h`: 960Mi total RAM, 705Mi used, 82Mi free, 19Gi swap total / 17Gi available

## What changed in understanding

- `FS-07` does not need final transport semantics to prove a bounded role-run floor, but it does need a separate root and surface so `FS-06` projection evidence is not silently reinterpreted as runtime execution.
- The safe-side role-run floor is target-manifest constrained first, transport-coupled second. Admitted target entries can run now; cross-target dataflow remains a later transport/runtime semantics obligation.
- The shared helper can cover both `FS-06` and `FS-07` without collapsing their meaning, as long as row families and generated artifacts stay distinct.

## Open questions

- No blocker remains for `P-PROJ-04`.
- `P-ENG-02` still needs the bounded provider admission shape:
  - accepted provider manifest row
  - capability/authority overreach rejection
  - rollback/replay/cut policy rejection
  - disabled-native default preserved
- Docker/deployment planner realization for `FS-07` remains later than this package.

## Suggested next prompt

```text
P-ENG-02 provider admission
```

## Plan update status

Updated:

- `plan/58-full-system-v1-roadmap.md`
- `plan/62-projection-backend-roadmap.md`

## Documentation.md update status

Updated for `P-PROJ-04` closeout, corrected role-split-floor wording, and promoted `P-ENG-02 provider admission`.

## progress.md update status

Updated to show:

- `P-PROJ-04` closed
- current promoted package `P-ENG-02`
- next promoted package after current closeout `P-ENG-03`
- `FS-07` now actualized as bounded same-binary local role-run evidence

## tasks.md update status

Updated to keep `P-ENG-02 provider admission` as the current promoted package and to preserve `P-PROJ-04` closeout semantics in the package summary.

## samples_progress.md update status

Updated to keep `samples/full-system-v1/server-client/` evidence-closed, to widen the Full System V1 roadmap row through `FS-07`, and to record the package closeout in the recent validation log.

## Reviewer findings and follow-up

- Explorer `Goodall` (`019e4e3e-39a0-75a2-ac77-1ad3afae5e37`) completed and gave useful direction:
  - keep `project-full-v1` pure projection
  - use a separate `samples/full-system-v1/server-client/` root
  - add a client-owned transition to prove actual client execution
- Those points were integrated in the final package shape.
- Reviewer `Confucius` (`019e4e58-7ed1-7d73-b98a-c139e8fb159e`) was spawned for projection/backend plus docs/release review but did not return within two waits; the session was then closed.
- Follow-up used local self-review plus the full package validation floor and major anchors instead of reviewer findings.

## Skipped validations and reasons

- None. Required package-close validations were executed.

## Commit / push status

- Pending at report authoring time. This package is intended to be committed with:
  - `git commit --no-gpg-sign -m "P-PROJ-04: server/client local split"`
  - `git push`

## Sub-agent session close status

- `Goodall` (`019e4e3e-39a0-75a2-ac77-1ad3afae5e37`) completed, its recommendations were incorporated, and the session was closed.
- `Confucius` (`019e4e58-7ed1-7d73-b98a-c139e8fb159e`) was closed after two no-result waits; no reviewer output was available to integrate.
- No additional sub-agent sessions remain open for this package close.
