# tasks

最終更新: 2026-05-02 10:39 JST

## この文書について

- この文書は repo 全体の **current task map** です。
- 規範判断の正本は `specs/`、長期比較と source trace は `plan/`、runnable sample 状態は `samples_progress.md`、実行証跡は `docs/reports/` に置きます。
- append-only 履歴ではありません。current checkpoint、次に詰める gate、blocker を読める snapshot として保ちます。

## current task-level status

- active executable floor は維持されています:
  `samples/clean-near-end/`、Sugoroku world、avatar follow、typed external preview、network canary、projection/codegen bridge、viewer prototype inventory。`samples/current-l2/` は base source corpus、`samples/lean/` は Lean evidence / generated theorem stub corpus として分けて扱います。
- Mirrorea Spaces alpha-0 line は current self-driven package として reopen しています:
  `specs/13..17`、`plan/39..43`、`samples/alpha/` を軸に、Stage A floor を崩さずに Stage B local runtime へ向かう theory-freeze / checker-runtime preparation lane を進めます。
- Alpha-0 closeout validation freshness は 2026-05-02 10:39 JST に更新済みです:
  `P-A0-12` rerun では `runtime_substrate` 16 tests、`alpha_local_runtime` + `alpha_cut_save_load_runtime` 4 tests、Rust `save-load-resume` example、cut/save-load runner 1/1、integrated `E2E-06` run、integrated E2E runner 9/9 with planned-only `E2E-08`、integrated closeout inventory、Python unit 16 tests、source hierarchy、docs scaffold、`cargo fmt --check`、`git diff --check` が pass しました。current repo state では `LR-01/02` non-public Rust local-runtime floor、`LI-01..05` non-public Rust layer-insertion floor、`NET-02/03/04/05/07/09` non-public Rust Stage-C network / Docker floor、`AV-01/02/06/08/09` + `HP-11/12/15` runtime-private package/avatar manifest-admission floor に加えて、`CUT-04` room-local runtime savepoint bridge と `E2E-01/02/03/04/05/06/07/09/10` thin integrated bridge floor が actualize 済みです。
- `P0..P18`、`P19`、`P20`、`P21`、`R1..R7`、post-`P21` later-family docs-first trilogy は close 済みです。
  これらは repo-local alpha-ready current layer / docs-first boundary closeout であり、final public parser/API/ABI、rollback、durable migration、distributed ordering、production transport、final viewer/verifier completion ではありません。
- historical post-`P21` docs-first family は close したままです。
  ただし current promoted implementation line が存在しないという historical snapshot には戻らず、今は alpha-local package `P-A0-01..12` closeout 後の `P-A0-13` queue を current line として読みます。
- `U1` actual commitment は依然 separate gate です。
  Packaging / installed binary target、host integration target、first shipped public surface scope、final shared-space operational catalog breadth は user-facing decision を要します。
- self-driven に残るのは maintenance lane です:
  stale docs cleanup、validation rerun、report creation、formatting cleanup、guardrail maintenance、regression repair、dashboard freshness、source hierarchy / docs scaffold guardrail maintenance。
  これに加えて alpha-local theory-freeze / checker-runtime preparation lane が current autonomous package として並走します。
- `scripts/current_l2_guided_samples.py` の active compatibility front door は `list / smoke-all / closeout` です。legacy bundle / lane / reserve / hold-line / emit-* helper command claims は historical memory として `plan/` / `docs/reports/` / relevant specs examples に委譲し、active command claim へ戻しません。
- 2026-04-30 以降の detailed maintenance chronology はこの snapshot では再列挙しません。package-by-package evidence は package close 後の committed reports を正本にし、in-flight package は当該 report の commit / push status を authority とします。long-lived comparison / boundary memory は relevant `plan/` files、runnable sample status は `samples_progress.md` を参照します。
- 2026-05-01 の `1051` 以降の guardrail / snapshot / validation / storage follow-up packages は maintenance-only closeout です:
  dashboard freshness、validator/source-hierarchy/report-template guardrail、full/docs/storage validation checkpoint、warning/formatting cleanup、Makefile alias parity、task/progress/sample dashboard compression、active front-door / active-doc wording repair を継続的に閉じています。new implementation queue は reopened していません。
