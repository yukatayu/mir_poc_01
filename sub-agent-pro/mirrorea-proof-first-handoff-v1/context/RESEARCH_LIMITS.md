# 継承してはいけない過大評価と、候補間の不一致

## 入力の成熟度

F0.1は小さな式／状態、fallback、DAG、checkpoint completion、Z-path、局所理論の具体例。F0.2は第一階の反復・再帰・同一locus fork/join、算術証拠、owner invariant、分散patchの2PC型モデル、journal等。F0.3はsupport closure、同一κの多層認可、参照/fallback、契約、要求・live結果、OCC、観測、全image replayを一つの抽象状態へ接続する。

この順は、全機能の包含順ではない。F0.3の統合VMはF0.2のspawn/joinを未対応として拒否し、外部effectも全て統合していない。F0.3をコピーすれば前二版も完成という読みは禁止する。

F0.3の「24命題」は手証明であり、実Rust/QUICや全Mirの機械証明ではない。F0.2の27、F0.1の28と単純加算して全体の強さを示さない。有限caseの全探索、SMT、unit tests、proof-assistant kernel確認を別々に記録する。

## 互換と見なしてはいけない選択

1. F0.2のprepare後の権限予約と、F0.3のcommit時の厳密な現在認可は別の意味。Q-18を解かずに貼り合わせない。
2. F0.2のfresh instance importと、F0.3のtrusted current headへのsame-instance recoveryは別の能力。
3. F0.3の全image・全状態を持つ抽象モデルは、全世界に唯一の中央controlplaneを要求する設計ではない。分散実現の線形化・整合の義務が別にある。
4. 数学的IntとBoolのモデルはInt64/float/textの意味や実storageを完成させない。overflow/NaN/codec/型付き同一性は別契約。
5. all/anyの帰納的supportは、任意の否定・委譲・主体変換を含むpolicy calculusではない。live/nonliveで全意味を還元しない。
6. F0.3のtyped trusted control APIは、通常sourceのmodule/patch生成言語ではない。A/Cへ構成を追加するsource経路の不足は残る。
7. 固定label・同low scheduleの二実行保証は、任意のscheduler・秘密依存の終了・動的declassification・物理timingを含まない。
8. readonlyは観測の無影響・非漏洩・忠実性の全部ではない。active debuggerは別の認可された作用。
9. 理論側の原子的commitは、実行側のlock/queue/durable protocolが既に正しいという証明ではない。
10. private Python属性、hash、構造等値は、guest sandbox、署名付き認証、未信頼入力の真正性を代替しない。

## 実際に引き継ぐ反例

- fallbackの位置が履歴最大値以下という条件だけでは後退を禁止できない。
- 個別DAGの検査だけでは、種類をまたぐ根拠循環を検出できない。
- 変更先が非重複でも、read／到達性／不在の依存が競合する。
- incarnationなしのversionだけでは、削除後の同名再生成によるABAを拒否できない。
- prepareで同じrecordを再読取りし最後のstampへ上書きすると、最初の判断の根拠が失われる。
- 古いκ同士を比較しても現在性の検査にならない。現在recordから期待文脈を再構成する必要がある。
- ownerの成功後にcaller/moduleが退出しても、成功の履歴は消せない。ただしlive結果の使用許可は再検査する。
- 成功時の事後条件だけでは、assertを追加して成功入力を減らす交換を防げない。
- 全mutatorを閉じないと、証明済みstate invariantを別操作が破壊する。
- metadataのhashがあっても、ソースと証拠と引数が正確に同じ対象へ束縛されているとは限らない。
- 秘密状態の存在が処理の成否に出る。右辺の値だけをlabel付けしては不足する。
- 保持を先に行い後で秘密行を除くと、秘密の追加によって公開行が押し出される。
- 古いsnapshotだけでは、その後の失効を知れない。current headの独立根拠が必要。
- state-only journalの再送規則を、外部effectのunknown-startへそのまま適用できない。

各REDは元のAUDITとlogを確認する。意図的mutantと自然に発見した研究モデルの欠陥、既存Mirの欠陥を区別する。

## 軽微な収録時の注意

F0.3のDECISION_DISPOSITIONS.md/Q-22には95 testsという古い表記がある一方、最終VALIDATIONは98/98を記録している。原本は修正せず、最終実行logと新しい追試を根拠にする。この種の数字の整合性は証明の成立と別である。

以前の「全部通読済み」「全体理論が完成」「もう実装だけ」という意味への拡大は引き継がない。提供済み資料を全収録しても、実repo全件の監査が完了したことにはならない。
