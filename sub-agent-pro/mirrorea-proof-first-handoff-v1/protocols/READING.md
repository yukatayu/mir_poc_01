# 読取りとcontext管理

## 原則

索引を作ったこと、grepで見つけたこと、要約を読んだことは全文読了ではない。`read ledger`に path、SHA-256、range、役割、状態（unread/partial/full/equivalent-copy）を記録する。hashが変わったら既読を無条件に流用しない。

全ファイルを一度にcontextへ投入する必要はない。有限のまとまりで全文を読み、意味・未解消点・参照を保存して次へ進む。byteが一致する重複コピーだけは一度読んだ内容へ対応づけてよい。名前が似ているだけの新旧仕様を同じものとしない。

## 提供資料

全体設計の前提にする前に、要件表119行、Q30、PT18、SC24、α8を確認する。F0.3の本文・証明・台帳・計画・監査を全文読み、F0.1/F0.2への証明・実装の依存を追う。新しい命題に依存する前に、その命題の定義と証明、関係するcode/testを全文読む。

HTMLは同じ内容の表示版である。Markdown/JSONを読んで内容の同値を確認した場合、HTML全byteをモデルへ再投入しなくてよい。画像はその記載を根拠にする場合に見る。歴史的な実行logは必要な対象を完全に読むが、全raw logの読了を理論研究の開始条件へしない。

## 実リポジトリ

起動時に存在するルート／階層のAGENTS（override含む）を確認する。CanonのREADME、MAP、North Star、Constitution、source hierarchy、phase、ADR-0043、関係する理論とspec、plan/05、現Plan 250を読む。その後にREADME、Documentation、progress、tasks、samples_progress、.docs/progress-task-axes、oracle運用を読む。

元の依頼で重視された **plan、spec/specs、samplesの全text** および **docsのreports以外の文書** を、全体棚卸しの必読コーパスとして台帳へ登録し、段階的に通読する。mirrorea_canonのplan/spec/theory/architectureも対象。現行正本とLAB履歴の区別を保つ。例名、付録、否定例、generated artifactの説明を勝手に省略しない。

`docs/reports/`は一括で読まない。正本から直接必要な実行証拠を参照する場合だけ該当範囲を読む。ログ全体をcontextの代替にしない。

program/codeは全treeを把握し、現行実行routeと依存coneを読取り台帳に登録する。変更・移植・証明の主張に関係するproduction code、tests、formal codeは全文読む。無関係な旧コードまで完了したと主張しない。初期調査は台帳整備で終わらず、基礎の反例やformalizationへ進む。

## traceability

全119要件を消さずに `not-in-current-profile / design / hand-proof / mechanically-proved-scoped / runtime-tested / integrated / accepted` 等の別軸で管理する。scopeに含めない行は理由・将来consumerを残す。statusの単語より、対象・前提・証拠の対応が重要である。

全件読了が長期に及ぶ場合でも、読了した依存閉包について安全な研究は進められる。ただし、全体棚卸し完了の宣言や新しい全体計画の最終受理を、未読コーパスのまま行わない。context長による省略を読了に数えない。

旧ログの`/mnt/data/`や`/tmp/`は当時の環境での位置であり、現在の実行パスではない。収録された相対ファイルへ対応づけてから、作業コピーの実パスで再現する。旧commandの絶対パスを無批判に実行しない。
