# 反例・修正・検査範囲

## 実際に見つかった問題

以下はF0.3の新しい研究実装の問題であり、GitHubの既存Mir実装に同じ問題があると確認したものではない。

### A1. deep copyした証拠のPython object等値

最初のsnapshot照合はdataclass全体の通常等値を使っていた。内部ASTにPython object identityが含まれるため、同じ意味内容をdeep copyしたcheckpointを拒否した。最初の51テストで3 failure・2 errorとなった。意味recordの構造的なcanonical表現を比較するよう修正した。

これは不正な受理ではなく、正しいcheckpointを受理できない問題である。`evidence/first_run.log`と`second_run.log`を保持する。

### A2. 退出した構成へのlive結果の消費

serveの後に利用側moduleが退役しても、consumeがmemberとpolicyしか見ていなかったため、結果をConsumedへできた。`review_retired_consumer_RED.log`に実際の `Consumed != ReleaseDenied` を保持した。consumeで利用側moduleと要求に束縛した全live依存を再検査するよう修正した。過去のstore変更とServed事実は消さない。

この修正はlive-result profileの選択に対応する。歴史的snapshot値の別契約を禁止するものではない。

### A3. 書込み対象の存在からの情報流

秘密のcellへ定数を書き、公開の0を返す操作を、右辺のreadだけから低いlabelと判定していた。対象の存在/有効性によってacknowledgmentが変わるため不十分だった。`review_write_target_label_RED.log`に実際の `0 != 1` を保持し、write targetも潜在依存へ加えて修正した。

現モデルは値と構造のlabelを一つにした保守的profileである。より許容的なwrite-upを導入するなら、公開された構造の存在条件と秘密値を別にモデル化する必要がある。

### A4. 型付き引数とPythonのBool/Int等値

Pythonでは `(True,) == (1,)` なので、通常のdataclass等値だけではκの型付き引数を正確に比較できなかった。`review_typed_context_RED.log`は、誤った再検査が例外を返さなかった記録である。型を含む構造等値を使うよう修正した。

これは、数学上のtyped equalityを実装言語の `==` と無条件に同一視しない必要性を示す。

### A5. 準備中の反復readによる古いstampの消去

同じkeyを複数回読むと、Viewは最後のstampで前のstampを上書きしていた。二つのlookup間に外部更新が入る実現へ広げると、前の値を使った判定の根拠を消せる。`review_repeat_read_RED.log`に、異なる値への更新とABAの二つの実失敗を保存した。最初のstampと異なる再読取りは`UnstablePreparationRead`で検査を中断するよう修正した。

実験はViewのlookup間へ明示的にStore.writeを入れており、実threadレースを実行したものではない。各(value, stamp)の取得とStore.write自体の原子性は、参照モデルの前提として残る。実装側で同期せず二つのdictを読むことを許す保証ではない。

### A6. anchorの現在revisionを再構成していないaccess検査

access証拠のκにはanchorの構造revisionが含まれていたが、再検査に過去のκ自身を再び渡していた。そのため、同じanchorをreparentしても旧revisionのaccessがliveのままになった。`review_access_revision_RED.log`に実際の失敗を保持した。現在のanchor revisionを再構成して比較し、このprofileでは古いaccessを無効にするよう修正した。

これは「κを束縛している」という表現だけでは足りず、使用点が現在の比較対象を構成している必要を示す。すべての変更で必ず権利を失わせる最終仕様を採用したものではない。今回選んだrevision-bound access契約に合わせた修正である。

### A7. 構造化したcheckpoint／prepared deltaでも型を失った等値

κだけを型付き比較に直しても、別の封印境界に通常のPython等値が残っていた。checkpoint内のactivity引数`1`を`True`へ置き換えた場合と、準備済みdeltaの書込み値`1`を`True`へ置き換えた場合が、いずれも誤って受理された。`review_snapshot_typed_equality_RED.log`と`review_prepared_typed_equality_RED.log`に実際の失敗を保存した。

`model/identity.py`へ型を保つ構造等値を分離し、認可文脈だけでなくcheckpointのcanonical record、prepared delta、reparent封印にも同じ規則を適用した。ASTの正しいdeepcopyを拒否しないため、checkpointはcanonical表現を型付きで比較する。これは別の攻撃対象を扱う安全な任意Python deserializerではない。

## 設計時に確認して入れたguard

次は破れた実行を記録してから修正したとは主張しない。adapter作成時の照合で気づき、負例を追加した条件である。

- TotalBody証拠の前提を証拠提出側から取り込まず、受入れ側の期待前提と比較する。
- TotalBodyからPartialへの無断の弱化を交換時に拒否する。
- 現在のdurable headをsnapshot提出側が過去の値へ指定できないようにする。
- checkpoint対象のDataに未宣言fieldがあれば成功として保存しない。
- high claimの発行件数がlow claim IDを変えないよう、counterをlabelで分ける。

## 故意に弱めた実装の反例

`verification/run_mutants.py`は、六つの変更をそれぞれ隔離copyへ適用してテストする。全て、対応する単一の振る舞いassertionで失敗した。構文エラーや未定義APIを反例として数えていない。

| 弱化 | 観測された破れ |
|---|---|
| policyの主体/member束縛を除く | aliceのidentityとbobのpermissionを合成できる。 |
| serveでの現在認可の再検査を除く | 発行後に失効した要求がstateを変更する。 |
| 許可射影の前に観測bufferを切る | high行だけの追加で公開行が押し出される。 |
| accessの現在性再検査を除く | 権限失効後もprimary accessがliveのままになる。 |
| support certificateの閉包条件を除く | 導出可能なrootまで落とした全拒否証拠を受理する。 |
| 不在lookupを追跡しない | 不在条件が変更されても準備済み検査を使用できる。 |

詳細は `evidence/MUTANTS.json` と各logにある。これは意図的に挿入した反例であり、修正前の既存リポジトリの欠陥数ではない。

## 有限順序検査

同じsource-derived pending requestに対して、serve、operation revocation、access revocation、module retirement、consume、observeを各一度だけ行う全720順序を実行した。4,320 stepには、型付きで拒否された操作も含む。各prefixでgraph/currentness/invariantを検査し、owner stateの変更は高々一度、fallback cursorは同lineageで非減少であることを確認した。

この実験は任意回のretry、任意人数、任意のfault、一般的fairnessを全探索したものではない。一般の主張は `theory/PROOFS.md` の前提付き手証明と分ける。

## 入力資源profile

Formulaの参照実装には深さ128以下の明示budgetがある。数学上の最小不動点・帰納的導出の定理は任意の有限式について述べるが、checkerとの実装対応はbudget内に限定する。これは全体言語の最終制約ではない。通常のPythonメモリ/stackやhostの可用性も理想的な数学的計算と区別する。

## 残る監査上の限界

独立した第三者レビュー、全文の証明kernelによる機械検証、Rust/QUICコードの監査、guest sandbox、物理的storage障害、任意の動的情報流は行っていない。

参照実装の公開Python objectへhostからアクセスできることを、安全な本番APIだと扱わない。攻撃者に許される操作は、本文で定義したtyped boundaryである。型封印や暗号的束縛をproductionでどのように実現するかは、別の実装義務である。
