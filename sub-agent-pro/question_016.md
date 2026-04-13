# Question 016 — いま見落としていそうな大きい理論・設計リスク

## 前提

- この質問の前に `plan/` と `specs/` を読んでいる前提です。
- この質問は単独で答えてください。
- repo 側では、architecture-first だが fixed subset は runnable、という ratchet 方式で進めています。
- macro phase / maturity stage / shared-space docs-first boundary / theorem/model-check bridge / host-facing I/O boundary まではかなり整理してきました。
- そのうえで、memory-order 再構築、型理論、Mirrorea runtime、shared-space final catalog、backend、application layer など、まだ重い line が多く残っています。

## 相談したいこと

ここまでの計画や設計を見たうえで、「今の repo がまだ十分に意識できていない大きいリスク」や「今のうちに相談しておくべき blind spot」は何でしょうか。

特に次の観点で、厳しめに見てください。

1. 理論の整合性
2. language core と operational layer の境界
3. proof / model-check / static checker の責務分担
4. sample-driven ratchet が early commitment になる危険
5. shared-space / I/O / external integration line の見落とし
6. 将来の public API / tooling / backend へ進む際の爆発点

## 期待する回答

- 「最重要 blind spot」を 5〜10 個程度、優先度つきで挙げてください。
- 各項目について、
  - なぜ危ないか
  - 今から軽く打てる予防線
  - まだ later でよいかどうか
  を書いてください。
