# samples/lean

このディレクトリは、repo が Lean でどこまで mechanization を進めているかを **repo-local かつ inspectable** に保存するための場所です。

## layout

- `foundations/`
  - small actual proof fragments
  - finite-index first layer、IFC example、proof skeleton の最小 mechanization を置く
- `lab-statements/`
  - LAB-only Lean statement-shape drafts
  - compile-check only; no canon OBL status movement
  - current OBL-024 draft includes abstract diagnostic projection and
    trace-local replay vocabulary; this is not a final Diagnostic ABI
  - current OBL-025 draft includes abstract branch-local non-coverage
    vocabulary; this is not a final branch ID / JSON / repair ABI
- `clean-near-end/`
  - active clean sample suite から生成した theorem stub
  - Lean は通るが、full domain discharge を意味しない
- `manifest.json`
  - foundations、LAB statement drafts、generated stub corpus の verification result
- `old/2026-04-22-pre-clean-near-end/`
  - pre-clean-near-end corpus の archive

## current reading

- foundations は actual proof fragment
- `foundations/MirTheoryV0M8DeterministicRuntime.lean` is the manually-run
  finite M8 checked-artifact/admission evidence; its exact OBL boundary is
  documented beside the file and in the Canon ledger
- lab-statements は compile-check only の statement-shape draft
- generated stub は proof bridge の足場
- old corpus は historical appendix

## 実行コマンド

```bash
python3 scripts/current_l2_lean_sample_sync.py
```

## Proof-first task-local LAB evidence

`foundations/MirroreaProofFirst*.lean` は今回の独立した LAB 研究候補です。
Plan250 の I3-4 resume、正式 THM/OBL の更新、production / α受理ではありません。
以下の companion は定義・一般命題・固定反例・未接続の実装義務を分けます。

- [Support](foundations/MirroreaProofFirstSupport.md): positive support、最小閉包、rank checker。
- [Current use](foundations/MirroreaProofFirstCurrentUse.md): 同一文脈の検査と admission 履歴。
- [Tracked validation](foundations/MirroreaProofFirstTrackedValidation.md): 実際の読取りと版検査、論理的な publication。
- [Finite graph](foundations/MirroreaProofFirstGraphValidation.md): 独立した Path / Acyclic と checker の対応。
- [Producer flow](foundations/MirroreaProofFirstProducerFlow.md): 型付き代入・分岐の二実行保証と実際の数学的書込み列。source/runtime 接続は未証明。
- [General labels](foundations/MirroreaProofFirstGeneralLabels.md): 非全順序のlabel理論と有限checkerの一般証明。policy採用・実行時間上限は別義務。
- [Fallible assignment](foundations/MirroreaProofFirstFallibleFlow.md): 失敗を含む単一代入、条件付き型・範囲保存と二実行保証。
- [Aborting sequences](foundations/MirroreaProofFirstAbortFlow.md): 失敗による後続依存と完了 bit の分離。旧jobのChromeエラー後、有限cutのOracle reviewを回収・処置済み。全source/Rust対応は別義務。
- [Reference resolution](foundations/MirroreaProofFirstAddressFlow.md): 別名を許す固定参照解決と評価・checker・実行列の対応。同じ有限cutのreviewを回収・処置済み。
- [Passive observation](foundations/MirroreaProofFirstPassive.md): 有限消去と、別の二実行保証。限定された数学的範囲のreview済み。
- [W2 contracts/resources/functions](foundations/MirroreaProofFirstContracts.md): 資源の分割・移譲、局所契約からの値の輸出、純粋な高階関数・有限反復と実行対応。研究候補であり、選択した有限な構成のOracle指摘に対応済み。既存Mirへの接続は未完了。

- [W2 computations/continuations](foundations/MirroreaProofFirstResourceComputations.md): 実際のcurrent allocation、資源Delta、失敗後の残余、捕捉した継続と型保存。有限profileの差分Oracle review済み。production/αの受理ではない。

- [W3 dynamic composition dependencies](foundations/MirroreaProofFirstDynamicComposition.md): named catalog elaboration、動的support/個別DAG、既存handle/current-use、保持参照とsource/sessionを接続した有限候補。kernel/変異検査・通常source接続・最終reviewを経た有限候補です。

- [W4 publication dependency](foundations/MirroreaProofFirstPublication.md): 公開値・使用区間・返り値、明示的な結果受取りと待機中の代替完了経路の拒否、非空有限cohortの正常公開手順。reference checkerに `--with-publication` を付けて検査する。物理通信・owner実行との対応は未完了。

