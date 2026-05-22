# 2096 — P-FSV1-01 source operational suite

## Objective

Close `P-FSV1-01 source operational suite` by actualizing bounded source-first `world-core/`, `membership-chat/`, and `sugoroku-world/` roots under `samples/full-system-v1/`, synchronizing helper/runtime/doc surfaces, recording positive and negative operational evidence, and promoting the roadmap snapshot to `P-FSV1-02 portal/shard source samples`.

## Scope and assumptions

- Scope is limited to `P-FSV1-01` from `sub-agent-pro/full-system-completion-001/19-codex-package-sequence.md`.
- Normative source remains `.mir` source files plus `specs/33..38`; `package.mir.json` remains alpha compatibility/package artifact only.
- This package is allowed to actualize:
  - bounded source-first WorldCore bootstrap evidence
  - bounded source-first MembershipChat room-message transform evidence
  - bounded source-first Sugoroku roll/publish/witness/handoff/local-cut evidence
  - generated package-manifest expectations and runtime report expectations for those roots
  - positive and negative rows that fail for expected reasons
  - helper/runtime/test/doc/report closeout plus roadmap promotion to `P-FSV1-02`
- Safe-side narrowing used in this package:
  - the new source-first operational roots are `evidence-closed`, not `workflow-ready`
  - `typed_host_io.add_one` remains host-boundary evidence rather than Mir-owned language completion
  - provider/renderer/devtools/product alpha floors remain preserved comparison evidence, not semantic owners
  - the planned Full System V1 root name `gradient-observation/` is kept explicit as the source-first counterpart to Product Alpha `two-shard-gradient-observation/`
- This package does not claim final public grammar, final ABI/SDK, Rust-level language completion, LLVM/native codegen completion, final server/client binary split, arbitrary native/WASM execution, WAN/federation, distributed durable save/load R3/R4, Unity/Unreal/WASM provider execution, or final product workflow closure.

## Start state / dirty state

- Branch: `main`
- Start point: `15a2755c` (`P-ENG-03: renderer pose backend demo`)
- Initial local state for this package was not clean:
  - the tree already contained in-scope `P-FSV1-01` draft edits across helper/runtime/doc files
  - the new `samples/full-system-v1/world-core/`, `membership-chat/`, and `sugoroku-world/` roots already existed as draft package work
  - those in-scope edits were treated as package work and were not reverted
- Reviewer follow-up later exposed helper over-claim and coverage gaps, so the package was recut before final closeout.

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

1. Widened `scripts/full_system_v1_samples.py` with a bounded source-operational helper lane:
   - `operational-list`
   - `operational-matrix`
   - `run-operational`
   - `check-operational-all`
   - `check-all` now includes the operational rows in addition to checker/runtime rows
2. Added `samples/full-system-v1/world-core/` with 2 executable rows:
   - accepted `world-bootstrap-positive`
   - negative `world-observe-before-bootstrap-negative` rejecting as `missing_publication`
3. Added `samples/full-system-v1/membership-chat/` with 2 executable rows:
   - accepted `chat-room-message-positive`
   - negative `chat-stale-membership-negative` rejecting as `contract_require_failed`
4. Added `samples/full-system-v1/sugoroku-world/` with 2 executable rows:
   - accepted `sugoroku-turn-positive`
   - negative `sugoroku-stale-membership-negative` rejecting as `contract_require_failed`
5. Generated and committed `expected/manifest.json` plus `expected/run.json` for all 6 operational rows from the current helper/runtime projections.
6. Updated `samples/full-system-v1/computational/host-boundary-positive/expected/check.json` to include `place_ref` after the checker projection widened `transition_summaries`.
7. Added helper tests in `scripts/tests/test_full_system_v1_samples.py` and direct runtime coverage in `crates/mir-runtime/tests/full_system_v1_session.rs`.
8. Updated validator inventories in `scripts/check_source_hierarchy.py` and `scripts/validate_docs.py` for the new roots and helper/test files.
9. Recut snapshot/docs/repository-memory surfaces:
   - promoted current package to `P-FSV1-02`
   - added the new operational anchors and evidence wording
   - kept `world-core/`, `membership-chat/`, and `sugoroku-world/` at `evidence-closed`
