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

- `Macro 0〜4` は current scoped closeout fixed。
- `Macro 5` は boundary / pilot / framing closeout fixed。
- `Macro 6/7` は mixed-gate boundary fixed。
- `Macro 8` は application / domain realization であり、user specification が要る。

## いま reader がここで把握すべきこと

- semantics / invariants / boundary はかなり固まっている。
- runnable path も既にある。
  - authored current-L2 sample fourteen
  - corrected runnable prototype trio
  - exact rough stimulus preservation bucket
- ただし final parser grammar、final public API、concrete theorem / model-check tool binding、shared-space final catalog はまだ open である。

## 使い方

1. phase の役割を知りたいときは各 abstract を読む。
2. current status は `progress.md` と `tasks.md` を見る。
3. 規範の根拠が必要になったら `specs/` に戻る。
