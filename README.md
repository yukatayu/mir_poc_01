# Mir / Mirrorea / PrismCascade / Typed-Effect Wiring Platform

この repository は、4 系統を分離可能なまま扱う **specification-first research repo** です。

- **Mir**
  因果、effect、ownership、lifetime、contract、安全な進化を扱う意味論コア
- **Mirrorea**
  logical naming、routing、overlay insertion、audit、dynamic reconfiguration を扱う fabric/runtime 層
- **PrismCascade**
  media domain の独立 kernel
- **Typed-Effect Wiring Platform**
  inspectable / routable な effect integration 層

2026-04-23 時点で repo が主として検証しているのは、Mir current-L2 の **repo-local alpha-ready current layer** です。
これは final public product ではありませんが、docs-only の構想メモでもありません。active sample、helper CLI、Lean foundations、report 群を通して、現時点でどこまで実装と検証が進んでいるかを repo 内で再確認できます。

## 現在の到達点

- active sample suite は `samples/clean-near-end/`
- runnable sample dashboard は `samples_progress.md`
- first strong typing layer は **finite decidable index fragment**
- authority hierarchy / security label hierarchy / capture / region / cost は **user-defined finite theory**
- order / handoff は `publication_order`、`witness_order`、`scoped_happens_before` などの高水準関係で扱う
- mutex / weak-memory / broken mutex は **model-check second line**
- Sugoroku world vertical slice は `samples/clean-near-end/sugoroku-world/`
  と `scripts/sugoroku_world_samples.py` で repo-local に実行可能
- Lean 側は
  - `samples/lean/foundations/` の小さな実証明
  - `samples/lean/clean-near-end/` の generated theorem stub
  に分かれている

## 明示的にまだ完了していないもの

- final public parser grammar
- final public parser / checker / runtime / verifier API
- full dependent type theory
- concrete theorem prover / model-checker への production binding
- low-level `memory_order_*` を source principal syntax としてどう公開するか
- final public witness / provider / emitted-artifact contract
- packaging / installed binary / FFI / engine adapter

## Mirrorea の次軸

current repo の次の promoted line は、Mirrorea future-axis の docs-first / repo-local integration です。

- 主軸は
  **正しい理論に基づき、正しく hot-plug でき、Place をまたいで実行・通信・検証・可視化できる仮想空間システム**
  を崩さないことにあります。
- standard I/O は Mir core primitive ではなく、external world とは typed effect adapter で接続する方向です。
- authentication は transport そのものに埋め込まず、authorization / membership / capability / witness と分けて扱います。
- visualization と telemetry も情報を外へ出す effect なので、label / authority / redaction を持つ typed layer として扱います。
- current representative sample は Sugoroku world runtime attachment vertical slice と avatar fairy follow representative slice です。
  avatar follow の active subset は `samples/clean-near-end/avatar-follow/` にあり、`FAIRY-02` / `FAIRY-05` だけが `samples/not_implemented/avatar-fairy-follow/` に residual planned family として残っています。
- `TermSignature registry / debug output` の first cut は close してあり、Sugoroku helper の `--debug signatures` と clean near-end report / closeout inventory に helper-local / report-local carrier を追加しました。
- `LayerSignature system` の first cut も close してあり、Sugoroku helper の `--debug layers` と clean near-end report / closeout の `layer_signatures` inventory を追加しました。
- `MessageEnvelope / Auth seam` の first cut も close してあり、Sugoroku helper の `message_envelopes` / `--debug envelopes` と clean near-end report / closeout の `MessageEnvelope` inventory で、transport / auth / membership / capability / witness split を helper-local / report-local に actualize しました。
  current baseline は `auth none` で、`session_token` / `signature` は reserve です。
- `VisualizationProtocol` の first cut も close してあり、Sugoroku helper の `visualization_views` / `telemetry_rows` / `--debug visualization` と clean near-end report / closeout の report-local `VisualizationView` / `TelemetryRow` inventory で、label / authority / redaction を持つ typed visualization / telemetry line を actualize しました。
- `Typed external boundary / adapter` の docs-first sample plan も close してあり、phase 9 planned family `EXT-01..05` を `samples/not_implemented/typed-external-boundary/` に置き、provider boundary / local queue / typed failure / debug label restriction を current evidence anchor に結び直しました。
- `Projection / placement` の docs-first plan も close してあり、`plan/20-projection-and-placement-roadmap.md` で system-wide source と place-specific program の validity checklist を固定しました。
- `HotPlug Patch / AttachPoint` の executable widening も current line に入り、`scripts/sugoroku_world_samples.py` の `hotplug_lifecycle` / `--debug hotplug` / `hot-plug` layer inventory で compatibility / activation / post-detach rejection evidence を helper-local に確認できます。
- `Network transport` の docs-first plan も close してあり、`plan/22-network-transport-roadmap.md` で local queue / provider boundary current anchor、loopback / reconnect / failure matrix、stop line を固定しました。さらに `NET-01` helper-local loopback preview も actualize し、`--transport loopback_socket` で same-process envelope parity を確認できます。
- `Compiler/backend/LLVM preparation` guardrail も close してあり、`plan/23-compiler-backend-llvm-guardrail-roadmap.md` で external workdir、`CARGO_HOME`、LLVM path、cleanup safety、stop line を固定しました。
- `hands-on docs / closeout` も close してあり、`NET-01` helper-local loopback preview、avatar representative slice、hot-plug helper-local lifecycle canary まで actualize しました。current next queue は transport `NET-02..05`、その後 avatar residual widening です。
- phase 0〜16 の runnable sample / E2E / debug / storage 状態は `samples_progress.md` にまとめ、progress% は validation と report に基づいて更新します。
- repo の layer-aware structure と staged migration plan は `plan/19-repository-map-and-taxonomy.md` にまとめています。sample taxonomy は `samples/README.md`、script taxonomy は `scripts/README.md` を参照してください。
- current closeout を実行コマンド付きで読む最短入口は `docs/hands_on/current_phase_closeout_01.md` です。

