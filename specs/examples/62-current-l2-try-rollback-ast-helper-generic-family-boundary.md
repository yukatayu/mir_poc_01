# 62 — current L2 try/rollback AST helper generic family boundary

## 目的

この文書は、current L2 parser-free PoC、future dedicated AST structural helper の
wrapper family judgement、shared carrier promotion threshold、generic checker-side shared entry 非採用 judgment を前提に、
**future dedicated AST structural helper を future generic structural checker family とどこで合流させるか**
を narrow に整理する。

ここで固定するのは actual helper 実装ではない。
固定するのは、

- dedicated family をどこまで維持するか
- generic structural checker family へ合流させる条件
- later public checker API comparison とどこで接続するか

という docs-only judgment だけである。

## current 前提

current repo では次が成立している。

- same-lineage / missing-option / capability の 3 spike では shared support helper を導入しても family facade script を維持し、generic checker-side shared entry はまだ切らない
- future dedicated AST structural helper は static gate artifact loop の family-specific wrapper に留め、exact subcommand 名も actual helper actualization task まで deferred にする
- shared detached carrier へ上げる threshold も current state ではまだ未充足である
- malformed static family も actual corpus にまだ入れていない

したがって current 問いは、
**future dedicated AST structural helper を actualize するとしても、generic structural checker family と later public checker API comparison にいつ接続するのが最小か**
である。

## 比較観点

1. current family facade pattern と整合するか
2. dedicated helper-local contract を premature に generic 化しないか
3. later public checker API comparison と timing を混同しないか
4. structural family がまだ薄い段階で abstraction pressure を掛けすぎないか
5. actual helper 実装と corpus 実地反復の前に generic API を既成事実化しないか

## 比較対象

### 案 1. actual helper actualization の時点で generic structural checker family と合流させる

#### 利点

- generic abstraction は早く見える
- later public checker API への接続も見えやすい

#### 欠点

- dedicated helper-local contract と family-specific wrapper judgementを早く壊す
- structural family がまだ `TryFallback` / `AtomicCut` しか見えていない段階で generic pressure が強すぎる
- actual helper code / corpus / detached compare need より先に API comparison が走る

### 案 2. dedicated family を actual helper actualization・corpus 実地反復の間は維持し、generic family 合流は later public checker API comparison と同時に扱う

#### 利点

- current family facade pattern と整合する
- dedicated helper-local contract、future fixture-side fields、static gate wrapper family を先に安定化できる
- generic structural checker family への合流条件を public API comparison と同じ table に載せられる

#### 欠点

- generic family の見取り図は後段に残る

### 案 3. generic structural checker family とは合流させず、恒久的に dedicated family に留める

#### 利点

- dedicated helper の意味は最も明確

#### 欠点

- later public checker API comparison で structural family 間の整理を毎回個別にやることになる
- genericity が本当に必要になった時の比較軸を先送りしすぎる

## current judgment

current L2 の next narrow step として最も自然なのは、
**案 2. dedicated family を actual helper actualization・corpus 実地反復の間は維持し、generic family 合流は later public checker API comparison と同時に扱う**
である。

理由は次の通り。

1. current repo では existing checker spike 群でも family facade を維持し、generic checker-side shared entry は later comparison に送っている
2. future dedicated AST structural helper も、actual helper 実装、fixture-side field actualization、static corpus 実地反復より先に generic family へ寄せるべきではない
3. generic structural checker family への合流は later public checker API comparison と同じ問いであり、そこでまとめて比較した方が一貫する

## minimum convergence threshold

generic structural checker family と later public checker API comparison へ合流させる threshold は、少なくとも次である。

1. dedicated AST structural helper の actual helper-local 実装が存在する
2. future fixture-side fields
   - `checked_try_rollback_structural_verdict`
   - `checked_try_rollback_structural_findings`
   が actual compare contract として運用されている
3. static gate artifact loop の family-specific wrapper で数回の authoring / compare 反復が済んでいる
4. malformed static family か同等の AST-only structural corpus が actual fixtures として揃っている
5. dedicated family のままでは later public checker API comparison が重くなる具体的理由が見えている

この threshold が揃うまでは、generic structural checker family と later public checker API comparisonへは合流させず、dedicated family に留めるのが自然である。

## current guidance

current helper stack と roadmap では、次を守る。

1. future dedicated AST structural helper は current phase では dedicated family として扱う
2. generic structural checker family 合流の議論は later public checker API comparison と同時に行う
3. actual helper code が無い段階で generic command surface や generic detached carrier を追加しない

## current cut

この task では次を行わない。

- dedicated AST structural helper の actual実装
- generic checker-side shared entry actualization
- public checker API actualization
- malformed static family の actual corpus 追加

## next narrow step

current docs-only judgment の次段として自然なのは、
**later public checker API comparison に future dedicated AST structural helper family を載せる entry criteria をどこまで narrow に切るか**
を比較することである。

## OPEN に残すもの

- later public checker API comparison の entry criteria
- malformed static family を actual corpus に増やす timing
- actual helper 実装開始時の exact subcommand 名
