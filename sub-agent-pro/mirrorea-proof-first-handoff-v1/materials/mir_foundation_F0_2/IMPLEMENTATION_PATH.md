# 理論候補から実装へ進む経路

## 到達した位置

F0.2は、実装項目を列挙した計画だけではない。通常source、CFG、操作registry、証拠kernel、owner不変条件、要求と継続、動的追加・交換、状態移行のprotocol、fresh importを、具体的なモデルと手証明へ落とした。選択profileについて、どの意味をどの境界が実現するかは以前より明確になった。

一方、既存Mirへそのまま移植すれば最終目標が達成される、とはしない。採用する意味の選択、証明の独立検査、実装とのrefinement、任意の理論を扱う拡張が残る。以下を新しいcurrent roadmapやI3-3の追加gateとして自動適用しない。

## 1. 理論上の候補として固まった構造

中心は三つの対応である。

**記述から実行へ。** 純粋計算、owner-localな有限状態操作、非同期のtask/継続を分ける。通常の代入やcallから実現上の要求を生成するが、sourceでEventを操作させない。

**局所保証から境界へ。** 内部の証明手法を統一せず、common contractへ意味を移す。証拠検索は外、受理kernelは小さく保つ。新しいmoduleがIを壊さないためには、外へ出す結果の証明だけでなく、Iに作用する全操作の閉包が必要である。

**旧構成から新構成へ。** 更新を型検査されたstate/code変換とし、依存範囲、権限、未完了処理、永続化を接続する。F0.2では停止範囲付きのexplicit coordinationを実現候補とした。

この構造は、world固有の語彙や単一の物理serverを必要としない。coordinatorは一つのpatchについての役割であり、全システムの永久rootではない。

## 2. Canon採用前に決める意味

| 選択 | F0.2の候補 | 無断で同一視してはいけないもの |
|---|---|---|
| taskからの状態read | occurrenceごとのowner snapshot。owner assignment内部は一つのRMW | 一式全体のcoherent snapshot、無条件latest、sourceの最終文法 |
| retry | state-onlyの明示再送で保持結果を返せる。再評価しない | 既存の全operationのduplicate policy、exactly-once |
| owner body | 有限のloop-free原子操作、task側に反復/再帰 | 任意長の処理をowner transactionにすること |
| patch authority | Prepared時の限定予約をDecisionまで保持する候補 | 即時失効を要求するpolicy。実issuerでも同じ予約が必要 |
| distributed activation | explicit 2PC型、障害下でblocking可 | 普通の代入へのhidden transaction、nonblocking agreement |
| restore | restart/recovery、fresh import、隔離された再生を区別 | 過去snapshotだけから現在の権限を復活させること |
| information flow | public-only executable source + 別の逐次IFC証明 | private distributed trace/timing/resourceの非干渉 |
| local theories | concrete polynomial certificateと抽象的contract境界 | 任意axiom/inference ruleを無条件にloadすること |

これらは実装の命名ではなく意味の選択である。ownerが違う意味を要求するなら、対応する証明とprotocolを変更する。安全性を弱める事後解釈で済ませない。

## 3. 既存Mirへの対応候補

| 既存側の責任 | 持ち込む候補 | 採用のために示すこと |
|---|---|---|
| M6/M7 source/check/elaboration | named operation、task継続、明示署名、source-bound VC | 現行syntaxとの対応、Int64 overflow等、受理/拒否の保存 |
| per-locus projector / SYS3 | callsiteからのartifact/edge生成 | 操作の型・owner・effect/failure・source identityが欠落しない |
| M9 / authority | proofとgrantの独立、現在世代、公開契約とI | mathematical evidenceの適用と現在の権限を区別する |
| SYS4/SYS5 / I3 transport | continuationを保持するrequest/result機械 | 実codec/QUIC/queue/reconnectがF2-15の関係を満たす |
| I4 persistence | 同一recordのstate+decision、未確定書込み時のfault/reopen | hostのdurability契約、journals、継続、key非再利用を統合する |
| I4 live patch | source migration、fence所有、binding epoch、prepared image | 全componentのsnapshot/activation義務、実durable Prepare/Decision |
| I5 participant/View | observer-safe representationとtyped input | sandbox、資源、label、表示の因果対応。world semanticsをViewへ渡さない |
| local theory extension | data-only evidenceと共通契約 | 独立theoryの意味対応とproof-boundaryをkernelに根拠付きで追加する |

