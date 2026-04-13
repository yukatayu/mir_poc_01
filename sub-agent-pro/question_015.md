# Question 015 — representative sample corpus の増やし方

## 前提

- この質問の前に `plan/` と `specs/` を読んでいる前提です。
- この質問は単独で答えてください。
- current repo には fixed-subset source sample decet があります。
- runner / lowering / verification ladder / regression helper も narrow に揃っています。
- malformed family については missing-option と capability の source-backed widening を進めています。
- 一方で、grammar widening、shared-space、high-level ordering family、type surface はまだ後段です。

## 相談したいこと

sample corpus を今後どう増やすのが最も費用対効果が高いでしょうか。

知りたいのは次です。

1. representative sample は「理論 pressure をかける sample」と「実用っぽい sample」をどう配分すべきでしょうか。
2. next sample family を選ぶ基準は何でしょうか。
3. malformed sample、valid sample、proof/model-check visible sample、host-I/O visible sample をどの順で足すのが自然でしょうか。
4. sample corpus が semantics を誤って early-freeze しないようにするには、どの設計 rule が必要でしょうか。

## 期待する回答

- next 10 sample 前後の選び方の戦略をください。
- sample / lowering / runner / checker / theorem/model-check artifact をどう連動させるかも含めてください。