- 直近の repository-wide validation freshness checkpoint は 2026-05-01 14:27 JST です:
  report-schema unit、source hierarchy / docs scaffold、current-L2 inventory、source regression 23/23、guided / clean near-end / Sugoroku / avatar / typed external / network `check-all` / projection `check-all` + `closeout` / viewer helper floors、Lean sync、storage guardrail、Cargo crate tests、`cargo fmt --check`、`git diff --check` が pass しました。known `/mnt/mirrorea-work/llvm` root-owned warning 以外の新規 blocker はなく、generated output は external workdir のみです。
- report template compliance guardrail は 2026-05-01 14:08 JST に report closeout schema まで拡張済みです:
  `scripts/tests/test_validate_docs.py` が `docs/reports/TEMPLATE.md` と `scripts/validate_docs.py` の required heading 同期、missing-heading failure、latest-report order / empty section / 未置換 update-status placeholder failure を確認します。required headings は `Commands run`、`Documentation.md update status`、start dirty state、reviewer findings を含みます。actual numbered report 全体の semantic lint ではありません。
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
- public API / parser gate の storage anchor は 2026-05-01 13:23 JST に再確認済みです:
  `docs/hands_on/public_api_parser_gate_01.md` と `plan/27` は storage guardrail を env export だけで代表させず、`--ensure-dirs`、detach audit、cleanup list、external cargo no-run probe まで mirror します。これは repo-side public-gate inventory の再現性補強であり、actual LLVM build / backend choice / packaging adoption ではありません。
- current phase closeout guide の current-L2 / Lean anchors は 2026-05-01 13:39 JST に再確認済みです:
  `docs/hands_on/current_phase_closeout_01.md` は `current_l2_source_sample_regression.py inventory/regression`、`clean_near_end_samples.py closeout`、`current_l2_lean_sample_sync.py` を top-level closeout commands として mirror します。これは final parser / public API / all proof discharge ではありません。
- full validation floor は 2026-05-01 13:52 JST に再確認済みです:
  corrected current floor は全件 pass しました。network executable evidence は `check-all`、projection live alignment は `check-all`、projection manifest inventory は `closeout`、storage cleanup は `--list` only として維持します。これは public freeze / production transport / production prover binding / actual LLVM build ではありません。
- post-guardrail full validation floor は 2026-05-01 14:27 JST に再確認済みです:
  1093 / 1094 の report-schema guardrail と active docs freshness repair 後も corrected full floor は全件 pass しました。generated output は `/mnt/mirrorea-work/generated-artifacts/current-l2-regression-1095` に限定しています。
- `U1` readiness wording は 2026-05-01 14:33 JST に再確認済みです:
  current reader-facing docs / `plan/27` は `U1` の 4 軸を packaging / host integration / first shipped public surface / final shared-space operational catalog breadth に揃えています。これは option inventory / readiness audit であり、actual `U1` commitment ではありません。
- 1096 後の docs-focused validation は 2026-05-01 14:36 JST に再確認済みです:
  report-schema unit、source hierarchy、docs scaffold、`git diff --check` が clean tree で pass しました。full sample / Cargo floor は 1095 の post-guardrail checkpoint を参照します。

## Current Alpha-0 / Mirrorea Spaces stage

- Large stage:
  Stage B 35% local-runtime floor, Stage C 35% network / Docker floor, Stage D 35% hot-plug/runtime-package floor, Stage E 20% thin integrated devtools bridge, Stage F 50% thin integrated alpha demo bridge
- Concrete phase:
  Phase 8 — integrated alpha demo closeout
- Current package:
  `P-A0-13` dedicated alpha visualization/devtools bridge
- Current status:
  `P-A0-12` は current repo state で close 済み。`scripts/alpha_cut_save_load_samples.py` により `samples/alpha/cut-save-load/CUT-04` の local-only save/load positive bridge floor、`scripts/alpha_e2e_samples.py` により `samples/alpha/e2e/E2E-06` の integrated continue row が validation 可能になった。これは existing Stage B/C/D/F subset floor の composition + checker-backed invalid distributed-cut non-claim であり、distributed save/load completion、dedicated alpha visualization/devtools family、therefore Stage F completion 自体はまだ行っていない。
- Next autonomous package:
  `P-A0-13` dedicated alpha visualization/devtools bridge
- Public-decision gate kept separate:
  `U1` remains open and is not collapsed into this alpha-local package series

## executable floor

