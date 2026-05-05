# ドキュメント要約

## この文書の役割

この文書は、repo 全体の **current snapshot を短く正確に読む入口** です。

- 規範判断の正本は `specs/`
- 長期の repository memory は `plan/`
- 進捗 snapshot は `progress.md`
- current task map は `tasks.md`
- runnable sample dashboard は `samples_progress.md`
- 実行証跡は `docs/reports/`

## まず repo をどう読むべきか

この repo は、Mir / Mirrorea / PrismCascade / Typed-Effect Wiring Platform を **意図的に separable** に保った研究用 workspace です。主眼は Mir current-L2 と、その上に積む Mirrorea shared-space / hot-plug / observability / host-boundary line にあります。

読み分けで重要なのは、次の 5 つを混同しないことです。

- **repo-local alpha-ready current layer**
  `samples/clean-near-end/`、helper、Lean foundation、report まで含めて動かせる current-L2 の実行足場
- **current-scope evidence closeout**
  `samples/alpha/` と `specs/13..17` / `plan/39..43` による alpha-0 helper-local / runtime-private closeout 群
- **practical alpha-1 first-floor closeout**
  `samples/practical-alpha1/` と `specs/18` / `plan/44` による front-door / checker / runtime / hot-plug / transport / devtools / save-load / preview の first-floor toolchain
- **operational alpha readiness**
  `specs/19..24` / `plan/45..49` で定義した α-0.5 / α-0.8 / α-0.9 の same-session operational 条件
- **final public product**
  final parser grammar、public checker/runtime/verifier API、public adapter / viewer / projection / hot-plug / transport surface、packaging、external contract まで含む最終形

現在 repo が強いのは前 3 つであり、operational alpha readiness と final public product はまだ別の gate です。

## current active floor

active canonical sample は `samples/clean-near-end/` です。base current-L2 corpus は `samples/current-l2/`、active Lean mechanization evidence は `samples/lean/` に置きます。

- `typing/`
  finite-index first strong typing layer
- `order-handoff/`
  publication / witness / handoff relation family
- `model-check/`
  Peterson / broken mutex による second-line verification
- `modal/`
  `stable` / `later` / `published(room)` / `witnessed(...)` の current mode line
- `sugoroku-world/`
  empty world server へ SugorokuGame を runtime attach する Mir / Mirrorea vertical slice

旧 active sample line は active path から外し、archive に退避しています。

## Mirrorea line の現在地

```text
OS/network substrate
  -> Mir core
  -> Typed external boundary
  -> Mirrorea fabric/runtime
  -> practical toolchain
  -> Spaces product
  -> Reversed Library
```

- **alpha-0 evidence line**
  `specs/13..17`、`plan/39..43`、`samples/alpha/` が current-scope evidence closeout を担います。Stage A..F は `100% current-scope evidence closeout` であり、operational α-0.5 / α-0.8 / α-0.9 completion ではありません。
- **practical alpha-1 first-floor line**
  `specs/18`、`plan/44`、`samples/practical-alpha1/` が first-floor toolchain を担います。`RUN-01..04`、`HP-A1-01..07`、`TR-A1-01..07`、`VIS-A1-01..07`、`SL-A1-01..03`、`AV-A1-01..03`、`PE2E-01..09` は current repo state で actualize 済みですが、いずれも `100% first-floor closeout` として読むべきであり、same-session runtime completion を意味しません。
- **operational alpha theory-freeze line**
  `specs/19..24`、`plan/45..49` が α-0.5 local observable runtime、α-0.8 same-session hot-plug runtime、α-0.9 session-bound devtools readiness の completion condition を固定します。2026-05-05 時点の latest closeout は `P-A1-20` で、`P-A1-18` の bounded theory freeze と `P-A1-19` の session carrier に続けて、typed external `AddOne` direct execution lane を same-session α-0.5 surface へ接続しました。

## いま何があり、何がまだ無いか

既にあるもの:

- practical checker / runtime / hot-plug / transport / devtools / save-load / preview の **distinct carrier split**
- event DAG export、observer-safe route trace、membership timeline export、fallback degradation export、redacted observer view、report-local retention query trace
- local-only save/load roundtrip と stale-membership non-resurrection first-floor rows
- attach-time auth / rate-limit / object preview / deferred detach の first-floor rows
- bounded α-0.5 session carrier 上の minimal typed external `AddOne` direct execution lane

まだ無いもの:

- α-0.8 same-session hot-plug runtime
- α-0.9 live/session-bound devtools export

## 重要な境界

- `Place` は participant と同一ではなく、state / queue / capability / visibility / observation frontier を持つ execution locus として読む
- standard I/O は Mir core primitive ではなく、typed external adapter boundary 側に残す
- auth / authorization / membership / capability / witness は transport に潰さない
- visualization / telemetry は label / authority / redaction / retention を持つ typed effect として扱う
- `atomic_cut` は place-local rollback frontier であり、durable/distributed commit ではない
- local save/load は distributed durable save/load と同一ではない

## どこを次に読むか

- live status / next reopen point:
  `progress.md`、`tasks.md`
- runnable sample dashboard:
  `samples_progress.md`
- practical alpha-1 first-floor:
  `specs/18-practical-alpha1-scope.md`、`plan/44-practical-alpha1-roadmap.md`
- operational alpha theory freeze:
  `specs/19-verification-stratification.md`
  `specs/20-cut-save-load-semantics.md`
  `specs/21-auth-layer-algebra.md`
  `specs/22-observability-devtools-semantics.md`
  `specs/23-typed-external-host-boundary.md`
  `specs/24-operational-alpha05-alpha08-readiness.md`
  `plan/45-operational-alpha05-roadmap.md`
  `plan/46-operational-alpha08-roadmap.md`
  `plan/47-operational-alpha09-devtools-roadmap.md`
  `plan/48-theory-freeze-proof-obligations.md`
  `plan/49-host-io-and-session-runtime-roadmap.md`
- hands-on closeout commands:
  `docs/hands_on/current_phase_closeout_01.md`

## snapshot の読み方

- `progress.md` と `samples_progress.md` の裸の `100%` は operational-layer-ready 以上にだけ使う
- evidence line や first-floor line の達成は、`100% current-scope evidence closeout` または `100% first-floor closeout` と明示して読む
- rough % は rollback し得る snapshot であり、規範判断の正本ではない