10. Integrated reviewer findings:
   - changed `operational-matrix.workflow_ready` from `true` to `false`
   - made `run-operational` validate checker/runtime exit semantics in addition to JSON shape
   - added direct runtime tests for all remaining operational rows
   - repaired snapshot drift in `plan/58`, `docs/hands_on`, `samples_progress.md`, and naming notes for `gradient-observation/`

## Files changed

- Runtime/tests/helper logic:
  - `scripts/full_system_v1_samples.py`
  - `scripts/tests/test_full_system_v1_samples.py`
  - `crates/mir-runtime/tests/full_system_v1_session.rs`
- Validator inventory:
  - `scripts/check_source_hierarchy.py`
  - `scripts/validate_docs.py`
- Samples:
  - `samples/full-system-v1/world-core/*`
  - `samples/full-system-v1/membership-chat/*`
  - `samples/full-system-v1/sugoroku-world/*`
  - `samples/full-system-v1/computational/host-boundary-positive/expected/check.json`
- Snapshot/docs/repository memory:
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `samples/README.md`
  - `samples/full-system-v1/README.md`
  - `scripts/README.md`
  - `docs/hands_on/full_system_v1_roadmap_01.md`
  - `docs/research_abstract/full_system_v1_roadmap_01.md`
  - `plan/58-full-system-v1-roadmap.md`
  - this report

## Commands run

```bash
git status --short
git rev-parse --short HEAD
git branch --show-current
git log -1 --oneline
date '+%Y-%m-%d %H:%M %Z'
python3 -m unittest scripts.tests.test_validate_docs
python3 -m unittest scripts.tests.test_full_system_v1_samples
python3 scripts/full_system_v1_samples.py operational-matrix --format json
python3 scripts/full_system_v1_samples.py check-operational-all --format json
python3 scripts/full_system_v1_samples.py check-all --format json
cargo test -p mir-runtime --test full_system_v1_session -- --nocapture
cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
python3 scripts/minimal_alpha1_patterns.py check-all --format json
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release
OUT_DIR=$(mktemp -d /tmp/mirrorea-alpha1-release-XXXXXX) && echo "$OUT_DIR" && python3 scripts/product_alpha1_release_check.py --format json check-all --out "$OUT_DIR"
python3 scripts/operational_product_samples.py check-all --format json
cargo test -p mir-runtime --test full_system_v1_session -- --nocapture
cargo fmt --check
git diff --check
```

## Evidence / outputs / test results

- Source-first operational helper evidence:
  - `python3 scripts/full_system_v1_samples.py operational-matrix --format json`: accepted, 6 executable rows, `workflow_ready: false`
  - `python3 scripts/full_system_v1_samples.py check-operational-all --format json`: accepted, all 6 operational rows passed
  - `python3 scripts/full_system_v1_samples.py check-all --format json`: accepted, all 35 checker/runtime/operational rows passed
- Test evidence:
  - `python3 -m unittest scripts.tests.test_full_system_v1_samples`: passed, 14 tests
  - `cargo test -p mir-runtime --test full_system_v1_session -- --nocapture`: passed, 11 tests after follow-up recut
  - `cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture`: passed, 10 tests
- Docs/source validators:
  - `python3 -m unittest scripts.tests.test_validate_docs`: passed, 17 tests
  - `python3 scripts/check_source_hierarchy.py`: passed, 324 required paths present
  - `python3 scripts/validate_docs.py`: passed
  - `cargo fmt --check`: passed
  - `git diff --check`: passed
- Existing major anchors:
  - `python3 scripts/minimal_alpha1_patterns.py check-all --format json`: accepted
  - `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release`: first invocation failed as `output_dir_not_empty`; reran with `/tmp/mirrorea-alpha1-release-dlEExv` and the release check accepted
  - `python3 scripts/operational_product_samples.py check-all --format json`: accepted
