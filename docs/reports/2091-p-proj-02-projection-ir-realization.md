# 2091 — P-PROJ-02 projection IR realization

## Objective

Close `P-PROJ-02 projection IR realization` by actualizing a bounded source-first projection IR lane, proving one accepted row plus one negative row, tightening boundary-preservation invariants, updating roadmap/status docs, and recording package-close validation.

## Scope and assumptions

- Scope is limited to `P-PROJ-02` from `sub-agent-pro/full-system-completion-001/19-codex-package-sequence.md`.
- Semantic source of truth remains `.mir` source plus typed IR; `package.mir.json` remains alpha compatibility/package artifact only.
- This package is allowed to actualize:
  - textual-source to typed-IR projection lowering
  - projection IR and target manifest generation
  - preservation reports
  - bounded CLI/runtime/helper surfaces
  - one positive row and one negative row
- This package does not claim packet/FFI payload schema completion, executable server/client split, provider admission, LLVM/native codegen, final public ABI/SDK, WAN/federation, distributed durable save/load R3/R4, Unity/Unreal/WASM/native provider execution, or arbitrary native package execution.
- During closeout, reviewer findings forced narrower preservation behavior:
  - source-owned capability/failure rows stay on the owner target manifest
  - client/adapter mutation rejection uses authority plus required-capability evidence, not effect-name heuristics alone
  - every source place must be assigned to some target
  - save/load authority must stay on server targets

## Start state / dirty state

- Branch: `main`
- Start point: after `P-POSE-04` closeout (`91022d53`) had been committed and pushed.
- Initial local state for this package was not clean:
  - the tree already contained in-scope `P-PROJ-02` source/test/sample/doc work
  - those edits were treated as package work and were not reverted
- During closeout, docs/status closeout was paused after a reviewer found projection semantic gaps that made the package not yet closable.

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

1. Added `crates/mir-semantics::full_system_v1::projection` and exported it from `crates/mir-semantics/src/full_system_v1/mod.rs`.
2. Implemented source-first projection request loading, typed-IR/source analysis, source boundary collection, projection IR generation, target manifest generation, and preservation report emission.
3. Added `crates/mir-runtime::full_system_v1_projection` plus the `mir_full_system_v1_projection` example so the same bounded projection report is executable from Cargo.
4. Added `mirrorea-alpha project-full-v1` to `crates/mirrorea-cli/src/main.rs`.
5. Actualized `samples/full-system-v1/projection/` with:
  - one positive effectful Sugoroku-like row
  - one negative client-write row
  - `matrix.json`
  - `expected/run.json`
  - generated target-manifest / rejection artifacts
6. Added `scripts/projection_v1_samples.py` and `scripts/tests/test_projection_v1_samples.py` for the bounded projection sample family.
7. Added Rust regression coverage in:
  - `crates/mir-runtime/tests/projection_ir.rs`
  - `crates/mirrorea-cli/tests/full_system_v1_cli.rs`
8. After reviewer findings, tightened projector semantics:
  - kept source-owned capability/failure rows on the owner target manifest instead of unioning them onto every endpoint
  - rejected renamed client mutation rows when required capabilities still imply authoritative mutation
  - rejected requests that leave source places unassigned
  - rejected save/load authority on non-server targets
