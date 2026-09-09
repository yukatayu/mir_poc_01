# 命題と証拠の台帳 F0.1

**全28項目は本文に手証明を記載しましたが、全体の機械証明でも独立レビュー済みでもありません。** SMT列は関連する個別VCであり、その命題全体の証明とは限りません。

| ID | 命題／本文の節 | 厳密な対象範囲と前提 | 関連SMT入力番号 | 実行検査 |
|---|---|---|---|---|
| T01 | 式の型検査の終了・健全性・完全性（3.3） | 任意の有限な一階式・型付き環境。宣言的な正規形判定への相対完全性。 型のjoin計算が定義され、環境と構文が整っていること。 | なし | `model/test_core.py` |
| T02 | 純粋式の決定性・全域性・型安全性（3.3） | 数学的整数・Bool・有限積。一般再帰、通信、Int64除外。 T01の型付き入力。 | なし | `model/test_core.py` |
| T03 | 構文上の読取集合に関するframe性（3.3） | 依存集合上で同じ状態、同じ局所環境。 隠れた外部入力・副作用がないこと。 | 1, 3, 4 | `model/test_core.py` |
| T04 | 式のlow非干渉（3.3） | 固定schemaの値結果のみ。タイミング・transcript除外。 低labelの入力が一致し、式の結果labelがobserver以下。 | 41, 42, 43, 44 | `model/test_core.py` |
| T05 | owner側代入の型・frame・生成先（4.1） | 一つの同owner RMW。別ownerは不変snapshot境界。 使用時権限はT12。実通信との対応は別。 | 1, 2, 3, 4, 5, 6, 7, 8 | `model/test_core.py` |
| T06 | ループなしcommandのwlp正確性（4.2） | 孤立した型付きcommand、意味的predicate。 外部干渉なし。SMTの全predicate判定を主張しない。 | なし | `手証明のみ` |
| T07 | while不変条件とvariant（4.2） | partial correctness、終了するbodyとvariantによる条件付きtermination。 assertionは各ループ境界で成立。並行干渉は別。 | 53, 54, 55 | `verification/vcs/` |
| T08 | 区間refinementと正の整数契約（5.2） | 整数の加減算、abs、外部へ出すn>0。 数学的整数。境界の値が同じこと。 | 9, 10, 11, 12, 13, 14 | `model/test_local_theories.py` |
| T09 | 排他的領域tokenの分離と消費（5.3） | allocate/split/move/releaseの抽象有限モデル。 fresh block/token、現在policy、原子的遷移。malloc証明ではない。 | 51, 52, 60, 61 | `model/test_local_theories.py` |
| T10 | 環境安定性modalityの導入・使用・保存（5.4） | 定義したR*上のBox。MTT全体ではない。 実際の環境遷移がRに含まれること。 | 50 | `model/test_local_theories.py` |
| T11 | 異なる局所理論の契約合成（5.5） | 契約に対応したM1/M2、安定した境界前提。 局所契約の意味対応は各実例の証明。任意pluginの検査器ではない。 | 12 | `model/test_local_theories.py` |
| T12 | 現在権限と退役世代の使用拒否（6.2） | 明示的な信頼境界と原子的owner使用検査。 暗号・hostの真正性は理想化。古い値の存在と使用を分離。 | 24, 25, 26, 27, 28, 29 | `model/test_core.py` |
| T13 | 要求keyごとの高々一回のmutation（6.2） | 任意有限履歴、key非再利用、台帳非削除。 writeと決定記録が原子的。crash途中・exactly-once除外。 | 30, 31, 32 | `model/test_core.py` |
| T14 | 理想化輸送に対する有限trace refinement（6.3） | 抽象非同期ソース機械とコピー/遅延/破棄を持つモデル。 理想的な発行照合、同じservice規則。Rust/QUIC証明ではない。 | なし | `model/test_core.py` |
| T15 | 条件付き進行（6.3） | 一つの要求の最終配送とservice/consume。 公平性、必要配送、有限資源、権限/版/対象の安定。 | なし | `手証明のみ` |
| T16 | 有限DAGと自然数rankの同値（7.1） | 任意有限有向graph、初期単一root不要。 辺の意味はgraphごとに区別。 | 15, 16 | `model/test_core.py` |
| T17 | pure fallback正規化の結合則（7.2） | 副作用のないguard、同じ文脈、左からの候補順序。 試行effect/rollbackの一般結合則ではない。 | 22 | `model/test_core.py` |
| T18 | fallback単調性・選択妥当性・条件付きtotality（7.3） | 任意有限chain、nを枯渇とするcursor。 有効性は現在の一貫した判定。reacquireは別lineage。 | 18, 19, 20, 21, 23 | `model/test_core.py` |
| T19 | checked state/operation追加の保存（8.3） | 任意有限候補全体を先に検査した後の原子的追加。 既存意味の非変更、新世代、grantの別発行。 | 1, 15, 16, 40 | `model/test_extension_checks.py` |
| T20 | コード交換と利用側契約の保存（8.4） | 基本型契約と、別に証明した強い置換契約を区別。 局所quiescenceと現在の完全依存stamp。任意migrationは未実装。 | 36, 37, 38, 39, 40 | `model/test_core.py` |
| T21 | 閉包撤去・reparentの保存（8.5） | 子孫閉包、関連operationの退役、exact DAG再検査。 一般owner migrationではない。 | 17, 25, 26 | `model/test_core.py` |
| T22 | 局所安定性と独立patchの可換性（8.6） | 完全なread/write/meta/authority/freshness footprint。 端点だけのdisjointでは不足。多owner原子性は未実現。 | 3, 4, 33, 34, 35, 40 | `model/test_extension_checks.py` |
| T23 | 任意の有限回の継続進化（8.6） | 上記保存規則からの履歴長帰納法。 追加した全拡張が対応する保存義務を満たすこと。liveness別。 | なし | `model/test_core.py; model/test_extension_checks.py` |
| T24 | 最小checkpoint completion（9.3） | 任意有限の完全な閾値入力に対する終了/健全/完全/最小。 未知messageのない依存入力。実storageなし。 | 57, 58, 59 | `model/test_checkpoints.py` |
| T25 | 固定checkpoint間のZ-path特徴づけ（9.4） | 最終checkpointがそろう有限horizon。Z-cycleは単一固定の場合。 既知結果の有限モデルでの再導出。実OBL-014受理なし。 | 59 | `model/test_checkpoints.py` |
| T26 | cutの再生一意性（10.1） | 比較不能な出来事の交換で結果が変わらない場合。 全競合が追跡され、交換の有効性も保存。実I/Oは再実行しない。 | 3, 4, 45, 46, 47 | `手証明とSMT補助条件` |
| T27 | 現在制御を保持した復元のno-stale-use（10.2） | 失効集合和・現在世代・use-time guardによる拒否。 現在の制御状態の取得を仮定。完全Load/durability定理ではない。 | 24, 25, 26, 48, 49 | `model/test_core.py` |
| T28 | 固定schemaの観測値非干渉（11.1） | fieldの存在条件と値がlowである有限snapshot。 同schema/observer policy。全trace・時刻・行数は除外。 | 41, 43, 44 | `式レベルのテストからの手証明` |

## 全体の未確立事項

既存のRust/QUIC実装、一般的な局所理論loader、全module calculus、分散durability、全観測trace非干渉、WAN/敵対hostの保証をこの台帳で受理しません。個別の前提を「正しいから」と仮定して、その実装を証明済みにしません。

手証明とcode/modelの間にも未機械化の対応があります。実行ログは少なくとも記録された有限caseの挙動を示しますが、手証明の全量化範囲を検査したものではありません。
