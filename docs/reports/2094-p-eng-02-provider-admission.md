# 2094 — P-ENG-02 provider admission

## Objective

Close `P-ENG-02 provider admission` by actualizing a bounded Full System V1 provider-admission lane over accepted projection/local-role-split evidence, proving accepted and rejected provider rows, synchronizing runtime/CLI/sample/helper/doc surfaces, and promoting the roadmap snapshot to `P-ENG-03 renderer pose backend demo`.

## Scope and assumptions

- Scope is limited to `P-ENG-02` from `sub-agent-pro/full-system-completion-001/19-codex-package-sequence.md`.
- Semantic source of truth remains `.mir` source plus typed IR; `package.mir.json` remains alpha compatibility/package artifact only.
- This package is allowed to actualize:
  - bounded provider manifest admission above source-derived projection/local role split
  - accepted viewer-diagnostic inventory admission
  - accepted WASM inventory-only admission
  - explicit over-capability rejection
  - explicit missing rollback/replay/cut policy rejection
  - explicit native-disabled rejection
  - runtime/example/CLI/helper/generated-report evidence for the provider lane
  - snapshot/roadmap promotion to `P-ENG-03`
- This package does not claim final provider ABI, arbitrary native/WASM execution, renderer-owned world semantics, final packet/FFI transport semantics, final server/client binary split, LLVM/native codegen completion, WAN/federation, distributed durable save/load R3/R4, Unity/Unreal provider execution, or final public ABI/SDK.
- Safe-side narrowing used in this package:
  - admitted provider rows remain inventory-style boundary evidence rather than world-semantics owners
  - native execution remains disabled by default
  - WASM remains inventory-only evidence, not general execution admission
  - runtime receipts for provider-boundary effects stay synthetic boundary acknowledgements rather than arbitrary host execution

## Start state / dirty state

- Branch: `main`
- Start point: after `P-PROJ-04` closeout (`6aaf11d8`) had been committed and pushed.
- Initial local state for this package was not clean:
  - the tree already contained in-scope `P-ENG-02` draft edits in `crates/mir-runtime/src/lib.rs`
  - in-scope draft edits already existed in `crates/mir-semantics/src/full_system_v1/interpreter.rs`, `crates/mirrorea-cli/src/main.rs`, and `crates/mirrorea-cli/tests/full_system_v1_cli.rs`
  - untracked in-scope package files already existed for the provider runtime/example/test/script/sample roots
- Those edits were treated as package work and were not reverted.

## Documents consulted

- Repository policy/context:
  - `AGENTS.md`
  - `.docs/progress-task-axes.md`
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

## Actions taken

1. Added and exported `crates/mir-runtime::full_system_v1_provider_admission` as a bounded admission wrapper on top of `project_full_system_v1_path` and `run_full_system_v1_local_split_path`.
2. Kept provider semantics outside Mir-owned computation by treating `diagnostic_export`, `native_bridge`, and `wasm_adapter` as explicit provider-boundary receipts in `crates/mir-semantics::full_system_v1::interpreter`.
3. Added a runtime/example/CLI lane:
  - `crates/mir-runtime/examples/mir_full_system_v1_provider_admission.rs`
  - `mirrorea-alpha admit-provider-v1`
4. Created `samples/full-system-v1/provider-adapter/` with five executable rows:
  - `eng-02-viewer-diagnostic-positive`
  - `eng-02-over-capability-negative`
  - `eng-02-missing-rollback-negative`
  - `eng-02-native-disabled-negative`
  - `eng-02-wasm-inventory-positive`
5. Added `scripts/provider_admission_samples.py` plus `scripts/tests/test_provider_admission_samples.py` so the new sample root emits/generated `provider-admission-report.json` and checks exact expected summaries.
6. Added/updated regression coverage:
  - `crates/mir-runtime/tests/provider_admission.rs`
  - `crates/mirrorea-cli/tests/full_system_v1_cli.rs`
  - `scripts/tests/test_provider_admission_samples.py`