9. Updated generated positive artifacts and helper assertions to match the corrected manifest-preservation behavior.
10. Updated `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, sample/script READMEs, hands-on/summary docs, and relevant plan files so `P-PROJ-02` is closed and `P-PROJ-03 boundary schemas` is promoted.

## Files changed

- Rust source/runtime/CLI:
  - `crates/mir-semantics/src/full_system_v1/projection.rs`
  - `crates/mir-semantics/src/full_system_v1/mod.rs`
  - `crates/mir-runtime/src/full_system_v1_projection.rs`
  - `crates/mir-runtime/src/lib.rs`
  - `crates/mirrorea-cli/src/main.rs`
- Rust tests/examples:
  - `crates/mir-runtime/tests/projection_ir.rs`
  - `crates/mirrorea-cli/tests/full_system_v1_cli.rs`
  - `crates/mir-runtime/examples/mir_full_system_v1_projection.rs`
- Scripts/tests:
  - `scripts/projection_v1_samples.py`
  - `scripts/tests/test_projection_v1_samples.py`
- Samples:
  - `samples/full-system-v1/projection/README.md`
  - `samples/full-system-v1/projection/matrix.json`
  - `samples/full-system-v1/projection/effectful-sugoroku-positive/*`
  - `samples/full-system-v1/projection/client-write-authority-negative/*`
  - `samples/full-system-v1/README.md`
- Snapshot/docs/repository memory:
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `samples/README.md`
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
python3 scripts/projection_v1_samples.py check-all --format json
python3 scripts/projection_boundary_samples.py check-all --format json
cargo fmt
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
python3 scripts/minimal_alpha1_patterns.py check-all --format json
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-p-proj-02-fix
python3 scripts/operational_product_samples.py check-all --format json
git add <package files>
git commit --no-gpg-sign -m "P-PROJ-02: projection IR realization"
git push
```

## Evidence / outputs / test results

- Package-specific projection evidence:
  - `cargo test -p mir-runtime --test projection_ir -- --nocapture`: passed, 5 tests
  - `cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture`: passed, 2 tests
  - `python3 -m unittest scripts.tests.test_projection_v1_samples`: passed, 7 tests
  - `python3 scripts/projection_v1_samples.py check-all --format json`: passed, both executable rows matched expected projections/artifacts
  - `python3 scripts/projection_boundary_samples.py check-all --format json`: accepted and preserved the older product-alpha inventory scaffold
- Docs/source validators:
  - `python3 -m unittest scripts.tests.test_validate_docs`: passed, 17 tests
  - `python3 scripts/check_source_hierarchy.py`: passed
  - `python3 scripts/validate_docs.py`: passed
  - `cargo fmt --check`: passed after formatting
  - `git diff --check`: passed
- Existing major anchors:
  - `python3 scripts/minimal_alpha1_patterns.py check-all --format json`: accepted
  - `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-p-proj-02-fix`: accepted
  - `python3 scripts/operational_product_samples.py check-all --format json`: accepted
- Resource snapshot used for the long-running package closeout:
  - `df -h .`: `/dev/vda2` 99G total, 56G used, 39G available, 60%
  - `free -h`: 960Mi total RAM, 618Mi used, 77Mi free, 19Gi swap, 2.5Gi used

## What changed in understanding

- Projection target manifests cannot blindly union source capability/failure rows across every endpoint. That leaks source-side authority into client/adapter manifests and breaks the preservation boundary that later role-run/provider work will rely on.
- In the current bounded floor, authority-preserving negative behavior has to read the source-required capability row, not just effect-name heuristics, or a renamed mutating effect bypasses the rejection.
- Place ownership and save/load ownership are not optional metadata. They are minimum preservation invariants that must reject at projection time before later packages trust the manifest.
- The current projection lane is still bounded local evidence, but it is now strong enough to serve as a real floor for `P-PROJ-03` instead of only a superficial manifest preview.

## Open questions

- No blocker remains for `P-PROJ-02`.
- Packet/FFI payload schema semantics remain intentionally deferred to `P-PROJ-03`.
- Executable local server/client role-run and provider admission remain later packages; current manifests and projection reports are still alpha-local artifacts, not final public ABI.

## Suggested next prompt

```text
P-PROJ-03 boundary schemas
```

## Plan update status

Updated:

- `plan/58-full-system-v1-roadmap.md`
- `plan/62-projection-backend-roadmap.md`

## Documentation.md update status

Updated for `P-PROJ-02` closeout, corrected projection-floor wording, and retained `P-PROJ-03` as the promoted package.

## progress.md update status

Updated to show:

- `P-PROJ-02` closed
- current promoted package `P-PROJ-03`
- next promoted package after current closeout `P-PROJ-04`
- bounded projection IR now includes source-owned manifest rows plus unassigned-place/save-load ownership rejection

## tasks.md update status

Updated to keep `P-PROJ-03 boundary schemas` as the current promoted package and to preserve `P-PROJ-02` closeout semantics in the package summary.

## samples_progress.md update status

Updated to keep `samples/full-system-v1/projection/` evidence-closed and to record the stronger projection-floor invariants in the status row and recent validation log.

## Reviewer findings and follow-up

- Projection/backend reviewer `Descartes` completed a focused review and reported four issues:
  - target manifests leaked source capability/failure rows onto client/adapter endpoints
  - authority rejection could be bypassed by renaming a mutating effect
  - source places could be left unassigned while still being accepted
  - save/load authority was accepted on non-server targets
- Follow-up completed in the same package:
  - owner-scoped source capability/failure rows
  - capability-backed client/adapter mutation rejection
  - unassigned-place rejection
  - save/load-authority rejection on non-server targets
  - added direct Rust coverage for all four cases
- Docs/status reviewer `Hypatia` completed a focused review and reported four stale-doc issues:
  - hands-on verification block did not include current Full System V1 anchors
  - `plan/58` and `samples/README.md` still described the wider root as waiting for already-implemented parser/checker/runtime/projection floors
  - research summary still called the current PoseGraph line helper-only evidence
  - `README.md` still grouped already-actualized parser/checker/interpreter/projection floors with still-later provider work
- Follow-up completed in the same package:
  - updated hands-on verification commands
  - corrected roadmap/root status wording
  - corrected PoseGraph summary wording
  - separated actualized bounded floors from still-later provider/schema/server-client work
- No additional sub-agent re-review was requested after the fixes. Final closeout used targeted regression tests, helper checks, major anchors, and local diff inspection.
- Separate language/type and engine/provider reviewers were not spawned for this package because the active diffs were concentrated in projection lowering, manifest preservation, sample artifacts, and status docs; those perspectives were covered by local self-review against `specs/35`, `specs/36`, and `specs/38`.

## Skipped validations and reasons

- None. Required package-close validations were executed.

## Commit / push status

- Pending at report authoring time. This package is intended to be committed with:
  - `git commit --no-gpg-sign -m "P-PROJ-02: projection IR realization"`
  - `git push`

## Sub-agent session close status

- `Descartes` (`019e4e09-5070-7053-b0e2-536ceaeacca0`) completed and its findings were resolved locally.
- `Hypatia` (`019e4e09-673a-7543-b2f3-1211305c1684`) completed and its findings were resolved locally.
- No additional sub-agent sessions remain open for this package close.
