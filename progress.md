# progress

最終更新: 2026-05-05 11:17 JST

## この文書について

- この文書は repo 全体の **rough progress snapshot** です。
- 規範判断の正本は `specs/`、長期参照は `plan/`、実行証跡は `docs/reports/`、runnable sample dashboard は `samples_progress.md` です。
- 裸の `100%` は operational-layer-ready 以上を意味します。evidence line や first-floor line は `100% current-scope evidence closeout` または `100% first-floor closeout` と明示して読みます。
- 古い package 履歴の詳細は `docs/reports/` と `plan/90-source-traceability.md` を参照し、この snapshot では current checkpoint / next gate / validation floor を優先します。

## project axis

```text
正しい理論に基づき、
正しく hot-plug でき、
Place をまたいで実行・通信・検証・可視化できる
仮想空間システムを作る。
```

この軸は Mir / Mirrorea / PrismCascade / Typed-Effect Wiring Platform の separability を消すものではありません。

## current position

- latest closeout package:
  `P-A1-22` α-0.9 session-bound devtools export
- current promoted reopen point:
  α-1 product/public boundary recut or a narrow `P-A1-23` practical α-1 readiness audit
- current reading:
  repo は **theory / first-floor carriers / evidence closeout** に加えて **bounded operational α-0.5 local observable runtime**、**bounded operational α-0.8 same-session hot-plug runtime**、**bounded operational α-0.9 session-bound devtools export** を得た。ただし final public viewer / telemetry ABI、durable audit、distributed durable save/load、product-ready α-1 は未固定
- self-driven status:
  次は自走可能だが、α-1 を裸の `100%` と呼ぶには public/product boundary との混同を避ける recut が必要

## three-axis progress

| 軸 | Rough % | 読み |
|---|---:|---|
| 論理仕様 | 89 | `specs/19..24` で verification stratification、cut/save-load、auth layer algebra、observability、typed external boundary、operational α 条件の bounded freeze を置き、α-0.9 実装境界も source hierarchy に同期した |
| ユーザ向け仕様 | 78 | README / Documentation / progress / tasks / samples dashboard で evidence / first-floor / operational の読み分けを更新したが、final public surface や `U1` は未固定 |
| 実装 / 運用 | 84 | current-L2 active floor と practical alpha-1 first floors に加えて bounded operational α-0.5 / α-0.8 / α-0.9 の same-session line を actualize した |

## line snapshot

| Line | Category | Progress | Current status | Next gap |
|---|---|---:|---|---|
| current-L2 active floor | runnable floor | 90 | `samples/clean-near-end/`、Sugoroku、Lean foundations / generated stubs、helper stack は runnable | final public parser/API は未固定 |
| Spaces alpha-0 | evidence line | 100 | Stage A..F は `100% current-scope evidence closeout` | operational α-0.5 / α-0.8 / α-0.9 とは別 |
| practical alpha-1 | first-floor line | 100 | `RUN-01..04`、`HP-A1-01..07`、`TR-A1-01..07`、`VIS-A1-01..07`、`SL-A1-01..03`、`AV-A1-01..03`、`PE2E-01..09` は `100% first-floor closeout` | operational α line は別 package で追う |
| operational α-0.5 | operational line | 100 | `P-A1-20` で `practical_alpha05_session` carrier 上に `crates/mir-runtime::practical_alpha05_host_io`、example `host-io` subcommand、`OA05-07` `AddOne` direct execution lane を接続し、bounded local observable runtime を actualize | α-0.9 session-bound devtools export |
| operational α-0.8 | operational line | 100 | `P-A1-21` で `crates/mir-runtime::practical_alpha08_hotplug_session`、example `attach` subcommand、`scripts/practical_alpha08_session_hotplug.py`、`OA08-01..10` を追加し、debug/auth/rate-limit/object preview/deferred detach の same-session attach accepted/rejected/deferred/activation cut/observer-safe mutation を actualize | accepted detach execution / distributed ordering は later |
| operational α-0.9 | operational line | 100 | `P-A1-22` で `crates/mir-runtime::practical_alpha09_devtools`、example `export-devtools` subcommand、`scripts/practical_alpha09_devtools.py`、`OA09-01..09` を追加し、session-bound event DAG / route trace / membership timeline / witness relation / hot-plug lifecycle / fallback degradation / save-load timeline / redacted view / retention trace を actualize | final public viewer / telemetry ABI、durable audit は later |
| final public product | product/public | 10 | public boundary inventory はある | `U1` と public surface 決定が必要 |

## subsystem status

- **Mir core**
  finite decidable index fragment、effect row、lifetime/fallback、order/handoff、model-check second line、proof side export boundaryは current-L2 で整理済み
- **Mirrorea runtime / package line**
  `TermSignature`、`LayerSignature`、`MessageEnvelope`、`AuthEvidence`、`MembershipRegistry`、`PlaceCatalog`、`HotPlugRequest` / `HotPlugVerdict`、practical hot-plug / transport / save-load carriers、`practical_alpha05_session` session carrier はある
- **Typed external boundary**
  synthetic preview / canary に加えて、`AddOne` を bounded α-0.5 session carrier 上で direct semantic execution する minimal lane は actualize した。broader host family は later
- **Observability / devtools**
  export-side first floors に加え、α-0.9 で same session から event DAG / local route trace / membership timeline / witness relation / hot-plug lifecycle / fallback degradation / save-load timeline / observer-safe redacted view / retention-on-demand trace を引く non-final viewer/export lane が入った。final public telemetry service や durable audit backend は later
