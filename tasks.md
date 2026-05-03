# tasks

最終更新: 2026-05-03 18:23 JST

## この文書について

- この文書は repo 全体の **current task map** です。
- 規範判断の正本は `specs/`、長期比較と source trace は `plan/`、runnable sample 状態は `samples_progress.md`、実行証跡は `docs/reports/` に置きます。
- append-only 履歴ではありません。current checkpoint、次に詰める gate、blocker を読める snapshot として保ちます。
- この文書でいう `100%` は、明示的に **current-scope evidence closeout** と書かない限り、**practical alpha-1 readiness** 側の package/stage 達成度を指します。

## current task-level status

- active executable floor は維持されています:
  `samples/clean-near-end/`、Sugoroku world、avatar follow、typed external preview、network canary、projection/codegen bridge、viewer prototype inventory。`samples/current-l2/` は base source corpus、`samples/lean/` は Lean evidence / generated theorem stub corpus として分けて扱います。
- practical alpha-1 line が新しい promoted package line です:
  `specs/18`、`plan/44`、`samples/practical-alpha1/` を軸に、alpha-0 evidence closeout を prerequisite として保持しつつ、front-door -> checker -> runtime -> hot-plug -> transport -> devtools -> save/load -> product prototype の順で practical toolchain を組み上げます。
- `P-A1-04a` layer-only practical hot-plug first-floor freshness は 2026-05-03 18:23 JST に更新済みです:
  `crates/mir-ast::practical_alpha1_hotplug_plan`、`crates/mir-runtime::practical_alpha1_hotplug`、example `mir_practical_alpha1_attach`、`scripts/practical_alpha1_attach.py`、`HP-A1-01..05` expected reports、docs validators、source hierarchy、`cargo fmt --check`、`git diff --check` を rerun した。review follow-up で `P-A1-04` を stage-internal に recut し、`P-A1-04a` は layer-only first floor、`P-A1-04b` は freshness/object seam として切り分けた。`cargo test -p mir-ast --test practical_alpha1_front_door -- --nocapture`、`cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture`、`cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture`、`cargo test -p mir-runtime --test alpha_layer_insertion_runtime`、`python3 scripts/practical_alpha1_check.py check-all --format json`、`python3 scripts/practical_alpha1_run_local.py check-all --format json`、`python3 scripts/practical_alpha1_attach.py check-all --format json`、`python3 scripts/practical_alpha1_attach.py closeout --format json`、`python3 -m unittest scripts.tests.test_practical_alpha1_check scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha1_attach scripts.tests.test_validate_docs` が pass した。これは non-final alpha-local layer-only hot-plug floor の freshness であり、object package attach、missing-witness/stale-membership negatives、detach contract、run-docker、save/load、devtools、product prototype success はまだ claim しない。
- `P-A1-00` rebaseline validation freshness は 2026-05-03 15:27 JST に更新済みです:
  `P-A1-00` gate では `python3 -m unittest scripts.tests.test_validate_docs`、source hierarchy、docs scaffold、`cargo fmt --check`、`git diff --check` が pass し、new `specs/18` / `plan/44` と progress semantics repair が snapshot docs / validators に反映された。sub-agent review で見つかった `Documentation.md` / dashboard wording drift も反映済みである。これは docs/spec/taxonomy rebaseline package であり、practical front-door / parser / runtime implementation success はまだ claim しない。
- Alpha-0 evidence closeout line は retained reference です:
  `specs/13..17`、`plan/39..43`、`samples/alpha/` を軸に、large-stage-first で imported Stage A baseline の revalidation と current-scope Stage F alpha closeout まで完了した。以後は promoted product-readiness line ではなく、practical alpha-1 の prerequisite evidence / blocker inventory として読む。
