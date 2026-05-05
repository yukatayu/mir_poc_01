# 2044 — FAQ 016 current status recheck

## Objective

前回までの Mir / Mirrorea bottom-layer、host-I/O、Sugoroku、verification、transport、hot-plug、native bundle、membership/vector-clock 論点について、現在の repo 状態を読み直し、実際に代表コマンドを実行して確認した結果を `tmp_faq/faq_016.md` にまとめる。

## Scope and assumptions

- 説明・確認専用の task として扱う。
- 新機能実装、仕様変更、public API / ABI の凍結、sample の昇格は行わない。
- Docker / Docker Compose が利用可能なら Docker path も実行する。
- product alpha release-candidate workflow と final public product readiness を混同しない。
- current-L2 `.mir` helper sample と product alpha `package.mir.json` front-door を混同しない。

## Start state / dirty state

- start branch: `main`
- work branch: `docs/faq016-current-status-recheck-001`
- validation target HEAD before edits: `4a9a4ba`
- `git status --short` was clean before creating this FAQ/report.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/19-verification-stratification.md`
- `specs/20-cut-save-load-semantics.md`
- `specs/21-auth-layer-algebra.md`
- `specs/22-observability-devtools-semantics.md`
- `specs/23-typed-external-host-boundary.md`
- `specs/24-operational-alpha05-alpha08-readiness.md`
- `specs/25-product-alpha1-public-boundary.md`
- `plan/49-host-io-and-session-runtime-roadmap.md`
- `plan/50-product-alpha1-public-boundary-roadmap.md`
- `samples/practical-alpha1/README.md`
- `samples/product-alpha1/README.md`
- `docs/hands_on/product_alpha1_01.md`
- `scripts/README.md`
- `samples/product-alpha1/demo/package.mir.json`
- `samples/product-alpha1/docker/docker-compose.product-alpha1.yml`
- `crates/mirrorea-core/src/runtime.rs`
- `crates/mirrorea-core/tests/runtime_substrate.rs`

## Actions taken

- Discord task baseline was recorded.
- Created local work branch `docs/faq016-current-status-recheck-001`.
- Read current normative specs and product alpha roadmap/docs.
- Ran repository health checks.
- Ran current-L2 / clean-near-end typing, order/handoff, model-check, Lean sync, and Sugoroku checks.
- Ran practical alpha checker/session/hotplug/devtools/integrated workflow checks.
- Ran product alpha CLI command family over a temporary session directory.
- Verified Docker and Docker Compose availability, then ran product Docker transport.
- Built a product native host launch bundle and checked generated bundle files.
- Ran `demo` and `product_alpha1_release_check.py check-all` with Docker included.
- Created a temporary `AddOne` variant and verified `Int(7) -> Int(8)`.
- Wrote `tmp_faq/faq_016.md`.
- Wrote this report.

## Files changed

- `tmp_faq/faq_016.md`
- `docs/reports/2044-faq016-current-status-recheck.md`

## Commands run

Git / branch setup:

```bash
git branch --show-current
git status --short
git fetch origin
git pull --ff-only origin main
git switch -c docs/faq016-current-status-recheck-001
git rev-parse --short HEAD
date '+%Y-%m-%d %H:%M:%S %Z'
```

Discord baseline:

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
```

Repository health:

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

Current-L2 / clean-near-end / Sugoroku:

```bash
python3 scripts/current_l2_guided_samples.py closeout --format json
python3 scripts/clean_near_end_samples.py closeout
python3 scripts/clean_near_end_samples.py list --format json
python3 scripts/clean_near_end_samples.py run typing --format json
python3 scripts/clean_near_end_samples.py run order-handoff --format json
python3 scripts/clean_near_end_samples.py run model-check --format json
python3 scripts/sugoroku_world_samples.py closeout --format json
python3 scripts/current_l2_lean_sample_sync.py
```

One mistaken command shape was run and then corrected:

```bash
python3 scripts/clean_near_end_samples.py run 02_peterson_relaxed_counterexample --format json
```

Practical alpha:

```bash
python3 scripts/practical_alpha1_check.py check-all --format json
python3 scripts/practical_alpha05_session.py check-all --format json
python3 scripts/practical_alpha08_session_hotplug.py check-all --format json
python3 scripts/practical_alpha09_devtools.py check-all --format json
python3 scripts/practical_alpha1_integrated_workflow.py check-all --format json
```

Product alpha command family:

```bash
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/demo --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/demo --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- session 'session#product-alpha1-demo' --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/debug-layer --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/auth-layer --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/rate-limit-layer --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/placeholder-object --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/custom-avatar-preview --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- save 'session#product-alpha1-demo' --savepoint 'savepoint#r0-faq016-summary' --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- quiescent-save 'session#product-alpha1-demo' --savepoint 'savepoint#r2-faq016-summary' --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- load 'savepoint#r0-faq016-summary' --session 'session#product-alpha1-demo' --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- transport 'session#product-alpha1-demo' --mode local --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- transport 'session#product-alpha1-demo' --mode docker --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- export-devtools 'session#product-alpha1-demo' --out /tmp/mirrorea-faq016-viewer-summary --format json
cargo run -q -p mirrorea-cli -- view /tmp/mirrorea-faq016-viewer-summary --check --format json
cargo run -q -p mirrorea-cli -- build-native-bundle samples/product-alpha1/demo --out "$bundle_dir" --format json
sh "$bundle_dir/run.sh" check
sh "$bundle_dir/run.sh" view
cargo run -q -p mirrorea-cli -- demo samples/product-alpha1/demo --out /tmp/mirrorea-faq016-demo-summary --format json
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-faq016-release-summary
```

Negative/direct `.mir` checks:

```bash
cargo run -q -p mirrorea-cli -- check samples/clean-near-end/typing/01_authorized_declassification.mir --format json
cargo run -q -p mirrorea-cli -- build-native-bundle samples/clean-near-end/typing/01_authorized_declassification.mir --out /tmp/mirrorea-faq016-direct-mir-bundle --format json
```

Focused tests:

```bash
cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture
cargo test -p mir-runtime --test product_alpha1_session -- --nocapture
cargo test -p mir-runtime --test product_alpha1_transport_devtools -- --nocapture
cargo test -p mirrorea-cli --test alpha_cli -- --nocapture
cargo test -p mirrorea-core -- --nocapture
```

Docker environment:

```bash
docker --version
docker compose version
```

Temporary AddOne variant:

```bash
cp -R samples/product-alpha1/demo/. "$variant/"
python3 - "$variant/package.mir.json" <<'PY'
import json, pathlib, sys
path = pathlib.Path(sys.argv[1])
payload = json.loads(path.read_text())
payload["runtime_input"]["host_io"]["request_payload"]["value"] = 7
payload["runtime_input"]["host_io"]["expected_response"]["value"] = 8
path.write_text(json.dumps(payload, indent=2) + "\n")
PY
cargo run -q -p mirrorea-cli -- check "$variant" --format json
MIRROREA_ALPHA_SESSION_DIR="$variant_session" cargo run -q -p mirrorea-cli -- run-local "$variant" --format json
```

## Evidence / outputs / test results

- `check_source_hierarchy.py`: required 106, present 106, missing 0.
- `validate_docs.py`: documentation scaffold complete, found 1196 numbered reports after adding this report.
- `cargo fmt --check`: pass.
- `git diff --check`: pass before edits.
- `current_l2_guided_samples.py closeout`: clean-near-end families and inventory emitted.
- `clean_near_end_samples.py run typing`: authorized declassification valid/success; unauthorized declassification, high-to-low flow, capture escape, and cost bound rejected.
- `clean_near_end_samples.py run order-handoff`: authorized roll/publish/handoff success; missing witness and handoff-before-publication rejected; stage-block sample includes `atomic_cut(stage_block)`.
- `clean_near_end_samples.py run model-check`: Peterson SC pass; relaxed Peterson and broken mutex counterexamples found.
- `sugoroku_world_samples.py closeout`: sample_count 10; single OS process logical multi-place emulator; no real network / no durable distributed commit / detach TODO stop line.
- `current_l2_lean_sample_sync.py`: returned `samples/lean/manifest.json`.
- `practical_alpha1_check.py check-all`: 10 passed; first checker floor complete; `spec18_typed_checking_complete = false`.
- `practical_alpha05_session.py check-all`: 7 passed; `operational_alpha05_ready = true`.
- `practical_alpha08_session_hotplug.py check-all`: 10 passed; `operational_alpha08_ready = true`.
- `practical_alpha09_devtools.py check-all`: 9 passed; `operational_alpha09_ready = true`.
- `practical_alpha1_integrated_workflow.py check-all`: 8 passed; bounded workflow ready; `product_public_ready = false`.
- Product `check`: accepted, 10 accepted obligations, 6 residual obligations, standalone `product_alpha1_ready = false`.
- Product `run-local`: same-session carrier with `AddOne Int(41) -> Int(42)`, 9 DAG nodes, 8 DAG edges, explicit lane preservation.
- Variant run-local: `AddOne Int(7) -> Int(8)`.
- Product attach matrix: debug/auth/rate-limit accepted; placeholder object/custom-avatar-preview deferred.
- Product save/load: R0 save and load succeeded.
- Product quiescent save: R2 saved with `no_inflight = true`, `all_places_sealed = true`, `no_post_cut_send = true`.
- Product local transport: local loopback TCP accepted, wire roundtrip executed, lane preservation true.
- Product Docker transport: Docker Compose TCP accepted, world and participant accepted, lane preservation true.
- Product devtools export: non-final static viewer with 13 panel IDs, admin debug kept later, final viewer not frozen, durable audit not claimed.
- Native host bundle: generated `bin/mirrorea-alpha`, `packages/`, `devtools/`, `reports/`, `manifest.json`, `launch.json`, `provenance.json`, `run.sh`, `README.md`.
- Bundle run script: `run.sh check` accepted; `run.sh view` validated non-final viewer.
- Direct textual `.mir` product check/build-native-bundle: both returned `direct_mir_non_goal`.
- Product `demo`: accepted; product alpha release-candidate ready true; Docker included; final product not claimed; 16 reports.
- Product release check: accepted; product alpha release-candidate ready true; Docker included; 29 passed commands; failed commands empty.
- Focused Cargo tests:
  - `product_alpha1_package_schema`: 9 passed.
  - `product_alpha1_session`: 14 passed.
  - `product_alpha1_transport_devtools`: 3 passed.
  - `alpha_cli`: 19 passed.
  - `mirrorea-core` carriers: 12 passed.
  - `mirrorea-core` runtime_substrate: 16 passed.

