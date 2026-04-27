# plan/01 — 現況サマリ

## repo 全体の主眼

- 主眼は、Mir current-L2 の repo-local alpha-ready current layer を保ちながら、Mirrorea future-axis を sample / progress / storage discipline と結びつけて前進させることにある
- current active sample suite は `samples/clean-near-end/`
- Sugoroku world vertical slice は `samples/clean-near-end/sugoroku-world/` と `scripts/sugoroku_world_samples.py` で runnable
- `samples_progress.md` は phase 0〜16 の runnable sample dashboard として current snapshot に入る
- `crates/mirrorea-core` と `crates/mirrorea-control` は subsystem boundary を明示する placeholder skeleton であり、production logic はまだ入っていない
- Mirrorea / PrismCascade / Typed-Effect Wiring Platform は separable track として扱う

## current executable floor

- `samples/clean-near-end/` active suite 16 本は runnable
- `samples/clean-near-end/sugoroku-world/` vertical slice 10 本は runnable
- `crates/mir-runtime/src/clean_near_end.rs` が finite-index typing / order-handoff / model-check / modal current layer を持つ
- `scripts/sugoroku_world_samples.py` が logical multi-place runtime attachment emulator を持つ
- `scripts/check_source_hierarchy.py` と `scripts/validate_docs.py` が repository memory / report / dashboard 側の baseline check になる

## current storage audit snapshot

- root disk:
  `/dev/vda2` 99G 中 32G free
- repo size:
  `90M`
- `target/`:
  repo path は `/mnt/mirrorea-work/cargo-target` への symlink に切り替え済み
  - external usage:
    `5.3G`
- cargo registry cache:
  `/mnt/mirrorea-work/cargo-registry-cache`
  - current probe:
    `CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run`
- `.git/`:
  `69M`
- extra storage:
  `/dev/vdb1` ext4 `mirrorea-work` が `/mnt/mirrorea-work` に mounted
  - UUID:
    `a87650a8-e3e9-4977-8940-6c293a0ee23c`
  - `/etc/fstab`:
    UUID-based `defaults,nofail`
  - current active cutover:
    `target/` は SSD 側へ移送済み
  - LLVM path readiness:
    `/mnt/mirrorea-work/llvm/{src,build,install}` は作成済み、actual artifact はまだない

## twin peaks の current state

### Problem 1

- current first line:
  finite-index first strong typing layer、Lean-first proof skeleton、model-check second-line handoff
- active sample:
  clean typing 5 本
- repo-local evidence:
  `run typing`
  `matrix`
  `closeout`
  `samples/lean/foundations/`
  `samples/lean/clean-near-end/`
- still later:
  final typed source principal、final theorem result object、final public checker / verifier contract

### Problem 2

- current first line:
  order / handoff relation decomposition、witness / publication discipline、model-check second-line split
- active sample:
  clean order-handoff 6 本 + model-check 3 本
- repo-local evidence:
  `run order-handoff`
  `run model-check`
  `closeout`
- still later:
  final source wording、final emitted-artifact / public contract、exhaustive shared-space catalog

### Sugoroku world / Mirrorea vertical slice

- current first line:
  empty world server、runtime attach、membership epoch / incarnation、publish / witness / handoff、late join、leave、owner leave、reset model-check
- active sample:
  `samples/clean-near-end/sugoroku-world/00...09`
- repo-local evidence:
  `python3 scripts/sugoroku_world_samples.py check-all`
  `python3 scripts/sugoroku_world_samples.py model-check`
  `python3 scripts/sugoroku_world_samples.py closeout`
- still later:
  real network、multi-server consensus、durable distributed commit、detach lifecycle implementation、final public API

### phase / sample / storage foundation lane