W3の依存だけを外部fresh copyで再検査するコマンドは
`python3 scripts/proof_first_dynamic_composition_check.py --work-root /tmp` です。
11依存と19変異を検査し、結果の保存先を表示します。

W3の通常sourceから実構成machineまでの候補は
`python3 scripts/proof_first_composition_source_check.py --work-root /tmp` で検査します。
Rust parserを外部buildし、source由来の値・操作・private write/producer履歴を同じ状態から得ます。
保持参照・fallback／再取得・取消し・source継続の追加候補は
`python3 scripts/proof_first_reference_source_check.py --work-root /tmp` です。
独立checkerとの対応、全対象entryの保存、actual event/source由来、単調authority履歴を
一般証明へ接続しています。W3の有限候補を検証・review・source統合済みです。実network／永続復旧／秘密観測／α受理は別です。

これらは `current_l2_lean_sample_sync.py` の生成対象・manifest 集計外です。
一般命題は Lean4.29.1 の kernel で検査し、各ファイルの `#print axioms`
で前提を確認します。固定 `#guard` と有限 differential は一般証明ではありません。
原本や repo 内に `.olean` を生成せず、repo root から次で再検査できます。
`PROOF_WORKDIR` は利用可能な作業ディスク上の既存ディレクトリを指定できます。
未指定時は小容量の一時ディレクトリを使い、保存先を表示します。

```bash
python3 - <<'PYCODE'
import os, pathlib, shutil, subprocess, tempfile
version = subprocess.check_output(["lean", "--version"], text=True)
if "version 4.29.1," not in version:
    raise SystemExit("Lean4.29.1 is required for this evidence cut")
names = ["Support", "CurrentUse", "CurrentUseReview", "TrackedValidation",
         "GraphValidation", "GraphReview", "Passive", "ProducerFlow",
         "GeneralLabels", "FallibleFlow", "AbortFlow", "AddressFlow",
         "ResourceBoundary", "LocalContract", "ContractExport", "PureFunctions",
         "FunctionContractBridge", "ModuleContractBoundary", "OwnerAssignment",
         "ProfileGuarantees", "HandleValues", "PureHandleFunctions",
         "ResourceComputations", "ResourceComputationScopeControls",
         "SharedAuthorityUse", "ObserverLabels", "OwnerReadReport", "OperationIdentity"]
work = pathlib.Path(tempfile.mkdtemp(prefix="mir-proof-first-",
                                  dir=os.environ.get("PROOF_WORKDIR")))
print(work, flush=True)
env = dict(os.environ, LEAN_PATH=str(work))
for name in names:
    file = "MirroreaProofFirst" + name + ".lean"
    shutil.copyfile(pathlib.Path("samples/lean/foundations") / file, work / file)
    subprocess.run(["lean", "--trust=0", "-o", file[:-5] + ".olean", file],
                   cwd=work, env=env, check=True)
PYCODE
```

Support の有限 Lean/Python 比較は別に
`python3 scripts/proof_first_support_check.py --help` から実行条件を確認します。
実行済み cut と範囲は `docs/proof-first/` の検査記録、作業証跡は W1 の report2611 と W2 の report2612 にあります。

W2のDurableDispatch一般証明は既存ModuleContractBoundaryに含まれます。
別のLinux process停止試験は `python3 scripts/proof_first_durable_dispatch_check.py test <existing-work-directory>`。
これは予約同期と排他の参照実験であり、LeanとPythonの一般refinement、電源断、
真正な復旧、Mir sourceからのE2E、production採用を示すものではありません。

## 境界

- Lean built-in として repo が使うのは Lean 自体の構文と基本型
- security label、authority-sensitive predicate、capture / lifetime / cost model、review-unit / stub 構造は foundation file の user-defined definition
- final public theorem contract や full discharge をここで確定したわけではない


W4のqualified owner・codec・応答profile・完了用処理枠・実owner観測と通知履歴の一般証明候補は、既存の
`proof_first_reference_source_check.py` に `--with-owner-boundary` を付けて
fresh検査する（`--with-publication` を含む）。既存のW3 source検査と所有宣言監査を
再実行する仕組みであり、native worker／QUIC／権限の発行／W4完了を意味しない。
定義と未接続条件は [MirroreaProofFirstPublication.md](foundations/MirroreaProofFirstPublication.md)、今回の結果はReport2614に記録する。
active rootの追加や移動はない。