reader-facing な要約は `docs/research_abstract/mirrorea_future_axis_01.md` を参照してください。これは roadmap summary であり、規範正本ではありません。

## 何が built-in で、何が user-defined か

current clean near-end layer では、次を built-in vocabulary として扱います。

- `module`
- `index`
- `policy`
- `principal`
- `resource`
- `effect`
- `place`
- `option`
- `chain`
- `fallback`
- `lineage`
- `perform`
- `via`
- `require`
- `ensure`
- `atomic_cut`
- `transition`
- `stage`
- `publish`
- `observe`
- `handoff`
- `witness`
- `model`
- `property`

一方で、次のような domain vocabulary は built-in ではありません。

- `SecurityLabel`
- `FingerprintAuthority`
- `CaptureScope`
- `Region`
- `CostBudget`
- `FingerprintReleasePolicy`
- `Public`
- `KeyMaterial`
- `Observer`
- `Releaser`
- `Admin`
- `RoomHistory`
- `EphemeralToken`

つまり、旧来の権限専用 predicate 名を magical builtin として言語が暗黙に持つのではなく、sample 側が有限理論として宣言し、その上で checker / helper が読む構成です。

## まず実行するコマンド

active clean near-end suite の確認:

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/current_l2_guided_samples.py smoke-all --format json
python3 scripts/current_l2_guided_samples.py closeout --format json
```

family ごとの確認:

```bash
python3 scripts/clean_near_end_samples.py run typing --format json
python3 scripts/clean_near_end_samples.py run order-handoff --format json
python3 scripts/clean_near_end_samples.py run model-check --format json
python3 scripts/clean_near_end_samples.py run modal --format json
python3 scripts/clean_near_end_samples.py matrix --format json
```

Sugoroku world runtime attachment vertical slice:

```bash
python3 scripts/sugoroku_world_samples.py list
python3 scripts/sugoroku_world_samples.py check-all
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug signatures
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes
python3 scripts/sugoroku_world_samples.py model-check
python3 scripts/sugoroku_world_samples.py closeout --format json
```

sample code viewer:

```text
mir_hilight.html
```

`mir_hilight.html` は repo 直下の単一 HTML です。ブラウザで開くと
`samples/clean-near-end/**/*.mir` の current active sample を Solarized Dark
標準で表示し、行番号、スマホ対応、theme 切替、予約語と sample 内定義名の
別色 highlight を確認できます。custom source panel に任意の Mir 風コードを貼ると、
同じ highlighter で browser-local preview できます。CSS は外部 framework ではなく
HTML 内の hand-written original CSS です。final parser / checker ではなく readable viewer
です。文法、active sample path、reserved keyword、定義宣言形、custom input UI が変わった場合は、
HTML 内の embedded samples / syntax token list / symbol extraction rule と docs
を同じ task で更新してください。

Lean foundations と generated stub の同期:

```bash
python3 scripts/current_l2_lean_sample_sync.py
```

## 読み始める順序

1. `AGENTS.md`
2. `Documentation.md`
3. `progress.md`
4. `tasks.md`
5. `specs/00-document-map.md`
6. `specs/01-charter-and-decision-levels.md`
7. `specs/02-system-overview.md`
8. `specs/03-layer-model.md`
9. `specs/09-invariants-and-constraints.md`
10. 必要な subsystem spec と `plan/00-index.md`

## いま参照すべき docs

- `Documentation.md`
  現在の repo を短く読むための入口
- `progress.md`
  現在地、rough progress、recent log
- `tasks.md`
  自走可能な package と mixed gate / user-spec gate の整理
- `samples_progress.md`
  phase / layer ごとの runnable sample、E2E、debug surface、build / storage 環境の dashboard
- `samples/README.md`
  active / base corpus / planned / prototype / archive / generated sample の置き場所
- `scripts/README.md`
  active runner、repo-local helper、detached loop、storage/env script の current taxonomy
- `docs/research_abstract/README.md`
  日本語での短い研究概要と `_detail` への導線
- `docs/hands_on/README.md`
  実行コマンド付きの hands-on landing page
- `docs/hands_on/current_phase_closeout_01.md`
  current phase closeout、remaining mixed gate、next queue
- `docs/reports/`
  実行証跡と変更履歴

## active path と archive path

- active sample:
  `samples/clean-near-end/`
- active Sugoroku world vertical slice:
  `samples/clean-near-end/sugoroku-world/`
- active Lean material:
  `samples/lean/`
- historical archive:
  `samples/old/2026-04-22-pre-clean-near-end/`
  と
  `samples/lean/old/2026-04-22-pre-clean-near-end/`

archive は比較用の履歴です。active canonical sample としては扱いません。
