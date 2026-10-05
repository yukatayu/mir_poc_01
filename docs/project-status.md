# Project status

最終更新: 2026-10-05 19:56 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project direction, theory, ADRs, conformance, and process.
Everything outside `mirrorea_canon/` is LAB: evidence, history, implementation, and operational notes. If LAB text conflicts with canon, canon wins.

Plan250 owner control: I3-3 is accepted at the finite source/evidence cut and execution
is paused. I3-3 accepted; owner pause leaves no active semantic milestone. Plan
250 remains the sole retained current roadmap. I3-4 requires explicit owner
resume; this is not blocked, stale or program close.

## この文書の役割

これは人間向けの短い **派生ビュー** である。規範判断は
`mirrorea_canon/`、closed execution historyはPlan 247 / 249、詳細証跡はmilestone
reportsにある。この文書はGate/Phase、OBL、proof、compatibility、goalを決めない。

## 全体の進行チェックリスト

```text
closed M0--M10 finite reference baseline
-> [x] SYS-0 baseline / goal alignment
-> [x] SYS-1 runtime kernel / internal carrier
-> [x] SYS-2 ST / OW1 concurrency refinement
-> [x] SYS-3 checked Core -> per-locus artifacts and generated plans
-> [x] SYS-4 in-process generated dispatch
-> [x] SYS-5 four-locus toy + typed devtools
-> [x] SYS-6 finite I2 assurance / lifecycle closeout
-> [x] SYS-7 inactive I3 entry contract only / program closed
-> [x] ALIGN-0 bounded-program activation / meta-drift alignment (completed)
-> [x] ALIGN-1 project/product layer constitution (completed)
-> [x] ALIGN-2 Browser/Host/package/View/provider boundary contracts (completed)
-> [x] I3-0 transport candidate evidence and private selection (completed; `mirrorea_canon/adr/ADR-0037.md`)
-> [x] I3-1 checked private adapter/encoding boundary (completed; `mirrorea_canon/adr/ADR-0038.md`)
-> [x] I3-2 two-or-more-process generated-artifact runtime (completed/accepted bounded evidence; `mirrorea_canon/adr/ADR-0039.md`)
-> [x] I3-3 network failure/order refinement (accepted; owner-paused): `mirrorea_canon/adr/ADR-0043.md`
```

Plan 247とPlan 249はclosed recordsである。PROPOSAL-037 / ADR-0034の
Mirrorea I3 Distributed Foundation bounded programは未完了だが、ADR-0043により
実行は一時停止している。ALIGN-0--2 / I3-0--3の7/11 milestoneが受理済み。
I3-3 accepted; owner pause leaves no active semantic milestone。
Plan 250 remains the sole retained current roadmap。
I3-4/I3-5/I3-6/NEXT-0 remain dependency-gated inactive; I3-4 requires explicit owner resume。

## 現在地

今回の別依頼は task-local proof-first LAB 研究と理論gate後の限定実装。
W2の有限な研究成果は検証・review・Git統合済みで保持しています。
W3の有限source候補は81f82a0bで統合済みです。76module監査・通常source32件・Oracle reviewの確定記録と再現手順はReport2613と既存foundation runnerに保持します。
W4-Cの実装前基礎条件は、選択した通常代入列と局所ownerの限定LAB範囲で技術的に閉じました。文書検査とcommit/push（78756ad5、remote一致）を完了し、W4-Dの境界設計へ進みました。単一source進行、元の全checked Core・引数、現在のM9利用、実結果の回収、一度だけの完了、共有資源と全入口の条件を対応付けました。190module・19502所有宣言・156偽命題対照の監査結果を保持します。一般証明は明示した前提の下の命題であり、物理的な認証を発行しません。

最新の外部参照foreign-fault-retention-green-v1は、default全806件とprocess-test feature全818件を通過し、失敗・skipはありません。最後のOracle指摘は、容量不足の別要求が注入済みfaultを失う実経路でした。ST/OW1の4失敗と4正例で再現し、fault消費を移動成功後へ移した修正は追加8件と両full profileで確認しました。Oracleの回答全文・23入力・実model設定を照合し、14項目を処置しました。この最後の小差分は主担当が検査し、Oracleの再実行とは記録していません。

CのRust参照15ファイルは未採用で、通常source cursorはtest-onlyの局所実験です。既存の採用cut e5450e38と通常libraryの動作を置き換えていません。Dでは既存source→checked Core→生成edge→実private QUIC→同じ実行の観測へ接続します。唯一のrequester custodyと他processの非実行descriptor、元source/Core/引数/activation/ordinalと実request/result、M9とTLSの分離、新しい資源poolと未公開識別子の非escapeを使用前に確認します。単に局所Arcを渡す、操作IDを外部loopで順に呼ぶ、手書きreceiptを返す方法では完了にしません。

