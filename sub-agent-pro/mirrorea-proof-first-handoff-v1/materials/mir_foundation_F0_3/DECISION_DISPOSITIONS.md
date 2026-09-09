# 30の未決事項の扱い

今回の研究profileで試した選択を、最終設計の採用と区別する。既に説明された目的を再質問する台帳ではない。次の定義・実装の直接の前提になった判断から解く。

| ID | 問い | 今回の候補 | 次の検討 | 担当 |
|---|---|---|---|---|
| Q-01 | 通常readの一貫性 | F0.2の参照ごとのowner snapshotを継承する有限profile。 | 式/関係単位の整合readを表す契約を比較し、暗黙分散transactionにしない。 | W2 |
| Q-02 | ownerと共同状態 | 抽象ownerの局所原子的更新。物理的唯一サーバを要求しない。 | 論理ownerの実現と多owner invariantを別に定義する。 | W2 / W4 |
| Q-03 | αに必要な言語の表現力 | 第一階の一部CFGのみ。F0.3 VMはspawn/join未対応。 | 純粋高階値・計算型・affine handleの最小正例と型規則を作る。 | W2 |
| Q-04 | 数値モデル | 数学的IntとBool。認可ctxではBool/Intを区別する。 | Int64/float/textへの意味とoverflowを決める。 | W2 |
| Q-05 | 親が複数の場合と退出 | positive all/anyと最小不動点。same-kind DAG条件は別に維持。 | ORと集団生成の使い方、局所証拠の十分性を利用例で確認する。 | W1 / W3 |
| Q-06 | fallbackと再取得の表層 | reacquireは別認可の明示操作。新claim生成だけでは復帰しない。 | 宣言から再取得操作を生成するsurfaceを検討する。 | W3 |
| Q-07 | 参照・識別・履歴 | instance/name/incarnation/revisionを区別。live結果のみ厳格再検査。 | historical ref/snapshot/first-class handle契約を分ける。 | W2 / W8 |
| Q-08 | 循環と遅延 | 同kindと関係計算はDAG、cross-kind根拠は帰納的closure。 | 許された時間feedbackの意味とcache/表示境界を比較する。 | W3 |
| Q-09 | 時間・締切の意味 | presentation contextは一致する外部tagで、実clock保証なし。 | logical time・deadline・clock/leaseの前提を閉じる。 | W3 / W4 |
| Q-10 | 提示と近似 | 関係式は正確な整数演算のみ。 | 誤差とapproximationを権限/失敗/表示のどこで明示するか定義する。 | W3 / W7 |
| Q-11 | retryと結果保持 | 同一runtimeのrequestは重複serve拒否。結果使用は別に現在性検査。 | 操作別retryとtombstone回収を永続化条件に結びつける。 | W4 / W5 |
| Q-12 | 取消しとquiescence | 一般的cancel/補償は未統合。 | 取り消す対象と既実行効果の非巻戻しを型にする。 | W2 / W4 |
| Q-13 | 故障下の進行 | 任意有限prefixの安全性のみ。 | 公平性、連結性、復旧条件ごとの進行性を宣言する。 | W4 |
| Q-14 | 効果と継続の拡張 | sourceからのrequest/resumeを使うが、一般handler変更は未定義。 | 再開権の回数・所有・保存を計算型へ接続する。 | W2 |
| Q-15 | 補償が必要な外界 | 外部effectはF0.3 Kernelに未搭載。 | F0.2のunknown-startモデルと現在性・完全snapshotの接続を研究する。 | W4 / W5 |
| Q-16 | 認証・policyの合成 | 同じtyped κを全leafへ渡すnonempty all/explicit any。 | 任意policy transformerの順序・委譲・issuer trustを選択profileごとに証明する。 | W1 / W8 |
| Q-17 | 対等性と信頼域 | ideal trusted issued records。cryptographyやByzantine hostを検証していない。 | guest/host/controlplaneのTCBと信頼翻訳を決める。 | W4 / W7 / W8 |
| Q-18 | 失効とprepared patch | F0.3 reparentはcommit時再検査。F0.2の準備後予約を採用しない。 | strict invalidationと予約方式の違いをowner判断の対象に残す。 | W3 / W5 |
| Q-19 | 理論拡張の受理方式 | 多項式/closure/labelの既知検査器だけ。 | 依存型/様相の局所理論adapterとsound bridgeを機械化する。 | W1 / W2 |
| Q-20 | save/loadの連続性 | 現在headに基づくsame-instance全image replay。 | fresh import/過去分岐/実full resumeの意味とscopeを別に選択する。 | W5 |
| Q-21 | 互換性と旧版の寿命 | 同schema/signatureとTotalBody非弱化。 | 契約変更とversion coexistence、受入れ側の合意を定義する。 | W3 / W5 |
| Q-22 | 基礎の証拠水準 | 24手証明、95tests、有限探索。Lean等の一般機械証明なし。 | 基礎受理は独立確認・機械化・実装対応の対象範囲で判定する。 | W1 / W4 |
| Q-23 | αのhot-plug範囲 | 動的node/source追加・退役・reparent・同schema交換。 | 実αで必要な追加/撤去/状態移行をSC-24へ明示する。 | W3 / W7 |
| Q-24 | 影響範囲の算出 | 完全tracked reads、absence、versioned index、incarnation。 | 影響閉包の差分化と分散validation原子性の実現を検証する。 | W3 / W4 |
| Q-25 | 保持・圧縮・削除 | 古いrecords/journalは保持。 | 根拠/ログ/claim/requestの安全な忘却条件はまだ選ばない。 | W5 / W8 |
| Q-26 | 観測の対象モデル | 固定labels、同low control/schedule、filter後buffer。 | dynamic IFC・timing/termination/存在の脅威モデルを別に選ぶ。 | W6 |
| Q-27 | 能動デバッグ | 受動observeのみ。active debuggerなし。 | stop/edit/rollbackを別の認可された操作にする。 | W6 / W7 |
| Q-28 | host脅威モデルと実装対象 | trusted Python host、guestはtyped APIに限定。 | sandbox、quota、provider isolationと実host前提を定義する。 | W7 |
| Q-29 | ネットワーク越し構築の最小体験 | 再現用CLIと静的HTML。製品UIではない。 | 最小の構築/参加/診断経路とheadless APIを同じ検査に結ぶ。 | W7 |
| Q-30 | 性能と規模の数値 | 性能値を目標として捏造しない。今回の時間は測定記録だけ。 | 対象機器とnetwork条件で検査/観測/実行の予算を決める。 | W4 / W6 / W7 |
