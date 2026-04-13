# Question 009 — `place` と process/server/executable の対応づけ

## 前提

- この質問の前に `plan/` と `specs/` を読んでいる前提です。
- この質問は単独で答えてください。
- current spec では、`place` は local state / rollback scope / ownership transfer が解釈される最小の意味論的 locus です。
- まだ server / process / executable そのものとは同一視していません。
- 一方で将来的には、複数ノードからなる system を統一的に書き、必要な runtime や binary 群へ落としたいと考えています。

## 相談したいこと

最終的に `place` を deployment unit へどう写像するのが自然でしょうか。

具体的には次を知りたいです。

1. `place` と process / thread / address space / server / pod / actor / room authority などの関係は、どの層で切るのがよいでしょうか。
2. compile / build / packaging の観点では、1 つの Mir program から複数 executable / runtime instance へ落とす形が自然でしょうか。それとも別の構成でしょうか。
3. route binding、host plan、runtime parameter、authority placement は、言語 core ではなくどの operational layer に置くのが良いでしょうか。
4. `place` を今の semantics のまま保ちつつ、将来 multi-node 実行へ進める際の中間段階は何でしょうか。

## 期待する回答

- semantic `place` と deployment artifact の対応関係を整理してください。
- current repo が今のうちに docs-first で切っておくべき boundary があれば示してください。
