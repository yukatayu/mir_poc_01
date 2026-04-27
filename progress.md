# progress

最終更新: 2026-04-27 18:10 JST

## この文書について

- この文書は repo 全体の **rough progress snapshot** です。
- 規範判断の正本は `specs/`、長期参照は `plan/`、runnable sample dashboard は `samples_progress.md`、詳細証跡は `docs/reports/` にあります。
- 進捗率は **repo-local alpha-ready current layer**、**Mirrorea future-axis docs-first integration**、**phase/sample/progress/storage foundation** に scoped した rough estimate であり、final public completion を意味しません。

## project axis

```text
正しい理論に基づき、
正しく hot-plug でき、
Place をまたいで実行・通信・検証・可視化できる
仮想空間システムを作る。
```

## current snapshot

- execution lane:
  `Macro 4` は active clean near-end suite と Sugoroku world vertical slice を中心に runnable
- theory / proof lane:
  `Macro 5` は finite-index typing / order-handoff / model-check second line / Lean foundation の repo-local alpha floor に到達
- sample/progress/storage lane:
  `samples_progress.md` は phase 0〜16 matrix、Sugoroku per-sample alignment、avatar skeleton line、mounted external workdir row を current dashboard として持つ
- repository-structure lane:
  `plan/19-repository-map-and-taxonomy.md`、`samples/README.md`、`scripts/README.md`、`docs/research_abstract/repository_layer_structure_01.md` が current layer-aware repo map と staged migration 読みを与える
- Mirrorea future lane:
  `TermSignature registry / debug output`、`LayerSignature system`、`MessageEnvelope / Auth seam`、`VisualizationProtocol`、`Typed external boundary / adapter` docs-first sample plan を close し、next semantic carrier package を projection / placement と読む
- reserve integration lane:
  real transport、final public contract、packaging、final public auth / visualization / projection / hot-plug surface は still later

## 現在の一言での読み

2026-04-27 時点の repo は、
**clean near-end current layer と Sugoroku world vertical slice を runnable floor として維持しつつ、
phase/sample/progress/storage foundation を runnable dashboard に押し上げ、
next representative slice 候補として avatar fairy follow skeleton family まで切り出した**
段階です。

ただし、次はまだ終わっていません。

- final public parser grammar
- final public parser / checker / runtime / verifier API
- final public theorem / model-check / witness-provider contract
- final public auth / visualization / projection / hot-plug API
- real network / multi-server consensus / durable distributed commit
- packaging / installed binary / FFI / engine adapter

## 3 軸 progress

| layer / track | 論理仕様 | ユーザ向け仕様 | 実装 / 運用 | 現在の読み |
|---|---:|---:|---:|---|
| Mir core / current-L2 | 93% | 90% | 81% | finite-index current layer は強いが、public parser / API は未完 |
| order / handoff / cut family | 92% | 95% | 80% | high-level relation line は動くが、public wording と contract は残る |
| theorem / model-check boundary | 95% | 94% | 89% | repo-local bridge は強いが、production binding は未完 |
| Lean foundations / proof spine | 89% | 92% | 69% | small proof fragment はあるが、full discharge ではない |
| Sugoroku world / current shared-space sample | 84% | 86% | 70% | runnable vertical slice と per-sample dashboard は揃ったが、real transport / durable commit は未着手 |
| samples progress dashboard | 84% | 90% | 69% | phase 0〜16 matrix に加え、Sugoroku per-sample row と avatar skeleton line まで reader-facing に整理した |
| storage / external workdir guardrail | 66% | 76% | 64% | mount と `target/` cutover は入ったが、cargo registry cache / LLVM actual probe はまだ |
| Mirrorea future axis | 69% | 80% | 42% | queue は維持したまま phase 9 docs-first sample ladder まで通し、next は projection / placement |

## macro phase map

| Macro phase | 主眼 | 現在位置 | rough progress % | 自走可否 |
|---|---|---:|---:|---|
| `Macro 0` | repository memory / docs / traceability | samples dashboard + reports + storage discipline active | 98% | 着手可能 |
| `Macro 1` | semantic kernel / invariant stabilization | late | 87% | 着手可能 |
| `Macro 2` | parser-free validation substrate | late | 88% | 着手可能 |
| `Macro 3` | compile-ready minimal actualization | late | 84% | public parser / API 以外は着手可能 |
| `Macro 4` | executable sample floor | active clean suite | 98% | 着手可能 |
| `Macro 5` | typed / theorem / model-check bridge | repo-local alpha-ready current layer | 97% | public seam 以外は自走可能 |
| `Macro 6` | shared-space / fabric boundary | shared-space helper/report-local carrier を順に actualize 中 | 91% | production transport / final catalog 以外は自走可能 |
| `Macro 7` | toolchain / backend / host integration | mixed | 66% | mounted workdir と target cutover までは自走済み |
| `Macro 8` | application realization | early | 22% | 要仕様確認 |

## feature family snapshot

