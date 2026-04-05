# 71 — current L2 try/rollback first-tranche shared carrier threshold recheck

## 目的

この文書は、current L2 parser-free PoC、`specs/examples/68`〜`70` を前提に、
dedicated `TryFallback` / `AtomicCut` AST structural helper first tranche が
**shared detached carrier threshold を本当に満たしたか**
を narrow に再比較する。

ここで扱うのは、current first tranche と saved artifact compare need の関係だけである。
generic structural checker family、public checker API、second malformed static tranche の actualization、
shared detached carrier の actual 実装は扱わない。

## current source-backed anchor

current repo では、次が source-backed に成立している。

1. dedicated helper-local first tranche は actualize 済みである
   - `checked_try_rollback_structural_verdict`
   - `checked_try_rollback_structural_findings`
   - `e23`
   - `e24`
   - `smoke-try-rollback-structural-checker`
2. first-tranche wording / row family は exact working set に固定した
   - `TryFallback` / `missing_fallback_body`
   - `AtomicCut` / `disallowed_fallback_placement`
3. helper-local checker は saved artifact path を直接受け取り、
   emitted static gate artifact の
   - `checker_core.static_verdict`
   - `checker_core.reasons`
   を fixture-side expected row と照合できる

したがって current 問いは、
**saved artifact compare が実際にできるようになった current state でも、
shared detached carrier へ上げる pressure が本当に生じたか**
である。

## 比較対象

### 案A. first tranche はもう shared detached carrier へ上げてよい

この案では、first tranche の

- structural verdict
- finding rows
- maybe reason wording mirror

を detached artifact の shared section に上げる。

### 案B. current first tranche は helper-local dedicated contract に留める

この案では、saved artifact compare need が見えても、
current first tranche ではまだ shared detached carrier へ上げない。

理由は、saved artifact compare 自体は
current helper-local checker が artifact path を読むことで既に満たせるからである。

### 案C. reference-only mirror を detached non-core に薄く追加する

この案では、shared carrier core には上げず、
detached non-core の reference-only mirror としてだけ row family を追加する。

これは full promotion より軽いが、artifact shape には新しい mirror surface が増える。

## 比較軸

### 1. saved artifact compare need を current helper-local path で満たせるか

current helper-local checker が、fixture JSON と saved artifact path を直接受け取れるなら、
saved artifact compare need だけを理由に shared carrier へ上げる必要は弱くなる。

### 2. shared detached carrier を premature に固定しないか

current first tranche は helper-local dedicated family であり、
generic structural checker family や public checker API と混ぜない判断が続いている。
shared carrier へ早く上げると、この boundary を先食いしやすい。

### 3. second family / generic consumer がまだ無い current state に合うか

current actual family は `e23` / `e24` の 2 件だけであり、
later generic family comparison はまだ残っている。
consumer が 1 family / 2 fixture に閉じている間は、
helper-local compare だけで十分な可能性が高い。

### 4. artifact-to-artifact compare pressure が本当にあるか

saved artifact compare need といっても、

- fixture + artifact
- artifact + artifact

では pressure の強さが違う。
current first tranche では前者は actual need として見えるが、後者はまだ source-backed でない。

## 比較結果

### 案A の利点

- detached artifact world に早く乗せられる
- future shared compare を先取りできる

### 案A の欠点

- current helper-local family を shared contract に premature 昇格しやすい
- second family や generic consumer がまだ無い current stateでは、artifact shape を先に固定する理由が弱い
- public checker / shared carrier / generic family の比較を先食いしやすい

### 案B の利点

- current helper-local checker が saved artifact path を直接比較できるので、saved artifact compare need 自体は narrow に満たせる
- shared detached carrier を増やさず、helper-local dedicated family のまま反復できる
- second family、generic family、public checker API の比較と混ざらない

### 案B の欠点

- artifact-to-artifact diff が later に必要になったときは、再度 threshold comparison が要る
- helper-local compare と shared detached carrier world は引き続き分離される

### 案C の利点

- full promotion ほど重くなく、artifact 側から row family を読める

### 案C の欠点

- reference-only とはいえ mirror surface を増やす
- current phase では consumer が無いのに artifact shape だけ増える
- later shared carrier / generic family comparison の前倒しになりやすい

## current judgment

current L2 の next narrow step として最も自然なのは、
**案B. current first tranche は helper-local dedicated contract に留める**
である。

理由は次の通り。

1. helper actualization、fixture-side contract、static corpus、loop stabilizationは current first tranche で満たした
2. しかし saved artifact compare need は、current helper-local checker が `fixture_path + artifact_path` を直接受けることで narrow に満たせる
3. current source では、artifact-to-artifact diff や shared consumer がまだ無い
4. second concrete family、generic family comparison、public checker API comparison もまだ外に残っている
5. したがって shared detached carrier へ上げる pressure は、まだ threshold に達していない

## threshold recheck の結論

current recheck では、threshold 5 条件のうち

1. helper actualization
2. fixture-side contract actualization
3. static corpus actualization
4. loop stabilization

は first tranche で満たしたとみなしてよい。

ただし 5 番目の

5. detached compare need

については、**shared detached carrier が無いと満たせない need まではまだ見えていない**。

current actual need は、

- emitted static gate artifact を保存する
- helper-local checker にその artifact path を渡して compare する

までで足りている。
したがって current phase では、threshold は **未充足のまま** と読むのが自然である。

## current cut

この task では次を行わない。

- shared detached carrier actualization
- detached non-core reference-only mirror の追加
- artifact-to-artifact diff helper 追加
- generic structural checker family 合流
- public checker API comparison

## next narrow step

current threshold recheck の次段として自然なのは、
**generic structural checker family / public checker API comparison に進む前提条件を、current first tranche を基準にもう一段 narrow に整理する**
ことである。
