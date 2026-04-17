# Phase 2 要約 — parser-free PoC と detached validation loop

## この phase の役割

Phase 2 は、Phase 1 の semantics を **parser をまだ固定せずに回せる parser-free validation substrate** へ落とす phaseである。

## 固まった current reading

- fixture / host-plan / interpreter / compare helper を narrow に分ける。
- detached artifact は compare 用の helper 出力であり、言語コアの public surface ではない。
- parser-free loop は
  `fixture -> run_bundle -> detached artifact -> compare`
  の ratchet として保つ。

## source-backed evidence

- `mir-semantics` current L2 minimal interpreter
- detached bundle / aggregate / static-gate helper
- regression / smoke / compare helper 群

## まだここで決めていないこと

- public exporter API
- final storage / retention policy
- richer host interface
- typed / theorem / model-check concrete binding

## 次へ渡したもの

compile-ready actualization と syntax-backed source sample lane は、この parser-free loop を壊さずに上へ積む。
