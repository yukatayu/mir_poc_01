# Question 004 — `atomic_cut` と memory-order 再構築 family

## 前提

- この質問の前に `plan/` と `specs/` を読んでいる前提です。
- この質問は単独で答えてください。
- current repo で source-backed なのは、`atomic_cut` の place-local rollback frontier / finalizing cut と、`try` / rollback locality line までです。
- higher-level async-control、global ordering、fairness、scheduler、hardware-memory-like surface はまだ後段です。
- repo 側の current recommendation は、「settled subset の sample を厚くするのはよいが、low-level memory-order-like surface を早く mainline に入れるのはまだ早い」です。

## 相談したいこと

この言語では、C++ の `memory_order` をそのまま取り込むのではなく、より高位で、かつ表現力を失わない ordering / authority / commit / witness family として再構築したいと考えています。

ここで聞きたいのは次です。

1. `atomic_cut` から先へ進むとき、どの層分割が自然でしょうか。
   - local cut
   - room / authority / control-plane ordering
   - global commit / witness / fairness
   - external verifier / scheduler
2. low-level memory-order vocabulary を language core に直接持ち込まない方針は妥当でしょうか。
3. もし高位 family として再構築するなら、どんな primitive 群を first-class 候補として比較すべきでしょうか。
4. この line は sample code にいつ入れるのが自然でしょうか。今はまだ docs-only / theory-only に留めるべきでしょうか。

## 期待する回答

- 高位 ordering family の設計候補を 2〜3 方向で示してください。
- current mainline に対して「今やるべきこと / まだ早いこと」を分けてください。
- external verifier や model checker とどう境界を切るかも含めてください。
