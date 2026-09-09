# 反例、修正、検査範囲

ここで記録する不具合は、特記しない限り**今回作成したF0.2の参照モデルの不具合**であり、Mirroreaの既存Rust/QUIC実装に同じ不具合があると確認したものではない。独立した人・別agentによるreviewを受けたという意味でもない。

## 1. 実際の失敗記録を残した修正

| 対象 | 実際に観測したもの | 修正 | 記録 |
|---|---|---|---|
| 代入の生成identity | 同じ行の二つの代入が同じ生成operationを共有し、13になるはずの結果が14になった | function/line/columnにより構文occurrenceを区別 | `source_first_run.txt`, `source_repaired_run.txt` |
| structured child failure | 未joinの子が失敗しても、親の結果が成功として確定した | root completionで全子を確認し、失敗を伝播 | 同上 |
| 論理的な戻り値の衛生性 | ローカル`result=1; return -1`に対して、論理変数resultをcaptureして正の戻り値を証明できた | source環境の追加後に実returnを論理resultへ束縛 | `certificate_capture_red.txt`, `certificate_repaired_run.txt` |
| owner invariantの範囲 | 別ownerの状態を含むIが、明示的拒否でなくKeyErrorになった | Iのfree state variablesが当該ownerのschema内であることを先に検査 | `invariant_scope_red.txt`, `invariant_scope_repaired.txt` |
| client bridgeとsourceの対応 | 元sourceと異なるASTを持つcandidateから保証の接続が作れた | 元sourceから再parseしてcanonical AST/操作を照合 | `bridge_source_binding_red.txt`, `bridge_source_binding_repaired.txt` |
| replacementと実schema | 関数signatureが同じなら、source内で状態をIntからBoolへ再解釈する候補が通った | 全read/write cellの実schemaとcandidateを比較 | `replacement_schema_red.txt`, `replacement_schema_repaired.txt` |
| 複数patchのfence | 準備していないpatchのAbortが、別のpatchの予約範囲を解除した | patch固有identityとfence所有者を保持し、自分のfenceだけ解放 | `patch_composition_red.txt`, `patch_composition_repaired.txt` |
| 完了後の抽象状態 | patchの古いshadowが、その後の正しいowner更新を隠した | shadowはCommit後かつPreparedのparticipantだけに使う | 同上 |
| journal書込みの不確定なerror | appendを実際に完了してからOSErrorを投げると、旧writerを再利用できた | append例外後はwriterをfault状態にし、再open前の継続を禁止 | `journal_uncertain_red.txt`, `journal_uncertain_repaired.txt` |

owner invariantの範囲に関するREDは、型付き拒否の不足と例外漏れの記録である。そのログだけから「不変条件を破る操作が実行された」とは結論していない。

journalのREDは保存した修正前moduleをテストへ読み込み、`実append→例外→同じwriter再利用`を実行して得た失敗記録である。改変版の正しさを仮定して反例を捏造したものではない。

## 2. 理論的な反例を実行したもの

`activation.py`の三つのmutantは、通常のprotocolにない遷移を意図的に許したモデルである。Preparedからの一方的timeout解除、durable Prepareの消失、durable Decisionの消失は、それぞれinvariantを破る到達traceを持つ。これを本来のprotocolの失敗数へ混ぜない。

`test_ifc_effect.py`には、単一trace inclusionを満たす実装が秘密値に応じて挙動を選び、非干渉を破る反例がある。F2-27のsimulationからprivacyが自動的に従うとする誤りを防ぐ。

Partial契約では、assertを強めて成功する入力を減らしても「成功時にpostを満たす」を保てる。TotalBodyはこの差を明示的に扱い、新assertを元のI/Pから証明できない交換を拒否する。これはPartialの論理矛盾ではなく、保証の種類の違いである。

外部効果のStarted後のcrashでは、呼出し前と呼出し後を区別できない二履歴を作った。両方を同じ再実行方針で処理して、常に一度だけ物理実行されるとは言えない。

## 3. 検査の足場自体のエラー

保存した途中ログには、テストのimport不足、仮のAPI名と実際のmoduleの不一致、辞書key名の取り違え、bytesをtext書込みへ渡す例、solver wrapper呼出しの不一致がある。これらは成功として数えていない。

具体的に、`declaration_boundary_red.txt`の先頭は`unittest`のimport不足によるNameErrorであり、型検査器が不正な宣言を受理した反例ではない。importを修正した上で、duplicate parameter/default argument/decorator/owner annotationの四つの拒否検査を最終suiteに含めた。

`multi_owner_setup_error.txt`は、最後に追加した二ownerの結合検査で、存在しない検査用API名を呼んだsetupエラーである。実際の`semantic_values`とstateの照合へ修正し、15件のpatch結合検査および153件の新規全suiteで成功を確認した。このエラーをprotocolの安全性反例として数えていない。

`RUN_SUMMARY.json`は最終の実行結果であり、途中の未通過runを上書きして過去の成功とするものではない。初回の全体runnerでは統合demo・activation runner・SMT wrapperの呼出しが失敗した。これらの修正と実再実行を経た結果だけをVALIDATIONへ記す。

## 4. 証明と実装の照合で残ること

手証明の対象は、文書で明示した数理規則である。コードがその規則に完全に一致することを、別のproof assistantで証明したわけではない。今回のRED修正は、その差を発見し得ることを具体的に示している。

依存する環境条件として、exact source identity、理想的なmessage認証、単一writerのdurable prefix、patch予約のissuer側実現、使用時authority、完全なfootprint、直接storeを書かないhost境界がある。F0.2のPython内部を呼べる任意native programに対する防御を保証しない。

SHA-256は実行モデルでidentity照合と破損検知に利用する。数学的な証明は構造の同一性を前提とし、実装ではcanonical encodingと衝突耐性に関する仮定が別に必要になる。hashだけを認証や証明そのものとは呼ばない。

## 5. F0.1の継承

入力ZIPと添付本文のhashを`provenance/input_manifest.json`へ記録する。F0.1の理論本文・モデルのsource・SMT入力は継承し、再実行は別のevidenceへ記録する。前回の69テストと61 SMTの結果は、新規テストや新しい一般証明の数へ加算しない。

## 6. 未監査の範囲

GitHubリポジトリ全件、既存Leanの全証明、実Rust/QUIC、実装環境のOS/hardware、任意のlocal theory、第三者plugin、製品Browser/rendererは監査対象の完了を主張しない。`docs/reports/`は本研究の必要な一次仕様として読んでいない。全件通読を要求された以前の依頼を、今回の研究成果で完了扱いにすることもない。