| lane | current floor | current command | not a claim of |
|---|---|---|---|
| Mir current-L2 base corpus | `samples/current-l2/` | `python3 scripts/current_l2_source_sample_regression.py inventory`; `python3 scripts/current_l2_source_sample_regression.py regression --run-label <label> --artifact-root <root>`; `python3 scripts/current_l2_guided_samples.py closeout --format json` は clean-near-end active suite への compatibility front door | final parser grammar / public API / all proof discharge / production prover binding |
| clean near-end suite | `samples/clean-near-end/` | `python3 scripts/clean_near_end_samples.py closeout` | full language completion |
| Sugoroku world | `scripts/sugoroku_world_samples.py` | `python3 scripts/sugoroku_world_samples.py closeout --format json` | real network / durable distributed runtime |
| avatar follow | `scripts/avatar_follow_samples.py` | `python3 scripts/avatar_follow_samples.py closeout --format json` | `FAIRY-05` implementation / public avatar API |
| typed external | `scripts/typed_external_boundary_samples.py` | `python3 scripts/typed_external_boundary_samples.py closeout --format json` | final host schema / final adapter API |
| network transport | `scripts/network_transport_samples.py` | `python3 scripts/network_transport_samples.py check-all --format json` | production socket / durable replay |
| alpha network / Docker floor | `crates/mir-runtime`, `samples/alpha/network-docker/`, `scripts/alpha_network_docker_e2e.py` | `cargo test -p mir-runtime --test alpha_network_runtime`; `cargo run -q -p mir-runtime --example mirrorea_alpha_network_runtime -- closeout`; `python3 scripts/alpha_network_docker_e2e.py check-all --format json` | production WAN / durable replay / final public transport ABI |
| alpha avatar/package floor | `crates/mir-runtime`, `samples/alpha/avatar-runtime/`, `samples/alpha/hotplug-runtime/`, `scripts/alpha_avatar_runtime_samples.py` | `cargo test -p mir-runtime --test alpha_avatar_runtime`; `cargo run -q -p mir-runtime --example mirrorea_alpha_avatar_runtime -- closeout`; `python3 scripts/alpha_avatar_runtime_samples.py check-all --format json` | final avatar API / native execution / final package ABI |
| alpha cut/save-load bridge | `crates/mirrorea-core`, `crates/mir-runtime`, `samples/alpha/cut-save-load/`, `scripts/alpha_cut_save_load_samples.py` | `cargo test -p mirrorea-core --test runtime_substrate`; `cargo test -p mir-runtime --test alpha_cut_save_load_runtime`; `cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- save-load-resume`; `python3 scripts/alpha_cut_save_load_samples.py check-all --format json` | distributed save/load / durable cut / Z-cycle completion |
| alpha integrated E2E bridge | `samples/alpha/e2e/`, `scripts/alpha_e2e_samples.py` | `python3 scripts/alpha_e2e_samples.py run E2E-06 --format json`; `python3 scripts/alpha_e2e_samples.py check-all --format json`; `python3 scripts/alpha_e2e_samples.py closeout --format json` | Stage F completion / distributed save-load completion / dedicated alpha visualization family |
| projection / placement | `scripts/projection_codegen_samples.py` + committed generated manifest | `python3 scripts/projection_codegen_samples.py check-all --format json` | final emitted executable family / generated place-program emission / placement optimizer / deployment planner / equivalence checker / proof completion / final public emitted-program ABI |
| viewer prototype | `scripts/visual_debugger_viewer_samples.py` | `python3 scripts/visual_debugger_viewer_samples.py closeout --format json` | final viewer API / telemetry service |
| hot-plug runtime | `crates/mirrorea-core`, `crates/mir-runtime` | `cargo test -p mir-runtime --test hotplug_runtime_skeleton` | rollback / durable migration / distributed ordering / final ABI |
| storage / backend | `/mnt/mirrorea-work`, `scripts/env/`, `scripts/storage/` | `bash scripts/env/mirrorea_storage_env.sh --ensure-dirs`; `bash scripts/storage/detach_prepare.sh`; `bash scripts/storage/cleanup_disposable_artifacts.sh --list`; `CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run` | actual LLVM build / backend choice |

## large stage map

