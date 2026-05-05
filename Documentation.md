# ドキュメント要約

## この文書の役割

この文書は、repo 全体の **current snapshot を短く正確に読む入口** です。

- 規範判断の正本は `specs/`
- 長期の repository memory は `plan/`
- workflow / evidence snapshot は `progress.md`
- current task map は `tasks.md`
- runnable sample dashboard は `samples_progress.md`
- 実行証跡は `docs/reports/`

## まず repo をどう読むべきか

この repo は、Mir / Mirrorea / PrismCascade / Typed-Effect Wiring Platform を **意図的に separable** に保った研究用 workspace です。主眼は Mir current-L2 と、その上に積む Mirrorea shared-space / hot-plug / observability / host-boundary line にあります。

読み分けで重要なのは、次の 6 つを混同しないことです。

- **repo-local alpha-ready current layer**
  `samples/clean-near-end/`、helper、Lean foundation、report まで含めて動かせる current-L2 の実行足場
- **current-scope evidence**
  `samples/alpha/` と `specs/13..17` / `plan/39..43` による alpha-0 helper-local / runtime-private closeout 群
- **practical alpha-1 first-floor evidence**
  `samples/practical-alpha1/` と `specs/18` / `plan/44` による front-door / checker / runtime / hot-plug / transport / devtools / save-load / preview の first-floor toolchain
- **operational alpha readiness**
  `specs/19..24` / `plan/45..49` で定義した α-0.5 / α-0.8 / α-0.9 の same-session operational 条件
- **product/public-ready Mirrorea Spaces alpha-1**
  `specs/25` / `plan/50` で定義した alpha-stable CLI、versioned package format、same-session product demo、quiescent save、viewer、native launch bundle、clean-clone validation の product alpha line。final textual `.mir` grammar、WAN/federation、distributed durable save/load、arbitrary native execution、final viewer / telemetry service は non-goal
- **final public product**
  final parser grammar、public checker/runtime/verifier API、public adapter / viewer / projection / hot-plug / transport surface、packaging、external contract まで含む最終形

現在 repo は repo-local alpha-ready current layer、current-scope evidence、practical alpha-1 first-floor evidence に加えて、bounded operational α-0.5 / α-0.8 / α-0.9 と bounded practical α-1 integrated workflow carrier まで actualize 済みです。`P-A1-25` で product/public-ready alpha-1 の境界と alpha defaults は固定し、`P-A1-26` で `mirrorea-alpha check` と versioned product package schema first cut を追加しました。`P-A1-27` では `mirrorea-alpha run-local` / `session` / `attach` と product same-session carrier first cut を追加し、`P-A1-28` では bounded message recovery rows、R0 local `save` / `load`、R2 local `quiescent-save` を同じ session file に接続しました。ただし product alpha-1 release validation、local/Docker transport command behavior、viewer、native bundle、final public product、R3/R4 durable distributed save/load はまだ別 gate です。

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
  `specs/13..17`、`plan/39..43`、`samples/alpha/` が current-scope evidence を担います。Stage A..F は evidence reference であり、operational α-0.5 / α-0.8 / α-0.9 completion ではありません。
- **practical alpha-1 first-floor / workflow line**
  `specs/18`、`plan/44`、`samples/practical-alpha1/` が first-floor toolchain を担います。`RUN-01..04`、`HP-A1-01..07`、`TR-A1-01..07`、`VIS-A1-01..07`、`SL-A1-01..03`、`AV-A1-01..03`、`PE2E-01..09` は current repo state で actualize 済みですが、いずれも first-floor evidence です。`P-A1-23` で `scripts/practical_alpha1_integrated_workflow.py` と `PA1W-01..08` を追加し、source front-door / checker / same-session runtime / host-I/O / hot-plug / save-load / session devtools / product-preview evidence を 1 つの bounded developer workflow として再現できるようにしました。ただし final public product-ready completion は意味しません。
- **operational alpha theory-freeze line**
  `specs/19..24`、`plan/45..49` が α-0.5 local observable runtime、α-0.8 same-session hot-plug runtime、α-0.9 session-bound devtools readiness の completion condition を固定します。2026-05-05 時点の latest operational closeout は `P-A1-22` で、`P-A1-18` の bounded theory freeze、`P-A1-19` の session carrier、`P-A1-20` の typed external `AddOne` lane、`P-A1-21` の same-session hot-plug lane に続けて、`crates/mir-runtime::practical_alpha09_devtools`、example `mir_practical_alpha05_session -- export-devtools`、`scripts/practical_alpha09_devtools.py`、`OA09-01..09` により bounded operational α-0.9 session-bound devtools export を actualize しました。

## いま何があり、何がまだ無いか

既にあるもの:

- practical checker / runtime / hot-plug / transport / devtools / save-load / preview の **distinct carrier split**
- event DAG export、observer-safe route trace、membership timeline export、fallback degradation export、redacted observer view、report-local retention query trace
- local-only save/load roundtrip と stale-membership non-resurrection first-floor rows
- attach-time auth / rate-limit / object preview / deferred detach の first-floor rows
- bounded α-0.5 session carrier 上の minimal typed external `AddOne` direct execution lane
- bounded α-0.8 same-session hot-plug runtime over the same session carrier
- bounded α-0.9 session-bound devtools export over the same carrier
- bounded practical α-1 integrated workflow carrier over the existing first-floor and operational evidence

まだ無いもの:

- final public viewer / telemetry ABI
- durable audit backend / remote retained-artifact retrieval
- distributed durable save/load
- product-ready α-1 implementation / release validation

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
- product/public alpha-1 boundary:
  `specs/25-product-alpha1-public-boundary.md`
  `plan/50-product-alpha1-public-boundary-roadmap.md`
- hands-on closeout commands:
  `docs/hands_on/current_phase_closeout_01.md`

## snapshot の読み方

- `progress.md` と `samples_progress.md` は進捗率ではなく workflow status / evidence classification を primary metric にする
- helper / sidecar / report / expected JSON / first-floor runner は completion ではなく evidence として読む
- `100%` は外部開発者が実際に再現・使用できる operational workflow または product/public layer だけに使う
- `PA1W-*` は bounded practical workflow ready として読み、product/public-ready α-1 とは読まない