## What changed in understanding

- 前回「まだ実装が必要」と整理した typed host-I/O / same-session runtime / hot-plug / devtools / product CLI line は、bounded operational alpha と product alpha release-candidate workflow として実際に動く状態になっている。
- `Hello, Taro!` の string pipeline はまだ product alpha の reproducible path ではないが、`AddOne` pipeline は実際に実行できる。
- native output は direct Mir-to-machine-code ではなく native host launch bundle として実装されている。
- Docker Compose TCP path は今回の環境では実行でき、accepted だった。
- vector clock 実装は見当たらず、current implemented path は event DAG + membership epoch/incarnation で freshness を扱っている。
- membership leave は `mark_inactive` で epoch/incarnation を進めるが、rejoin semantics / compaction は未完。

## Open questions

- product alpha に `EchoText("Taro") -> "Hello, Taro!"` を追加するか。
- per-Place / per-node binary split をどの alpha gate で扱うか。
- accepted detach execution をいつ product workflow に入れるか。
- Docker two-node から network-wide patch rollout / distributed activation ordering へ進める最小 scope。
- membership rejoin / tombstone compaction / long-running session retention policy。
- model-check / proof side line のどの obligation を Lean / Isabelle / other prover に外出しするか。

## Suggested next prompt

`FAQ 016 を踏まえて、product alpha の typed host-I/O に EchoText("Taro") -> "Hello, Taro!" を追加する最小実装計画を立て、必要な spec / sample / CLI / test / devtools 変更範囲を分けてください。`

## Plan update status

plan/ 更新不要。今回は既存 roadmap / repository memory の読み直しと FAQ 追記であり、roadmap 内容や long-term repository memory の変更は行っていない。

## Documentation.md update status

Documentation.md 更新不要。既存の product alpha / operational alpha status を変更する新規実装は行っていない。

## progress.md update status

progress.md 更新不要。現行 status snapshot の読みを変更する作業ではなく、既存到達点の再確認と FAQ 化である。

## tasks.md update status

tasks.md 更新不要。新しい task package の採用・完了・blocker 入れ替えは行っていない。

## samples_progress.md update status

samples_progress.md 更新不要。sample / runner / validation command の taxonomy は変更していない。

## Reviewer findings and follow-up

No sub-agent reviewer was used. Local diff inspection and validation were used because the task is documentation/FAQ explanation only and no implementation behavior was changed.

Follow-up:

- If `EchoText` is implemented next, use a normal implementation task with tests and update samples/docs/report accordingly.
- If binary split / placement is addressed next, create a separate design + implementation scope; do not fold it into this FAQ branch.

## Skipped validations and reasons

- Full `cargo test --workspace` was not run; focused product alpha and mirrorea-core tests plus release check were run because the task is explanatory and the product release-check already exercises the relevant command family.
- No production WAN / federation validation was run because the repo explicitly does not implement that path.
- No distributed durable save/load R3/R4 validation was run because the repo explicitly marks it as non-goal / later.
- No final public API / ABI validation was run because those are not frozen.

## Commit / push status

Pending at report creation. This report and `tmp_faq/faq_016.md` should be committed and pushed after final validation.

## Sub-agent session close status

No sub-agents were spawned.
