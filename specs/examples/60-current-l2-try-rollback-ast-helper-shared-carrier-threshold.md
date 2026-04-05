# 60 — current L2 try/rollback AST helper shared carrier threshold

## 目的

この文書は、current L2 parser-free PoC、future dedicated AST structural helper の compare contract、
expected field 名、detached-loop insertion judgment、structural verdict carrier を前提に、
**future dedicated AST structural helper を detached artifact shared carrier へ上げる閾値をどこに置くか**
を narrow に整理する。

ここで固定するのは actual helper 実装ではない。
固定するのは、

- helper-local dedicated contract のまま維持すべき期間
- shared detached carrier へ上げる条件
- その条件がまだ満たされていない current state

という docs-only judgment だけである。

## current 前提

current repo では次が成立している。

- dedicated AST structural helper は future option であり、current phase では actualize しない
- malformed static family は current phase ではまだ actual corpus に増やさない
- compare contract は helper-local dedicated contract から始める
- optional expected field 候補は
  - `expected_static.checked_try_rollback_structural_findings`
  - `expected_static.checked_try_rollback_structural_verdict`
  である
- verdict 値 shape は helper-local string enum `no_findings` / `findings_present` に留める
- detached validation loop に載せるとしても、bundle-first runtime path ではなく static gate artifact loop の helper-local smoke family に留める

したがって current 問いは、
**future dedicated AST structural helper を shared detached carrier に昇格させるなら、何が揃った時点を最小閾値とみなすべきか**
である。

## 比較観点

1. helper-local dedicated contract と shared detached carrier を premature に混ぜないか
2. actual fixture schema field actualization 前に artifact shape を凍らせてしまわないか
3. malformed static family 未actualizeの current phase でも docs-only judgment を前へ進められるか
4. static gate artifact loop と bundle-first runtime path の責務分離を保てるか
5. future public checker API comparison と conflation しないか

## 比較対象

### 案 1. dedicated helper が actualize されても、shared carrier へは上げず helper-local dedicated contract に長く留める

#### 利点

- shared detached carrier の premature 固定を避けやすい
- helper-local smoke family と fixture-side optional field compare に集中できる

#### 欠点

- saved artifact compare を将来行いたいときに、helper-local output と detached artifact world が長く分断される
- actual helper が安定した後でも repo 外 artifact comparison に入りにくい

### 案 2. dedicated helper actualization と minimal evidence cluster が揃った段階で shared carrier へ昇格させる

ここでいう minimal evidence cluster は、少なくとも次を指す。

1. dedicated AST structural helper の actual helper-local 実装がある
2. static gate artifact loop の helper-local smoke family が actual command として存在する
3. fixture-side expected field 候補
   - `checked_try_rollback_structural_verdict`
   - `checked_try_rollback_structural_findings`
   が actual compare contract として stable である
4. malformed static family か、それと同等の AST-only structural corpus が actual fixtures として複数件ある
5. helper-local compare だけでは足りず、saved artifact comparison や cross-run regression で shared carrier が実利を持つ

#### 利点

- shared carrier 化を「actual helper がある」「actual corpus がある」「detached compare need がある」の 3 条件に結びつけられる
- helper-local contract と shared carrier の責務分離を保ちやすい
- docs-only comparison の結果を actual narrow step に移しやすい

#### 欠点

- threshold 判定には evidence の蓄積が要る

### 案 3. dedicated helper actualization の時点で shared carrier へすぐ上げる

#### 利点

- detached artifact world と早く接続できる
- saved artifact compare の形式は早く見える

#### 欠点

- malformed static family actualization や fixture-side expected field stabilizationより先に artifact shape を固定しやすい
- helper-local dedicated contract を飛ばし、shared carrier が実質の primary contract になってしまう
- static gate helper と public-looking detached carrier の境界が濁る

## current judgment

current L2 の next narrow step として最も自然なのは、
**案 2. dedicated helper actualizationと minimal evidence cluster が揃った段階で shared carrier へ昇格させる**
である。

ただし current state では、その閾値はまだ満たされていない。

理由は次の通り。

1. dedicated AST structural helper 自体をまだ actualize していない
2. malformed static family も actual corpus にまだ入れていない
3. fixture-side expected field 候補も current fixture schema の actual field にはまだ上げていない
4. static gate artifact loop に future dedicated smoke family を置く judgmentまでは固まったが、actual helper-local smoke command はまだ存在しない
5. saved artifact compare の actual need も、helper-local smoke より優先される段階にはまだない

## minimum promotion threshold

current docs-only minimum として、shared detached carrier へ上げる threshold は次である。

1. **helper actualization**
   - dedicated AST structural helper が actualize されている
   - helper-local compare が fail-closed に動く
2. **fixture-side contract actualization**
   - `checked_try_rollback_structural_verdict`
   - `checked_try_rollback_structural_findings`
   が actual fixture schema compare field として導入されている
3. **static corpus actualization**
   - runtime representative だけでなく、AST-only structural malformed / underdeclared corpus が actual fixtures として存在する
4. **loop stabilization**
   - static gate artifact loop の dedicated smoke family が actual運用され、少なくとも数回の authoring / compare 反復を通っている
5. **detached compare need**
   - helper-local compare だけでは不足し、saved artifact compare か cross-run regression comparison に shared carrier の実利がある

この 5 条件が揃うまでは、shared detached carrier へ上げず helper-local dedicated contract に留めるのが自然である。

## current guidance

current helper stack と fixture authoring では、次を守る。

1. future dedicated AST structural helper を actualize するとしても、最初は helper-local dedicated contract に留める
2. static gate artifact loop に載せる判断と、shared detached carrier へ上げる判断を分ける
3. `checked_try_rollback_structural_verdict` / `checked_try_rollback_structural_findings` は current phase では still future field 候補であり、shared carrier へ先に mirror しない
4. bundle-first runtime path と shared detached carrier の convenience を理由に threshold を前倒ししない

## current cut

この task では次を行わない。

- dedicated AST structural helper の actual実装
- fixture schema への actual field 追加
- shared detached carrier actualization
- actual subcommand 名の固定
- generic structural checker family との統合

## next narrow step

current docs-only judgment の次段として自然なのは、
**future dedicated AST structural helper の actual subcommand 名と wrapper family をいつ narrow に切ってよいか**
を比較することである。

## OPEN に残すもの

- actual subcommand 名をいつ決めるか
- malformed static family を actual corpus に増やす timing
- future generic structural checker family とどこで合流させるか
- shared carrier に入れる row shape が string enum verdict / finding rows で十分か
