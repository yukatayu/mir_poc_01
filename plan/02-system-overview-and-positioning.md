# plan/02 — 全体像と位置づけ

## 全体像

この repo は、1 つの単一 runtime を作っているのではない。
現在の構造は、少なくとも次の層を分けて考える。

1. **Mir**
2. **Mirrorea**
3. **Typed-Effect Wiring Platform**
4. **PrismCascade**
5. shared space / VRSNS / Reversed Library のような上位空間

## どれが主眼か

| 層 | 位置づけ | 現在の優先度 |
|---|---|---|
| Mir | 主眼。意味論コア、effect / contract / ownership / lifetime / failure / safe evolution を扱う | 最優先 |
| Mirrorea | Mir で定義された system を安全に実行・進化させる fabric | 次段。現在は大局設計中心 |
| Typed-Effect Wiring Platform | operational effect boundary の観測・rewire 層 | 隣接層。Mir と密接だが別 project idea |
| PrismCascade | media-processing kernel。独立 kernel として扱う | optional / side-track 寄り |
| shared space / VRSNS / Reversed Library | 上位アプリケーション・空間構想 | まだ遠い future |

## 現在どこまで current L2 / PoC が存在するか

### 現在 current L2 / PoC が明示的にある部分

- Mir current L2 semantics
- representative examples
- parser-free PoC fixture schema
- parser-free minimal interpreter
- host harness / sidecar / bundle / batch / selection / profile / catalog
- fallback / `lease` / monotone degradation の regression fixtures

### まだ大局設計・方針レベルに留まる部分

- Mirrorea fabric の実装
- Typed-Effect Wiring Platform の concrete architecture
- PrismCascade kernel の具体実装
- shared space / VRSNS / Reversed Library の application 層

## Mir / Mirrorea / Typed-Effects / Prism / 上位空間の関係

### Mir

- 計算の意味を定義する層である。
- event DAG、`place`、effect / contract、failure、rollback、`atomic_cut`、ownership / lifetime、安全な進化制約を扱う。
- current repo で最も進んでいるのはこの層である。

### Mirrorea

- logical name、route rebinding、overlay insertion、audit、patch activation を扱う fabric である。
- Mir を「どこで・どう route して・どう進化させるか」に関与する。
- ただし現時点では PoC 実装よりも boundary と原則整理が中心である。

### Typed-Effect Wiring Platform

- effect boundary の inspectable / routable / contract-aware な operational layer である。
- Mir そのものではなく、Mir と非 Mir system をつなぐ lower or adjacent layer として位置づく。
- legacy integration や effect routing を扱うが、current repo ではまだ概念整理段階に近い。

### PrismCascade

- offline / live の audiovisual pipeline に特化した独立 kernel である。
- Mir runtime に早期統合すると設計が歪むため、現時点では意図的に分離する。
- 統合点は shared identifier、shared trace、remote execution、meta-layer provider などに絞る。

### shared space / VRSNS / Reversed Library

- これらは上位構想であり、Mir / Mirrorea / Typed-Effects / Prism を使って構築される application / environment 層である。
- 現時点の repo では、本格実装対象ではなく future direction として扱う。

## 既存サービス・既存概念との差分

この project は、単なる新サービスや service mesh 置換ではない。
既存の多くのシステムでは別々に扱われている次を、理論的に整列させようとしている。

- 言語意味論
- effect / contract
- route / overlay / patch
- observability / audit
- runtime evolution

要するに、**言語・運用・進化・検証の境界を 1 つの体系で説明すること**が狙いである。

## ユーザ視点と実装者視点の違い

| 視点 | 何が重要か |
|---|---|
| ユーザ視点 | request がどの contract で受理され、どこで degrade / `Reject` され、どの trace / audit が残るか |
| 実装者視点 | どこまでを language core に入れ、どこからを fabric / host / verifier / kernel に分けるか |
| 現在の repo の実務視点 | Mir current L2 を parser-free PoC で machine-check しつつ、重い workstream を premature に既成事実化しないこと |

## current repo における重要な位置づけ

- **Mir が主眼**である。
- PrismCascade は重要だが、現段階では main line を決める対象ではない。
- current L2 / PoC は Mir 側に偏っており、それは設計意図と一致している。
- 今後 `plan/` でも、この優先順位を崩さずに status と roadmap を更新する。
