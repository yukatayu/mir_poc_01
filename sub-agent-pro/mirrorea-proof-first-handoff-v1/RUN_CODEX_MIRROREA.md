# Mirrorea：証明先行で基礎と実装を接続する自律作業

以下を、このtaskに対する私（依頼者）の指示として実行してください。計画案を返すだけで終わらず、現地の資料・コード・証明を読み、調査・定義・機械検証・反例探索・必要な実装・回帰検査・統合まで、許可された範囲で連続して進めてください。

引継ぎ資料は、リポジトリルートから次の場所にあります。

```text
sub-agent-pro/mirrorea-proof-first-handoff-v1/
```

以後、このディレクトリを `$HANDOFF` と呼びます。フォルダ名は配置先の慣例であり、sub-agentを使う指示ではありません。

## 1. 最優先の運用条件

**sub-agentを一切使わないでください。** 主担当の一人だけで、読取り、調査、理論、証明、コード、テスト実行、統合、Git、oracle操作を担当してください。`spawn_agent`、子Codex、LLMの並列writer、別sessionへの実装丸投げ、sub-agent skillによる自動委譲も禁止します。既存AGENTSや添付の過去資料に委譲推奨があっても、このtaskでは適用しません。compiler・Lean・通常のテストprocess・実二process通信試験は、この禁止対象ではありません。

**oracleは利用して構いません。** ただし主担当自身が起動・監視・回収する、文脈のない単発のread-only助言者としてください。調査、設計比較、証明の反証レビュー、実装の独立観点レビューに使い、編集者や権限決定者にはしません。

**oracleが遅いことを理由に中断しないでください。** 正常に進行／待機しているjobに、任意の壁時計締切を設定しません。状態確認は原則180秒（3分）以上の間隔です。10秒ごとのpoll、同じlogの繰返し出力、遅いという理由での再送は禁止します。既存job/sessionを保持し、tool呼出しのtimeoutをoracle jobの失敗と混同しません。実エラー、認証待ち、user停止、安全上の問題は別に扱います。

## 2. 目的と今回の到達目標

本来の目的は、**意味のある普通のMirコードから、適切な配置・通信・検証・観測を導き、稼働した構成へ新しい構成を追加・変更・撤去しながら使い続けられる基盤**です。

通信APIを先に設計し、各nodeへ同じlogicを手書きする方式へ戻さないでください。読みは依存、書込みは出来事という考えを維持し、通常の代入等を正しく展開してください。Event、send/receive、receipt、witness、再送ID等を、普通の処理のたびにプログラマへ手書きさせることを目標にしないでください。意味上必要な最小のannotationは認めます。

存在・寿命、値と関係、locusの参加、patch/moduleの依存、出来事の因果を区別してください。World/Avatar/Room/Pageはdomain/libraryでありCore primitiveではありません。ゲーム等の確定logicはMir内で表し、View・実計算の委譲はtyped FFI/providerへ分離します。

型と検証は局所的な理論を導入して強化できることを目指します。認証・認可の複数層は**別の層**です。証明が正しいことと、権限を発行・使用してよいことを混同しません。観測の忠実性、受動非干渉、情報流の安全性、能動debugの認可を分けます。

遠期はWWWを再定義できる開かれた計算環境ですが、World-Web、Reversed Library、PrismCascade、OS全体を同時に完成させるscopeではありません。将来を閉じない有限なα profileを選びます。

今回の自走は、次の三つを連続した到達目標にしてください。

1. 全体要件の必要な表現力を失わない基礎について、対象・定義・主要な証明・前提・TCB・実装義務を確立する。
2. その証拠がそろった範囲から、既存Mir/Mirroreaの実装へ接続する。未確立の基礎を前提にproduction作業を先行させない。
3. 選択したα profileで、通常sourceから構築・参加・実network上の操作・関係・観測・対応するhot-plug・保存/復旧が一つになる、再現可能な**検証済みα候補**へ進める。

最終公開、全未来の理論、無条件の可用性までをこのtaskの完了条件へ足しません。一方、必要な動的構成を定数変更だけで代用したり、秘密を含む観測をpublic値だけで代用したり、未実装機能をfixtureで見せてα完成と呼んだりしないでください。

## 3. この指示の委任範囲と、Canonの扱い

