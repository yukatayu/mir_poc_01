# 55 — current L2 try/rollback malformed static family actualization comparison

## 目的

この文書は、current L2 parser-free PoC、`TryFallback` / `AtomicCut` runtime representative、
dedicated AST structural helper の entry criteria、structural malformed source placement を前提に、
**`try` / rollback locality の malformed static family を actual corpus に追加する価値が current phase に本当にあるか**
を narrow に整理する。

ここで固定するのは actual helper 実装ではない。
固定するのは、

- runtime representative だけで十分か
- malformed static family を足すなら何が増えるか
- まだ足さないなら何が不足しているか

という docs-only judgment だけである。

## current 前提

current repo では次が揃っている。

- runtime representative として `E2` / `E21` / `E22` がある
- `specs/examples/51-current-l2-try-rollback-structural-floor-and-restore-scope.md` により、structural floor と dynamic gate / restore scope の split は固定済みである
- `specs/examples/52-current-l2-try-rollback-fourth-checker-spike-comparison.md` により、existing reason-row family helper の fourth spike には actualize しない
- `specs/examples/53-current-l2-try-rollback-ast-structural-helper-entry-criteria.md` により、future dedicated AST structural helper を actualize するなら AST-only floor、dedicated carrier、複数 family が必要である
- `specs/examples/54-current-l2-try-rollback-structural-malformed-source-placement.md` により、semantic structural malformed source は static gate / dedicated AST helper 側に置く

したがって current 問いは、
**current phase で malformed static family まで actual corpus に増やすと、entry criteria を次段へ進めるだけの前進量があるか**
である。

## 比較観点

1. current runtime representative で既に何が source-backed になっているか
2. malformed static family を足すと static companion evidence として何が増えるか
3. dedicated AST structural helper の actualization に本当に近づくか
4. hidden semantics invent を起こさないか
5. fixture authoring / detached validation loop のコストに見合うか

## 比較対象

### 案 1. runtime representative だけで当面十分とし、malformed static family はまだ増やさない

#### 利点

- current `E2` / `E21` / `E22` で structural floor と dynamic gate の contrast は既に source-backed である
- malformed static family を作るために新しい structural prohibition を premature に発明しなくて済む
- fixture authoring / helper actualization のコストを抑えられる

#### 欠点

- dedicated AST structural helper を actualize するだけの static family density はまだ得られない
- structural malformed source placement を決めても、actual malformed corpus は依然 absent のままである

### 案 2. malformed static family を 2 例以上 actual corpus に追加する

候補例としては、少なくとも次のような family が考えられる。

- `TryFallback` structural floor violation
- `AtomicCut` structural placement violation

#### 利点

- dedicated AST structural helper の actualization へ一歩近づく
- runtime representative と static malformed family の split を実地で確認できる

#### 欠点

- current docs だけでは exact malformed family をまだ十分に列挙していない
- parser / loader / static gate の wording / artifact cut も未決のままで、fixture を先に入れると wording invent が起きやすい
- helper actualization と malformed family actualization を同時に欲しくなり、task が肥大化しやすい

### 案 3. malformed static family は actual corpus に入れず、候補 family の inventory だけ docs に残す

#### 利点

- premature fixture 追加を避けつつ、次段比較の入口を残せる
- actual helper 実装や wording contract を先に凍らせずに済む

#### 欠点

- executable evidence は増えない
- dedicated AST structural helper の着手条件はまだ partially unmet のまま残る

## current judgment

current L2 の next narrow step として最も自然なのは、
**案 1. runtime representative だけで当面十分とし、malformed static family はまだ増やさない**
である。

理由は次の通り。

1. `E2` / `E21` / `E22` で structural floor と dynamic gate / frontier contrast は既に source-backed である
2. malformed static family を actualize するには、family 候補、wording、artifact cut、helper contractを同時に詰める必要があり、current phase ではまだ早い
3. `specs/examples/53` の entry criteria は「複数 family が必要」であって、「今すぐ family を actual corpus に入れるべき」とまでは言っていない
4. fixture authoring / detached validation loop のコストを払うなら、current phase では他の narrower bottleneck に使う方が前進量が大きい

## current cut

この task では次を行わない。

- malformed static fixture family の actual追加
- dedicated AST structural helper の actual code 追加
- `TryFallback` / `AtomicCut` の new static reason family 追加
- wording / artifact taxonomy の finalization

## next narrow step

current docs-only judgment の次段として自然なのは、
**future dedicated AST structural helper を actualize する場合の compare contract を dedicated carrier 前提でどう切るか**
を narrow に比較することである。

## OPEN に残すもの

- malformed static family 候補をどこまで inventory 化するか
- dedicated AST structural helper の future compare contract
- wording / artifact cut をどの層で持つか
