---
id: plan/03-risks
status: L2-working
maturity: draft
depends_on: [plan/02-operating-model]
summary: リスク台帳 R-01..R-14。LAB plan/12 の蒸留と Mir v0/I1+ bounded execution の構造リスク。
open_items: []
---

# 03 — リスク台帳

| ID | リスク | 兆候 | 対策 |
|---|---|---|---|
| R-01 | エージェントのフロンティア回避(閉じやすい所だけ掘る) | 台帳が「〜してよい/later」で増える | active semantic frontier=1、direct consumer/acceptance criterion、milestone close gate(plan/02) |
| R-02 | 文書エントロピー(ガードレール文の肥大) | ファイル 15KB 超、重複防衛文 | 四半期 recut、style-guide のサイズ規律 |
| R-03 | 理論ドリフト(載せたかったものとのズレ) | SCN が説明できない改訂 | SCN 凍結+変更に ADR 必須 |
| R-04 | 実装先行による理論の歪み | verdict なし実行路、隠れ辺 | BND-004/001、C-static を CI の門に |
| R-05 | overclaim | 「完成」「証明済み」の早期宣言 | 非宣言欄、theory/11 のみが証明状態を語る |
| R-06 | 語彙ドリフト(新旧混在) | 旧予約語の再流入 | GLOSSARY 旧語彙対応、E-PARSE-005 |
| R-07 | 単独保守(bus factor=1) | 暗黙知の口頭依存 | canon 自体が対策。外部レビュー(plan/02) |
| R-08 | 検証コストの盲点(witness/trace 肥大) | I2 以降の性能崩れ | cost_bound 義務(OPEN-013)、I2 で計測 spike |
| R-09 | 観測の運用意味不足(DoS・肥大) | trace 無制限成長 | retention 必須、on-demand trace 規律 |
| R-10 | auth の transport 吸収 | envelope に権限が滲む | BND-005、SCN-03 の系譜検査 |
| R-11 | 二層時間の未成熟(theory/09) | pose 線の場当たり実装 | I5 前に G4/09 の再訪を必須化 |
| R-12 | 文献接続の停滞 | 新規性主張の宙吊り | theory/12 の row 追加を proof package に随伴 |
| R-13 | delegated research の semantic / proof laundering | L2 や bounded result が core / completed proof として扱われる | ADR-0015 の evidence classification と independent review。program 外は ADR-0014 の standing boundary / L2 fail-closed |
| R-14 | roadmap/report 増殖による current queue の消失 | 複数 plan/report が同時に current を名乗る | current execution roadmap は一つ、原則一 milestone 一 report、snapshot は derived view(plan/02) |