現在のRustコードを監査・変更したわけではない。上表は既存層へ移す際のrefinement義務であり、「I3のテストが通るからF2-15は証明済み」のように読み替えない。

## 4. 実装へ進む前の最小の証明ゲート

**source/CFGと証拠のゲート。** F2-01〜04/06〜10を、選択した証明支援系上で定義に沿って再構成する。caller/callee対応の再帰を、同時small-step simulationとして閉じる。既存検査器とのdifferentialは補助とする。placeholderの`SafeExtension`を仮定して全体の健全性と呼ばない。

**更新・復旧のゲート。** F2-19/20/23について、実際の永続状態・cache・準備像を対応関係へ入れる。とくに権限予約、影響範囲の完全性、未完了要求のsettlementを選択する。分散patchの途中でcoordinatorが復帰しない場合に停止し得ることを、failure契約と利用者向け説明へ残す。

**情報境界のゲート。** public-only prototypeの結果をprivate languageへ一般化しない。sourceのpc、metadata、objectの存在、終了・資源の影響をどこまで観測に含めるか定義し、二実行の対応を別に証明する。

ここで必要なのは全将来理論の完成ではない。ただし次の層が依存する保証を、完成済みと扱う前に閉じる。未証明部分を明示して実験することと、その上を確定製品として拡大することは分ける。

## 5. α版に至る一貫した能力の受入れ

最終文法を固定せずとも、次の利用経路を通常のsourceから実現できることを要求できる。

1. 一つの構成を定義して複数のlocusへ配置し、通常の代入・関数・反復で操作する。
2. 別々に定義したmoduleを、型と必要な意味契約によって接続する。新moduleごとにruntimeへ名前固有の分岐を追加しない。
3. 検証したmoduleに不正なmutator/証拠/権限を差し込むと拒否し、正しい拡張は実行する。
4. 実行中に対応範囲内の構成を追加・変更し、影響しない活動を継続させ、更新の前後を観測できる。
5. 実装契約に従う停止・再起動の後で、状態とコードと未完了処理の意味を保持する。fresh importなら権限を別に付与する。
6. 意図したViewから入力し、結果・失敗・未確定・拒否をsourceと対応づけて観測する。

F0.2がモデル上で接続したのは1〜4と限定fresh importの主要部分である。real multi-process deployment、全componentを含むdurable session、safe Browser/Viewは未実装である。別々のdemoを足しただけでα完成とはしない。

## 6. 残件を三種類に分ける

| 種類 | 主な項目 | 現在の評価 |
|---|---|---|
| 主に実現/refinement | 実メッセージ認証、source/artifact binding、durable継続、fenceとjournalの実OS表現 | 抽象的な契約・失敗・対応関係の候補がある。実装しただけで保証済みにはならない |
| 明示的な意味選択 | read policy、patch権限reservation、blocking、restore種別、retention | 先に選択して証明へ反映する必要がある |
| なお本質的研究 | 任意局所理論のmeta-language、higher-order/first-class handle、動的moduleの一般interface、多owner invariant、広い観測非干渉 | 今回の証明と一般性を混同しない。単なるコード量の問題ではない |

この分類により「全部が未知」でも「もう研究は不要」でもない位置を示す。選択profileに限れば、具体的な実装設計と検証経路がある。最終WWW的基盤に必要な一般性は、そのprofileの保守的拡張として成立するかを引き続き検査する。

## 7. 長期目標を保つための境界

大域的な単一server、固定cohort、固定個数の関係anchor、固定testcaseの操作名を、最終Coreの法則へ昇格させない。第一級の参照やmodule instanceを追加する際には、同じraw名を再利用せずincarnation付きで抽象化する。型や証明を弱い側へ渡すときに必要な所有・寿命規律を消さない。

この研究候補を採用する目的は、独自の小さなPython言語を製品化することではない。通常の記述、局所保証、分散実現、進化の間をつなぐ規則を検査し、Mirの基礎として使える部分を選別することである。
