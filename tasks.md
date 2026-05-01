# tasks

最終更新: 2026-05-01 13:13 JST

## この文書について

- この文書は repo 全体の **current task map** です。
- 規範判断の正本は `specs/`、長期比較と source trace は `plan/`、runnable sample 状態は `samples_progress.md`、実行証跡は `docs/reports/` に置きます。
- append-only 履歴ではありません。current checkpoint、次に詰める gate、blocker を読める snapshot として保ちます。

## current task-level status

- active executable floor は維持されています:
  `samples/clean-near-end/`、Sugoroku world、avatar follow、typed external preview、network canary、projection/codegen bridge、viewer prototype inventory。`samples/current-l2/` は base source corpus、`samples/lean/` は Lean evidence / generated theorem stub corpus として分けて扱います。
- `P0..P18`、`P19`、`P20`、`P21`、`R1..R7`、post-`P21` later-family docs-first trilogy は close 済みです。
  これらは repo-local alpha-ready current layer / docs-first boundary closeout であり、final public parser/API/ABI、rollback、durable migration、distributed ordering、production transport、final viewer/verifier completion ではありません。
- current promoted implementation line は存在しません。追加の self-driven post-`P21` docs-first family も残っていません。
- next reopen point は `U1` actual commitment です。
  Packaging / installed binary target、host integration target、first shipped public surface scope、final shared-space operational catalog breadth は user-facing decision を要します。
- self-driven に残るのは maintenance lane です:
  stale docs cleanup、validation rerun、report creation、formatting cleanup、guardrail maintenance、regression repair、dashboard freshness、source hierarchy / docs scaffold guardrail maintenance。
- `scripts/current_l2_guided_samples.py` の active compatibility front door は `list / smoke-all / closeout` です。legacy bundle / lane / reserve / hold-line / emit-* helper command claims は historical memory として `plan/` / `docs/reports/` / relevant specs examples に委譲し、active command claim へ戻しません。
- 2026-04-30 以降の detailed maintenance chronology はこの snapshot では再列挙しません。package-by-package evidence は `docs/reports/1001` 以降の committed reports、long-lived comparison / boundary memory は relevant `plan/` files、runnable sample status は `samples_progress.md` を参照します。
- 2026-05-01 の `1051` 以降の guardrail / snapshot / validation / storage follow-up packages は maintenance-only closeout です:
  dashboard freshness、validator/source-hierarchy/report-template guardrail、full/docs/storage validation checkpoint、warning/formatting cleanup、Makefile alias parity、task/progress/sample dashboard compression、active front-door / active-doc wording repair を継続的に閉じています。new implementation queue は reopened していません。
- 直近の repository-wide validation freshness checkpoint は 2026-05-01 12:46 JST です:
  source hierarchy / docs scaffold、current-L2 inventory、source regression 23/23、clean near-end / Sugoroku / avatar / typed external / network / projection / viewer helper floors、Lean sync、Cargo crate tests、`cargo fmt --check`、`git diff --check` が pass しました。補助の storage guardrail は known `/mnt/mirrorea-work/llvm` root-owned warning 以外の新規 blocker なしです。
- report template compliance guardrail は 2026-05-01 11:13 JST に template-side `Commands run` 欄まで拡張済みです:
  `scripts/tests/test_validate_docs.py` が `docs/reports/TEMPLATE.md` と `scripts/validate_docs.py` の required heading 同期、および missing-heading 時の validator failure を確認します。actual numbered report 全体の semantic lint ではありません。
- latest report heading guardrail は 2026-05-01 11:21 JST に追加済みです:
  `validate_docs.py` は historical reports 全体を遡及 lint せず、最新 numbered report 1 本だけを template required headings で scaffold check します。unit test は latest missing failure と historical-only missing pass の両方を確認します。
- 2026-05-01 11:27 JST の post-guardrail docs-focused freshness checkpoint では、clean tree で latest-report heading unit / source hierarchy / docs scaffold / whitespace diff が pass しています。
- network transport executable validation anchor drift は 2026-05-01 11:31 JST に active hands-on / research abstract 側も修正済みです:
  executable canary は `python3 scripts/network_transport_samples.py check-all --format json`、`closeout` は inventory evidence として扱います。
- guided helper retirement audit は 2026-05-01 11:42 JST に再確認済みです:
  `current_l2_guided_samples.py` は `list / smoke-all / closeout` の compatibility wrapper であり、pre-clean-near-end prototype / bundle / reserve wording は `plan/00` / `plan/10` で historical comparison memory へ冷却済みです。
