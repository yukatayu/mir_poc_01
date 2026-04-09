# Phase 0 要約 — repository memory と decision boundary

## 何をした phase か

Phase 0 は、Mir 本体の semantics を直接増やす phase ではなく、
**この repo 自体が長期研究を支えられるようにする phase** である。

主に次を整えた。

- `specs/` を規範判断の正本にする
- `plan/` を長期参照用の repository memory にする
- `docs/reports/` を task 単位の研究ノートにする
- `progress.md` を rough snapshot にする
- L0 / L1 / L2 / L3 の decision level を明示する

## なぜ重要か

この repo では、agent が過去会話を信用せずに途中参加することが前提である。
したがって、

- 何が決定済みか
- 何が提案段階か
- 何が未決か

を文書構造そのもので区別できないと、研究結果がすぐ壊れる。

Phase 0 は、その壊れにくさを作る phase である。

## 具体例

たとえば fallback について、次の 2 つは全く違う。

```text
決定済み:
  fallback は guarded option chain で読む

未決:
  final parser grammar で A2 surface を唯一の concrete syntax にするか
```

前者は `specs/` に置く。
後者は `specs/10-open-questions.md` や `plan/` に置く。

この分離を崩すと、「まだ決まっていない syntax」が「規範意味論」のように誤読される。

## この phase で得たもの

- 研究を ratchet 方式で進めるための最低限の memory
- source-backed かどうかを追跡する provenance
- phase / roadmap / autonomy gate を使った大局把握

## まだここで決めていないもの

- Mir の final surface syntax
- 型システムの final shape
- shared-space や upper layer の最終仕様

これらは Phase 0 の役割ではない。

## 次 phase へ渡したもの

Phase 1 以降は、この memory / boundary を前提にして、
semantics と PoC を少しずつ強くしていく。
その際に出てくるコード風の記法は、まず companion notation / representative example として読み、final parser grammar や built-in 群と早合点しないことが重要である。