- Reviewer-fix evidence:
  - the first follow-up `cargo test -p mir-runtime --test full_system_v1_session -- --nocapture` failed because `runtime_session_executes_membership_chat_positive_sample` asserted the wrong channel name/order
  - the assertion was narrowed to sorted published-channel comparison against `chat_message` + `membership_epoch`
  - the rerun then passed, 11/11 tests

## What changed in understanding

- The source-first operational helper must not claim `workflow_ready` from matrix discovery alone. The safe machine-readable classification for `P-FSV1-01` is `evidence-closed`.
- For this package, expected JSON equality is not sufficient by itself; exit semantics for checker/runtime surfaces must also be part of the close condition.
- `P-FSV1-01` needed direct runtime coverage for all 6 operational rows, not only aggregate helper counts, because the package claim is about operational behavior rather than helper shape alone.
- The planned Full System V1 root `gradient-observation/` needed an explicit note tying it to Product Alpha `two-shard-gradient-observation/` so `P-FSV1-02` starts from one documented naming decision.

## Open questions

- No blocker remains for `P-FSV1-01`.
- Remaining later work for this line is explicit:
  - `P-FSV1-02` source-first portal/shard/gradient roots
  - `P-FSV1-03` Full System V1 release check
  - final product/public decisions listed in `tasks.md`

## Suggested next prompt

```text
P-FSV1-02 portal/shard source samples
```

## Plan update status

Updated:

- `plan/58-full-system-v1-roadmap.md`

## Documentation.md update status

Updated so `P-FSV1-01` is described as an actualized bounded source-first operational-suite floor and the current promoted package is `P-FSV1-02`.

## progress.md update status

Updated to show:

- `P-FSV1-01` completed
- current package `P-FSV1-02`
- next promoted package after current closeout `P-FSV1-03`
- runnable operational helper commands and the new recent log entry

## tasks.md update status

Updated to show:

- current promoted package `P-FSV1-02 portal/shard source samples`
- next promoted package `P-FSV1-03 full V1 release check`
- the explicit `gradient-observation/` naming note for the Full System V1 source-first lane

## samples_progress.md update status

Updated to:

- mark the new source-first operational roots as `evidence-closed`
- rename the section from “Planned Sample Line” to “Full System V1 Sample Line”
- add the `P-FSV1-01` recent validation log entry

## Reviewer findings and follow-up

- Reviewer `Hegel` found:
  - `operational-matrix` over-claimed `workflow_ready`
  - `run-operational` ignored checker/runtime exit semantics
  - only 3 of the 6 operational rows had direct runtime coverage
- Follow-up:
  - set `workflow_ready: false`
  - made `run-operational` require correct checker/runtime return codes
  - added direct runtime tests for `world-observe-before-bootstrap-negative`, `chat-room-message-positive`, and `sugoroku-stale-membership-negative`
- Reviewer `Jason` found:
  - no numbered closeout report yet existed
  - `plan/58` baseline/milestone mapping was inconsistent after promotion
  - `docs/hands_on` missed the new operational anchors in the main validation block
  - `samples_progress.md` still used a “planned” section title
  - `gradient-observation/` vs `two-shard-gradient-observation/` naming drift was undocumented
- Follow-up:
  - added this report
  - repaired `plan/58` baseline, source-first anchors, and package-order mapping
  - moved `operational-matrix` / `check-operational-all` into the hands-on validation block
  - renamed the `samples_progress.md` section
  - documented the intentional naming distinction in `tasks.md`, `plan/58`, `samples/README.md`, and `scripts/README.md`

## Skipped validations and reasons

- None for this package.
- `python3 scripts/full_system_v1_release_check.py --format json check-all --out /tmp/mirrorea-full-v1-release` remains planned-only because `P-FSV1-03` has not created that script yet.

## Commit / push status

- Pending at report write time.

## Sub-agent session close status

- Reviewer `Hegel` completed and was closed.
- Reviewer `Jason` completed and was closed.