- Alpha-0 closeout validation freshness は 2026-05-03 12:15 JST に更新済みです:
  `P-A0-28` gate では `current_l2_guided_samples.py closeout --format json`、`current_l2_lean_sample_sync.py`、`clean_near_end_samples.py closeout`、`sugoroku_world_samples.py closeout --format json`、`avatar_follow_samples.py closeout --format json`、`typed_external_boundary_samples.py closeout --format json`、`network_transport_samples.py check-all --format json`、`projection_codegen_samples.py check-all --format json`、`visual_debugger_viewer_samples.py closeout --format json`、`cargo test -p mir-runtime --test hotplug_runtime_skeleton`、source hierarchy、docs scaffold、`cargo fmt --check`、`git diff --check` が pass しました。current repo state では imported Stage A baseline と `E2E-01/02/03/04/05/06/07/09/10` existing integrated bridge subset を合わせて `Stage A..F` sequential closeout reading が fixed されており、`E2E-08`、public alpha / `U1`、active runnable-root promotion、distributed save/load completion は引き続き未claimです。
- `P0..P18`、`P19`、`P20`、`P21`、`R1..R7`、post-`P21` later-family docs-first trilogy は close 済みです。
  これらは repo-local alpha-ready current layer / docs-first boundary closeout であり、final public parser/API/ABI、rollback、durable migration、distributed ordering、production transport、final viewer/verifier completion ではありません。
- historical post-`P21` docs-first family は close したままです。
  ただし current promoted implementation line が存在しないという historical snapshot には戻らず、今は `P-A1-00` / `P-A1-01` / `P-A1-02` / `P-A1-03` / `P-A1-04a` で practical alpha-1 line を昇格した。alpha-local package `P-A0-01..28` closeout は current-scope evidence reference として残し、`LIF-15` / `VAR-14` は future carrier blocker split に留めたまま、practical line 側では `P-A1-04b` freshness negatives and object-attach seam を next gate とする。
- `U1` actual commitment は依然 separate gate です。
  Packaging / installed binary target、host integration target、first shipped public surface scope、final shared-space operational catalog breadth は user-facing decision を要します。
- self-driven に残るのは maintenance lane です:
  stale docs cleanup、validation rerun、report creation、formatting cleanup、guardrail maintenance、regression repair、dashboard freshness、source hierarchy / docs scaffold guardrail maintenance。
  これに加えて practical alpha-1 package sequence が current autonomous package として主線になり、alpha-local theory-freeze / checker-runtime work は reserve/reference lane に退きます。
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

## Current Practical Alpha-1 stage

- Large stage:
  `PA1-0` 100% practical-alpha-rebaseline closeout; `PA1-1` 100% alpha-source front-door closeout; `PA1-2` 100% first typed IR/checker floor closeout; `PA1-3` 100% local runtime from runtime plan closeout; `PA1-4` 45% package/hot-plug practical API in progress via `P-A1-04a`
- Concrete phase:
  Phase 5/9 — front-door、first checker floor、first local-runtime floor、layer-only hot-plug first floor are closed; freshness/object seam is next inside `PA1-4`
- Package status:
  `P-A1-04a` layer-only practical hot-plug first floor is the last closed package in repo state
- Current status:
  `P-A1-04a` は `crates/mir-ast::practical_alpha1_hotplug_plan`、`crates/mir-runtime::practical_alpha1_hotplug`、example `mir_practical_alpha1_attach`、`scripts/practical_alpha1_attach.py`、および `HP-A1-01..05` practical fixtures を actualize し、source/package front-door と first checker floor に続く layer-only hot-plug first floor を practical line に追加した。これは non-final alpha-local hot-plug floor であり、object package attach、missing-witness/stale-membership negatives、Docker/local TCP、save/load、devtools はまだ later である。
- Next autonomous package:
  `P-A1-04b` freshness negatives and object-attach seam
- Public-decision gate kept separate:
  `U1` remains open and is not collapsed into the practical alpha-1 package series

## Current-scope alpha-0 evidence reference

- Large stage:
  Stage A 100% imported alpha-ready baseline, Stage B 100% alpha-0.5 local-runtime closeout, Stage C 100% alpha-0.7 transport closeout, Stage D 100% alpha-0.8 hot-plug lifecycle closeout, Stage E 100% alpha-0.9 devtools closeout, Stage F 100% current-scope Spaces alpha closeout
- Concrete phase:
  Phase 7/7 — current-scope evidence line complete; later-family and public-boundary blockers remain
- Package status:
  `P-A0-28` Stage A imported-baseline reconciliation is the last closed evidence package
