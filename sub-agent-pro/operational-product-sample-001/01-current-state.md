# 01 — current state and gap

## 現在できていること

現行 repo は product alpha-1 release-candidate workflow まで到達している。

現在ある代表機能:

- `mirrorea-alpha` CLI crate / binary
- versioned `package.mir.json` alpha schema
- product demo root: `samples/product-alpha1/demo/`
- check / run-local / session / attach
- local R0 save / load
- bounded local R2 quiescent-save
- local loopback TCP transport
- Docker Compose TCP transport
- non-final devtools export / viewer check
- native host launch bundle
- demo orchestration
- product release-check script

ただし、これは final public product ではない。

## 現在まだないこと

- final textual `.mir` grammar
- final public API / ABI / SDK
- direct Mir-to-machine-code compiler
- LLVM backend / optimization pipeline
- server/client emitted binaries from Mir source
- final placement optimizer / deployment planner
- production WAN / federation
- distributed durable save/load R3/R4
- accepted detach execution / migration / distributed activation ordering completion
- arbitrary native package execution
- final viewer / telemetry service
- full VRM / VRChat / Unity compatibility
- Reversed Library implementation
- PrismCascade integration

## 今回の gap

現 product alpha demo は、single demo root と command family を示す。
次に必要なのは、実運用に近い **開発プロセス sample** である。

ユーザの意図:

```text
1. WorldCore を書く
2. WorldCore を import して Membership/Chat を書く
3. Membership/Chat を import して Sugoroku を書く
4. それを server/client/runtime target へ分ける意図を持つ
5. 実行基盤上で動かし、debug/devtools で観測する
6. Portal / future spatial federation へ拡張可能であることを確認する
```

今回の package は、この flow を operational sample suite として repo に追加する。

## current product alpha との関係

- `samples/product-alpha1/demo/` は release-candidate demo。
- 新 root `samples/product-alpha1/operational/` は実運用プロセス再現 sample。
- `samples/practical-alpha1/` は first-floor fixture root。
- `samples/alpha/` は alpha-0 evidence root。
- `samples/clean-near-end/` は current-L2 active runnable suite。

これらを混ぜない。
