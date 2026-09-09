# F0.2 命題・証拠台帳

F2-01〜F2-27はPROOFS.mdに記載した**手証明**です。全文のLean/Rocq検査や独立査読を通過したという分類ではありません。モデルコードとの対応も全文機械化されていません。SMTのSAT/UNSATは個別の式の結果であり、この27項目の一般的な証明を代替しません。

| ID | 主張 | 厳密な範囲・前提 | 主な実行側の検査 |
|---|---|---|---|
| F2-01 | checker終了・相対健全性・完全性 | A.1の宣言制限、明示署名、固定局所型、Int/Bool、一階task。全意味的安全programへの完全性ではない | `test_source.py`, `test_language.py` |
| F2-02 | owner bodyの型・frame・失敗非更新 | while/再帰/外部callなし、同owner mutable reads、stagingの一括更新 | `test_source.py`, `test_engine.py` |
| F2-03 | 式CFGと継続の対応 | fresh register、左からのsnapshot意味、callee入口/帰還、同じ結果供給 | source/AST/CFG differential |
| F2-04 | command/再帰stackの有限prefix対応 | source/target継続の同時simulation。無条件の終了性は主張しない | 反復・再帰・相互再帰・分岐・return |
| F2-05 | structured futureの非循環・回収 | 子futureは作成した親内、first-class輸出不可、root completionで子をdrain | child failure、join、局所並行 |
| F2-06 | 多項式certificateの健全性 | exact rational polynomial、前提ge>=0/eq=0、6種の検査規則。checker実装の機械証明ではない | `test_certificates.py`, 個別SMT |
| F2-07 | sourceからのsymbolic path/VC対応 | loop-free integer body、条件枝、assert成功経路、衛生的result、entry invariants | forged source/AST、result capture負例 |
| F2-08 | Partial export契約 | 成功した場合のI/post。assert失敗の除去や成功可能性を保証しない | partial証拠、改変/省略拒否 |
| F2-09 | TotalBody契約 | 明示I/P下で全assertの事前証明、有限body。ネット到達/現在許可とは別 | stronger guard、異なるpre、partialへの降格拒否 |
| F2-10 | owner不変条件の合成的閉包 | Iはowner-local、すべてのmutator/交換/移行/importを検査、外界direct writeなし | foreign invariant、unchecked mutator、更新後保存 |
| F2-11 | 異なるlibraryの不変値の契約接続 | 実際のclient source、同じ不変値、calleeの指定した前提部分のみ | `prove_call_chain`、AST差込み・中間代入拒否 |
| F2-12 | 使用時権限・版の保持 | trusted issuer/current policy、op/code/contract/epoch束縛。暗号/host真正性は前提 | stale/cross-target/revoke、proof非grant |
| F2-13 | state-only durable高々一回 | write+決定key+結果が一durable record、単一writer、prefix保証、append不確定時は再open | 実file truncation/再送/不明error再open |
| F2-14 | 継続の一回再開 | issueとkey/継続、consumeと再開が局所原子的durable操作。prototypeでは理想化 | crash_resume_fiber、duplicate response |
| F2-15 | ideal transportの有限trace refinement | 非捏造なmessage、source-derived service、同じ決定。Rust/QUIC証明ではない | drop/duplicate/reorder/explicit resend |
| F2-16 | 外部Started後の非実行を推定しない | 二つのcrash履歴で同じdurable情報、物理callは0/1。相手側の追加idempotencyなし | `test_ifc_effect.py` |
| F2-17 | 忘却floor以下の再実行抑止 | durableなprefix settled acknowledgement、floor先行保存、incarnation非再利用 | journal prefix/rollback/replay |
| F2-18 | addition/replacementの保存 | actual schema、基本署名、footprint、I、post、TotalBody pre、quiescence、現在epoch | source addition、schema再解釈・保証弱化拒否 |
| F2-19 | 任意有限cohortのactivation不変条件 | 単一正直coordinator、durable Prepare/Decision、全YES、表D.1の遷移 | 1/2/3 participants全reachable states |
| F2-20 | patchの抽象原子性 | Commit後の未適用ownerはfenced、Preparedだけshadowを使用。通常multi-read原子性ではない | late request、mixed state、適用後の更新 |
| F2-21 | fence所有と局所性 | patch固有id、完全D/W、競合prepare拒否、他者fenceを解放しない | 複数patchの未準備Abort反例 |
| F2-22 | 条件付き進行/停止性の限界 | 最終的復帰と通信、既存要求settlement。permanent故障ならblocking可能 | reachable stateからの完了、coordinator crash |
| F2-23 | checked migrationとactor接続 | 各移行F2-10、準備後scope凍結、F2-19、durable PatchRecord/epoch | `test_live_patch.py`, 統合例 |
| F2-24 | fresh importの保存/非復活 | quiescence、実際code更新、全proof再検査、新instance、grant空。archive真正性/完全resumeではない | snapshot/code/evidence改変、fresh grant |
| F2-25 | affine関係DAGのprojection | acyclic topo評価、exact frontier/incarnation、定義label+入力label、許可されたinvalidate | `test_relations.py`; F0.1 T18を使用 |
| F2-26 | 逐次終了非依存のlow trace非干渉 | 同low初期、同command、両方終了、pc規律。時間・資源・分散scheduler除外 | high loop/secret行数/implicit flow |
| F2-27 | 選択profileの統合された有限進化の安全性 | 前記の具体component前提、完全なmutator閉包、理想auth/storage、合成時のscope検査 | 統合例と各suite。全体系の機械証明ではない |

CE-01は、単一trace inclusionだけでは二実行の非干渉を保存しない具体的反例です。F2-27へF2-26の機密性を自動的に含めていません。現在の`.mirx`profileはpublic-onlyであり、独立IFC componentを統合したprivate Mir言語の証明は未完です。

## 非空性と相対完全性

F2-01は、実際に実行できる反復・再帰・呼出しを含む選択構文への完全性です。証拠検索器の完全性ではありません。F2-06〜09は、提出されたcertificateの検査であり、不足する証明を発見できない場合はUnknownになります。F2-19は任意有限nへの手証明、1/2/3は別の有限モデル検査です。

## TCBと未成立の強い主張

parser、checker、VC generator、certificate kernel、runtime、journal adapterの全実装がこの手証明どおりであることは、コードに対する機械証明で確定していません。Pythonのデータクラス・辞書・hashを、悪意あるnative codeへの境界と見なさず、実装時にはtrusted kernel/adapterとsandboxの境界を実現する必要があります。

任意の局所理論loader、full dependent/MTT calculus、匿名higher-order closure、多owner共同不変条件の一般規則、Byzantine coordinator、無条件liveness、全観測非干渉、OS/ネットワーク/永続化まで含むend-to-end proofは未成立です。

F0.1のT01〜T28と今回の番号は別の台帳です。既存リポジトリのTHM/OBL番号の受理状態は変更していません。
