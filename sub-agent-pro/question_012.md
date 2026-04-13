# Question 012 — host-facing I/O / FFI / visualizer / game engine adapter の層分け

## 前提

- この質問の前に `plan/` と `specs/` を読んでいる前提です。
- この質問は単独で答えてください。
- current repo では、privileged `stdin/stdout` primitive を language core に固定せず、capability-scoped input/output port / adapter boundary を docs-first に切る方針を採っています。
- visualizer / host substrate / host runtime / FFI / game engine adapter は later gate です。
- user 側の意図としては、「未接続ならただの port であり、host や visualizer が attach されたときだけ effect が流れる」ような方向を想定しています。

## 相談したいこと

この I/O / integration line を、どんな層分けと順序で進めるのが自然でしょうか。

聞きたいことは次です。

1. capability-scoped port / adapter boundary という考え方は妥当でしょうか。
2. host plan、visualizer、effect-base substrate、FFI、game engine adapter をどう階層化するとよいでしょうか。
3. raw FFI を早く入れず、まず docs-first host-facing boundary を切る方針は妥当でしょうか。
4. 今の fixed-subset sample line の後で、この統合 line を開くなら、どの順で package を切るのがよいでしょうか。

## 期待する回答

- recommended layering を示してください。
- 「まず docs-only / bridge-only」「次に narrow runtime adapter」「最後に concrete engine / FFI」という staged plan が妥当かも含めて評価してください。
