# 二大問題 representative sample bundle

この directory は、2026-04-22 clean-near-end migration 前に使っていた
**historical sample bundle 入口** である。

## archive status

- ここにある guide と command 例は archived closeout memory であり、
  current active sample root や current active command surfaceではない。
- current active compatibility front door は
  `python3 scripts/current_l2_guided_samples.py list`
  `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  `python3 scripts/current_l2_guided_samples.py closeout --format json`
  に置く。
- current live checker-shaped CLI example は archived prototype path ではなく、
  clean-near-end `.mir` sample に対する
  `mir-current-l2 check-source-sample`
  を使う。

## archived bundles

- [Problem 1 typed / theorem / model-check](./problem1-typed-theorem-model-check.md)
  - representative sample `p06`
  - supporting sample `p10 / p11 / p12 / p15 / p16`
- [Problem 2 order / handoff / shared-space](./problem2-order-handoff-shared-space.md)
  - representative pair `p07 / p08`
  - witness strengthening contrast `p05`
  - reserve route `p09`
  - negative pair `p13 / p14`

## historical usage memory

- 当時は各 bundle doc 冒頭の `historical quickstart memory` から
  `smoke` → `matrix` → `bundle` → `parser companion inspector`
  の導線を辿っていた。
- 当時は `quickstart` / `emit-*` / `quickstart-parity` / `reopen-map` /
  `residuals` / `reserve` / `lane` / `hold-line`
  helper も併用していたが、
  これらは current repo では archived helper memory であり、
  多くは migration note を返して exit 2 になる。
- current repo では archived workflow を再生する代わりに、
  `smoke-all` と `closeout` から active clean-near-end representative suite を見る。