| feature family | 現在地 | できていること | まだ残ること |
|---|---|---|---|
| executable sample corpus | `S6` | active clean suite 16 本、family run、matrix、closeout | final public sample catalog |
| finite-index typing | `S6` | authority / label / capture / region / cost finite theory | final typed source principal と public checker contract |
| order / handoff | `S6` | witness / publication / handoff relation family、negative sample も整備 | final source wording と public artifact contract |
| model-check second line | `S5-S6` | Peterson SC pass、relaxed CE、broken mutex CE | concrete external tool binding |
| Lean foundations / proof spine | `S6` | small actual proof fragment、generated stub corpus | full domain discharge と public proof contract |
| Sugoroku world / current shared-space sample | `S6` | runtime attach / membership / witness / handoff / leave / reset model-check が runnable、per-sample debug reading も整理済み | detach lifecycle、real transport、durable evidence |
| samples progress dashboard | `S4-S5` | phase 0〜16 matrix、Sugoroku per-sample alignment、avatar skeleton row、recent validation、storage row | future rows の helper actualization、phase 9 以降の sample ladder |
| repository structure / taxonomy | `S3-S4` | layer-aware repo map、sample/script taxonomy docs、no-risk-first staged migration plan | risky crate/path move をしないまま staged physical migration 条件を固める |
| storage / detachable workdir guardrail | `S4` | audit、env script、detach prepare、cleanup script、mounted workdir、`target/` cutover | cargo registry cache relocation、LLVM probe、broader generated-artifact policy |
| typed external effect / adapter boundary | `S3` | core-free I/O principle と phase 9 `EXT-01..05` sample ladder、provider boundary / local queue / typed failure / debug label restriction anchor を docs に固定 | dedicated adapter helper / carrier actualization |
| layer composition / auth / visualization / projection / hot-plug | `S3-S4` | package queue、stop line、phase 8 representative skeleton family、`TermSignature` / `LayerSignature` / `MessageEnvelope` / `VisualizationProtocol` first cut、`--debug signatures` / `--debug layers` / `--debug envelopes` / `--debug visualization` を actualize | typed external boundary、projection、AttachPoint actualization |

## 着手可能 / 要仕様確認 / 後段依存

| 項目 | 状態 | 理由 |
|---|---|---|
| `Projection / placement plan` | 着手可能 | docs-first validity line を先に切れる |
| `Typed external boundary / adapter` widening | 後段依存 | final host schema、browser/network/VR adapter 実装、public adapter API は mixed gate に残す |
| `VisualizationProtocol` widening | 後段依存 | final public viewer contract、retention、multi-tenant telemetry は後段 mixed gate に残す |
| `HotPlug Patch / AttachPoint` | 後段依存 | signature / layer / projection line の下敷きを使った方が手戻りが少ない |
| `Network transport` | 後段依存 | envelope / auth seam / projection なしで進めると drift しやすい |
| final public auth / visualization / hot-plug API | 要仕様確認 | public contract と retention / ecosystem target が未定 |
| packaging / FFI / broader application target | 要仕様確認 | distribution target と acceptance criteria が未定 |

## 再現性アンカー

- docs / hierarchy:
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
- active suite smoke:
  `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  `python3 scripts/clean_near_end_samples.py smoke-all --format json`
- active suite closeout:
  `python3 scripts/current_l2_guided_samples.py closeout --format json`
- family run:
  `python3 scripts/clean_near_end_samples.py run typing --format json`
  `python3 scripts/clean_near_end_samples.py run order-handoff --format json`
  `python3 scripts/clean_near_end_samples.py run model-check --format json`
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json`
  `python3 scripts/clean_near_end_samples.py run modal --format json`
- Lean sync:
  `python3 scripts/current_l2_lean_sample_sync.py`
- Sugoroku world vertical slice:
  `python3 scripts/sugoroku_world_samples.py check-all`
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary --format json`
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug signatures`
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes`
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug layers`
  `python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership`
  `python3 scripts/sugoroku_world_samples.py run 08_reset_interleaving_model_check --debug verification`
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
  `python3 -m unittest scripts.tests.test_sugoroku_world_samples`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
  `cargo test -p mir-runtime --test clean_near_end_samples`
- diff hygiene:
  `git diff --check`
- storage:
  `bash scripts/storage/setup_mirrorea_workdisk_root.sh --plan`
  `bash scripts/env/mirrorea_storage_env.sh`
  `bash scripts/storage/detach_prepare.sh`
  `bash scripts/storage/cleanup_disposable_artifacts.sh --list`

## recent log

