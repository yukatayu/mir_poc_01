# WRK-0006 - OBL-020 familywise/global preservation boundary

## 位置付け

- 規範正本は `mirrorea_canon/`。この文書は LAB の実験計画と証跡である。
- 対応する事前登録は
  `mirrorea_canon/working/WRK-0006-obl020-familywise-global-boundary.md`。
- 対象は既存 abstract `OBL020StatementDraft` と
  `FamilyStepPreservesWF` の論理的接続だけである。coverage は実験内の条件であり、
  Canon requirement、step taxonomy、final theorem interface、proof architecture を選ばない。

## 問い

global draft が family-qualified wrapper を含意するか、familywise wrapper だけでは
未分類の actual step によって global draft が失敗し得るか、そして明示された
experiment-local coverage があれば familywise wrapper から global draft が導けるかを
Lean で確認する。

## 実施順

1. WRK-0006 の registration commit 後に target source の不在を確認する。
2. 既存 `StepWFStatementDraft` を変更せず import し、global-to-familywise と
   coverage-conditioned converse を定理として記述する。
3. canonical family と classified preserving step を一つ持つ有限 model に、別の
   unclassified non-preserving step を加える。wrapper は満たすが global draft は
   満たさないことを Lean で検査する。
4. placeholder/escape-token audit と既存 Lean synchronization test を実行する。
5. source evidence commit と、その commit/hash を append-only に記録する WRK manifest
   を分離する。

## 成功・停止条件

成功は、二つの条件付き implication と非自明 model が compile し、model が
family の空集合や classified step の不在に依存しないことである。retained result は、
family-local reasoning が aggregate preservation を与えるには明示された bridge が必要、
という LAB-only の論理境界に限る。

停止条件は、compile に Canon carrier、step taxonomy、coverage rule、追加 helper、既存
statement draft の変更、または final theorem interface が必要になることである。その場合は
WRK を `frozen` とし、必要ならより狭い successor 又は escalation を作る。

## 変更境界

source evidence は `samples/lean/lab-statements/obl020/`、説明、及びこの `plan/` 文書に
限定する。model の carrier と family 名はこの単一 source の experiment-local な定義であり、
Mir core primitive、runtime helper、public API、Canon family、transport semantics ではない。
Canon working record と current LAB snapshot は source evidence の後の manifest package でのみ
更新する。

## 実施結果

未 manifest の source evidence package で、Lean 4.29.1 は二つの implication と有限 model
を compile した。global draft は wrapper の追加 antecedentを無視して familywise form を導く。
逆向きは明示 coverage の下だけで導かれる。有限 model は canonical family と classified
preserving step を含みながら、unclassified actual step が `good` から `bad` へ進むため
familywise は満たして global draft を満たさない。

import を含む source は repository module search path を持たない bare `lean` では直接起動
できない。これは既存 WRK-0005 source と同じ runner 条件であり、外部一時 workdir に既存
draft の `.olean` を置き、`LEAN_PATH` で供給して compile した。これは coverage や Canon
semantics に関する失敗ではない。commit identity、artifact hash、及び current status は次の
WRK manifest で append-only に記録する。ここで coverage を Canon requirement として読んでは
ならない。
