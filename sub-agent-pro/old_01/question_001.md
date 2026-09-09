# Question 001 — 大局 roadmap と優先順位の再点検

## 前提

- この質問の前に `plan/` を読み、その後 `specs/` を読んでいる前提です。
- この質問は単独で答えてください。他の `question_*.md` には依存しません。
- current repo は specification-first です。
- 現在は architecture / semantics first ですが、current L2 の narrow fixed subset については runnable path があります。
- fixed-subset source sample は current authored decet (`e1`, `e2`, `e3`, `e4`, `e16`, `e18`, `e19`, `e21`, `e22`, `e23`) を持っています。
- theorem/model-check 側は、tool-neutral formal hook、`proof_notebook_review_unit` first pilot、model-check concrete carrier first actualization、sample-facing summary / bless flow までは current line に入っています。
- 一方で、final parser grammar、full 型システム、production-grade theorem/model-check binding、Mirrorea/shared-space operational runtime、LLVM-family backend はまだ後段です。

## 相談したいこと

現状の理解では、repo の大局は次の macro phase で読むのが自然だと考えています。

- `Macro 4` executable fixed-subset sample expansion
- `Macro 5` theorem / model-check / static reasoning bridge
- `Macro 6` fabric / shared-space / runtime evolution
- `Macro 7` toolchain / backend / developer surface
- `Macro 8` domain / application realization

この切り方自体と、近接順序

1. fixed-subset runnable line の ratchet
2. theorem/model-check の sample-visible milestone
3. docs-first I/O / adapter boundary
4. shared-space docs-first boundary
5. backend / public API / tooling finalization
6. domain application

という順序は妥当でしょうか。

特に知りたいのは次です。

1. 今の repo が **先に進めるべき line** と **まだ later gate に留めるべき line** は何ですか。
2. 逆に、今の順序にはどんな見落としや危険な前提がありますか。
3. sample-visible theorem/model-check milestone と shared-space docs-first boundary の間に、別の重要 line を挟むべきでしょうか。
4. full 型システムや backend に入る前に、どの checkpoint が満たされていると「安全に次へ進める」と言えますか。

## 期待する回答

- 「source-backed に妥当」「未決」「推奨」の 3 つを分けてください。
- 可能なら、近接 10 package 前後の rough ordering を提案してください。
- 「今やるべきではない line」も明示してください。