- 2026-04-22 23:53 JST — clean near-end active suite を `samples/clean-near-end/` に切り替え、finite-index typing・order/handoff・model-check second line・Lean sync・crate test・docs validation を再実行して repo-local alpha current layer を同期した。
- 2026-04-23 00:07 JST — stale reference を active lane から除去し、guided sample / clean suite / Lean sync / docs validation を fresh 実行して alpha-ready 判定の evidence を固定した。
- 2026-04-23 14:32 JST — Sugoroku world runtime attachment vertical slice を追加し、single-process logical multi-place emulator、10 sample、CLI、model-check、hands-on docs を実装・検証した。
- 2026-04-23 22:54 JST — `mir_hilight.html` を repo root に追加し、active `.mir` sample 全体を single-file browser viewer で読めるようにした。
- 2026-04-27 09:13 JST — Mirrorea future-axis handoff を docs / specs / plan / AGENTS / progress / tasks に統合し、package 1 audit と package 2 reporting discipline を close したうえで、docs validation、clean near-end smoke / closeout、Sugoroku closeout、crate tests を通し、next queue を `TermSignature registry` に昇格させた。
- 2026-04-27 10:14 JST — phase 0〜16 runnable sample dashboard `samples_progress.md`、source hierarchy check、storage env / detach-safe cleanup script、current-L2 guided smoke / closeout、Sugoroku `check-all` / closeout を同期し、current near-term reading を `Sugoroku sample progress alignment` -> `Avatar fairy follow sample plan` -> `TermSignature registry / debug output` へ組み替えた。
- 2026-04-27 13:20 JST — extra SSD `/dev/vdb1` を `/mnt/mirrorea-work` に mounted し、UUID-based `fstab` 永続化、`target/` SSD cutover、`cargo test -p mir-ast --no-run` を確認した。
- 2026-04-27 15:21 JST — Sugoroku per-sample matrix と debug reading を `samples_progress.md` / sample README / hands-on doc に反映し、avatar fairy follow phase 8 skeleton family を `samples/not_implemented/avatar-fairy-follow/` と reader-facing doc に切り出し、`check-all` / focused debug run / docs validation を通して next promoted package を `TermSignature registry / debug output` に確定した。
- 2026-04-27 15:39 JST — Sugoroku helper に `term_signatures` と `--debug signatures` を追加し、clean near-end Rust report / closeout に `TermSignature` inventory と reserved signature kinds を actualize したうえで、Python unittest、`cargo test -p mir-runtime --test clean_near_end_samples`、`python3 scripts/validate_docs.py`、`git diff --check` を通して next promoted package を `LayerSignature system` へ進めた。
- 2026-04-27 15:51 JST — `TermSignature registry / debug output` の reader-facing docs / specs / plan / report を同期し、`current_l2_guided_samples.py smoke-all --format json`、Sugoroku `--debug signatures`、`cargo test -p mir-runtime`、`check_source_hierarchy.py`、`validate_docs.py` を fresh 実行して package close evidence を固めた。
- 2026-04-27 15:59 JST — reviewer sub-agent の指摘に従って Sugoroku signature provenance、Rust closeout の source-read failure mode、`hands_on_sugoroku_detail.md` の snippet drift を修正し、Python unittest、`cargo test -p mir-runtime --test clean_near_end_samples`、`current_l2_guided_samples.py smoke-all --format json`、`cargo test -p mir-runtime`、`check_source_hierarchy.py`、`validate_docs.py`、`git diff --check` を再実行して package close evidence を更新した。
- 2026-04-27 17:03 JST — repo layer map / staged restructuring task で `plan/19`、`samples/README.md`、`scripts/README.md`、`samples_progress.md` required format、`docs/reports/0920-*` を同期し、`check_source_hierarchy.py`、`validate_docs.py`、full runtime / sample validation suite、reviewer re-check を通し、missing helper は `scripts/avatar_follow_samples.py` の未実装 1 件だけと確認した。
- 2026-04-27 17:46 JST — pre-existing helper/runtime `MessageEnvelope / Auth seam` first cut を docs / spec / plan / snapshot / report へ同期し、Sugoroku `--debug envelopes`、Sugoroku `check-all`、Python unittest、runtime sample / closeout、`cargo test -p mir-runtime --test clean_near_end_samples`、`cargo test -p mir-runtime`、`check_source_hierarchy.py`、`validate_docs.py`、`git diff --check` を通して next promoted package を `VisualizationProtocol` へ進めた。
- 2026-04-27 18:04 JST — `VisualizationProtocol` first cut として Sugoroku helper の `visualization_views` / `telemetry_rows` / `--debug visualization`、clean near-end report / closeout の `VisualizationView` / `TelemetryRow` inventory、hands-on / plan / specs / snapshot を同期し、Python unittest、Sugoroku `check-all` / `--debug visualization` / closeout、runtime closeout、`cargo test -p mir-runtime --test clean_near_end_samples`、`cargo test -p mir-runtime`、`check_source_hierarchy.py`、`validate_docs.py`、`git diff --check` を通して next promoted package を typed external boundary / adapter へ進めた。
- 2026-04-27 18:10 JST — `Typed external boundary / adapter` docs-first sample plan として phase 9 `EXT-01..05` planned family を `samples/not_implemented/typed-external-boundary/` に追加し、provider boundary / local queue / typed failure / debug label restriction の evidence anchor、validation、stop line を docs / plan / snapshot / report へ同期し、`check_source_hierarchy.py`、`validate_docs.py`、runtime `05_delegated_rng_service`、Sugoroku `--debug envelopes`、`git diff --check` を通して next promoted package を projection / placement に進めた。