- network transport active sample docs は 2026-05-01 11:53 JST に再確認済みです:
  sample README / hands-on canary / `plan/22` は `check-all` executable canary anchor と `closeout` inventory evidence を分離し、`NET-01` を Sugoroku loopback parity anchor として扱います。
- projection/codegen bridge evidence audit は 2026-05-01 12:14 JST に再確認済みです:
  `projection_codegen_samples.py check-all --format json` は live anchor / manifest alignment validation、`closeout --format json` は manifest inventory evidence として扱います。current `equivalence` reading は review-category alignment inventory であり、generated place-program / optimizer / deployment planner / checker / proof / final emitted ABI ではありません。
- current-L2 / Lean active-floor wording audit は 2026-05-01 12:23 JST に再確認済みです:
  `samples/current-l2/` は base source corpus、`samples/lean/` は foundations + generated theorem stub evidence、`current_l2_guided_samples.py list/smoke-all/closeout` は clean-near-end active suite への compatibility front door として扱います。source-sample regression は stale deleted emitted-artifact Cargo target を呼ばず、formal-hook smoke + theorem Lean-stub conformance + model-check carrier conformance を current route とします。
- stale validation-command/reference audit は 2026-05-01 12:59 JST に再確認済みです:
  `.docs/current-l2-source-sample-authoring-policy.md`、`plan/27`、public-gate hands-on、projection / Mirrorea research abstracts は current command anchors に同期済みです。network executable evidence は `check-all`、projection live alignment は `check-all`、projection manifest inventory は `closeout` として扱います。
- storage/env entrypoint guardrail は 2026-05-01 13:13 JST に再確認済みです:
  `/mnt/mirrorea-work` は mounted、`target` は external cargo target への symlink、cleanup list は `llvm/src` を除外し、削除には `--confirm` を要求します。known `/mnt/mirrorea-work/llvm` root-owned warning 以外の新規 blocker はありません。

## executable floor

| lane | current floor | current command | not a claim of |
|---|---|---|---|
| Mir current-L2 base corpus | `samples/current-l2/` | `python3 scripts/current_l2_source_sample_regression.py inventory`; `python3 scripts/current_l2_source_sample_regression.py regression --run-label <label> --artifact-root <root>`; `python3 scripts/current_l2_guided_samples.py closeout --format json` は clean-near-end active suite への compatibility front door | final parser grammar / public API / all proof discharge / production prover binding |
| clean near-end suite | `samples/clean-near-end/` | `python3 scripts/clean_near_end_samples.py closeout` | full language completion |
| Sugoroku world | `scripts/sugoroku_world_samples.py` | `python3 scripts/sugoroku_world_samples.py closeout --format json` | real network / durable distributed runtime |
| avatar follow | `scripts/avatar_follow_samples.py` | `python3 scripts/avatar_follow_samples.py closeout --format json` | `FAIRY-05` implementation / public avatar API |
| typed external | `scripts/typed_external_boundary_samples.py` | `python3 scripts/typed_external_boundary_samples.py closeout --format json` | final host schema / final adapter API |
| network transport | `scripts/network_transport_samples.py` | `python3 scripts/network_transport_samples.py check-all --format json` | production socket / durable replay |
| projection / placement | `scripts/projection_codegen_samples.py` + committed generated manifest | `python3 scripts/projection_codegen_samples.py check-all --format json` | final emitted executable family / generated place-program emission / placement optimizer / deployment planner / equivalence checker / proof completion / final public emitted-program ABI |
| viewer prototype | `scripts/visual_debugger_viewer_samples.py` | `python3 scripts/visual_debugger_viewer_samples.py closeout --format json` | final viewer API / telemetry service |
| hot-plug runtime | `crates/mirrorea-core`, `crates/mir-runtime` | `cargo test -p mir-runtime --test hotplug_runtime_skeleton` | rollback / durable migration / distributed ordering / final ABI |
| storage / backend | `/mnt/mirrorea-work`, `scripts/env/`, `scripts/storage/` | `bash scripts/env/mirrorea_storage_env.sh --ensure-dirs`; `bash scripts/storage/detach_prepare.sh`; `bash scripts/storage/cleanup_disposable_artifacts.sh --list`; `CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run` | actual LLVM build / backend choice |

## package map

