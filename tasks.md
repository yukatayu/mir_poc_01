# tasks

最終更新: 2026-04-27 23:26 JST

## この文書について

- この文書は repo 全体の **current task map** です。
- 規範判断の正本は `specs/`、長期比較は `plan/`、runnable sample 状態は `samples_progress.md`、詳細証跡は `docs/reports/` に置きます。
- append-only 履歴ではなく、現況に合わせて毎回全体を書き直す snapshot として扱います。

## current status at task level

- active clean near-end suite、Sugoroku world vertical slice、avatar follow representative slice は runnable です。
- Mirrorea future-axis の first-cut / widening package は、`TermSignature`、`LayerSignature`、`MessageEnvelope / Auth seam`、`VisualizationProtocol`、typed external synthetic preview helper、projection preview、hot-plug lifecycle canary、network transport helper-local canary、storage / LLVM guardrail まで close 済みです。
- current promoted next line は **Typed external boundary residual planned family review** です。
- next reopen point は **Projection / placement residual emitted-program gate** です。
- current snapshot を短く追う入口は `progress.md`、`samples_progress.md`、`docs/hands_on/current_phase_closeout_01.md` です。

## current executable floor

| lane | current floor | note |
|---|---|---|
| Mir current-L2 | `samples/current-l2/` | base corpus と current-L2 source execution を維持 |
| clean near-end suite | `samples/clean-near-end/{typing,order-handoff,model-check,modal}/` | finite-index typing、order / handoff、model-check second line、modal family が runnable |
| Sugoroku world | `samples/clean-near-end/sugoroku-world/` + `scripts/sugoroku_world_samples.py` | attach / membership / handoff / reset model-check / hot-plug helper-local lifecycle canary |
| avatar follow | `samples/clean-near-end/avatar-follow/` + `scripts/avatar_follow_samples.py` | `FAIRY-01/02/03/04/06` active、`FAIRY-05` は planned |
| typed external / transport | `scripts/typed_external_boundary_samples.py`、`scripts/network_transport_samples.py` | helper-local synthetic preview / canary。final public adapter / transport contract ではない |
| projection / placement | Sugoroku `projection_view`、runtime `cross_place_projection` | helper-local / report-local preview floor。final emitted program ではない |
| storage / backend guardrail | `/mnt/mirrorea-work`、`scripts/env/mirrorea_storage_env.sh` | root disk を既成事実化しない external workdir floor |

## 自走可能な task package

### Package 1. Typed external boundary residual planned family review

- phase / stage:
  `Macro 6`, reserve
- objective:
  `EXT-01` / `EXT-02` / `EXT-05` residual planned family と `EXT-03` / `EXT-04` synthetic preview subset を projection / visualization / host-schema pressure と照らして reopen 条件まで整理する
- expected deliverables:
  synthetic preview helper と final public host-facing gate を混同しない reopen criterion
- validation command:
  `python3 scripts/typed_external_boundary_samples.py closeout --format json`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json`
- report requirement:
  新しい report を作り、scenario label / host schema / visualization gate の分離を明記する
- stop line:
  console / overlay / viewer contract を final public API として固定しない

### Package 2. Projection / placement residual emitted-program gate

- phase / stage:
  `Macro 6`, reserve
- objective:
  helper/report-local preview floor の次に残る emitted program / optimizer / equivalence gate を docs-first に切り出す
- expected deliverables:
  projection preview と final placement artifact family を混同しない reopen criterion
- validation command:
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json`
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  `python3 scripts/validate_docs.py`
- report requirement:
  新しい report を作り、preview floor と emitted-program stop line の分離を明記する
- stop line:
  final projection IR / optimizer / equivalence checker / deployment planner は固定しない

## reserve reopen queue

- `FAIRY-05` residual reacquire widening
  docs-first reopen 条件は固定済み。explicit state timeline / anchor switch evidence が要る
- `HotPlug Patch / AttachPoint` real migration / rollback
  helper-local lifecycle canary の先で、attachpoint migration contract を別 gate に残す
- `Network transport` real socket / durable replay
  `NET-01..05` helper-local canary の先で、process 境界を超える本実装は still later
- `Macro 7` packaging / FFI / host integration
  external workdir guardrail はあるが、installed binary / engine adapter は user-spec gate

## research を通して見つけること

- `TermSignature` の最小粒度と residual obligation surface
- `LayerSignature` helper/runtime naming をどこまで shared law surface に寄せるか
- `VisualizationProtocol` の最小 view kind / telemetry row / redaction wording
- projection / placement validity report の最小 shape
- `AttachPoint` compatibility と detach lifecycle の最小 contract

## user が決める必要があること

### Blocker 1. final public contract の固定範囲

- 概要:
  auth / visualization / projection / hot-plug をどこまで public API として固定するか
- 影響:
  `Macro 6` reserve package 以降の naming と stop line
- 主要な選択肢:
  repo-local helper 優先のまま進める / early public-contract drafting を始める
- current recommendation / 見解:
  まだ repo-local helper と docs-first carrier に留め、public contract は mixed gate に残す

### Blocker 2. packaging / FFI / host integration target

- 概要:
  installed binary、FFI、engine adapter、deployment contract の最終受け皿
- 影響:
  `Macro 7` 以降の acceptance criteria
- 主要な選択肢:
  CLI / library / engine-adapter / hybrid
- current recommendation / 見解:
  current task では固定せず、backend / LLVM preparation の後に user-spec gate として reopen する

### Blocker 3. final public adapter contract / host schema scope

- 概要:
  typed external helper-local canary からどの host-facing contract を public surface へ上げるか
- 影響:
  phase 9 residual planned family、projection / visualization lane、future adapter ABI
- 主要な選択肢:
  helper-local scenario label のまま進める / host schema draft を早めに起こす
- current recommendation / 見解:
  `EXT-03` / `EXT-04` synthetic preview subset を evidence floor に保ち、final public host-facing contract は mixed gate に残す

## short validation anchors

- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `python3 scripts/current_l2_guided_samples.py closeout --format json`
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
- `python3 scripts/avatar_follow_samples.py closeout --format json`
- `python3 scripts/typed_external_boundary_samples.py closeout --format json`
- `python3 scripts/network_transport_samples.py closeout --format json`
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
