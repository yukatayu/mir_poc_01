# 65 — current L2 try/rollback AST helper first tranche cut

## 目的

この文書は、current L2 parser-free PoC、future dedicated AST structural helper の
malformed static family timing、wrapper family judgement、public checker entry criteria を前提に、
**future dedicated AST structural helper を actualize するときの first tranche をどこまで一体で切るか**
を narrow に整理する。

ここで固定するのは actual helper 実装ではない。
固定するのは、

- first tranche に必ず含めるもの
- first tranche から外に残すもの
- tranche を細かく割りすぎないための最小一体性

という docs-only judgment だけである。

## current 前提

current repo では次が成立している。

- future dedicated AST structural helper は helper-local dedicated contract から始める
- optional fixture-side field 候補は
  - `expected_static.checked_try_rollback_structural_verdict`
  - `expected_static.checked_try_rollback_structural_findings`
  である
- detached validation loop に載せるなら static gate artifact loop の family-specific smoke family に留める
- shared detached carrier は threshold 未充足の current state ではまだ actualize しない
- exact subcommand 名は actual helper actualization task まで deferred にする
- generic structural checker family / public checker API comparison はさらに後段である
- malformed static family は current phase の今すぐではなく、dedicated helper actualization first tranche と同時に actual corpus へ足すのが自然である

したがって current 問いは、
**first actualization tranche を切るとき、helper code / fixture fields / malformed static family / smoke path のどこまでを一体で切るのが最小か**
である。

## 比較観点

1. helper-local dedicated contract を hidden elaboration なしで actualize できるか
2. malformed static family timing judgement と矛盾しないか
3. fixture-side fields と smoke path がずれて半完成状態にならないか
4. shared carrier / public checker API / generic family を premature に巻き込まないか
5. first tranche 後に数回の authoring / compare 反復へすぐ入れるか

## 比較対象

### 案 1. helper code だけ先に切り、fixture fields / malformed corpus / smoke path は後で足す

#### 利点

- 実装の第一歩は小さく見える

#### 欠点

- helper code だけが存在しても compare contract を実地で確認しにくい
- fixture-side field と smoke path が未整備だと半完成状態が長く残る
- malformed static family timing judgement と切り離され、proof-of-use が弱い

### 案 2. helper code、fixture fields、最小 malformed static family、static gate smoke pathを first tranche として一体で切る

ここでいう最小 first tranche は、少なくとも次を含む。

1. dedicated helper-local 実装
2. fixture-side actual fields
   - `checked_try_rollback_structural_verdict`
   - `checked_try_rollback_structural_findings`
3. minimal malformed static family tranche
4. static gate artifact loop の family-specific smoke path

#### 利点

- helper contract、fixture expectation、AST-only corpus、compare path を同じ tranche で閉じられる
- malformed static family timing judgement と整合する
- first tranche 直後から authoring / compare の実地反復に入れる

#### 欠点

- tranche はやや重くなる

### 案 3. helper code、fixture fields、malformed corpus、smoke path に加え shared carrier mirror や public-looking compare surface まで first tranche に入れる

#### 利点

- 将来 shape が一気に見える

#### 欠点

- shared carrier threshold judgement と public checker entry criteria を破りやすい
- actual helper actualization の first tranche と later comparison を混ぜすぎる
- first tranche の review scope が不必要に太くなる

## current judgment

current L2 の next narrow step として最も自然なのは、
**案 2. helper code、fixture fields、最小 malformed static family、static gate smoke pathを first tranche として一体で切る**
である。

理由は次の通り。

1. helper-local dedicated contract は helper code だけでは閉じず、fixture-side field と compare path が揃って初めて fail-closed に観察できる
2. malformed static family は dedicated helper actualization first tranche と同時に actual corpus へ足す judgmentが既にある
3. shared carrier や public checker API を first tranche に混ぜると、後段 threshold を壊す
4. first tranche の目的は helper-local dedicated family を実地反復可能な最小単位へすることであり、そのためには helper / fields / corpus / smoke path の 4 点が最小一体である

## minimum tranche cut

current docs-only minimum として、first tranche は次を含む。

1. **helper code**
   - dedicated AST structural helper の helper-local 実装
2. **fixture-side fields**
   - `checked_try_rollback_structural_verdict`
   - `checked_try_rollback_structural_findings`
   の actual compare 化
3. **minimal malformed static family tranche**
   - AST-only structural malformed / underdeclared corpus の最小セット
4. **static gate smoke path**
   - family-specific wrapper から static gate artifact emit と helper-local compare を回す path

また、first tranche から外に残すものは次である。

- shared detached carrier
- generic structural checker family 合流
- public checker API comparison
- bundle-first runtime path integration
- final exact subcommand 名の長期固定

## current guidance

current helper stack と roadmap では、次を守る。

1. first tranche は helper-local dedicated family の actualization に閉じる
2. fixture-side fields と smoke path を helper code から切り離さない
3. malformed static family は first tranche に含めるが、shared carrier や public checker API までは同じ tranche に入れない
4. first tranche 後に数回の authoring / compare 反復を挟み、その結果を見て shared carrier / generic family / public checker comparison の後段判断へ進む

## current cut

この task では次を行わない。

- dedicated AST structural helper の actual実装
- fixture schema actual field 追加
- malformed static fixture の actual追加
- shared detached carrier actualization
- public checker API comparison actualization

## next narrow step

current docs-only judgment の次段として自然なのは、
**future dedicated AST structural helper の minimal malformed static family tranche の exact size をどこまで narrow に切るか**
を比較することである。

## OPEN に残すもの

- minimal malformed static family tranche の exact size
- actual helper actualization 時の exact subcommand 名
- malformed wording family をどこまで fixture-side expected に載せるか
