# F0.3 命題・証拠台帳

24項目は、量化範囲と前提を持つ手証明である。機械検証・独立レビュー済みとはしない。最後の接続命題は、実装がstep対応を満たすことを未解消の前提として持つ。

| ID | 命題 | 対象範囲 | 残る境界 |
|---|---|---|---|
| F3-01 | 正の単調性と有限停止 | 任意の有限な正のsupport式と有限record集合 | 任意のsource述語や否定を含む条件の完全性ではない。 |
| F3-02 | 帰納的根拠に関する健全性と完全性 | 正のsupportの帰納的な有限導出 | 共同循環をcoinductiveな存在根拠として許す方式は選ばない。 |
| F3-03 | 独立certificate checkerの健全性・完全性 | 独立rank/closure証拠検査 | 任意の理論pluginの証拠検査ではない。 |
| F3-04 | 退役の単調性と正規化の冪等性 | 同じ宣言でeligible集合を減らす退役 | 新incarnationや宣言拡張は別の遷移。 |
| F3-05 | 低いsupportの独立性 | 下方閉label範囲と局所eligibility | high制御でlowのpolicy/identityを変更する場合は対象外。 |
| F3-06 | 同一文脈のpolicy合成 | 有限claim集合、名前付きleaf、非空all/any | 暗号認証・一般の委譲・policy transformerではない。 |
| F3-07 | 過去の証拠と現在の使用 | 正確なκと現在policy/issuer/claim | 過去の数学的命題を失効で偽とする意味ではない。 |
| F3-08 | ソース束縛の契約受理 | F0.2の多項式証拠と今回のsource adapter | F0.2 checker健全性は引継ぎ前提。任意の証明言語ではない。 |
| F3-09 | 全mutatorによる不変条件保存 | 同owner、同schema、全mutatorに同じ不変条件 | 多owner不変条件・一般的な資源移行は未証明。 |
| F3-10 | 正規化された多グラフの保存 | 今回の宣言追加・退役・reparent・policy変更・owner遷移 | 実際の分散制御プロトコルの線形化は別。 |
| F3-11 | 認可失効を含むfallbackとfresh reacquire | 任意有限候補列、live access、同lineage cursor | 最終候補の到達性や権限が必ずあるとはしない。 |
| F3-12 | 認可付き関係DAGの純粋射影 | 有限整数affine関係DAGと型付きsample | sampleの物理的真実性・暗号provenance・実時計同期は前提外。 |
| F3-13 | 一つの要求の一回の状態確定 | 保持された同request IDとPendingからの一回遷移 | 外部効果exactly-once・global deliveryではない。 |
| F3-14 | live結果の使用は、過去の成功とは別に検査する | live-result profileの現在member/module/依存/権限 | snapshot値を独立契約で保持する別profileは未実装。 |
| F3-15 | 検査read traceの再生補題 | 停止し決定的なvalidator、完全なtracked lookup | 未追跡query・clock・外部I/Oを参照するvalidatorには適用しない。 |
| F3-16 | 現在認可付きOCCの保存 | 発行済みdelta封印、全read/write stamp、現在権限 | 抽象制御の原子性を実現する分散adapterは別義務。 |
| F3-17 | reparentの局所検査 | 同kind DAGの全枝DFS、tracked revision | 全cross-kindグラフをDAGにはしない。 |
| F3-18 | 独立変更の可換性と適用範囲 | 完全なfootprintが独立な二変更 | domain recordの可換性でありaudit順序の一致ではない。 |
| F3-19 | 現在の確定prefixに基づく全成分復元 | 完全manifest・完全journal・独立に信頼された現head | in-memory image model。物理storage・オフラインcurrentnessは未証明。 |
| F3-20 | 受動観測の消去同値 | 観測stateがdomain実行の入力にならない遷移系 | 物理ゼロコスト・任意scheduler・能動debugは対象外。 |
| F3-21 | 固定label境界での二実行非干渉 | 固定label・同じlow制御・同じlow scheduleの二実行 | 任意の動的declassification、high管理、time/covert channelは未証明。 |
| F3-22 | 許可射影と有限保持の順序 | 許可filter後に有限last-kを保持 | 全履歴無制限保存や本番負荷の保証ではない。 |
| F3-23 | 任意の有限回の合成保存 | 選択したF0.3各遷移の任意有限合成 | 全Mir機能・無限資源・一般的な進行性は出ない。 |
| F3-24 | 実現への条件付き接続義務 | 具体と抽象のstep対応が別に成立する場合 | Rust/QUICの対応前提は未解消。機密性と進行性は追加義務。 |

一般証明の本文は `PROOFS.md`、実行した証拠は `../VALIDATION.md` にある。F0.2から取り込んだcompilerとcertificate checkerは無変更であり、それらの証明をF0.3の新しい証明件数へ重複計上しない。
