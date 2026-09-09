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
- [Aborting sequences](foundations/MirroreaProofFirstAbortFlow.md): 失敗による後続依存と完了 bit の分離。追加候補の Oracle review は回収待ち。
- [Reference resolution](foundations/MirroreaProofFirstAddressFlow.md): 別名を許す固定参照解決と評価・checker・実行列の対応。同じ review cut。
- [Passive observation](foundations/MirroreaProofFirstPassive.md): 有限消去と、別の二実行保証。限定された数学的範囲のreview済み。
- [W2 contracts/resources/functions](foundations/MirroreaProofFirstContracts.md): 資源の分割・移譲、局所契約からの値の輸出、純粋な高階関数・有限反復と実行対応。10ファイルの研究候補であり、Oracle review と既存Mirへの接続は未完了。

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
         "ProfileGuarantees", "HandleValues", "PureHandleFunctions"]
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