| Package | Macro phase | Status | Current reading |
|---|---|---|---|
| `P0` / `P1` | `Macro 0` | closed | source hierarchy, repo layer map, dashboard stabilization |
| `P2` / `P12` | `Macro 6-7` | closed | typed external residual review and helper-local host-boundary inventory |
| `P3` / `P15` | `Macro 6-7` | closed | projection preview boundary and committed generated bridge evidence |
| `P4` / `P5` | `Macro 6` | closed | `TermSignature` and `LayerSignature` current lanes / scope split |
| `P6` / `P7` | `Macro 6-7` | closed | `MessageEnvelope` / `AuthEvidence` seam and typed visualization / telemetry security |
| `P8` / `P9` | `Macro 6` | closed | Sugoroku attach hardening and avatar follow representative slice hardening |
| `P10` / `P11` | `Macro 6-7` | closed | `mirrorea-core` carrier substrate and logical multi-place shell floor |
| `P13` / `P14` | `Macro 6-7` | closed | helper-local network canaries and hot-plug package-manager preview inventory |
| `P16` / `P17` | `Macro 7` | closed | typed viewer prototype inventory and storage / LLVM guardrail |
| `P18` | `Macro 7` mixed gate | closed repo-side | public-boundary inventory / mixed-gate split; final freeze still later |
| `U1` option matrix | `Macro 8` prep | closed docs-first, actual commitment open | option inventory exists; actual choice remains user-facing |
| `R1` / `R2` / `R3` | `Macro 8` prep | closed | verification widening threshold, AttachPoint minimal contract, `FAIRY-05` carrier recommendation |
| `R4` / `R5` / `R6` / `R7` | `Macro 8` prep | closed | hot-plug kept-later boundary, owner split, carrier admission, next-package inventory |
| `P19` | `Macro 6-7` | closed | engine-neutral hot-plug request/verdict carrier in `mirrorea-core` |
| `P20` | `Macro 6-7` | closed | thin runtime/report assembly in `mir-runtime` |
| `P21` | `Macro 6-7` | closed | runtime-side engine-state progression narrow floor |
| post-`P21` rollback / durable migration | `Macro 8` prep | closed docs-first | first recommendation boundary; no actual rollback / migration engine completion |
| post-`P21` distributed activation ordering | `Macro 8` prep | closed docs-first | second recommendation boundary; no actual distributed activation protocol |
| post-`P21` final public hot-plug ABI | `Macro 8` mixed gate | closed docs-first | third recommendation bridge: `freeze prerequisite fixed; public ABI still unfrozen` |

## ordered current work

| Order | Work item | Owner | Status | Completion condition |
|---:|---|---|---|---|
| 1 | Documentation freshness / stale-current audit | repo | active maintenance | stale wording removed, report created, validation run, commit/push |
| 2 | Regression / formatting cleanup | repo | active maintenance | focused tests pass, formatting clean, commit/push |
| 3 | `U1` actual commitment | user + repo | open | actual choices recorded for packaging, host target, first shipped public surface, final catalog breadth |
| 4 | Post-`U1` first implementation tranche | repo after user choice | blocked | chosen public / host / packaging surface has enough scope to implement without guessing |

## self-driven maintenance tasks

These are safe to do without new product decisions.