- **PrismCascade / Reversed Library**
  separable kept-later line。今回の operational α theory freeze の実装対象ではない

## macro phase map

| Macro | 主眼 | 現在位置 | 重さ | 自走可否 |
|---|---|---|---|---|
| `Macro 0` | repository memory / docs / traceability | 維持中 | light | 着手可能 |
| `Macro 1` | semantic kernel / invariant / boundary stabilization | current-L2 側は強い。operational α line は theory freeze 完了 | medium | 着手可能 |
| `Macro 5` | theorem / model-check / external verifier bridge | obligation export boundary は fixed、広い discharge は後段 | medium | 着手可能 |
| `Macro 6` | distributed fabric / shared-space / runtime evolution boundary | bounded α-0.8 same-session hot-plug runtime まで到達。accepted detach execution / distributed ordering は後段 | heavy | 着手可能 |
| `Macro 7` | toolchain / developer surface / public operational interface | α-0.5 session surface、minimal host-I/O lane、bounded α-0.8 same-session hot-plug lane、bounded α-0.9 session-bound devtools export は actualize 済み。next は α-1 product/public boundary と remaining non-final gaps の recut | heavy | 着手可能 |
| `Macro 8` | domain / application realization | public/product line | heavy | 要仕様確認 |

## feature maturity rows

| Feature | Rough % | 読み | 着手可否 |
|---|---:|---|---|
| multi-node / fabric | 48 | helper-local transport と alpha-0 evidence はあるが same-session operational shared-space は未完成 | 後段依存 |
| robustness via contracts / theorem / model-check boundary | 82 | static checker / model-check / proof side の stratification は fixed | 着手可能 |
| dynamic attach / detach / DAG-safe evolution | 84 | attach-time first-floor rows と bounded same-session lifecycle はあるが accepted detach execution / migration / distributed ordering は未完成 | 着手可能 |
| `atomic_cut` と ordering / memory-order family | 74 | place-local rollback frontier と consistent-cut boundary は fixed、distributed durable family は later | 着手可能 |
| executable sample corpus | 98 | current-L2、practical alpha-1 first floors、bounded operational α-0.5 / α-0.8 / α-0.9 line は runnable。残る gap は final/public α-1 boundary と product surface | 着手可能 |

## current blockers

- α-0.9 session-bound devtools は bounded ready になったが、final public viewer / telemetry ABI、admin/full debug view、durable audit backend は未固定
- distributed durable save/load、stale witness / stale lease non-resurrection、queue/channel persistence は current promoted reopen point の外側
- `U1` packaging / host target / shipped surface / final operational catalog は user decision gate のまま

## validation floor

- docs / hierarchy:
  `python3 -m unittest scripts.tests.test_validate_docs`
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
- formatting / diff:
  `cargo fmt --check`
  `git diff --check`
- `cargo test -p mir-runtime --test practical_alpha1_local_runtime -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha05_host_io -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha05_session -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha08_session_hotplug -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha09_devtools -- --nocapture`
- `python3 scripts/practical_alpha1_run_local.py check-all --format json`
- `python3 scripts/practical_alpha05_session.py check-all --format json`
- `python3 scripts/practical_alpha08_session_hotplug.py check-all --format json`
- `python3 scripts/practical_alpha09_devtools.py check-all --format json`
- `python3 -m unittest scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha05_session`
- `python3 -m unittest scripts.tests.test_practical_alpha08_session_hotplug scripts.tests.test_practical_alpha09_devtools`

## recent log

- 2026-05-05 11:17 JST
  `P-A1-22` で `crates/mir-runtime::practical_alpha09_devtools`、example `mir_practical_alpha05_session -- export-devtools`、`scripts/practical_alpha09_devtools.py`、`OA09-01..09` を actualizeし、α-0.5 / α-0.8 session carrier 上の event DAG / local route trace / membership timeline / witness relation / hot-plug lifecycle / fallback degradation / save-load timeline / observer-safe redacted view / retention-on-demand trace を bounded α-0.9 session-bound devtools export に接続した。
- 2026-05-05 10:18 JST
  `P-A1-21` で `crates/mir-runtime::practical_alpha08_hotplug_session`、example `mir_practical_alpha05_session -- attach`、`scripts/practical_alpha08_session_hotplug.py`、`OA08-01..10` を actualizeし、debug / auth / rate-limit / object preview / deferred detach を α-0.5 session carrier 上で same-session accepted/rejected/deferred と observer-safe lifecycle summary に接続した。
- 2026-05-05 09:47 JST
  `P-A1-20` で `crates/mir-runtime::practical_alpha05_host_io`、example `mir_practical_alpha05_session -- host-io`、`samples/practical-alpha1/packages/oa05-07-add-one-host-io`、`OA05-07` を actualizeし、typed external `AddOne` direct execution lane を α-0.5 same-session carrier と observer-safe export に接続した。
- 2026-05-05 09:26 JST
  `P-A1-19` で `RUN-03/04` capability / witness negative rows、`practical_alpha05_session` same-session carrier、session-bound observe/save/load loop、event DAG / observer-safe session export を actualizeし、operational α-0.5 の残 gap を typed host-I/O direct execution lane に絞った。
- 2026-05-05 08:32 JST
  `P-A1-18` で operational α-0.5 / α-0.8 / α-0.9 の completion condition、verification stratification、cut/save-load semantics、auth layer algebra、typed observability、typed host boundary を `specs/19..24` と `plan/45..49` に固定し、snapshot docs を evidence / first-floor / operational の読み分けへ同期した。
