# tasks

最終更新: 2026-10-29 21:31 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project direction, theory, ADRs, conformance, and process. Everything outside `mirrorea_canon/` is LAB: evidence, history, implementation, and operational notes. If LAB text conflicts with canon, canon wins.

## document role

Plan250/ADR0043のowner pauseを維持し、別途許可されたW6有限LABは2026-10-29 20:00JSTのowner指示で再開。I3-4/W7/Canon採用は開始しない。Sources: Canon `mirrorea_canon/adr/ADR-0043.md`、LAB `plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md`、`plan/proof-first-foundation-correspondence.md`。単一report2617とW6_CHECKへ証拠を集約する。

## current promoted package

Sources: Canon `mirrorea_canon/adr/ADR-0043.md`、LAB `plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md`、`plan/proof-first-foundation-correspondence.md`。

W6は再開・未完了。Native63全1750/45独立reader/全履歴とworkspace他1190/7doctestを受理。固定275formal（41526宣言/1995反例）と別cutの寿命・型補完・再取得・費用を検証。実3world/各2cold reader、registry27組/36費用区間、30process/1116prefixを受理。初回/fresh/V1/V2画像に続き、保存cacheのnative形状と条件付き費用Hを新45宣言/16反例で結合し、既存全90費用式を再照合。到達可能履歴・共通whole入力条件・全caller保持/物理資源/進行、他profile、最終docs/Oracleは未完了。

現全private実行/全readerまで受理済みなのはNative63。Native21/1678/21readerは旧baselineへ移した。Native59/60/62の失敗を保存。Native63はbuildと独立workspace partitionを受理。現runtime1750と45原本reader/全履歴を受理。残分類はW6_CHECK.implementation.unverified_checks。再開手順は `docs/proof-first/RESUME.md`。

## ordered self-driven packages

初回/fresh/V1/V2画像と保存cacheHの条件付き接続を受理し、到達可能履歴・共通whole入力条件から全caller資源へ、下表の依存順で進めます。10/29 ownerへのrough estimateは順調なら残り2〜4日の実作業、資源前提の修正/レビュー差戻しで1週間以上の可能性。Oracle利用制限待ちとquota停止は別の不確定要因。完了日・完成率を保証しない。

| Package / macro position | Direct consumer / exit | Current state |
|---|---|---|
| Current build / Macro3/6後段 | ordinary library/CLI/integration/example | Native63のcfg-only3file、4library構成check通過。14packages/148targets/32examples/5CLIのビルドと独立照合通過。147他target/1190試験・21追加runtime・13doctest/7例を独立受理 |
| Current full regression / Macro3/6後段 | 全runtimeと原本・全workspace | Native63全1750と45原本reader/全履歴を受理。他1190試験と7doctestも受理。別の計測構成は1790inventoryの全回帰を未実行 |
| Resource correspondence / Macro3/6後段 | allF/J/work/setup/host/physical | 保存cacheHと90費用式への代入を受理。Hは約205TiBの保守的累積上限。次は到達可能履歴/共通whole入力条件→全caller保持寿命と使える物理資源・進行 |
| Combined source/proof/history / Macro1/5・3/5後段 | future authority/full response/current/reload | 現33source/23対応/153出力は照合済み。275module/41526宣言/1995反例の両回と全4618ログを受理。寿命/snapshot/scratch3moduleは別途両回検証。現全履歴も45原本readerで受理 |
| Stable docs / Macro3/6後段 | 再現commandと受理証拠 | saved-cacheはfull6410入力不変・exit0。metadataはfocused356で別検証しcommit/pushへ進む。W6最終統合full/Oracleは残る |
| Independent final review / Macro3/6後段 | T01〜T15/全要件/反例/TCB | 10/29 21:07JSTもOracle Pro5of5/Usage limit reached。最終review未実施。次回確認は22:07JST以降 |
| W7 / Macro3/6後段 | W6受理後の別owner範囲 | inactive、今回範囲外 |

## self-driven macro phase reading

Macro1/5の一般証明とMacro3/5の実Source/kernelを接続し、Macro3/6のrelease/全channel/資源/回帰に未完了項目を継続する。技術的な残課題と採用判断を分ける。初期化DATA、実Source、独立authorityを分ける。current roadmapはPlan250、W6 LABは別途owner許可範囲。

## user decision gates

容量: 2026-10-29 21:22 JST root約6.6GiB、MemAvailable約10GiB。今回proofは24MiB/cut上限内で実行し、原本画像を再利用。最初の16MiB guard停止も失敗記録として保持。全cache復元なし。大きな出力前後に残量を確認する。

Q26の最終channel/timing/in-flight政策とQ27のdebugger範囲は未採用。可逆LAB候補と反証は継続可能。技術検証と最終採用判断を分ける。privacy弱化/L0-L1/public contract/production/Canon採用はowner判断。119要件/30Q/18PT/24SC/8alphaの出所・承認・採用区分を保持する。

## research discovery items

| Item | Effect / alternatives / current view |
|---|---|
| Historical currentness | 元publisher/current registryと独立leaseは保存DATAと別。same-owner再読込み/active再参加を検証。元custody全喪失後の一般再開は未選択 |
| Current release | 現案Aはcanonical M9/owner→observer guardをdurable intent/全DATA write中保持。最小代替Bの失効後writeは未採用。kernelにqueued済みの到着とwrite時currentは別 |
| Work and funding | 受動/能動のwire・暗号・journal入力を32query/8reloadへ合成。全M9入力Hと準備出力Jを分け、setup/driverと拒否後host-entryを別計上。実測最大値又は8MiB payloadを全service上限にする代替は不採用 |
| Temporary ownership | zero-retained scratchの逐次maxと全spent加算、initial/escape/並行ownerのframe、置換L+Cを明示。現30processの1116prefixを受理。全caller上限は未受理。phase peak/RSS/public保証を導かない |
| Graph/absence/loss | local closed/lossless/page remainder/distributed typed crossingを分ける。M8直接依存とSource順序/Flow crossingを混同しない。current全1750/全原本readerへ結合済み。最終要件別照合は残る |
| Mutation/refinement | 実7lifecycle/全6publication監査を保持。restricted-child2familiesと現147target受理結果の意味対応照合は残る。全writer/分類/historyを期待JSONだけで受理しない |
| Host and timing | honest host/key/filesystemは前提。scheduler/fsync/EINTR/同一UID隔離/一般availabilityは別の未証明境界。有限LABと本番保証を分ける |

## maintenance tasks

最新owner指示はW6の継続とストレージ残量への注意です。主担当一人で可逆な有限LABを進め、区切りごとのcommit/pushを行います。研究ソース・証明・原本・ログとexact archiveを保持し、必要なcacheだけを復元します。W7/Canon採用/Plan250-I3-4/未依頼の外部write・通知・共有folderは開始しません。quotaはQUOTA-OWNER-POLICY-v6の指定reset条件と使用後35%停止条件を維持します。

## non-promoted references

閉じたW4/W5/Astraと未採用W6 LABを区別する。focused Native/formal supplement/旧last fullはW6全体又はCanon採用ではない。旧失敗と全cut範囲はreport2617と凍結receiptへ保持。

既存Process/Fabric/Bootstrap/Flow割当式、registry27cut/36区間とdormant37入力116区間は再利用する。再実装するqueueへ戻さない。

2026-10-29 21:01JSTの週次残量51%、指定10/30 Full reset使用0回。次回確認は22:01JST以降。21:07JSTのOracleはPro5of5/Usage limit reached。指定resetの期限接近/枯渇条件と使用後35%停止はQUOTA-OWNER-POLICY-v6のまま。