| Stage | Progress | Name | Status | Main evidence | Not yet claimed |
|---|---:|---|---|---|---|
| A | 90% | repo-local alpha-ready floor | mostly reached | clean-near-end、Sugoroku、typed external preview、network canary、projection/codegen bridge、viewer prototype、hot-plug narrow floor | final public product |
| B | 35% | alpha 0.5 local runtime | first Rust local-runtime floor + local-only save/load bridge actualized | `specs/13..17`、`plan/39..43`、`samples/alpha/local-runtime/`、`samples/alpha/cut-save-load/`、`crates/mir-runtime/src/alpha_local_runtime.rs`、`scripts/alpha_cut_save_load_samples.py` | integrated local runtime completion / distributed save-load completion |
| C | 35% | alpha 0.7 transport | first Rust network / Docker floor actualized | helper-local `NET-02..05` canaries + `samples/alpha/network-docker/` + `alpha_network_runtime` + `alpha_network_docker_e2e.py` | production WAN / durable replay / partition completion / final transport ABI |
| D | 35% | alpha 0.8 hot-plug lifecycle | first attach-time layer floor actualized | `P19..P21` floor + `samples/alpha/layer-insertion/` + `samples/alpha/hotplug-runtime/` scaffold | detach / migration / final ABI |
| E | 20% | alpha 0.9 devtools | thin integrated JSON evidence surfaces actualized | viewer prototype inventory + `samples/alpha/visualization/` scaffold + `alpha_e2e_samples.py` bridge evidence | dedicated alpha visualization family runner / final viewer / telemetry API |
| F | 50% | alpha 1 Spaces alpha | thin integrated bridge + local save/load continue actualized; completion still blocked | `samples/alpha/e2e/` thin bridge runner + `scripts/alpha_cut_save_load_samples.py` + scope spec | dedicated alpha visualization family / distributed save-load completion / full VRChat / Reversed Library completion |
| G | 0% | Spaces product expansion | future | upper-layer roadmap only | alpha scope |
| H | 0% | Atlas | future | upper-layer roadmap only | alpha scope |
| I | 0% | Reversed Library | future | upper-layer roadmap only | alpha scope |

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
| 1 | `P-A0-13` dedicated alpha visualization/devtools bridge | repo | next | `samples/alpha/visualization/` subset runner + integrated visibility/redaction surfaces + report + commit/push |
| 2 | `U1` actual commitment | user + repo | open | actual choices recorded for packaging, host target, first shipped public surface, final catalog breadth |
| 3 | Post-`U1` first public-facing implementation tranche | repo after user choice | blocked | chosen public / host / packaging surface has enough scope to implement without guessing |

## self-driven maintenance tasks

These are safe to do without new product decisions.