- Current status:
  `P-A0-28` は imported Stage A validation floor を rerun し、`specs/17` / `plan/43` / snapshot docs を同期して alpha line の large-stage reading を `Stage A..F` sequential closeout として固定した。negative side / acceptance side / snapshot-selected side / anchor-handoff side / runtime-mirror side の separate carriers と Stage B..F closeouts は維持されるが、current queue authority は practical line とは別に later-family blocker selection または separate public-boundary `U1` lane へ残っている。

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
| alpha hot-plug lifecycle closeout | `samples/alpha/layer-insertion/`, `samples/alpha/avatar-runtime/`, `samples/alpha/hotplug-runtime/`, `scripts/alpha_hotplug_lifecycle_samples.py` | `cargo test -p mir-runtime --test alpha_layer_insertion_runtime`; `cargo run -q -p mir-runtime --example mirrorea_alpha_layer_insertion_runtime -- closeout`; `cargo test -p mir-runtime --test alpha_avatar_runtime`; `python3 scripts/alpha_avatar_runtime_samples.py check-all --format json`; `python3 scripts/alpha_hotplug_lifecycle_samples.py stage-d-closeout --format json` | detach runtime / durable migration / distributed activation ordering / native execution / final layer-package-avatar ABI |
| alpha cut/save-load bridge | `crates/mirrorea-core`, `crates/mir-runtime`, `samples/alpha/cut-save-load/`, `scripts/alpha_cut_save_load_samples.py` | `cargo test -p mirrorea-core --test runtime_substrate`; `cargo test -p mir-runtime --test alpha_local_runtime --test alpha_cut_save_load_runtime`; `cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- save-load-resume`; `cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- save-load-stale-membership`; `python3 scripts/alpha_cut_save_load_samples.py check-all --format json` | distributed save/load / durable cut / `CUT-10/12/16` completion |
| alpha local runtime closeout | `crates/mir-runtime`, `samples/alpha/local-runtime/`, `samples/alpha/cut-save-load/`, `scripts/alpha_local_runtime_samples.py` | `cargo test -p mir-runtime --test alpha_local_runtime --test alpha_cut_save_load_runtime`; `cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- local-sugoroku`; `cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- stale-membership`; `python3 scripts/alpha_local_runtime_samples.py check-all --format json`; `python3 scripts/alpha_local_runtime_samples.py stage-b-closeout --format json` | distributed save/load / CUT family completion / active runnable-root promotion |
| alpha visualization / devtools bridge | `samples/alpha/visualization/`, `scripts/alpha_visualization_samples.py` | `python3 scripts/alpha_visualization_samples.py list --format json`; `python3 scripts/alpha_visualization_samples.py check-all --format json`; `python3 scripts/alpha_visualization_samples.py closeout --format json`; `python3 scripts/alpha_visualization_samples.py stage-e-closeout --format json`; `python3 -m unittest scripts.tests.test_alpha_visualization_samples` | final viewer API / telemetry service / `VIS-04/09/12` completion |
| alpha integrated E2E bridge | `samples/alpha/e2e/`, `scripts/alpha_e2e_samples.py` | `python3 scripts/alpha_e2e_samples.py run E2E-06 --format json`; `python3 scripts/alpha_e2e_samples.py check-all --format json`; `python3 scripts/alpha_e2e_samples.py closeout --format json`; `python3 scripts/alpha_e2e_samples.py stage-f-closeout --format json` | public alpha / `U1` completion / late save-load / later-family blockers |
| projection / placement | `scripts/projection_codegen_samples.py` + committed generated manifest | `python3 scripts/projection_codegen_samples.py check-all --format json` | final emitted executable family / generated place-program emission / placement optimizer / deployment planner / equivalence checker / proof completion / final public emitted-program ABI |
| viewer prototype | `scripts/visual_debugger_viewer_samples.py` | `python3 scripts/visual_debugger_viewer_samples.py closeout --format json` | final viewer API / telemetry service |
| hot-plug runtime | `crates/mirrorea-core`, `crates/mir-runtime` | `cargo test -p mir-runtime --test hotplug_runtime_skeleton` | rollback / durable migration / distributed ordering / final ABI |
| storage / backend | `/mnt/mirrorea-work`, `scripts/env/`, `scripts/storage/` | `bash scripts/env/mirrorea_storage_env.sh --ensure-dirs`; `bash scripts/storage/detach_prepare.sh`; `bash scripts/storage/cleanup_disposable_artifacts.sh --list`; `CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run` | actual LLVM build / backend choice |

