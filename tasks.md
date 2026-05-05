# tasks

最終更新: 2026-05-05 14:27 JST

## この文書について

- この文書は repo 全体の **current task map** です。
- 規範判断の正本は `specs/`、長期比較と source trace は `plan/`、runnable sample 状態は `samples_progress.md`、実行証跡は `docs/reports/` に置きます。
- append-only 履歴ではありません。current checkpoint、次に詰める gate、blocker を読める snapshot として保ちます。
- 進捗率は primary metric ではありません。helper / sidecar / report / expected JSON / first-floor runner は completion ではなく evidence として書きます。
- `100%` は外部開発者がその layer を実際に再現・使用できる operational workflow または product/public layer だけに使います。

## current task-level status

- active executable floor は維持されています:
  `samples/clean-near-end/`、Sugoroku world、avatar follow、typed external preview、network canary、projection/codegen bridge、viewer prototype inventory。`samples/current-l2/` は base source corpus、`samples/lean/` は Lean evidence / generated theorem stub corpus として分けて扱います。
- `P-A1-18` により operational α-0.5 / α-0.8 / α-0.9 の completion condition と theory boundary は fixed されました:
  `specs/19..24` と `plan/45..49` が verification stratification、`atomic_cut` / save-load semantics、auth layer algebra、typed observability、typed host boundary、operational readiness definition を担います。
- `P-A1-19` により bounded α-0.5 same-session carrier は actualize 済みです:
  `crates/mir-runtime::practical_alpha05_session`、example `mir_practical_alpha05_session`、`scripts/practical_alpha05_session.py`、`RUN-03/04` capability / witness negatives により、check -> runtime plan -> run-local -> observe -> save -> load を同一 carrier に束ねる実行面が入りました。
- `P-A1-20` により bounded operational α-0.5 line は actualize 済みです:
  `crates/mir-runtime::practical_alpha05_host_io`、example `mir_practical_alpha05_session -- host-io`、`samples/practical-alpha1/packages/oa05-07-add-one-host-io`、`OA05-07` により、typed external `AddOne` direct execution lane を同じ session carrier、event DAG、observer-safe export に接続しました。
- `P-A1-21` により bounded operational α-0.8 line は actualize 済みです:
  `crates/mir-runtime::practical_alpha08_hotplug_session`、example `mir_practical_alpha05_session -- attach`、`scripts/practical_alpha08_session_hotplug.py`、`OA08-01..10` により、debug / auth / rate-limit / object preview / unsupported-runtime fallback companion source / deferred detach boundary を α-0.5 session carrier 上で same-session accepted/rejected/deferred / activation cut / observer-safe lifecycle summary に接続しました。
- `P-A1-22` により bounded operational α-0.9 line は actualize 済みです:
  `crates/mir-runtime::practical_alpha09_devtools`、example `mir_practical_alpha05_session -- export-devtools`、`scripts/practical_alpha09_devtools.py`、`OA09-01..09` により、event DAG / local route trace / membership timeline / witness relation / hot-plug lifecycle / fallback degradation / save-load timeline / observer-safe redacted view / retention-on-demand trace を同じ session carrier から export できるようにしました。
- `P-A1-23` により bounded practical α-1 integrated workflow carrier は actualize 済みです:
  `scripts/practical_alpha1_integrated_workflow.py`、`scripts/tests/test_practical_alpha1_integrated_workflow.py`、`PA1W-01..08` により、existing first-floor front-door / checker / runtime / host-I/O / hot-plug / save-load / session devtools / product-preview evidence を 1 つの bounded developer workflow に束ねました。product/public-ready α-1 ではありません。
- `P-A1-25` により product/public-ready alpha-1 boundary は fixed されました:
  `specs/25-product-alpha1-public-boundary.md`、`plan/50-product-alpha1-public-boundary-roadmap.md` により、alpha `U1` defaults、canonical Rust CLI direction、versioned `package.mir.json`、same-session product demo、message recovery / quiescent-save、product viewer、native host launch bundle、release validation の package line を固定しました。product alpha-1 implementation completion、arbitrary native package execution、signature-is-safety ではありません。
- `P-A1-26` により product alpha CLI / schema first cut は actualize 済みです:
  `crates/mirrorea-cli`、binary `mirrorea-alpha`、`crates/mir-ast::product_alpha1`、`samples/product-alpha1/demo/` により、versioned product `package.mir.json` schema、`check` accepted evidence、direct `.mir` non-goal diagnostic、full alpha command family の structured unsupported diagnostic を追加しました。same-session product runtime や product-ready α-1 completion ではありません。
- `P-A1-27` により product demo same-session runtime first cut は actualize 済みです:
  `crates/mir-runtime::product_alpha1_session`、`mirrorea-alpha run-local` / `session` / `attach`、CLI local session store により、product demo は runtime plan、core fabric envelope validation、typed host-I/O observation、debug-layer attach lifecycle、membership/witness/route/save-load/recovery state carrier を同じ session file に保持します。local/Docker transport command behavior、message recovery execution、quiescent-save、viewer、native launch bundle、product-ready α-1 completion ではありません。
- practical alpha-1 line は引き続き promoted implementation memory ですが、現在の closeout 群は **first-floor toolchain** です:
  `RUN-01..04`、`HP-A1-01..07`、`TR-A1-01..07`、`VIS-A1-01..07`、`SL-A1-01..03`、`AV-A1-01..03`、`PE2E-01..09` は first-floor evidence として読むべきであり、same-session operational α-0.5 / α-0.8 / α-0.9 ではありません。
- alpha-0 line は引き続き closed evidence reference です:
  Stage A..F は current-scope evidence として保持し、運用上の α-0.5 / α-0.8 / α-0.9 readiness と混同しません。