- `samples_progress.md` を current runnable dashboard として追加した
- `scripts/env/mirrorea_storage_env.sh`、`scripts/storage/detach_prepare.sh`、`scripts/storage/cleanup_disposable_artifacts.sh` を small-VPS safe default で置く
- `docs/reports/0913-*` で phase-sample-progress-storage-foundation package を close する
- `docs/reports/0915-*` で `/mnt/mirrorea-work` mount verification、`target/` SSD cutover、`cargo test -p mir-ast --no-run` の軽量確認を追加した
- `docs/reports/0916-*` で Sugoroku per-sample alignment と debug surface mapping を close した
- `docs/reports/0917-*` で phase 8 avatar fairy follow skeleton family と prototype boundary を close した
- `docs/reports/0918-*` で `TermSignature registry / debug output` を close し、Sugoroku `--debug signatures` と clean near-end report/closeout inventory を追加した
- `docs/reports/0919-*` で `LayerSignature system` を close し、helper-local / runtime report-local lane inventory を追加した
- `docs/reports/0921-*` で `MessageEnvelope / Auth seam` を close し、Sugoroku helper の `message_envelopes` / `--debug envelopes` と clean near-end report-local `MessageEnvelope` inventory を追加した
- `docs/reports/0922-*` で `VisualizationProtocol` を close し、Sugoroku helper の `visualization_views` / `telemetry_rows` / `--debug visualization` と clean near-end report-local `VisualizationView` / `TelemetryRow` inventory を追加した
- `docs/reports/0923-*` で `Typed external boundary / adapter` docs-first sample plan を close し、phase 9 planned family `EXT-01..05` を `samples/not_implemented/typed-external-boundary/` に置いた
- `docs/reports/0924-*` で `Projection / placement` docs-first plan を close し、`plan/20-projection-and-placement-roadmap.md` を追加した
- `docs/reports/0925-*` で `HotPlug Patch / AttachPoint` docs-first plan を close し、`plan/21-hotplug-attachpoint-roadmap.md` を追加した
- `docs/reports/0926-*` で `Network transport` docs-first plan を close し、`plan/22-network-transport-roadmap.md` と phase 13 planned family `samples/not_implemented/network-transport/` を追加した
- `docs/reports/0927-*` で `Compiler/backend/LLVM preparation` guardrail を close し、`plan/23-compiler-backend-llvm-guardrail-roadmap.md`、`CARGO_HOME` binding、non-destructive probe evidence を追加した
- `plan/19-repository-map-and-taxonomy.md`、`samples/README.md`、`scripts/README.md` で current repo taxonomy と staged migration plan を docs-first に固定した
- next reopen point は hands-on docs / closeout

### Mirrorea future-axis carrier lane

- project axis:
  **正しい理論に基づき、正しく hot-plug でき、Place をまたいで実行・通信・検証・可視化できる仮想空間システム**
- `docs/reports/0912-*` で package 1 current-state audit と package 2 AGENTS/reporting discipline を close した
- current reading は、sample/storage foundation と `TermSignature` / `LayerSignature` / `MessageEnvelope` / `VisualizationProtocol` first cut、phase 9 typed external boundary docs-first sample plan、phase 12 projection docs-first plan、phase 14 hot-plug docs-first plan、phase 13 network transport docs-first plan、phase 16 backend/LLVM guardrail を先に入れた上で、next promoted package を hands-on docs / closeout と読む
- phase 8 representative sample 候補は `samples/not_implemented/avatar-fairy-follow/` に skeleton family を切り出したが、current active helper はまだない
- package 6 以降は network transport plan、compiler/backend/LLVM prep guardrail、hands-on closeout
- reader-facing summary は `docs/research_abstract/mirrorea_future_axis_01.md`

## current stop line

- final public parser grammar
- final public parser / checker / runtime / verifier API
- final public theorem / model-check / witness-provider contract
- final public auth / visualization / projection / hot-plug API
- full dependent type theory
- real network / durable distributed commit / multi-server consensus
- packaging / installed binary / FFI / engine adapter

## current recommendation

- active current layer、sample progress discipline、Mirrorea future-axis carrier queue を 1 本の “実装済み” line に潰さない
- `world` は current Sugoroku sample では host/server-side sugar として読み、Mir core primitive として既成事実化しない
- authentication / authorization / membership / capability / witness / visualization / telemetry を transport や debug leak に潰さない
- `auth none` baseline を final public auth protocol と混同せず、helper-local / report-local carrier の stop line を明記する
- root disk 上の `target/` 膨張を放置せず、external workdir が使えるなら heavy disposable artifact をそこへ逃がす
- old sample archive と current active sample の区別を、README / Documentation / progress / tasks / `samples_progress.md` / research_abstract で常に対にして書く
