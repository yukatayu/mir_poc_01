# research_abstract

この directory は、repo 全体の研究を **phase ごとに短く読み返すための condensed summary** である。

- 正本は `specs/` と `plan/` である。
- ここでは、phase ごとに
  - 何をしようとしたか
  - 何が current reading として固まったか
  - 何が source-backed evidence か
  - 何をまだ決めていないか
  - 次へ何を渡したか
  だけを簡潔に書く。
- comparison の細部、open option の列挙、report chain の完全追跡は `specs/`、`plan/`、`docs/reports/`、`plan/90-source-traceability.md` を参照する。

## current global reading

- `Macro 4` は fixed authored/prototype floor の上で active である。
- `Macro 5` は post-runnable actual-adoption floor fixed + residual-gate compression closed の line である。
  current reading では compare floor、actual adoption floor、helper-local actualization / narrowing floor、deeper-theory / reserve / mixed-gate / actual-execution actualization floor まで close 済みであり、current self-driven queue は narrow だが nonzero で、remaining work は actual Lean execution hardening / helper orchestration と later mixed gate / user-spec residual に寄っている。
- `Macro 6` は minimal working subset actual default、`Macro 7` は mixed である。
- `Macro 8` は application / domain realization であり、user specification が要る。

## いま reader がここで把握すべきこと

- semantics / invariants / boundary はかなり固まっている。
- runnable path も既にある。
  - authored current-L2 sample sixteen
  - corrected runnable prototype nonet
  - exact rough stimulus preservation bucket
- corrected runnable version の current floorは already reached である。
- current self-driven queue は actual Lean execution hardening を主軸に narrow 化しており、Package 46〜54 は close 済みである。
- ただし final parser grammar、final public API、concrete theorem / model-check tool binding、shared-space final catalog はまだ open である。

## 使い方

1. phase の役割を知りたいときは各 abstract を読む。
2. current status は `progress.md` と `tasks.md` を見る。
3. 規範の根拠が必要になったら `specs/` に戻る。

## topic guide

- `static_analysis_01.md`
  - Problem 1 の typed / theorem / model-check と Lean foundation を、実行順に追う入門ガイド
- `order_01.md`
  - Problem 2 の order / handoff / authoritative-room を、success / reserve / static-stop の順に追う入門ガイド
