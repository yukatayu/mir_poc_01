# current phase closeout 01

## この文書の役割

この文書は、**repo-local alpha current line** と
**Mirrorea future-axis docs-first line** を、実行コマンドと stop line 付きで短く確認するための hands-on closeout guide です。

- final public completion ではありません
- active sample と planned sample を混同しません
- helper-local debug output を final public API として扱いません

## まず実行するコマンド

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 scripts/current_l2_guided_samples.py closeout --format json
python3 scripts/sugoroku_world_samples.py closeout --format json
python3 scripts/avatar_follow_samples.py closeout --format json
python3 scripts/typed_external_boundary_samples.py closeout --format json
python3 scripts/network_transport_samples.py check-all --format json
python3 scripts/projection_codegen_samples.py check-all --format json
python3 scripts/projection_codegen_samples.py closeout --format json
python3 scripts/visual_debugger_viewer_samples.py closeout --format json
cargo fmt --check
cargo test -p mirrorea-core
cargo test -p mir-runtime --test hotplug_runtime_skeleton
cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json
git diff --check
find samples/generated -maxdepth 3 -type f | sort
bash scripts/env/mirrorea_storage_env.sh
bash scripts/env/mirrorea_storage_env.sh --ensure-dirs
bash scripts/storage/detach_prepare.sh
bash scripts/storage/cleanup_disposable_artifacts.sh --list
free -h
ls -ld target /mnt/mirrorea-work/cargo-target /mnt/mirrorea-work/cargo-registry-cache /mnt/mirrorea-work/llvm /mnt/mirrorea-work/llvm/src /mnt/mirrorea-work/llvm/build /mnt/mirrorea-work/llvm/install
CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run
```

## 追加で見る debug lane

```bash
python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --transport loopback_socket --debug envelopes --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --transport loopback_socket --debug envelopes --format json
python3 scripts/network_transport_samples.py run NET-02 --debug route-trace --format json
python3 scripts/network_transport_samples.py run NET-03 --debug reconnect --format json
python3 scripts/network_transport_samples.py run NET-04 --debug failures --format json
python3 scripts/network_transport_samples.py run NET-05 --debug route-trace --format json
python3 scripts/avatar_follow_samples.py run 01_follow_remote_head_with_local_fallback --debug anchors --format json
python3 scripts/avatar_follow_samples.py run 03_remote_avatar_leaves_falls_back_to_local --debug membership --format json
python3 scripts/avatar_follow_samples.py run 06_model_check_no_detached_anchor_observed --debug verification --format json
python3 scripts/typed_external_boundary_samples.py run EXT-03 --debug envelopes --format json
python3 scripts/typed_external_boundary_samples.py run EXT-03 --debug visualization --format json
python3 scripts/typed_external_boundary_samples.py run EXT-04 --debug failures --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug signatures --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug layers --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json
python3 scripts/projection_codegen_samples.py run P15-GEN-01 --format json
python3 scripts/projection_codegen_samples.py run P15-GEN-03 --format json
python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-01 --format json
python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-03 --format json
python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-04 --format json
python3 scripts/visual_debugger_viewer_samples.py check-all --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json
python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json
python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json
```

## これで確認できること

- active clean near-end suite、Sugoroku world、avatar follow、typed external preview、network canary、projection/codegen bridge、viewer prototype inventory、hot-plug Rust floor が current runnable または closeout-backed surface として確認できること。
- `TermSignature`、`LayerSignature`、`MessageEnvelope`、`AuthEvidence`、`VisualizationProtocol` / telemetry envelope、helper `verification_handoff_witness`、runtime `verification_model_check` の current evidence carriers が helper-local / report-local inventory として読めること。
- `mirrorea-core` の `MembershipRegistry`、`PlaceCatalog`、`LogicalPlaceRuntimeShell`、engine-neutral `HotPlugRequest` / `HotPlugVerdict` と、`mir-runtime` の hot-plug skeleton / engine report が repo-local floor として確認できること。
- typed external `EXT-03/04`、network `NET-02..05`、projection/codegen generated manifest bridge、viewer `P16-VIEW-01..05` は final public ABI / service / emitted executable ではなく、typed helper evidence として読めること。projection/codegen current `equivalence` reading は review-category alignment inventory であり、generated place-program synthesis / placement optimizer / deployment planner / cross-place equivalence checker / proof completion ではないこと。
- `auth none` baseline のまま、transport / authentication / membership / capability / witness / visualization を collapse していないこと。
- NET-05 observer route trace が fail-closed であり、observer-safe debug output が principal / auth / freshness / authorization / raw witness refs を漏らさないこと。
- storage/backend guardrail が external workdir、`CARGO_TARGET_DIR` / `CARGO_HOME`、non-destructive cleanup list-mode、`llvm/src` exclusion、parent non-writable guard を current operational floorとして確認できること。
- package-by-package history はこの guide では再列挙せず、`docs/reports/` と relevant `plan/` を正本として読むこと。

## これではまだ確認できないこと

- final public parser grammar
- final public parser / checker / runtime / verifier API
- final public `AuthEvidence` schema
- final public `witness_refs` role taxonomy
- final public adapter API / FFI
- exact host schema
- real network transport
- final projection / placement public API
- final hot-plug runtime lifecycle
- actual LLVM build
- installed binary / FFI / engine adapter / deployment contract

## current closeout の読み

current closeout で揃ったのは、**仕様・sample・helper・report・progress dashboard が同じ現在地を指す状態** です。

- active sample:
  `samples/clean-near-end/`
- active base source corpus:
  `samples/current-l2/`
- active proof evidence:
  `samples/lean/`
- planned sample:
  `samples/not_implemented/`
- prototype / historical:
  `samples/prototype/` と `samples/old/`
- generated artifact reserve:
  `samples/generated/`
- helper-local preview:
  script の `--debug` 出力、detached artifact、report-local inventory
- dashboard:
  `samples_progress.md`
- current task map / current roadmap summary:
  `tasks.md` と `docs/research_abstract/mirrorea_future_axis_01.md`
- final public API:
  まだ deferred
- deferred public/mixed-gate categories:
  parser/public API、auth/public contract、adapter/public contract、exact host schema、visualization/public contract、projection/public API、hot-plug/public API

## deferred public/mixed-gate categories

- final parser grammar / public parser / checker / runtime / verifier surface
- final public auth / visualization / projection / hot-plug surface
- final public adapter / exact host schema
- final public viewer API / visualization schema / telemetry schema
- transport canary から real socket / session / durable replay への widening
- real migration / rollback / runtime-crate hot-plug engine actualization beyond current owner split
- `FAIRY-05` runnable reopen / final carrier naming beyond current provisional recommendation
- actual LLVM artifact と backend choice

## `U1` decision categories

- installed binary / packaging adoption target
- FFI / engine adapter / host integration target
- first shipped public surface scope
- broader application target
- final shared-space operational catalog

## closeout memory and current snapshot reading

この hands-on doc が保持するのは command-oriented closeout reading です。live status / next reopen point / remaining queue ownership は `progress.md` と `tasks.md` を参照してください。package-by-package history は `docs/reports/` の正本に残します。

current snapshot を読むときは、次だけ押さえれば十分です。

- live status / next reopen point:
  `progress.md`、`tasks.md`
- runnable sample dashboard:
  `samples_progress.md`
- reader-facing roadmap summary:
  `../research_abstract/mirrorea_future_axis_01.md`
- `U1` option inventory:
  `post_p18_true_user_spec_hold_01.md`、`../research_abstract/post_p18_true_user_spec_hold_option_matrix_01.md`、`../../plan/28-post-p18-true-user-spec-hold-option-matrix.md`
- future-axis closeout memory:
  `../../plan/29-verification-layer-widening-threshold.md` から `../../plan/38-post-p21-final-public-hotplug-abi-family.md` までの family docs

この guide で保持する stop line は次です。

- helper-local preview、generated bridge evidence、report-local inventory は final public API / emitted executable / production service contract ではありません。projection/codegen bridge evidence は generated place-program synthesis / placement optimizer / deployment planner / cross-place equivalence checker / proof completion / final public emitted-program ABI でもありません。
- `P0..P21` と post-`P21` docs-first trilogy は close 済みですが、この guide は closed package ledger を再列挙しません。
- next reopen point はこの guide では固定せず、`progress.md` と `tasks.md` の current snapshot を参照します。

## 関連文書

- `../research_abstract/mirrorea_future_axis_01.md`
- `../research_abstract/network_transport_plan_01.md`
- `network_transport_canaries_01.md`
- `../research_abstract/avatar_fairy_follow_plan_01.md`
- `avatar_fairy_follow_representative_slice_01.md`
- `typed_external_boundary_canaries_01.md`
- `projection_placement_views_01.md`
- `visual_debugger_viewer_01.md`
- `compiler_backend_llvm_preparation_01.md`
- `public_api_parser_gate_01.md`
- `../research_abstract/public_api_parser_gate_plan_01.md`
- `../research_abstract/compiler_backend_llvm_preparation_01.md`
- `../../plan/19-repository-map-and-taxonomy.md`
- `../../plan/27-public-api-parser-gate-roadmap.md`
- `../../samples_progress.md`
