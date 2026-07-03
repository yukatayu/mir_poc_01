---
id: mm/02-behind-the-scenes
status: L1-fixed
maturity: draft
depends_on: [mm/01-programmer-view, scenarios/SCN-02]
summary: SCN-02(attack)一行の裏で起きる全段の時系列。表の一行と裏の各段の対応。
open_items: []
---

# 02 — 同じコードの裏側(SCN-02 全段)

表で書いたのはこの一行である。

```mir
S { player[target].hp = player[target].hp - player[self].atk }
```

裏では次が起きる。番号は時系列。

1. **parse**: 構文木に span が付く。`S { }` は宣言済み place へのブロックと解決される(未宣言なら E-NAME-001 がこの span を指す)。
2. **check**: 現在 locus は `Participant[self]` 側で、`player` の持ち主は S。よってこれは cross-locus write。書き能力の義務、生成されうる失敗集合 {StaleMembership, MissingCapability, MissingWitness, RouteUnavailable} が算出され、`fails` 宣言との包含が検査される(足りなければ E-ROW-001 が「何を足すか」と共に返る)。
3. **elaborate**: 一行が Core に翻訳される — (a) `player[self].atk` と `player[target].hp` の cross-locus read(依存行/観測辺)、(b) S 宛の write request(能力参照・失敗行・span 同梱)。**この時点で通信境界が「導出」された。** あなたは IF を設計していない。
4. **(実行時) enqueue**: request が S の待ち行列に入る。S の store は S だけが直列に触る(これが並行モデルの全て)。
5. **validate**: S が dequeue し、epoch・incarnation・capability 系譜・witness・可視性を検証する。どれか欠ければ、宣言済みの失敗が**明示的な出来事**として立ち、store は無傷(fail-closed)。
6. **serve**: 検証を通れば write が実行され、履歴 DAG に request ≺ serve の辺付きで出来事が載る。
7. **publish**: hp が `visible` 宣言されていれば、redaction を通った publish 行が生成され、observer_safe な観測者に届く。
8. **audit/devtools**: 以上の全行(request、依存、検証結果、publish)が span 経由で表の一行に紐付いて閲覧できる。

表の一行 ↔ 裏の八段。この対応が壊れないこと(隠れ辺が無いこと)が THM-001 の中身である。
