---
id: mm/03-correspondence
status: L1-fixed
maturity: draft
depends_on: [mm/02-behind-the-scenes, theory/10-diagnostics, adr/ADR-0004]
summary: S0↔S1↔S2 対応表、診断の読み方、既知の直感ギャップ三つ。
open_items: []
---

# 03 — 対応表と既知の直感ギャップ

## S0 ↔ S1 ↔ S2 対応表

| S0 で書くもの | S1 に生成されるもの | S2 に残る出来事 |
|---|---|---|
| 読み(式の中の参照) | 依存行、cross なら observe/read request | (原則なし。監査対象の読みのみ行が残る) |
| 持ち主での代入 | write op(+visible なら publish 行) | write occurrence(+publication) |
| 非持ち主ブロック内の代入 | owner 宛 request(能力・失敗行・span 付き) | request ≺ serve、または明示的失敗 |
| `when h(...) fails φ` | 遷移入口+失敗包含義務 | handler 起動群 |
| `join ... via ...` | AdmissionRequest | admitreq ≺ verdict、epoch 更新 |
| `chain` 経由の参照 | 正準 chain 解決 | 劣化(前進)や Reject。lease 失効は監査 subreason |
| patch モジュール | pipeline 一式 | patchreq ≺ verdict ≺ activation_cut |
| `atomic_cut`(Core 記法) | — | cut occurrence(rollback frontier) |

## 診断の読み方

すべての差し戻しは「どの規則の・どの前提が・どの束縛で」落ちたかを持つ(theory/10)。読み方: ①span で表の箇所へ、②missing_evidence で「宣言が足りない場所」へ、③suggested_repair をそのまま適用してよいか自分の意図と照合する。診断は使用箇所ではなく**宣言の欠けた箇所**を責める。

## 既知の直感ギャップ(理論が直感と食い違う三箇所)

1. **fallback は「外側に包まれて戻れる」ではない。** 直感: 寿命の長い外側の値へ退避し、回復したら戻る。理論: 劣化は同一系譜上を前へ進むだけで、戻りは**新しい系譜を開く明示的再取得**(新 witness/epoch)。理由: 暗黙復帰は save/load の stale 復活禁止と両立しない。実務上「回線が戻ったら HD に戻る」は普通に起きる — それは再取得という新しい出来事として起きる(ADR-0004、SCN-08)。
2. **rollback は状態を戻すが、劣化順序は戻さない。** `try` の巻き戻しで store は戻っても、chain の位置と失効した lease は戻らない。「巻き戻したのに前の選択肢が使えない」は正しい挙動である。
3. **key は権限ではない。** `player[Alice]` の Alice は索引であって持ち主でも書き手でもない。「自分のエントリだから書ける」は成り立たず、書けるのは grant がある者だけ(ADR-0005、SCN-02/03)。

この三つは診断文言(spec/07)でも同じ言葉で説明される。ここの語彙を変えるときは spec/07 と同時に変えること。
