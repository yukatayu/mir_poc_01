# tasks

最終更新: 2026-04-23 08:57 JST

## この文書について

- この文書は repo 全体の **current task map** です。
- 規範判断の正本は `specs/`、長期比較は `plan/`、詳細証跡は `docs/reports/` に置きます。
- append-only の履歴ではなく、現況に合わせて毎回全体を書き直す snapshot として扱います。

## current status at task level

- active clean near-end suite は runnable
- first strong typing layer は finite-index fragment として runnable
- order / handoff は high-level relation family として runnable
- mutex / weak-memory line は model-check second line として runnable
- Lean foundations と generated clean stub corpus は runnable
- 現在の主な残課題は **public-seam residual** と **true user-spec residual** の切り分けです

## current executable floor

### Active clean near-end suite

- typing:
  `01_authorized_declassification`
  `02_unauthorized_declassification_rejected`
  `03_label_flow_rejected`
  `04_capture_escape_rejected`
  `05_cost_bound_rejected`
- order / handoff:
  `01_authorized_roll_publish_handoff`
  `02_missing_witness_rejected`
  `03_handoff_before_publication_rejected`
  `04_stage_block_authorized_handoff`
  `05_delegated_rng_service`
  `06_auditable_authority_witness`
- model-check:
  `01_peterson_sc_pass`
  `02_peterson_relaxed_counterexample`
  `03_broken_mutex_counterexample`
- modal:
  `01_stage_stable_later_minimal`
  `02_published_witnessed_mode_bridge`

### Helper entrypoints

- `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
- `python3 scripts/current_l2_guided_samples.py closeout --format json`
- `python3 scripts/clean_near_end_samples.py run typing --format json`
- `python3 scripts/clean_near_end_samples.py run order-handoff --format json`
- `python3 scripts/clean_near_end_samples.py run model-check --format json`
- `python3 scripts/clean_near_end_samples.py run modal --format json`
- `python3 scripts/clean_near_end_samples.py matrix --format json`
- `python3 scripts/current_l2_lean_sample_sync.py`

## 自走可能な task package

### 1. public parser / checker / runtime seam の narrowing

- phase:
  `Macro 3` / `Macro 7` mixed gate
- 目的:
  repo-local helper surface と final public surface の境界を、実装と docs の両方でさらに明確にする
- 何に効くか:
  alpha-ready と final public completion の誤読を減らせる
- rough estimate:
  中
- current recommendation:
  active clean suite を維持しつつ、public parser/API 以外の residual を先に整理する

### 2. Problem 1 public-seam residual の整理

- phase:
  `Macro 5` mixed gate
- 目的:
  finite-index typing、theorem handoff、model-check boundary を final public contract 手前まで整える
- 何に効くか:
  checker payload / theorem result / verifier handoff の境界がさらに明確になる
- rough estimate:
  中
- current recommendation:
  full dependent type や production prover binding に踏み込まず、repo-local bridge の contract を狭める

### 3. Problem 2 public-shape residual の整理

- phase:
  `Macro 6` mixed gate
- 目的:
  `memory_order` 再解釈 line、witness / provider / emitted-artifact の public shape residual を整理する
- 何に効くか:
  high-level relation family を source principal に保ちつつ、low-level backend family の位置づけを明確にできる
- rough estimate:
  中
- current recommendation:
  `memory_order_*` exact source syntax を導入せず、relation-first line を維持したまま boundary を狭める

### 4. Lean proof hardening

- phase:
  `Macro 5` maintenance / mixed gate
- 目的:
  foundations と generated stub の役割差を保ったまま、小さな actual proof fragment を増やす
- 何に効くか:
  theorem side の mechanization evidence を強められる
- rough estimate:
  小〜中
- current recommendation:
  generated stub 全面解消を一気に狙わず、foundation 側の再利用可能 lemma を増やす

### 5. docs / traceability maintenance

- phase:
  `Macro 0` maintenance
- 目的:
  README / Documentation / progress / tasks / research_abstract / report 間の drift を抑える
- 何に効くか:
  agent / human が current state を誤読しにくくなる
- rough estimate:
  小
- current recommendation:
  active path と archive path を常に対で書き、old line を current line と混ぜない

## research を通して見つけること

### low-level `memory_order` family の扱い

- 概要:
  low-level memory order family を source principal に上げるか、それとも backend/reference family に留めるか
- 何に影響するか:
  order / handoff wording、runtime policy、artifact contract、model-check explanation
- 主要な選択肢:
  - high-level relation family を principal のまま維持する
  - low-level exact surface を別 layer として reopen する
- current recommendation:
  今は relation-first line を維持する

### concrete theorem / model-check tool binding

- 概要:
  repo-local bridge を concrete public tool binding に進めるか
- 何に影響するか:
  verifier handoff、artifact schema、public contract、CI
- 主要な選択肢:
  - Lean-first / tool-neutral bridge を維持する
  - concrete tool brand と public checker surface を導入する
- current recommendation:
  まだ bridge / reserve floor に留める

## user が決める必要があること

### packaging / installed binary / FFI

- 概要:
  repo-local helper floor を配布可能 surface に進めるか
- 何に影響するか:
  Macro 7、artifact retention、host-facing contract、CI
- 主要な選択肢:
  - repo-local helper floor を維持する
  - installed binary / packaging を first-class target にする
  - FFI / engine adapter を先に設計する
- current recommendation:
  まだ repo-local floor を維持する

### broader application target

- 概要:
  次の concrete target を authoritative-room 延長、Typed-Effect integration、Prism / 上位アプリのどこに置くか
- 何に影響するか:
  Macro 8、acceptance criteria、non-functional requirement
- 主要な選択肢:
  - authoritative-room extension
  - Typed-Effect / host integration
  - Prism / higher application
- current recommendation:
  目的と保証範囲を user 側で先に固定したい
