# 72 — current L2 try/rollback first-tranche generic/public recheck

## 目的

この文書は、current L2 parser-free PoC、`specs/examples/68`〜`71` を前提に、
dedicated `TryFallback` / `AtomicCut` AST structural helper first tranche を
**generic structural checker family / later public checker API comparison へ進めてよいか**
を narrow に再比較する。

ここで扱うのは、current first tranche actual state から見た

- generic family 合流の pressure
- public checker comparison の pressure
- この branch を今も自走で前へ進められるか

だけである。

## current source-backed anchor

current repo では、次が source-backed に成立している。

1. first tranche は actualize 済みである
   - `e23`
   - `e24`
   - `checked_try_rollback_structural_verdict`
   - `checked_try_rollback_structural_findings`
   - `smoke-try-rollback-structural-checker`
2. wording / row family は current next phase で fixed working set に置いた
3. shared detached carrier threshold は、saved artifact compare need の narrow version が helper-local compare で満たせるため、まだ未充足と判断した
4. second concrete decode-valid family は、current source ではまだ inventory 不足である

したがって current 問いは、
**generic structural checker family / public checker API comparison に進めるだけの新しい pressure が、この actual first tranche で本当に生じたか**
である。

## 比較対象

### 案A. current first tranche だけで generic/public comparison へ進む

この案では、first tranche actualization、wording stability、shared carrier threshold recheck まで済んだことを根拠に、
generic structural checker family と later public checker API comparison を次段として始める。

### 案B. current branch はいったん pause し、generic/public comparison は trigger が揃うまで deferred にする

この案では、first tranche line は

- second-tranche close
- wording stability
- shared carrier threshold recheck

までで current self-drivable branch を一巡したとみなし、
generic/public comparison は trigger が揃うまで保留する。

### 案C. generic/public comparison の pressure を得るために、先に second family や shared output contract を invent する

この案では、current source に不足している trigger を、比較のために先に作る。

## 比較軸

### 1. generic family に寄せる concrete pressure があるか

generic family comparison に進むには、
dedicated family のままでは不便だという concrete pressure が必要である。

例:

- second structural family との並置
- shared generic command surface の必要
- family 間共通 output contract の必要

### 2. public checker API table に載せる concrete pressure があるか

public checker comparison に進むには、

- shared output contract
- external verifier handoff
- command surface 整理

のいずれかが source-backed に見えている必要がある。

### 3. current actual first tranche だけでそれらが見えているか

current repo の first tranche は 2 fixture / 1 dedicated family に閉じている。
この状態で generic/public comparison を始めると、later pressure を先取りしやすい。

## 比較結果

### 案A の利点

- try/rollback line をさらに前へ進められる
- generic/public comparison の table 自体は早く見える

### 案A の欠点

- second family も shared detached carrier pressure もまだ無い current state では、comparison のかなりの部分が requirement invent に寄る
- existing 3 family checker spike と違い、current dedicated family は 1 line / 2 fixture の narrow family に留まる
- public checker / generic family の問いを前倒ししすぎる

### 案B の利点

- current source-backed line を clean に閉じられる
- invent せずに next branch へ移れる
- try/rollback line では「何が足りないと次へ進めないか」が明確になる

### 案B の欠点

- try/rollback checker branch は一旦 pause になる

### 案C の利点

- generic/public comparison の材料を自前で増やせるように見える

### 案C の欠点

- current source-backed progression を壊しやすい
- second family inventory や shared output contract を後付けで作ると、requirements invent に寄る
- current L2 の docs-first boundary に反する

## current judgment

current L2 の next narrow step として最も自然なのは、
**案B. current branch はいったん pause し、generic/public comparison は trigger が揃うまで deferred にする**
である。

理由は次の通り。

1. first tranche actualization、wording stability、shared carrier threshold recheck までで current branch の source-backed comparison は一巡した
2. second concrete decode-valid family はまだ無い
3. shared detached carrier pressure もまだ未充足である
4. generic family / public checker comparison に進む concrete pressure もまだ見えていない
5. したがって current branch をさらに前へ進めるには、新しい source-backed trigger が必要である

## current pause condition

current try/rollback checker branch を再開してよい最小 trigger は、少なくとも次のいずれかである。

1. second concrete decode-valid family が source-backed に見える
2. shared detached carrier が必要になる concrete compare need が見える
3. generic structural checker family へ寄せる shared command surface / shared output contract pressure が見える
4. public checker API comparison に載せる verifier handoff / command-surface pressure が見える

## branch handoff

current repo の self-drivable mainline としては、try/rollback line をここで一旦 pause し、
next branch を

- parser boundary / first parser cut inventory
- richer host interface ではない checker-side boundary inventory
- other source-backed first checker cut line

のいずれかへ移す方が自然である。

## next narrow step

current pause judgment の次段として自然なのは、
**current companion notation から first parser cut に入れてよい semantic cluster を、current actual checker / validation loop state を前提に narrow に棚卸しする**
ことである。