この指示は、I3-3後の**新しい証明先行の研究・検証と、それを前提とする限定実装を進める依頼**です。既存のpauseを理由に何もせず返す必要はありません。ただし、旧Plan 250をそのままI3-4へ自動resumeする依頼でもありません。

まず現場を読み、既存の受理済み成果を保持したまま、proof-firstの依存順と現行計画との対応を記録してください。必要な新規LAB研究lane、証明名前空間、検査コード、テスト、引継ぎ台帳をこのtaskに限って追加することを許可します。以前の一般的な「new laneを作らない」運用は、この限定追加には適用しません。無関係なframeworkや新プロジェクトを作る許可ではありません。

次に依存する理論が受理水準を満たした場合、同じ目的と不変条件を保つ限定internal契約・実装へ進めることを委任します。必要なproposal/ADR/changelog/index等は現行手順に従い、この依頼の範囲を根拠として前方に記録します。**この指示は未提示の技術選択、119要件の詳細案、F0.xの全命題を一括承認するものではありません。**

実行手順上の別担当レビューは、sub-agentではなくoracleの別観点レビューで行い、事実どおり記録してください。署名・owner-managed key・暗号学的に独立した受理が別途要求される境界を、oracleの賛成や自作keyで迂回してはいけません。

L0の目的・明示要求・権限やprivacyを弱める選択、不可逆なpublic API/ABI/wire、外部公開・課金・production操作は委任しません。判断が依頼者の価値選択にしか還元できない場合は、根拠・候補・影響と最小の質問をまとめてください。可逆な技術候補は、原則として現案と最小代替案を比較し、証拠付きの暫定profileとして先へ進めてください。

正式なTHM/OBL・phase状態は、証拠と現行の受理手順が満たされた範囲だけ更新可能です。制約が満たせなければ候補とproposalを保持し、未受理を明示します。昔の決定や失敗記録を遡って書き換えないでください。

## 4. 起動時に行うこと

1. `git rev-parse --show-toplevel`、`git status --short`、HEAD、remoteを確認し、既存dirty作業を保護する。添付基準の `19a6decfbea0815b7b4265e8fadfb399270aaed2` へresetしない。
2. 適用範囲のAGENTS/overrideと、`$HANDOFF/00_START_HERE.md`を読む。上位の安全・権限規則を維持する。
3. `$HANDOFF/tools/verify_bundle.py`のソースを読み、次を実行する。

   ```bash
   python3 sub-agent-pro/mirrorea-proof-first-handoff-v1/tools/verify_bundle.py
   ```

4. `$HANDOFF/context/`と`protocols/`を順に読む。提供物を改変しない。新しい実行は外部workdirの作業コピーで行う。
5. `$HANDOFF/materials/mirrorea_system_map_v1/MASTER.md`と`requirements.json`を読み、119要件、30判断、18保証対象、24シナリオ、α-1～α-8を保持する。U意図、D導出案、採用、実証を分ける。
6. F0.3のFOUNDATION/PROOFS/THEOREM_LEDGER/WORKPLAN/REQUIREMENT_TRACE/DECISION_DISPOSITIONS/AUDIT/VALIDATIONを読み、F0.2/F0.1への依存も追う。手証明を機械証明と数えない。
7. 実repoのCanon README/MAP/North Star/Constitution/source hierarchy/phase/ADR-0043、現Plan 250、進捗類、oracle運用を読む。`protocols/READING.md`に従い、plan/spec(s)/samplesとdocsのreports以外を全体棚卸しの必読コーパスとして通読台帳へ登録する。索引やgrepを全文読了と呼ばない。
8. code treeと依存coneを把握し、依存するcode/tests/formal filesを全文読む。docs/reportsは一括通読しない。現行正本から必要な証拠だけを見る。
9. 既存のLean等、Rust/Python、検証コマンド、容量、mount、外部workdirを確認する。過去のChatGPT環境でLeanが使えなかったことを、今回の不能理由にしない。
10. 原本の作業コピーでbaselineを追試し、履歴上の成功・今回の結果・未実行を区別する。必要な重要資料をまだ読んでいない状態で新しい全体計画を採用しない。