同じowner-boundary検査には、保持された設置事実・実owner世代の単調性・source登録との対応も含む。
初期化時の記録一致と全ownerのモデル残量に結び付いた設置consumerを一般証明する。
公開操作の全経路、予約容量・freshness、実通信との対応は別の未完了条件である。


## W4-B preserved host correspondence model (integration candidate)

The same LAB foundation root now retains the exact 206-module dependency cut
in `docs/proof-first/W4_SOURCE_MANIFEST.json`: 130 existing byte-identical sources
and 76 newly mirrored sources, including the independent EntryAcquisition and
ProducedRootedness roots. Original import/declaration names are preserved;
compiled objects and external captures are not source samples.

The existing runner has the optional `--with-host-model` flag, implying
`--with-owner-boundary`. It checks the exact source closure and rebuilds/audits
in bounded serial batches. The first full model run passed277 commands and
236module/22215declaration audits. A strengthened successor also binds actual
import paths and successful source/object outputs before/after use; its fresh
full validation passed278commands with236build receipts and9168file bindings.
The current V2 observed model passed278commands/9169bindings; native preparation
passed235commands and the repaired physical stage passed68commands/15profiles/53
controls. Whole exits and bound inputs were verified. W4-B bounded integration
is closed; C premises and Rust/Core/privateQUIC correspondence remain open; see the three-stage workflow in `scripts/README.md`.

Candidate command (existing external workroot required):

```bash
python3 scripts/proof_first_reference_source_check.py --work-root <existing-external-workdir> --with-host-model
```

The source-manifest controls in `scripts/tests/proof_first_host_manifest_cases.py`
accept the exact source cut and reject eight damaged copies. Those are integrity
checks, not semantic theorems or actual-process E2E evidence. See
`docs/proof-first/W4_CHECK.json` and Report2614 for exact run status, failed attempts,
compiler/CPython/capture assumptions and remaining integration obligations.


`host-reference/` retains two native entry sources, the source-input control
consumer and an extracted negative-test header. `HostControlHeader.lean` is a
code-generation fragment, not a standalone module or theorem. These private
reference sources are staged externally by `scripts/proof_first_host_prepare.py`;
the current observed native preparation and repaired physical runs passed235 and68
commands respectively, with whole exits and input bindings checked. Final W4-B
scope review was collected and locally disposed; B is a bounded LAB integrated
candidate. C admission/authority premises remain open; Rust/Core/privateQUIC
remains unconnected. Their source
origins/transformations are in `docs/proof-first/W4_HOST_MANIFEST.json`.


W4-C の局所共有 authority floor の限定修正は
[SharedAuthorityUse](foundations/MirroreaProofFirstSharedAuthorityUse.md) に定義・
一般証明・Rust 対応・TCB を記録する。上の手動 fresh-copy 検査に含め、凍結済みの
W4-B 206-module manifest には追加しない。実networkやW4-C全体の完了ではない。

W4-Cの供給ラベル保持と一権限での出力許可は
[ObserverLabels](foundations/MirroreaProofFirstObserverLabels.md) に記録します。
元ラベルの真正性・現行権限・二実行の機密性は別義務です。

W4-Cの実測読取り記録から応答への限定変換は
[OwnerReadReport](foundations/MirroreaProofFirstOwnerReadReport.md) に記録します。
存在しない読取りの0補完を除き、実際の0は保持します。全キーの網羅性は
checked Coreと実測mapの対応が前提であり、認可・機密性の証明ではありません。

W4-Cの既存操作識別子の一意検索は
[OperationIdentity](foundations/MirroreaProofFirstOperationIdentity.md) に記録します。
重複する同一レコードも拒否し、正常なsingletonは受理します。sourceの文順序、
異なるowner間のfragment識別子、source-free復元の保証には広げません。

W4-Cの必要なowner schemaの保持は
[OwnerSchema](foundations/MirroreaProofFirstOwnerSchema.md) に記録します。
同じownerの別namespaceを読む式について、targetと全RHS readの宣言を保持する
一般命題と実M7/SYS3検査を対応付けます。39依存moduleと全所有宣言auditは
companion内の外部fresh-copyコマンドで再構築できます。既存W4-Bの凍結manifestや
上の小規模手動loopには追加しません。namespace全体の一意性・typedなsourceへの
結合・現在の認可・private imageの真正性は別の境界です。
