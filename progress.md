# progress

最終更新: 2026-05-05 09:26 JST

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
  `P-A1-19` α-0.5 session runtime carrier
- current promoted reopen point:
  `P-A1-20` typed external host-I/O direct execution lane
- current reading:
  repo は **theory / first-floor carriers / evidence closeout** に加えて **bounded same-session α-0.5 carrier** を得たが、host-I/O lane と same-session hot-plug/runtime-wide live devtools は未完成
- self-driven status:
  `P-A1-20` は自走可能。`P-A1-21` same-session hot-plug runtime は `P-A1-19` carrier と `P-A1-20` host-I/O lane を前提にする

## three-axis progress

| 軸 | Rough % | 読み |
|---|---:|---|
| 論理仕様 | 88 | `specs/19..24` で verification stratification、cut/save-load、auth layer algebra、observability、typed external boundary、operational α 条件の bounded freeze を置けた |
| ユーザ向け仕様 | 76 | README / Documentation / progress / tasks / samples dashboard で evidence / first-floor / operational の読み分けを明示したが、final public surface や `U1` は未固定 |
| 実装 / 運用 | 64 | current-L2 active floor と practical alpha-1 first floors に加えて bounded α-0.5 session carrier と session-bound observe/save/load loop は actualize したが、host-I/O direct execution、same-session hot-plug、live/session-bound full devtools は未実装 |

## line snapshot

| Line | Category | Progress | Current status | Next gap |
|---|---|---:|---|---|
| current-L2 active floor | runnable floor | 90 | `samples/clean-near-end/`、Sugoroku、Lean foundations / generated stubs、helper stack は runnable | final public parser/API は未固定 |
| Spaces alpha-0 | evidence line | 100 | Stage A..F は `100% current-scope evidence closeout` | operational α-0.5 / α-0.8 / α-0.9 とは別 |
| practical alpha-1 | first-floor line | 100 | `RUN-01..04`、`HP-A1-01..07`、`TR-A1-01..07`、`VIS-A1-01..07`、`SL-A1-01..03`、`AV-A1-01..03`、`PE2E-01..09` は `100% first-floor closeout` | host-I/O / same-session hot-plug / live devtools |
| operational α-0.5 | operational line | 78 | `P-A1-19` で `practical_alpha05_session` carrier、session-bound observe/save/load、event DAG / observer-safe session export、membership/capability/witness negatives が actualize | typed host-I/O demo |
| operational α-0.8 | operational line | 30 | hot-plug contract/update law、attach accept/reject/deferred semantics、activation-cut trace の理論境界は fixed | α-0.5 session 上で debug/auth/rate-limit/object/avatar-placeholder attach を同一 session で観測 |
| operational α-0.9 | operational line | 20 | event DAG / route trace / membership timeline / redacted observer view の export-side semantics は fixed | live/session export、witness relation、hot-plug lifecycle、save/load timeline、retention/on-demand trace |
| final public product | product/public | 10 | public boundary inventory はある | `U1` と public surface 決定が必要 |

## subsystem status

- **Mir core**
  finite decidable index fragment、effect row、lifetime/fallback、order/handoff、model-check second line、proof side export boundaryは current-L2 で整理済み
- **Mirrorea runtime / package line**
  `TermSignature`、`LayerSignature`、`MessageEnvelope`、`AuthEvidence`、`MembershipRegistry`、`PlaceCatalog`、`HotPlugRequest` / `HotPlugVerdict`、practical hot-plug / transport / save-load carriers、`practical_alpha05_session` session carrier はある
- **Typed external boundary**
  synthetic preview / canary はあるが、`EchoText` / `AddOne` を direct semantic execution する active lane はまだ無い
- **Observability / devtools**
  export-side first floors に加え、α-0.5 local session から event DAG / observer-safe summary を引く lane はあるが、route trace / membership timeline / witness relation まで live/session 化したわけではない
- **PrismCascade / Reversed Library**
  separable kept-later line。今回の operational α theory freeze の実装対象ではない

## macro phase map

| Macro | 主眼 | 現在位置 | 重さ | 自走可否 |
|---|---|---|---|---|
| `Macro 0` | repository memory / docs / traceability | 維持中 | light | 着手可能 |
| `Macro 1` | semantic kernel / invariant / boundary stabilization | current-L2 側は強い。operational α line は theory freeze 完了 | medium | 着手可能 |
| `Macro 5` | theorem / model-check / external verifier bridge | obligation export boundary は fixed、広い discharge は後段 | medium | 着手可能 |
| `Macro 6` | distributed fabric / shared-space / runtime evolution boundary | 現在の主戦場。`P-A1-20` / `21` がここに入る | heavy | 着手可能 |
| `Macro 7` | toolchain / developer surface / public operational interface | α-0.5 session surface は着手済み。host-I/O lane と α-0.9 live export が次 | heavy | 着手可能 |
| `Macro 8` | domain / application realization | public/product line | heavy | 要仕様確認 |

## feature maturity rows

| Feature | Rough % | 読み | 着手可否 |
|---|---:|---|---|
| multi-node / fabric | 48 | helper-local transport と alpha-0 evidence はあるが same-session operational shared-space は未完成 | 後段依存 |
| robustness via contracts / theorem / model-check boundary | 82 | static checker / model-check / proof side の stratification は fixed | 着手可能 |
| dynamic attach / detach / DAG-safe evolution | 58 | attach-time first-floor rows と theory freeze はあるが same-session lifecycle 実行は未完成 | 着手可能 |
| `atomic_cut` と ordering / memory-order family | 74 | place-local rollback frontier と consistent-cut boundary は fixed、distributed durable family は later | 着手可能 |
| executable sample corpus | 90 | current-L2、practical alpha-1 first floors、bounded α-0.5 session carrier は runnable。残る operational α gap は host-I/O lane が中心 | 着手可能 |

## current blockers

- typed external host-I/O direct execution lane が無く、`EchoText` / `AddOne` の最小 operational sample が無い
- devtools は α-0.5 local session summary までは live 化したが、route trace / membership timeline / witness relation / save-load timeline を live/session-bound に統合していない
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
- `cargo test -p mir-runtime --test practical_alpha05_session -- --nocapture`
- `python3 scripts/practical_alpha1_run_local.py check-all --format json`
- `python3 scripts/practical_alpha05_session.py check-all --format json`
- `python3 -m unittest scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha05_session`

## recent log

- 2026-05-05 08:32 JST
  `P-A1-18` で operational α-0.5 / α-0.8 / α-0.9 の completion condition、verification stratification、cut/save-load semantics、auth layer algebra、typed observability、typed host boundary を `specs/19..24` と `plan/45..49` に固定し、snapshot docs を evidence / first-floor / operational の読み分けへ同期した。
- 2026-05-05 09:26 JST
  `P-A1-19` で `RUN-03/04` capability / witness negative rows、`practical_alpha05_session` same-session carrier、session-bound observe/save/load loop、event DAG / observer-safe session export を actualizeし、operational α-0.5 の残 gap を typed host-I/O direct execution lane に絞った。