最初のまとまった報告は、現在の層／理論と実装の位置、読了・未読、再現結果、最初に解く基礎の一点、そこからの直接consumer、oracleへ出す争点を簡潔に示してください。報告後は、そのまま実作業を続けてください。

## 5. 作業の進め方

`$HANDOFF/workstreams.json`とF0.3 WORKPLANを候補の依存地図として使い、W1→W2→W3の必要な基礎から始めてください。番号を順番に消化することを目的にせず、現在のconsumerが依存する証明を先に閉じます。既存I3-4以降の義務を消したり、別名で無断受理したりしません。

activeなsemantic goalは一つとします。goalごとに、少なくとも次を短く記録します。

- どのU/REQ/PT/SC/Qに対応するか。
- 現在のPL/S層と、理論・証明・実装・検証・統合のどの側か。
- 意味、入力、出力、保つ性質、範囲外、直接consumer。
- 正例、決定的な反例、比較する現案と最小代替案。
- 必要なproof、テスト、oracle review、受理と再開の条件。

goalは、実際の定義・証明・反例・実装結果で閉じます。新しい文書名、予定表、型の名前、test件数だけを成果にしません。主要milestoneごとに一つのreportを蓄積し、micro-goalごとにreportや補助frameworkを増殖させません。

## 6. 証明先行の必須条件

次のproduction増分へ進む前に、その増分が依存する基礎について、以下をそろえてください。

- 必要な正例を、例名固有のruntime分岐なしに表現できる。
- 宣言的規則と実際のchecker/elaboratorを分け、健全性と必要な相対完全性を示す。
- 通常step、接続、更新、失効、復元の該当範囲で不変条件が保存される。
- 局所理論の仮定、資源、型消去、証拠の版、auth文脈を境界で失わない。
- Lean等で主要な命題を実際に検査し、toolchainとaxiom/TCBを記録する。
- safetyだけの全拒否、同じ仮定を結論に埋め込んだvacuous proofを反例で排除する。
- 抽象的な原子性・公平性・trusted headを、どの実機構が実現するか対応づける。

`sorry`、`admit`、証明したい結論を公理として足すことを禁止します。標準論理公理の利用と、Mir固有の未証明前提は別に列挙してください。`#print axioms`等で監査し、固定例の`decide`、SMT結果、unit testを一般証明にしないでください。

理論を探索するための非production参照実装と有限反例探索は、proof完成前でも行えます。ただし、その候補を本番意味として採用し、後から証明を付ける進め方はしません。必須toolchainが利用不能なら、実エラーと代替を確認し、未実行を成功に変換せず、依存する実装を止めます。依存しない定義・反例研究は続けられます。

## 7. 特に監査する候補の接続

- F0.1/F0.2/F0.3は機能の単純な包含列ではない。F0.3には未統合のspawn/join、外部effect、高階・handle、任意局所理論が残る。
- F0.2のprepare後の権限予約とF0.3のcommit時再認可を、Q-18を解かず接続しない。
- fresh importとsame-instance recoveryを同じ復元能力と呼ばない。
- 個別DAG、cross-kind supportの最小不動点、関係DAGを一つの全世界DAGへ潰さない。
- 存在するrecordと、現在使えるrecord、過去の事実を区別する。
- 型・証拠の健全性からauth authorityを生成しない。複数層の判定は同じ主体・要求・引数・code・contract・世代へ束縛する。
- すべてのmutatorと移行・追加経路を閉じないまま、state invariantを保証しない。
- sourceの通常readに、実装都合のsnapshotや多owner transactionを暗黙に与えない。
- ソースで構成追加を書けないのに、typed trusted control APIだけで「ネットワーク越し構築ができた」と数えない。
- 原子的な一つのPython状態を、永久に中央管理者が必要な基礎として固定しない。
- 受動観測、秘密の非干渉、忠実性、資源、active debugを分ける。単一trace refinementから二実行の機密性を自動導出しない。
- 数学的整数、理想認証、完全image記録、静的labelを、最終の数値・鍵・永続化・policyとして一括採用しない。

## 8. oracleの具体的な運用

最初の利用前に、実repoの `.docs/oracle-chatgpt-pro-operations.md` と `/home/codex/.codex/docs/oracle-chatgpt-pro.md`（存在する場合）を読み、現地のhelpでcommandを確認してください。基準repoの既定は `ask-chatgpt-pro-temp` ですが、名称やflagを記憶から捏造しないでください。