## practical alpha-1 package map

| Package / stage | Progress | Status | Close condition |
|---|---:|---|---|
| `PA1-0` / `P-A1-00` | 100% | closed | practical/evidence progress semantics repair、`specs/18` + `plan/44`、snapshot docs sync、validators、report、commit/push |
| `PA1-1` / `P-A1-01` | 100% | closed | limited alpha source/package format、initial practical sample fixtures、front-door parser/loader、positive/negative parse tests、report、commit/push |
| `PA1-2` / `P-A1-02` | 100% | closed | distinct lowered IR、non-final checker report、`scripts/practical_alpha1_check.py`、`CHK-LIF-01..04` / `CHK-VAR-01..03` / `CHK-CUT-01` / `CHK-PKG-01/02` の explicit accepted/rejected evidence |
| `PA1-3` / `P-A1-03` | 100% | closed | checked package を distinct runtime-plan boundary が consume し、practical source package から local world を起動し event DAG を export |
| `PA1-4` / `P-A1-04a+b` | 45% | in progress | `P-A1-04a` closed the layer-only hot-plug first floor; `P-A1-04b` remains for missing-witness/stale-membership negatives and object package attach seam |
| `PA1-5` / `P-A1-05` | 10% | pending | same practical package input で Docker/local TCP を動かし、route trace と separated lanes を export |
| `PA1-6` / `P-A1-06` | 10% | pending | JSON schema、viewer command、event DAG / route / membership / hot-plug / fallback の可視化 |
| `PA1-7` / `P-A1-07` | 10% | pending | practical `save` / `load` command、local roundtrip、stale membership non-resurrection、invalid distributed cut reject |
| `PA1-8` / `P-A1-08` | 0% | pending | small product prototype を local + Docker で動かし、layer/package attach、avatar fallback、save/load、devtools export、hands-on docs を揃える |

## current-scope evidence closeout map

| Stage | Progress | Name | Status | Main evidence | Not yet claimed |
|---|---:|---|---|---|---|
| A | 100% | imported alpha-ready baseline | current-scope imported baseline revalidated and fixed for the alpha line | current-L2 / clean-near-end / Lean、Sugoroku、avatar、typed external preview、network canary、projection/codegen bridge、viewer prototype、hot-plug narrow floor | final public product / `samples/alpha/` runnable-root promotion |
| B | 100% | alpha 0.5 local runtime | current-scope closeout reached as local runtime + local-only save/load subset | `specs/15/17`、`plan/41/43`、`samples/alpha/local-runtime/`、`samples/alpha/cut-save-load/`、`crates/mir-runtime/src/alpha_local_runtime.rs`、`scripts/alpha_local_runtime_samples.py`、`scripts/alpha_cut_save_load_samples.py` | distributed save-load completion / CUT family completion / final public runtime ABI |
| C | 100% | alpha 0.7 transport | current-scope closeout reached as Docker/local-subprocess transport narrow cut | helper-local `NET-02..05` canaries + `samples/alpha/network-docker/` + `alpha_network_runtime` + `alpha_network_docker_e2e.py` + `stage-c-closeout` | `NET-06/08/10` / production WAN / durable replay / partition completion / final transport ABI |
| D | 100% | alpha 0.8 hot-plug lifecycle | current-scope closeout reached as attach-time layer subset + runtime-private package/avatar admission subset | `P19..P21` floor + `samples/alpha/layer-insertion/` + `samples/alpha/avatar-runtime/` + `samples/alpha/hotplug-runtime/` + `scripts/alpha_hotplug_lifecycle_samples.py` | detach / migration / native execution / final ABI |
| E | 100% | alpha 0.9 devtools | current-scope closeout reached as implemented visualization/devtools subset | `samples/alpha/visualization/` + `scripts/alpha_visualization_samples.py` + `stage-e-closeout` + viewer prototype inventory + existing alpha bridge evidence | `VIS-04/09/12` / final viewer / telemetry API |
| F | 100% | Spaces alpha evidence closeout | current-scope closeout reached as thin integrated bridge + Stage-E closeout + local-only save/load subset + checker-backed invalid distributed-cut non-claim | `samples/alpha/e2e/` thin bridge runner + `scripts/alpha_e2e_samples.py` + `stage-f-closeout` + `scripts/alpha_visualization_samples.py stage-e-closeout` + `scripts/alpha_cut_save_load_samples.py` + scope spec | public alpha completion / full VRChat / Reversed Library completion |
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
| `P-A0-25` | `Macro 6-7` | closed | Stage D current-scope closeout over existing layer/package/avatar floors |
| `P-A0-26` | `Macro 6-7` | closed | Stage E current-scope closeout over the existing visualization/devtools subset |
| `P-A0-27` | `Macro 6-7` | closed | Stage F current-scope closeout over the existing integrated alpha bridge |
| `P-A0-28` | `Macro 6-7` | closed | imported Stage A baseline rerun + stage-order reconciliation for the large-stage-first alpha evidence line |
| `P-A1-00` | `Macro 8` practical line | closed | practical alpha-1 rebaseline: future `100%` means practical readiness, Stage A..F remain evidence-only, and `specs/18` / `plan/44` become the promoted roadmap memory |
| post-`P21` rollback / durable migration | `Macro 8` prep | closed docs-first | first recommendation boundary; no actual rollback / migration engine completion |
| post-`P21` distributed activation ordering | `Macro 8` prep | closed docs-first | second recommendation boundary; no actual distributed activation protocol |
| post-`P21` final public hot-plug ABI | `Macro 8` mixed gate | closed docs-first | third recommendation bridge: `freeze prerequisite fixed; public ABI still unfrozen` |