観測は既存private I3のredacted reference/count範囲を保持します。一般の公開observer、広いcallee/混合source、秘密依存の時刻・件数の非干渉や復旧の条件を満たしたとは扱いません。これらのconsumerが必要になれば、依存するC条件を先に再開します。R01〜R12の義務と119行のU/D・承認区分は保持し、Eが行う最終和集合照合へ使用中の前提を先送りしません。

W4-A/Bは限定証拠・統合候補として完了済みです。Bの206依存source、V2モデル278command・236module、native準備235command、実process68commandの保存証拠はそれぞれのcutで保持します。ownerがSolへ切替えた後にD実装を再開し、2026-10-01に一時停止し、2026-10-05の指示で現在は再開しています。AstraによるDの統合判断前で次のmodel切替checkpointを設けます。主担当一人、sub-agent禁止を保持し、W5+・alpha・Plan250/I3-4を開始しません。Dの確定実装packageを保存した地点から再開しています。Dの実process/network接続は未完了です。

W4-Dは同じgoalで継続中です。外部未採用38path参照owning-original-owner-resolve-controls-v2で、元sourceの実要求発行・実QUIC受信・Ready・保護された受付に、既存I3のResolveを接続しました。元の全Core・全引数・activation・global ordinal・実要求ID・符号化データ、現権限と実際の受付済み要求を照合し、既存の実clockで次の待機要求が同じ要求か確認してから解決します。実際の一度限りの本体許可、期限切れ返信、拒否を、記録・通知より先に保持します。通知失敗時は実許可・未送信通知・受付区間を、親の照合失敗時は読み取った実完了を残します。正常系1・異常条件と一度だけの利用14、3種類の検査省略対照、通常default/private-QUIC build・test-seamなしの実正例と同cut全1100件を検証し、38pathを復元しました。本体とsource・親の進行位置は0のままです。本体での実許可消費、予算なし要求の本体区間、実M8下位戻り値の保持、実行前の数値・割当資源、返信・source受領、S→T→Sと次activation、後続updateでの再freezeは残ります。データ照合やResolve完了を本体の実行許可にはしません。既存199module監査は別cutの条件付き証拠です。重要な契約変更の反例またはD統合判定ではAstraへ、週間残量30%未満では検証済みの区切りで停止します。

W4-Dは2026-10-05のowner指示で同じgoalの実装を再開しています。主担当一人で、sub-agent・Oracle・新goal・E/W5+/Plan250-I3-4は開始しません。Astraでの重要な境界判断又はD統合判定が必要な地点、または週間残量30%未満を確認した後の検証済み区切りで、実ソース・検証ログ・再開地点を保存して停止します。2026-10-01の作業都合によるpauseと旧50%条件は更新されました。

週間残量は2026-10-05 10:13:11 UTCの確認で59%（使用41%）でした。次の確認は2026-10-05 11:13:11 UTC以降。最新owner条件は残量30%未満の後の区切り、またはAstraへの判断引継ぎで停止。account resetはownerのみ。

対応は `plan/proof-first-foundation-correspondence.md`、現在の証跡はreport2614、W1/W2/W3の履歴はreport2611/2612/2613です。
既読範囲は `docs/proof-first/READ_LEDGER.json` に記録し、未読と部分読了を区別しています。
W1のpassive/accepted producer・一般label・失敗を伴う単一代入は限定review済み。W1補助source cutのreview未完了は、後のW3有限source候補のreview完了とは別に保持します。119行dispositionは元のU/D・承認区分を保持して作成済みですが、要件を一括受理・実証したものではありません。
M8 trusted setupの実効label不一致を実再現し、観測refinementの未解決義務とした。
根拠は `plan/proof-first-foundation-correspondence.md`、`docs/reports/2611-mirrorea-proof-first-w1-foundations.md`、`docs/reports/2612-mirrorea-proof-first-w2-contract-boundary.md`。
これは正式THM/OBL/phase更新、I3-4 resume、署名済み受理ではない。

| 観点 | 状態 | 根拠 |
|---|---|---|
| theory | **T1** | `mirrorea_canon/plan/01-phases.md` |
| broad PHASE-I1 | **unaccepted**; OPEN-026/027とfull carrier freezeが残る | `mirrorea_canon/architecture/04-runtime-carriers.md` |
| bounded I2 lifecycle | **official entry accepted, then official exit accepted** | `mirrorea_canon/adr/ADR-0032.md` |
| ADR-0026 program | **SYS-0--SYS-7 closed** | `mirrorea_canon/adr/ADR-0033.md` |
| active roadmap / goal | **Plan 250 retained / I3-3 accepted and owner-paused** | `plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md` |
| I3 / OPEN-032 | **I3-3 accepted and owner-paused; I3-4+ dependency-gated inactive; official I3 lifecycle entry remains unaccepted** | `mirrorea_canon/adr/ADR-0043.md`, `mirrorea_canon/adr/ADR-0037.md` |
| public/product | final grammar/CLI/API/ABI/wireもproductionも未受理 | `mirrorea_canon/adr/ADR-0033.md` |