## ordered self-driven packages

| Order | Package | Macro / stage | Objective | Close condition | Rough estimate |
|---:|---|---|---|---|---|
| 1 | `P-A1-28` message failure/recovery + quiescent-save | `Macro 6/7`, `S4 -> S5` | `MessageState` / `TransportContract` / `RecoveryPolicy` と bounded R2 quiescent-save を実装する | `NoInFlight` / `AllPlacesSealed` / `NoPostCutSend` positive/negative reports が通る | heavy |
| 2 | `P-A1-29` product devtools viewer UX | `Macro 7`, `S5 -> S6` | product demo devtools JSON と non-final viewer panels を揃える | viewer openability と observer-safe leak tests が通る | medium |
| 3 | `P-A1-30` native launch bundle | `Macro 7`, `S5 -> S6` | product demo native host launch bundle を作る | bundle manifest / run script / `NativeExecutionPolicy = Disabled` / native non-claims validation が通る | medium |
| 4 | `P-A1-31` clean-clone product alpha-1 validation / release candidate closeout | `Macro 7/8`, `S6` | clean-clone guide、release check、hands-on/research docs を揃える | `check/run-local/session/attach/transport/save/load/quiescent-save/export-devtools/view/build-native-bundle/demo` を含む full product validation floor と report/commit/push が揃う | heavy |
| 5 | maintenance / dashboard freshness | `Macro 0`, `S6` | docs / dashboard / validator freshness を維持する | source hierarchy / docs scaffold / diff / report が current queue と一致する | small |

## current recommendation

- recommended reopen point:
  `P-A1-28` message failure/recovery + quiescent-save
- recommendation reason:
  `P-A1-27` で product demo local same-session carrier と CLI session persistence が入ったため、次の実質 gap は declared message recovery obligations と R2 quiescent-save execution に移った
- stop line:
  final public parser / viewer / telemetry ABI、distributed durable save/load、WAN/federation、arbitrary native execution、product alpha complete claim へ踏み込まない

## research-discovery items

| Item | Impact | Main options | Current recommendation |
|---|---|---|---|
| proof side discharge granularity | `plan/48` と external prover bridge に影響 | obligation family を coarse に束ねる / static-row ごとに細かく切る | まずは `specs/19` の residual obligation carrier を保ち、proof target 側で coarse-to-fine に展開する |
| distributed durable save/load line | `specs/20` の後段 family に影響 | local-only line のまま保つ / distributed durable family を reopen する | current promoted reopen point では扱わない。α-0.5 / α-0.8 same-session line 完了後に再評価する |
| auth policy catalog breadth | `specs/21` と host/runtime package line に影響 | minimal stdlib-like initial set / broader policy catalog | minimal initial set から始め、policy breadth は same-session attach line の実 evidence に合わせて widen する |
| integrated practical workflow boundary | `specs/18` の practical α-1 読みに影響 | one bounded workflow carrier / final public toolchain claim | `P-A1-23` で bounded workflow carrier は作成済み。final public toolchain claim はしない |
| product checker finite fragment breadth | `specs/25` と `P-A1-26/27/28` に影響 | existing practical rows only / product demo finite fragmentを少し拡張 | product demoに必要な package schema、effect/failure、capability/witness、message recovery、savepoint policy だけを bounded に足す |
| admin/debug product viewer breadth | `P-A1-29` に影響 | observer-safe + kept-later marker / bounded admin debug panel | まず observer-safe leak testを優先し、admin/debug full viewは実装できない場合 explicit kept-later |

## user decision items

| Item | Impact | Main options | Current recommendation |
|---|---|---|---|
| `U1` beyond alpha packaging / host target / shipped surface | final public product line | repo-local alpha / installed binary / hosted service / other | product alpha-1 の defaults は `specs/25` で固定済み。final public surface はまだ固定しない |
| final shared-space operational catalog breadth | product/public scope | product alpha narrow showcase / broader final product line | product alpha-1 では narrow showcase を採る。broader final catalog は product alpha release candidate 後の user/final decision として残す |
| final public grammar / ABI | final public product line | alpha `package.mir.json` を進化させる / textual grammar を固定する | product alpha-1 では固定しない。alpha package format は migration可能と明記する |
| hosted service / production WAN | final public product line | local/Docker alpha / hosted service / WAN federation | product alpha-1 では local/Docker に限定する |

## self-driven maintenance tasks

| Task | Objective | Validation | Report requirement | Stop line |
|---|---|---|---|---|
| docs freshness audit | `README.md`、`Documentation.md`、`progress.md`、`tasks.md`、`samples_progress.md`、index docs を current queue に合わせる | `python3 scripts/check_source_hierarchy.py`、`python3 scripts/validate_docs.py`、`git diff --check` | new `docs/reports/NNNN-*.md` | snapshot docs で新しい規範判断を勝手に増やさない |
| runnable dashboard refresh | sample status、validation timestamp、operational gap を evidence-backed に保つ | relevant helper closeout commands | report + `samples_progress.md` | conceptual-only row を workflow-ready と書かない |
| Rust formatting / regression repair | docs-only package でも formatting floor を崩さない | `cargo fmt --check`、affected tests | report if touched | unrelated feature workを混ぜない |

## non-promoted references

- `P-A0-23` / Stage B local runtime closeout は current-scope evidence reference であり、operational α-0.5 same-session runtime package ではない
- `P-A0-25` / Stage D lifecycle closeout は current-scope evidence reference であり、operational α-0.8 same-session hot-plug runtime package ではない
- `P-A0-26` / Stage E devtools closeout は current-scope evidence reference であり、`P-A1-22` bounded operational α-0.9 session-bound devtools package とは別 category である