| Task | Objective | Validation | Report requirement | Stop line |
|---|---|---|---|---|
| docs freshness audit | keep `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, research summaries, and traceability current | `check_source_hierarchy.py`, `validate_docs.py`, `git diff --check` | new `docs/reports/NNNN-*.md` | do not create new normative decisions in snapshot docs; do not use naive banned-phrase scans over `docs/reports/`, `specs/examples/`, `sub-agent-pro/`, or `progress.md` recent log |
| guided helper retirement audit | keep `scripts/current_l2_guided_samples.py` docs mirror aligned with the live `list / smoke-all / closeout` front door | retired helper commands for negative evidence + `python3 scripts/current_l2_guided_samples.py list` + `smoke-all --format json` + `closeout --format json` + docs floor | report if touched docs change | do not re-promote `emit-*`, `reserve`, `hold-line`, `reopen-map`, `lane`, or `residuals` helper memories into active command claims without implementation evidence |
| runnable dashboard refresh | keep sample status, validation timestamps, and blockers evidence-backed | relevant helper closeout commands | report + `samples_progress.md` | do not mark conceptual rows over 25% or use 100% without current-scope commit/push |
| Rust formatting / regression repair | keep cargo formatting and focused test floors green | `cargo fmt --check`, affected `cargo test` commands | report if behavior or docs status changes | do not mix unrelated feature work into formatting cleanup |
| storage guardrail check | keep external workdir and cleanup scripts safe on small VPS | `df -h`, `free -h`, `findmnt`, storage scripts | report with resource output | no destructive cleanup / mount / format without explicit confirmation |

## user decision blockers

### Blocker 1. packaging shape / installed binary target

- overview:
  choose whether the first public shape is `CLI`, `library`, `engine-adapter`, or `hybrid`.
- affects:
  `P18`, `U1`, backend / storage / distribution, future public API shape.
- current recommendation:
  `library-first` remains the provisional recommendation. CLI or installed binary promotion should be a second gate.
- reason:
  current shell / helper actualization is not an installed binary adoption fact.

### Blocker 2. host integration target

- overview:
  choose `browser`, `native process`, `engine`, or `mixed`.
- affects:
  typed external boundary, network transport, viewer, engine ABI, packaging.
- current recommendation:
  `native process` remains the provisional recommendation because current evidence is strongest around same-process / process-boundary helpers.
- reason:
  browser / engine targets require exact host schema and adapter ABI decisions that are still user-facing.

### Blocker 3. first shipped public surface scope

- overview:
  choose `parser/checker/runtime/verifier first`, `adapter/viewer/projection/hot-plug first`, or `two-step split`.
- affects:
  final public parser/API, viewer, adapter, projection, hot-plug ABI.
- current recommendation:
  `two-step split`: first narrow core library surface, then reopen integration surfaces with host target.
- reason:
  public core and integration surfaces have different dependencies and validation floors.

### Blocker 4. final shared-space operational catalog breadth

- overview:
  choose `minimal subset`, `portal / multi-world expansion`, or `fairness / quorum / exhaustive catalog`.
- affects:
  Sugoroku, avatar, network transport, hot-plug migration / replay, application realization.
- current recommendation:
  keep `minimal subset` until packaging / host / shipped surface are fixed.
- reason:
  broader catalog choices can force durability, replay, fairness, and host integration commitments too early.

## research-discovery items

- exact public `VerificationLayer` law surface, public naming, theorem-bridge/runtime-policy widening relation, and composition contract among emitted rows / evidence carriers / downstream consumers / emitted verifier handoff artifacts beyond the current floor (`verification_handoff_witness`, `verification_model_check`).
- `FAIRY-05` reopen criteria, final implementation carrier, and negative companion shape.
- projection equivalence evidence boundary, future checker relation, and public emitted-program contract beyond committed generated bridge manifest.
- witness/provider public-shape route, schema candidate, combined-contract candidate, and final emitted-handoff boundary.
- order-handoff source-wording route, emitted-artifact schema candidate, and final source-surface wording / emitted-handoff / public witness-provider residual boundary.
- order-handoff / witness-provider compressed final-public-seam carry-over among source-wording route, `serial` reserve surface, representative trace alignment, and shared-space residual matrix.
- rollback / durable migration engine state machine after `P21`.
- distributed activation ordering evidence once multi-place / multi-server pressure is available.
- production transport / replay design that preserves transport / auth / membership / capability / witness separation.

## validation floor for this snapshot

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 scripts/current_l2_source_sample_regression.py inventory
python3 scripts/current_l2_source_sample_regression.py regression --run-label <label> --artifact-root <root>
python3 scripts/current_l2_guided_samples.py closeout --format json
python3 scripts/clean_near_end_samples.py closeout
python3 scripts/current_l2_lean_sample_sync.py
python3 scripts/sugoroku_world_samples.py closeout --format json
python3 scripts/avatar_follow_samples.py closeout --format json
python3 scripts/typed_external_boundary_samples.py closeout --format json
python3 scripts/network_transport_samples.py check-all --format json
python3 scripts/projection_codegen_samples.py check-all --format json
python3 scripts/projection_codegen_samples.py closeout --format json
python3 scripts/visual_debugger_viewer_samples.py closeout --format json
bash scripts/env/mirrorea_storage_env.sh --ensure-dirs
bash scripts/storage/detach_prepare.sh
bash scripts/storage/cleanup_disposable_artifacts.sh --list
CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run
cargo test -p mir-ast
cargo test -p mirrorea-core
cargo test -p mir-runtime
cargo test -p mir-semantics
cargo fmt --check
git diff --check
```

## reporting requirement

Every non-trivial change must add a new report under `docs/reports/`.
For the current docs freshness task, the report must include:

- dirty state and formatting cleanup commit / push status
- documents consulted
- files changed
- reviewer findings and follow-up
- validation commands and results
- skipped validations and reasons
- remaining user decision blockers