7. Updated `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, sample/script READMEs, reader summaries, and relevant plan files so `P-ENG-02` is closed and `P-ENG-03 renderer pose backend demo` is promoted.
8. Re-ran package-specific validations, docs/source validators, and the required existing major anchors.

## Files changed

- Rust source/runtime exports:
  - `crates/mir-runtime/src/lib.rs`
  - `crates/mir-runtime/src/full_system_v1_provider_admission.rs`
  - `crates/mir-runtime/examples/mir_full_system_v1_provider_admission.rs`
  - `crates/mir-semantics/src/full_system_v1/interpreter.rs`
  - `crates/mirrorea-cli/src/main.rs`
- Rust tests:
  - `crates/mir-runtime/tests/provider_admission.rs`
  - `crates/mirrorea-cli/tests/full_system_v1_cli.rs`
- Scripts/tests:
  - `scripts/provider_admission_samples.py`
  - `scripts/tests/test_provider_admission_samples.py`
- Samples:
  - `samples/full-system-v1/provider-adapter/README.md`
  - `samples/full-system-v1/provider-adapter/matrix.json`
  - `samples/full-system-v1/provider-adapter/*`
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
  - `plan/63-engine-provider-roadmap.md`
  - `docs/hands_on/full_system_v1_roadmap_01.md`
  - `docs/research_abstract/full_system_v1_roadmap_01.md`

## Commands run

```bash
git status --short
date '+%Y-%m-%d %H:%M %Z'
cargo fmt
python3 -m unittest scripts.tests.test_validate_docs
python3 -m unittest scripts.tests.test_provider_admission_samples
cargo test -p mir-runtime --test provider_admission -- --nocapture
cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture
python3 scripts/provider_admission_samples.py check-all --format json
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
python3 scripts/minimal_alpha1_patterns.py check-all --format json
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release
out_dir=$(mktemp -d /tmp/mirrorea-alpha1-release-XXXXXX) && python3 scripts/product_alpha1_release_check.py --format json check-all --out "$out_dir"
python3 scripts/operational_product_samples.py check-all --format json
df -h .
free -h
git add <package files>
git commit --no-gpg-sign -m "P-ENG-02: provider admission"
git push
```

## Evidence / outputs / test results

- Package-specific provider evidence:
  - `cargo test -p mir-runtime --test provider_admission -- --nocapture`: passed, 6 tests
  - `cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture`: passed, 7 tests
  - `python3 -m unittest scripts.tests.test_provider_admission_samples`: passed, 10 tests
  - `python3 scripts/provider_admission_samples.py check-all --format json`: passed, 5 executable rows matched expected summaries
- Docs/source validators:
  - `python3 -m unittest scripts.tests.test_validate_docs`: passed, 17 tests
  - `python3 scripts/check_source_hierarchy.py`: passed
  - `python3 scripts/validate_docs.py`: passed
  - `cargo fmt --check`: passed after `cargo fmt`
  - `git diff --check`: passed
- Existing major anchors:
  - `python3 scripts/minimal_alpha1_patterns.py check-all --format json`: accepted
  - first `product_alpha1_release_check.py` invocation failed only because `/tmp/mirrorea-alpha1-release` was not empty (`diagnostic_code = output_dir_not_empty`); rerun with `mktemp` accepted
  - `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-HvCswl`: accepted
  - `python3 scripts/operational_product_samples.py check-all --format json`: accepted
- Resource snapshot captured during closeout:
  - `df -h .`: `/dev/vda2` 99G total, 60G used, 35G available, 64%
  - `free -h`: 960Mi total RAM, 586Mi used, 61Mi free, 19Gi swap total / 16Gi available

## What changed in understanding

- `FS-08` can stay above the existing projection/local-split floor: provider admission does not need a new semantics owner, only a bounded runtime/report layer that preserves schema/capability/authority evidence.
- A safe first-floor provider lane can admit inventory-style diagnostic/export providers and explicit WASM inventory-only rows while still refusing native execution and capability overreach.
- Product Alpha `samples/product-alpha1/engine-adapter/` should remain comparison inventory even after Full System V1 actualizes bounded provider admission under a separate source-first root.

## Open questions

- No blocker remains for `P-ENG-02`.
- `P-ENG-03` still needs a narrow pose-snapshot handoff shape:
  - renderer consumes PoseGraph snapshots
  - renderer does not become semantic owner
  - provider/runtime evidence must remain layered above the existing `FS-05..08` floors

## Suggested next prompt

```text
P-ENG-03 renderer pose backend demo
```

## Plan update status

Updated:

- `plan/58-full-system-v1-roadmap.md`
- `plan/63-engine-provider-roadmap.md`

## Documentation.md update status

Updated for `P-ENG-02` closeout, promoted `P-ENG-03 renderer pose backend demo`, and narrowed non-claims around arbitrary native/WASM execution and renderer pose backend.

## progress.md update status

Updated to show:

- `P-ENG-02` closed
- current promoted package `P-ENG-03`
- next promoted package after current closeout `P-FSV1-01`
- `FS-08` now actualized as bounded provider-admission evidence

## tasks.md update status

Updated to keep `P-ENG-03 renderer pose backend demo` as the current promoted package, `P-FSV1-01` as the next package, and `P-ENG-02` as preserved closed evidence.

## samples_progress.md update status

Updated to keep `samples/full-system-v1/provider-adapter/` evidence-closed, to widen the Full System V1 roadmap row through `FS-08`, and to record the package closeout in the recent validation log.

## Reviewer findings and follow-up

- Explorer `Laplace` (`019e4e62-38af-7003-91fe-9d97b22eaa20`) completed earlier and gave useful direction:
  - keep `samples/product-alpha1/engine-adapter/` inventory-only
  - create a separate `samples/full-system-v1/provider-adapter/` root
  - use viewer-diagnostic accepted evidence plus bounded negative rows
- Those points were incorporated in the final package shape.
- Reviewer `Schrodinger` (`019e4e86-5b3d-7512-ba45-21be7a948ed0`) was spawned for read-only provider/docs/release review but did not return after two waits; the session was then closed.
- Follow-up used focused self-review plus the full package validation floor and required major anchors instead of reviewer findings.

## Skipped validations and reasons

- None. Required package-close validations were executed.

## Commit / push status

- Pending at report authoring time. This package is intended to be committed with:
  - `git commit --no-gpg-sign -m "P-ENG-02: provider admission"`
  - `git push`

## Sub-agent session close status

- `Laplace` (`019e4e62-38af-7003-91fe-9d97b22eaa20`) completed, its recommendations were incorporated, and the session was closed.
- `Schrodinger` (`019e4e86-5b3d-7512-ba45-21be7a948ed0`) was closed after two no-result waits; no reviewer output was available to integrate.
- No additional sub-agent sessions remain open for this package close.
