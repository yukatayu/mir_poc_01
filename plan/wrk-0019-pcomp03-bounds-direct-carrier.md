# WRK-0019 P-COMP-03 bounds direct-carrier evidence

## 位置付け

これは `mirrorea_canon/working/WRK-0019-pcomp03-bounds-direct-carrier.md`
で事前登録した一件の LAB evidence memo である。規範判断は Canon に残り、
本書は既存 Product Alpha package route の固定した観測だけを保持する。

## 入力と実行 cut

- 事前登録 / authority cut: `4ea2d008f631d2f62e54645f06e79e0963348154`
- 実験 sidecar:
  `samples/product-alpha1/computational/arrays-bounds/negative/direct-world/package.mir.json`
- sidecar SHA-256:
  `7d833a2c2e41a5dfc695246716e1a8343a5a3d1dca26bc29d9b6d0d86370d8f3`
- 登録済みの四 input digest は実行前に全て一致した。

## 観測

登録済み command は成功した。15 行の computational matrix と `check-all`
は全行を期待結果どおりに通し、対象 sidecar の Product Alpha `check` は
`accepted` だった。続く `run-local` は exit 2 で、JSON に
`diagnostic_code: MirCompute` と
`OutOfBounds: array index 1 is out of bounds for length 1` を返した。

登録済みの Rust focused tests も通過した。

- `mir-semantics` の負例安定理由 test: 1 passed
- `mir-runtime` の Product Alpha 負例 rejection test: 1 passed

生成 JSON と session directory は `/tmp` の disposable output であり、repository
artifact としては保持しない。

## 解釈と非効果

この結果は一つの non-production `world` manifest が既存 package path で上記の
固定 error を返すという LAB 観測である。Python helper の分類との同一実装性、
closed registry の評価 phase、全 P-COMP-03 coverage、textual `.mir` 入力、
general direct carrier、runtime correctness、public failure contract、workflow
readiness、Canon theory、OBL、Gate、Phase は導かない。

helper、schema、validator、CI/Make surface、Rust crate、runtime、CLI、adapter、
public API、transport、production behavior は変更していない。`WRK-0019` の
Reliance status は `not-promoted` のままである。

## 次の再開条件

この同じ観測を coverage widening や repair に使わない。新たな自走 research は、
既存 lane にある別の literal input と、事前登録可能な adverse branch、および
現在の下流 decision を示せるときだけ別 record として再評価する。Core/result
correspondence、global-step coverage、outcome-totality placement、public transport
はそれぞれの未決境界として残る。
