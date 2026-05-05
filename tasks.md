# tasks

最終更新: 2026-05-05 11:33 JST

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
- practical alpha-1 line は引き続き promoted implementation memory ですが、現在の closeout 群は **first-floor toolchain** です:
  `RUN-01..04`、`HP-A1-01..07`、`TR-A1-01..07`、`VIS-A1-01..07`、`SL-A1-01..03`、`AV-A1-01..03`、`PE2E-01..09` は first-floor evidence として読むべきであり、same-session operational α-0.5 / α-0.8 / α-0.9 ではありません。
- alpha-0 line は引き続き closed evidence reference です:
  Stage A..F は current-scope evidence として保持し、運用上の α-0.5 / α-0.8 / α-0.9 readiness と混同しません。

## ordered self-driven packages

| Order | Package | Macro / stage | Objective | Close condition | Rough estimate |
|---:|---|---|---|---|---|
| 1 | `P-A1-25` alpha-1 product/public boundary recut | `Macro 7/8`, `S6` | bounded workflow と product/public-ready α-1 の差を audit し、`U1` decision inventory と public surface stop lines を整理する | final public parser / CLI / viewer / telemetry / packaging / host target の未決項目を明示し、自走実装可能な範囲と user decision gate を分ける | medium |
| 2 | maintenance / dashboard freshness | `Macro 0`, `S6` | docs / dashboard / validator freshness を維持する | source hierarchy / docs scaffold / diff / report が current queue と一致する | small |

## current recommendation

- recommended reopen point:
  `P-A1-25` alpha-1 product/public boundary recut, or user decision on `U1`
- recommendation reason:
  bounded operational α-0.5 / α-0.8 / α-0.9 と bounded practical α-1 workflow は揃ったため、次の実質 gap は public/product-ready boundary と user-facing shipped surface の決定に移った
- stop line:
  user decision なしに final public parser / viewer / telemetry ABI、distributed durable save/load、WAN/federation、product/public runtime complete へ踏み込まない

## research-discovery items

| Item | Impact | Main options | Current recommendation |
|---|---|---|---|
| proof side discharge granularity | `plan/48` と external prover bridge に影響 | obligation family を coarse に束ねる / static-row ごとに細かく切る | まずは `specs/19` の residual obligation carrier を保ち、proof target 側で coarse-to-fine に展開する |
| distributed durable save/load line | `specs/20` の後段 family に影響 | local-only line のまま保つ / distributed durable family を reopen する | current promoted reopen point では扱わない。α-0.5 / α-0.8 same-session line 完了後に再評価する |
| auth policy catalog breadth | `specs/21` と host/runtime package line に影響 | minimal stdlib-like initial set / broader policy catalog | minimal initial set から始め、policy breadth は same-session attach line の実 evidence に合わせて widen する |
| integrated practical workflow boundary | `specs/18` の practical α-1 読みに影響 | one bounded workflow carrier / final public toolchain claim | `P-A1-23` で bounded workflow carrier は作成済み。final public toolchain claim はしない |

## user decision items

| Item | Impact | Main options | Current recommendation |
|---|---|---|---|
| `U1` packaging / host target / shipped surface | final public product line | repo-local only / installed binary / hosted service / other | repo はまだ固定しない。operational α line の evidence を揃えてから user decision として reopen する |
| final shared-space operational catalog breadth | product/public scope | narrow showcase / broader product line | α-0.5 / α-0.8 / α-0.9 operational line 完了前に固定しない |

## self-driven maintenance tasks

| Task | Objective | Validation | Report requirement | Stop line |
|---|---|---|---|---|
| docs freshness audit | `README.md`、`Documentation.md`、`progress.md`、`tasks.md`、`samples_progress.md`、index docs を current queue に合わせる | `python3 scripts/check_source_hierarchy.py`、`python3 scripts/validate_docs.py`、`git diff --check` | new `docs/reports/NNNN-*.md` | snapshot docs で新しい規範判断を勝手に増やさない |
| runnable dashboard refresh | sample status、validation timestamp、operational gap を evidence-backed に保つ | relevant helper closeout commands | report + `samples_progress.md` | conceptual-only row を 25% 超にしない |
| Rust formatting / regression repair | docs-only package でも formatting floor を崩さない | `cargo fmt --check`、affected tests | report if touched | unrelated feature workを混ぜない |

## non-promoted references

- `P-A0-23` / Stage B local runtime closeout は current-scope evidence reference であり、operational α-0.5 same-session runtime package ではない
- `P-A0-25` / Stage D lifecycle closeout は current-scope evidence reference であり、operational α-0.8 same-session hot-plug runtime package ではない
- `P-A0-26` / Stage E devtools closeout は current-scope evidence reference であり、`P-A1-22` bounded operational α-0.9 session-bound devtools package とは別 category である
