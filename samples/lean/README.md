# samples/lean

このディレクトリは、repo が現在 Lean でどこまで検証しているかを、
repo-local かつ inspectable な形で保存する。

## 構成

- `foundations/`
  - 実際に小さな証明を含む self-contained Lean file を置く
  - 現在の主眼は IFC / label-model first fragment、secret valid/invalid concrete example、proof-skeleton / obligation-shape first fragment である
- `current-l2/`
  - 現在の current-L2 定理ブリッジから representative sample set `e5-underdeclared-lineage, p06-typed-proof-owner-handoff, p10-typed-authorized-fingerprint-declassification, p11-typed-unauthorized-fingerprint-release, p12-typed-classified-fingerprint-publication-block, p07-dice-late-join-visible-history, p08-dice-stale-reconnect-refresh, p09-dice-delegated-rng-provider-placement` 向けに生成された Lean theorem stub を置く
  - これらの file は Lean に受理されるが、まだ `sorry` を含む

## 読み方

- `foundations/` は、すでに小さな fact を証明できる **mechanization-ready core** を示す。
- `current-l2/` は、repo が representative sample から生成する **actual emitted theorem bridge surface** を示す。
- generated current-L2 stub は artifact alignment と Lean acceptance を示すのであって、completed theorem discharge を示すものではない。

## 再生成

次を実行する:

```bash
python3 scripts/current_l2_lean_sample_sync.py
```

これにより committed Lean sample corpus を再生成し、`lean` で検証する。