Accepted SYS-6 implementation/evidence cutは
`5429712de89a7e41c46cfd7fb4a39c4a492864c4`、Canon/status integration cutは
`bcb0f767edbb3e9e581c3b4c7f2a49e077f44067`である。provisional
`mir conform-i2`はexact 22 finite rowsを検査するがlifecycleをself-authorizeしない。

SYS-7は候補A TLS-over-TCP framed reliable-stream adapterと候補B QUIC reliable-
stream adapterをともにUNSELECTEDで保持した。I3-0は両候補の同一private
source/Core-bound nine-case actual-process canaryを通し、criteria 1--7のtie後、criteria
8/9に勝者がなく、criterion 10 future browser relevanceを最初のmaterial differenceとして
Bをprivate selected adapterとした。Aはrejected/deferred replacement baseline、QUIC datagramは未admit・
未評価である。public version、codec、wire、certificate、API/ABI、deployment、platform
又はproductionは未選定で、transport/session/certificate/route metadataはauthorityではない。

## 現在の停止線

ALIGN-0 / ALIGN-1 / ALIGN-2 / I3-0 / I3-1 / I3-2 / I3-3 accepted; owner pause leaves no active semantic milestone。I3-4/I3-5/I3-6/NEXT-0 remain dependency-gated inactive; I3-4 requires explicit owner resume。ALIGN-2 は Browser/Host/package/View/provider の責任境界を Canon 化し、BND-010..BND-016、trust tier T0–T4（Theory T0–T2 とは別）、package admission と semantic grant の分離、raw FFI 禁止、redaction と resource/termination 責任を明示した。固定順序はALIGN-0..2 → I3-0..6 → NEXT-0である。
ALIGN-1ではsemantic strata S0--S6、project/product PL-0--PL-6、lifecycle T0--T2 / I1--I6を独立したmany-to-many座標としてCanon化した。PL-4は責任境界のみ、PL-6は別application、satellitesは別系統である。
ALIGN-0 acceptanceはI3 lifecycle entry、transport選定、production/public freezeを
含まない。これらは各後段gate又はowner-reserved boundaryへ残る。
Current authority and milestone gates are
`mirrorea_canon/adr/ADR-0034.md` and
`plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md`.
I3-3 is accepted and owner-paused with no active semantic milestone;
I3-4/I3-5/I3-6/NEXT-0 remain inactive pending their dependency gates. The
accepted I3-2 source cut and remote parity are the preserved entry evidence.

Accepted source/evidence cut: `fe5dd972e2ddb3a513c785458a07702e4d4d99fa`.
Accepted Canon/status integration: `aafde92229bb0ff18116f38d4750a0a8f61cb069`.
Retained acceptance evidence: workspace1573, runtime doctests4, format/Clippy
and independent source review P0/P1/P2=0. These are source-cut results, not
fresh Rust runs for this documentation maintenance.

The finite twenty-family/order profile includes real two-process QUIC,
ambiguity/reconnect/duplicate rejection, current membership/capability checks,
source-declared owner admission ticks, read-only provider calls and A-local
continuing-runtime cut custody. Actual process evidence, LOCAL checks and the
432-state/2136-transition bounded model remain distinct. Private child fault
assertions are not exported observer traces; injected `AdapterUnavailable`
is LOCAL evidence. The local cut emits no saved image and does not implement
save/restore, distributed quiescence, live patch or durable restart.

