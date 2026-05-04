# tasks

最終更新: 2026-05-05 08:32 JST

## この文書について

- この文書は repo 全体の **current task map** です。
- 規範判断の正本は `specs/`、長期比較と source trace は `plan/`、runnable sample 状態は `samples_progress.md`、実行証跡は `docs/reports/` に置きます。
- append-only 履歴ではありません。current checkpoint、次に詰める gate、blocker を読める snapshot として保ちます。
- 裸の `100%` は operational-layer-ready 以上を意味します。evidence line や first-floor line は category を明示して書きます。

## current task-level status

- active executable floor は維持されています:
  `samples/clean-near-end/`、Sugoroku world、avatar follow、typed external preview、network canary、projection/codegen bridge、viewer prototype inventory。`samples/current-l2/` は base source corpus、`samples/lean/` は Lean evidence / generated theorem stub corpus として分けて扱います。
- `P-A1-18` により operational α-0.5 / α-0.8 / α-0.9 の completion condition と theory boundary は fixed されました:
  `specs/19..24` と `plan/45..49` が verification stratification、`atomic_cut` / save-load semantics、auth layer algebra、typed observability、typed host boundary、operational readiness definition を担います。
- practical alpha-1 line は引き続き promoted implementation memory ですが、現在の closeout 群は **first-floor toolchain** です:
  `RUN-01/02`、`HP-A1-01..07`、`TR-A1-01..07`、`VIS-A1-01..07`、`SL-A1-01..03`、`AV-A1-01..03`、`PE2E-01..09` は `100% first-floor closeout` として読むべきであり、same-session operational α-0.5 / α-0.8 / α-0.9 ではありません。
- alpha-0 line は引き続き closed evidence reference です:
  Stage A..F は `100% current-scope evidence closeout` として保持し、運用上の α-0.5 / α-0.8 / α-0.9 readiness と混同しません。

## ordered self-driven packages

| Order | Package | Macro / stage | Objective | Close condition | Rough estimate |
|---:|---|---|---|---|---|
| 1 | `P-A1-19` α-0.5 session runtime carrier | `Macro 6-7`, `S3 -> S5` | `PracticalAlphaSession` を導入し、check -> runtime plan -> run-local -> observe -> save/load を同一 session に束ねる | α-0.5 required rows のうち session carrier 依存の accepted local dispatch / observe / local save-load loop が 1 session で再現できる | medium |
| 2 | `P-A1-20` typed external host-I/O direct execution lane | `Macro 6-7`, `S3 -> S5` | `EchoText` または `AddOne` を stdio builtin ではなく typed external adapter として direct semantic execution する | host input payload schema、effect row、failure row、authority、observation policy を持つ minimal demo が α-0.5 session から観測できる | medium |
| 3 | `P-A1-21` α-0.8 same-session hot-plug runtime | `Macro 6-7`, `S3 -> S5` | debug / auth / rate-limit / object / avatar-placeholder package を α-0.5 session 上で attach し、accepted / rejected / deferred / activation cut / trace を観測する | same-session hot-plug attach が runtime state と devtools output を変えることを確認できる | heavy |
| 4 | maintenance / dashboard freshness | `Macro 0`, `S6` | docs / dashboard / validator freshness を維持する | source hierarchy / docs scaffold / diff / report が current queue と一致する | small |

## current recommendation

- recommended reopen point:
  `P-A1-19` α-0.5 session runtime carrier
- recommendation reason:
  operational α-0.5 の不足は `PracticalAlphaSession` 不在に集中しており、host-I/O direct lane と same-session hot-plug runtime の両方がこれに依存する
- stop line:
  `P-A1-19` の scope では host-I/O builtin 化、distributed durable save/load、full public devtools API へ踏み込まない

## research-discovery items

| Item | Impact | Main options | Current recommendation |
|---|---|---|---|
| proof side discharge granularity | `plan/48` と external prover bridge に影響 | obligation family を coarse に束ねる / static-row ごとに細かく切る | まずは `specs/19` の residual obligation carrier を保ち、proof target 側で coarse-to-fine に展開する |
| distributed durable save/load line | `specs/20` の後段 family に影響 | local-only line のまま保つ / distributed durable family を reopen する | current promoted reopen point では扱わない。α-0.5 / α-0.8 same-session line 完了後に再評価する |
| auth policy catalog breadth | `specs/21` と host/runtime package line に影響 | minimal stdlib-like initial set / broader policy catalog | minimal initial set から始め、policy breadth は same-session attach line の実 evidence に合わせて widen する |

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

- `P-A0-23` / Stage B local runtime closeout は `100% current-scope evidence closeout` reference であり、operational α-0.5 same-session runtime package ではない
- `P-A0-25` / Stage D lifecycle closeout は `100% current-scope evidence closeout` reference であり、operational α-0.8 same-session hot-plug runtime package ではない
- `P-A0-26` / Stage E devtools closeout は `100% current-scope evidence closeout` reference であり、operational α-0.9 session-bound devtools package ではない