## ordered current work

| Order | Work item | Owner | Status | Completion condition |
|---:|---|---|---|---|
| 1 | `P-A1-04b` freshness negatives and object-attach seam | repo | ready | add missing-witness/stale-membership negatives and the narrow object package attach seam without collapsing runtime/package boundaries |
| 2 | `P-A1-05..08` practical toolchain completion | repo | staged later | transport E2E, devtools, save/load, and product prototype sequence after package API exists |
| 3 | alpha-0 evidence later-family blockers | repo | reserve lane | reopen only if a practical package is blocked by `CUT-10/12/16`, `LIF-15`, `VAR-14`, or transport/lifecycle widening decisions |
| 4 | `U1` actual commitment | user + repo | later | actual choices recorded for packaging, host target, first shipped public surface, final catalog breadth |
| 5 | maintenance / dashboard freshness | repo | active | keep practical/evidence split wording and validation anchors current while package line advances |

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
| `P-A0-13` | dedicated alpha visualization/devtools bridge | closed | `VIS-01/03/06/07/08/10/11` subset runner + integrated visibility/redaction surfaces + report `1113` |
| `P-A0-14` | honest CUT widening closeout | closed | `CUT-17` local stale-membership rejection bridge + `CUT-11` checker-backed Z-cycle inadmissibility row + report `1114` |
| `P-A0-15` | remaining Stage-E visualization widening after the honest CUT subset closeout | closed | `VIS-02` thin place-catalog projection + `VIS-05` thin membership epoch/incarnation timeline actualization with `VIS-04/09/12` planned-only retention + report `1115` |
| `P-A0-16` | selected LIF/VAR checker widening after the widened Stage-E subset sync | closed | `LIF-01` raw dangling reference reject + `VAR-05` synthetic mutable/read-write invariance negative row + floor-scope confinement hardening + report `1116` |
| `P-A0-17` | accept-side evidence carrier for positive LIF/VAR rows | closed | helper-local synthetic acceptance artifact schema for `LIF-02/03/04` + `VAR-01/04/06` with separate `acceptance_scope` confinement |
| `P-A0-18` | runtime-mirror bridge | closed | `VAR-08/11/13` runtime-mirror floor via `LI-04/01/03` with separate `runtime_mirror.scope` confinement, no parser/runtime bridge claim, and report `1123` |
| `P-A0-19` | remaining positive-row carrier inventory | closed | docs-first blocker inventory for `LIF-11/13/15` + `VAR-14`, no carrier widening, and report `1124` |
| `P-A0-20` | snapshot-selected floor | closed | dedicated helper-local synthetic snapshot-selected carrier for `LIF-13` only, without widening acceptance/runtime-mirror floors, and report `1127` |
| `P-A0-21` | anchor-handoff floor | closed | dedicated helper-local synthetic anchor-handoff carrier for `LIF-11` only, without widening acceptance/snapshot/runtime-mirror floors, and report `1128` |
| `P-A0-22` | remote / adapter blocker split | closed | docs-first split fixing planned-only `alpha-remote-observe-floor` for `LIF-15` and `alpha-adapter-transform-floor` for `VAR-14`, without actualizing either row, and report `1129` |
| `P-A0-23` | Stage B alpha-0.5 closeout | closed | dedicated local-runtime runner plus Stage B closeout surface over `LR-01/02` and `CUT-04/17` supporting subset; distributed save/load and CUT family completion remain later |
| `P-A0-24` | Stage C transport closeout | closed | dedicated Docker/local-subprocess closeout surface over `NET-02/03/04/05/07/09`; `NET-06/08/10` and WAN/final-ABI claims remain later |
| `P-A0-25` | Stage D lifecycle closeout | closed | dedicated Stage D closeout surface over `LI-01/02/03/04/05` and `AV-01/02/06/08/09` / `HP-11/12/15`; detach/migration/native/final-ABI claims remain later |
| `P-A0-26` | Stage E devtools closeout | closed | dedicated Stage E closeout surface over `VIS-01/02/03/05/06/07/08/10/11`; `VIS-04/09/12` and public viewer/telemetry claims remain later |
| `P-A0-27` | Stage F integrated alpha closeout | closed | dedicated Stage F closeout surface over `E2E-01/02/03/04/05/06/07/09/10` plus current-scope Stage E dependency; `E2E-08` and public alpha / `U1` claims remain later |
| `P-A0-28` | Stage A imported-baseline reconciliation | closed | rerun imported Stage A validation floor and synchronize `specs/17` / `plan/43` / snapshot docs so `Stage A..F` reads sequentially for current scope |
| `P-A1-00` | practical-alpha-rebaseline | closed | add `specs/18` / `plan/44`, rebase future `100%` to practical alpha-1 readiness, and retain Stage A..F `100%` only as current-scope evidence closeout |
| `P-A1-01` | alpha-source front-door design | closed | limited alpha source/package format, initial practical sample fixtures, front-door parser/loader, parse positive/negative tests |
| `P-A1-02` | typed IR/checker first floor | closed | reusable checker route over practical front-door output, distinct lowered IR, non-final checker report, explicit accepted/rejected evidence |
| `P-A1-03` | local runtime from runtime plan | closed | consume checked practical package in a reusable local runtime path with a distinct runtime-plan carrier and non-final local-runtime report |
| `P-A1-04a` | layer-only practical hot-plug first floor | closed | practical manifest admission plus debug/rate-limit/auth layer attach path over `HP-A1-01..05`, with distinct hotplug-plan carrier and exact expected reports |
| `P-A1-04b` | practical hot-plug freshness/object seam | promoted next | missing-witness/stale-membership negatives plus narrow object package attach seam without claiming full Stage-D/public ABI completion |
| `P-A1-05` | transport practical E2E | pending | same practical package input for local and Docker/local TCP modes |
| `P-A1-06` | devtools viewer | pending | practical export schema and viewer command |
| `P-A1-07` | local save/load command | pending | practical save/load CLI or library path with negative stale-state checks |
| `P-A1-08` | practical alpha product prototype | pending | one small product-like world package with local + Docker + hot-plug + save/load + devtools |

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
python3 scripts/alpha_visualization_samples.py check-all --format json
python3 scripts/alpha_visualization_samples.py closeout --format json
python3 scripts/visual_debugger_viewer_samples.py check-all --format json
python3 scripts/alpha_e2e_samples.py run E2E-06 --format json
python3 scripts/alpha_e2e_samples.py check-all --format json
python3 scripts/alpha_e2e_samples.py closeout --format json
python3 -m unittest scripts.tests.test_alpha_cut_save_load_checker scripts.tests.test_alpha_cut_save_load_samples scripts.tests.test_alpha_visualization_samples scripts.tests.test_alpha_e2e_samples
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