Detailed component commands, falsifiers and limitations remain in immutable
Report2606; they are not current implementation tasks. No provider/session
authority, hidden retry, exactly-once, public API or production guarantee follows.
Detailed edge contracts: [`mirrorea_canon/architecture/07-browser-host-trust-boundaries.md`](../mirrorea_canon/architecture/07-browser-host-trust-boundaries.md); cross-edge binding/freshness/revocation/redaction/resource rules: [`mirrorea_canon/architecture/08-browser-host-security-invariants.md`](../mirrorea_canon/architecture/08-browser-host-security-invariants.md).
View は authoritative domain semantics を所有せず、presentation-local computation のみを許可する。View からの入力は typed command/effect request とし direct store を禁止する。I3-0 はprivate transport選定をclosedし、OPEN-032はこのbounded programだけresolvedした。I3-1とI3-2とI3-3はbounded evidenceとしてclosedした。official I3 lifecycle entry remains unaccepted、I3-3 accepted; owner pause leaves no active semantic milestone、I3-4/I3-5/I3-6/NEXT-0 remain dependency-gated inactive、I5 implementation は inactiveである。
I3-2の履歴証拠はlocalnet 12/12（repeat）、full probe 62/62、runtime default 29/29、seam 47/47、library 281/281、docs compile-fail 1/1（default/private）である。現在はI3-3を含む7/11 milestones acceptedであり、これは重み付き完成率ではない。FM-5 localhost evidenceであり、public workflowや100% completionは主張しない。過去の最終検証後は約63GiB free。今回のM8 baseline後は約60GiB free、target4.4GiB。大きなvariant buildは測定付きstorage planまで保留し、小容量の証明・反例検査は継続する。
The retained bounded I3 programはinternal carrierとpublic wireを分離し、route/handshake/framing/
disconnect/reconnect/ambiguous delivery/duplicate/reorder/stale authority/backpressure/
timeout/provider/redaction/patch/cut failureをtypedに扱い、network occurrencesをMir
orderingへrefineしなければならない。hidden retry、exactly-once、hidden transactionは不可。
I3-4/I3-6のC-distributed evidenceはordinary-source SCN-01/02/03/06のpositive/falsifier、
source/Core/artifact/carrier/network/runtime correspondence、observer-safe diagnostics、
evidence classification、independent reviewを必要とする。I2 evidenceだけでは満たさない。
Reopen accepted I2 evidenceはmissing/manual edge、owner movement、direct remote store、
source-free mint、selected ST/OW divergence、stale cut/patch mutation、relation/designated
drift、observer leak、lower-layer conformance dependency、M10 regressionの場合だけ。
## オーナーの確認・判断待ち
OPEN-032はPROPOSAL-040 / ADR-0037によりこのbounded programだけresolvedした。
owner/userのresumeにより開始したI3-3はADR-0043で受理済みである。
Plan250はowner pauseであり、I3-4は追加の明示的resumeまでactivateしない。今回の独立した可逆LAB研究には追加回答を要しない。
このpause gateに加え、次のbounded sequence外の変更もowner decisionを必要とする。
- public API/ABI/wire/grammar/CLI compatibility freeze;
- production deployment、external publication、paid resource;
- North Star、authority/privacy/redaction/no-stale guaranteeの変更;
- World/Avatar等のCore primitive化、hidden multi-owner transaction; および
- Constitutionでも解けないirreversible semantic tie。
Authority boundaryは`mirrorea_canon/meta/agent-instructions.md`と
`mirrorea_canon/adr/ADR-0034.md`を参照する。これらは未完了SYS-7 taskではない。
## 根拠と詳細
| 読みたい内容 | 一次の確認先 |
|---|---|
| Canon entry | `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md` |
| lifecycle | `mirrorea_canon/plan/01-phases.md` |
| SYS-6 acceptance | `mirrorea_canon/adr/ADR-0032.md`, `mirrorea_canon/spec/15-sys6-i2-conformance.md` |
| inactive I3 contract | `mirrorea_canon/adr/ADR-0033.md`, `mirrorea_canon/plan/05-i3-entry-contract.md` |
| retained owner-paused I3 program | `mirrorea_canon/adr/ADR-0034.md`, `mirrorea_canon/adr/ADR-0043.md`, `plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md` |
| I3-0 private transport selection | `mirrorea_canon/adr/ADR-0037.md`, `docs/reports/2603-mirrorea-i3-distributed-foundation-i3-0-transport-selection.md` |
| Browser/Host trust edges | `mirrorea_canon/architecture/07-browser-host-trust-boundaries.md` |
| cross-edge security invariants | `mirrorea_canon/architecture/08-browser-host-security-invariants.md` |
| proof/evidence class | `mirrorea_canon/theory/11-metatheory-ledger.md` |
| closed I2 roadmap | `plan/249-mirrorea-i2-systems-foundation-current-roadmap.md` |
| SYS-6 close evidence | `docs/reports/2598-mirrorea-i2-systems-foundation-sys6-i2-conformance-closeout.md` |
| SYS-7 close evidence | `docs/reports/2599-mirrorea-i2-systems-foundation-sys7-i3-entry-contract-closeout.md` |
| runnable commands | `samples_progress.md` |
Inherited validation floor: SYS-6 25+8、SYS-2/3/4/5 28/28/104/62、M10 67+4、
workspace。I3-0 adds private facade/frame/source/supervisor/TLS/QUIC/equality/
observer test evidence, format, warnings-denied focused Clippy, diff and final
independent ACCEPT; exact current results and skipped reruns are in Report 2603.
## 更新規約
active program/roadmap、official lifecycle、major blocker、accepted cut、evidence class、
またはuser-visible commandが変わるtaskで同期する。authorityは常にCanonへ戻し、
詳細履歴はone milestone reportへ置く。未実行validationをpassと書かず、helper/reportを
general proofやpublic product completionとして数えない。