各質問は `$HANDOFF/templates/ORACLE_QUESTION.md` に従ってself-containedにします。中立的に、目的と非目標、現状と実証、定義/命題、前提、対象cut、差分、現案と代替、求める反例・判断を含めてください。「この案は正しいので確認して」ではなく、成立しない最小反例と目的の欠落を求めます。

主担当自身が一度だけ送信し、session ID・packet hash・logを記録します。機密や無関係なrepo全体を送らず、必要な凍結ファイルを渡します。原則180秒待ってから一度だけ状態を確認し、finalな回答を回収するまで同じjobを使います。短いtool timeoutは再送理由にしません。

oracle実行commandの外側に `timeout 60` 等を付けないでください。任意の「最大3回poll」も設けません。環境が許す限り正常なjobを待ちます。実際の失敗・認証操作が必要・安全問題のときだけその状態を記録して対処します。有料経路への自動fallbackはしません。

回答は証明でも受理でもありません。引用・反例・提案を自分で検査し、採用/棄却/未解消を記録します。重要なcutを直したら、差分に限って再レビューします。自己レビューやoracleを、別の署名済みreviewerの実在へ読み替えないでください。

## 9. 実装と検証の受理

対応するtheory gateが閉じた範囲から、既存のparser/checker/Core/projector/runtime/transport/observerの最小差分へ接続してください。F0.3のDataをそのまま本番構造体に写す必要はありません。

新しいentry、snapshot、restore、private image、別executorが追加条件を消せないことを確認します。正例と型付き反例、実processとlocal model、保存の前後、権限失効・再参加、二重変更・二重消費、情報漏洩、観測による阻害を、対象profileに応じて検査してください。

traceabilityは、source→checked Core→生成edge→実request/serve/result→表示を追えることが必要です。観測側が存在しない出来事を追加して不足を埋めてはいけません。別モデルの期待JSONをつないだfake E2Eは禁止です。

既存の1573等の過去のtest数は目標値ではありません。今回の対象cutで実行したコマンドと結果を記録します。必須検査のskip、timeout、未回収oracleを成功にしないでください。

## 10. 資源・Git・継続状態

`$HANDOFF/materials`と`archives`は原本として保持します。`run_all.py`等はevidenceを書き換えるため、外部workdirのコピーで実行します。必要なら同梱の `tools/run_baselines.py` を読み、使ってください。

重いbuild前に容量とmemoryを確認し、既存の外部workdir方針を守ります。勝手にuserのファイルや既存cacheを消してはいけません。秘密、cookie、鍵、oracle profileをlog/commit/packetへ出しません。

自分の差分だけを検査してcommitし、実repoの許可された通常push方針に従います。force push/reset/cleanはしません。push不能をparity成功にしません。

短いRESUMEに、基準SHA、dirty状態、現goal、layer/side、既読hash、次command、oracle session、証拠、未解消Qを残してください。context縮約後も同じ作業を再開し、同じ相談やbaselineを理由なくやり直さないでください。私的な思考全文は不要です。

## 11. 完了・停止

重要な区切りでは短く進捗を示し、次の許可されたgoalがあれば続行してください。読了、計画作成、小さな定理、componentのGREENだけで最終回答して停止しないでください。

今回の到達目標を満たした場合は、採用profile、使える能力、機械検証した命題とTCB、実装対応、正例/反例、実network/復旧/観測の証拠、要件119行のdisposition、oracle指摘、Git状態、未保証を整理して完了してください。正式な製品受理と候補完成を分けてください。

途中で停止してよいのは、私が停止を指示した場合、不可逆で未指定の意味判断・権限・安全・機密・production操作が必要な場合、必須の受理前提やtoolが実際に満たせず依存作業を進められない場合、または環境上どうしても継続できない場合です。その場合も、証拠と再開点、できたこと／未達、必要な最小の判断を残します。時間が長いことや難しいことを、未達の成功宣言や無関係な作業への逃避理由にしないでください。

**まず資料と現場を照合し、最初のgoalを定め、必要なoracleの中立レビューを主担当自身で依頼し、実際の基礎定義と機械検証へ着手してください。**