| Task | Objective | Validation | Report requirement | Stop line |
|---|---|---|---|---|
| docs freshness audit | keep `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, research summaries, and traceability current | `check_source_hierarchy.py`, `validate_docs.py`, `git diff --check` | new `docs/reports/NNNN-*.md` | do not create new normative decisions in snapshot docs; do not use naive banned-phrase scans over `docs/reports/`, `specs/examples/`, `sub-agent-pro/`, or `progress.md` recent log |
| guided helper retirement audit | keep `scripts/current_l2_guided_samples.py` docs mirror aligned with the live `list / smoke-all / closeout` front door | retired helper commands for negative evidence + `python3 scripts/current_l2_guided_samples.py list` + `smoke-all --format json` + `closeout --format json` + docs floor | report if touched docs change | do not re-promote `emit-*`, `reserve`, `hold-line`, `reopen-map`, `lane`, or `residuals` helper memories into active command claims without implementation evidence |
| runnable dashboard refresh | keep sample status, validation timestamps, and blockers evidence-backed | relevant helper closeout commands | report + `samples_progress.md` | do not mark conceptual rows over 25% or use 100% without current-scope commit/push |
| Rust formatting / regression repair | keep cargo formatting and focused test floors green | `cargo fmt --check`, affected `cargo test` commands | report if behavior or docs status changes | do not mix unrelated feature work into formatting cleanup |
| storage guardrail check | keep external workdir and cleanup scripts safe on small VPS | `df -h`, `free -h`, `findmnt`, storage scripts | report with resource output | no destructive cleanup / mount / format without explicit confirmation |

## autonomous alpha packages

| Package | Objective | Current status | Close condition |
|---|---|---|---|
| `P-A0-01` | theory freeze specs `13..17` | closed | carried into commit/push by the Alpha-0 theory-freeze package line |
| `P-A0-02` | roadmap memory `39..43` | closed | carried into commit/push by the Alpha-0 theory-freeze package line |
| `P-A0-03` | `samples/alpha/` matrix scaffold | closed | carried into commit/push by the Alpha-0 theory-freeze package line |
| `P-A0-04` | snapshot docs / taxonomy / validator sync | closed | report `1098`, commit `56e16a3` |
| `P-A0-05` | checker skeleton first cut | closed | key LIF / VAR diagnostics + tests + report `1100` |
| `P-A0-06` | cut/save/load checker skeleton | closed | CUT diagnostics + tests + report `1101` |
| `P-A0-07` | local Mirrorea runtime integration | closed | first Rust local-runtime floor + report `1102` |
| `P-A0-08` | layer insertion runtime | closed | first Rust attach-time layer-insertion floor + report `1103` |
| `P-A0-09` | network / Docker E2E | closed | first Rust Stage-C network / Docker floor + report `1104` |
| `P-A0-10` | runtime package/avatar skeleton | closed | first runtime-private package/avatar manifest-admission floor + report `1105` |
| `P-A0-11` | Mirrorea Spaces alpha demo closeout | closed | thin integrated bridge runner + stop-line docs + validation floor + report `1111` |
| `P-A0-12` | local save/load positive bridge | closed | `CUT-04` / `E2E-06` local save/load continuation path + report `1112` |
| `P-A0-13` | dedicated alpha visualization/devtools bridge | queued | `VIS-*` subset runner + integrated visibility/redaction surfaces + report + commit/push |

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

### Blocker 5. first network scope

- overview:
  choose whether alpha networking stays local-only, reaches Docker/local-subprocess, or targets broader WAN/federation.
- affects:
  `P-A0-09`, transport trait shape, validation floor, non-claim wording.
- current recommendation:
  `Docker / local-subprocess first` は narrow Stage-C floor として actualize 済みなので、WAN/federation widening は kept-later のまま維持する。
- reason:
  it exercises transport/auth/membership separation without prematurely claiming production WAN.

### Blocker 6. avatar compatibility first target

- overview:
  choose whether first alpha avatar scope is placeholder-only, custom Mir avatar runtime, VRM skeleton, or VRChat-compat skeleton.
- affects:
  `P-A0-07`, `P-A0-08`, runtime package scope, adapter policy.
- current recommendation:
  `placeholder + custom Mir runtime first`, with VRM / VRChat kept as non-core skeletons.
- reason:
  it demonstrates the substrate without importing product-specific runtime assumptions into Mir core.

### Blocker 7. native binary policy

- overview:
  choose whether native packages are forbidden, sandboxed/trust-policy limited, or admitted by stronger provenance policy.
- affects:
  `specs/16`, runtime package admission, sample matrix, deployment claims.
- current recommendation:
  `sandboxed / trust-policy limited`.
- reason:
  signature alone is not semantic safety; provenance and capability/effect limits both matter.

### Blocker 8. save/load initial scope and UI target

- overview:
  choose whether initial save/load is local-only or multi-Place, and whether alpha UI target is CLI, HTML viewer, or richer client.
- affects:
  `P-A0-06`, `P-A0-09`, visualization/devtools scope, non-claim wording.
- current recommendation:
  `local-only save/load + HTML/JSON viewer first`.
- reason:
  `P-A0-12` already actualized the local-only save/load bridge, distributed save/load without consistent cut is disallowed, and the current viewer evidence is closest to typed HTML/JSON surfaces.

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
find samples/alpha -maxdepth 3 -type f | sort
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
cargo test -p mirrorea-core --test runtime_substrate
cargo test -p mir-runtime --test alpha_local_runtime --test alpha_cut_save_load_runtime
cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- save-load-resume
python3 scripts/alpha_cut_save_load_samples.py check-all --format json
python3 scripts/alpha_e2e_samples.py run E2E-06 --format json
python3 scripts/alpha_e2e_samples.py check-all --format json
python3 scripts/alpha_e2e_samples.py closeout --format json
python3 -m unittest scripts.tests.test_alpha_cut_save_load_checker scripts.tests.test_alpha_cut_save_load_samples scripts.tests.test_alpha_e2e_samples
cargo test -p mir-ast
cargo test -p mirrorea-core
cargo test -p mir-runtime
cargo test -p mir-semantics
cargo fmt --check
git diff --check
```

## reporting requirement

Every non-trivial change must add a new report under `docs/reports/`.
Use the current report template and keep all required sections in order. At minimum, the report must include:

- objective, scope / assumptions, and start dirty state
- documents consulted, actions taken, files changed, and commands run
- validation evidence / outputs, changed understanding, open questions, and suggested next prompt
- `plan/`, `Documentation.md`, `progress.md`, `tasks.md`, and `samples_progress.md` update status
- reviewer findings and follow-up
- skipped validations and reasons
- commit / push status
- sub-agent session close status
