# 二大問題 representative sample bundle

この directory は、二大問題それぞれの representative sample を
**簡潔な日本語 guide 付きで辿るための sample bundle 入口** である。

## 置き方

- ここに置く文書は final public tutorial ではない。
- 既存の corrected prototype / Lean artifact / parser companion / helper command を再配置せず、
  どこから見ればよいかを 1 本の guide にまとめる。
- final public parser / checker / runtime API、final public verifier contract、exhaustive shared-space catalog はここで fixed しない。

## bundles

- [Problem 1 typed / theorem / model-check](./problem1-typed-theorem-model-check.md)
  - representative sample `p06`
  - supporting sample `p10 / p11 / p12 / p15 / p16`
- [Problem 2 order / handoff / shared-space](./problem2-order-handoff-shared-space.md)
  - representative pair `p07 / p08`
  - reserve route `p09`
  - negative pair `p13 / p14`

## current recommendation

- まず各 bundle doc を読む。
- 次に guide 内の `run-source-sample` / `bundle problem1|problem2` / `mapping` command を順に使う。
- 問題 1 / 問題 2 をまとめて手早く確認したいときは
  `python3 scripts/current_l2_guided_samples.py smoke-all`
  を使う。
- deeper theory や final public contract ではなく、current first line がどの sample で machine-check されているかを確かめる入口として使う。
