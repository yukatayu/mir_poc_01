# Report 0634 — FAQ 002 current-state clarification for place, patching, samples, and type system

- Date: 2026-04-12 15:53:02 JST
- Author / agent: Codex
- Scope: `faq_002.md` の新規作成による current repo 状態の Q&A 整理
- Decision levels touched: L0-L2 summary only, no normative decision change

## 1. Objective

user から指定された 4 つの質問

1. `place` の意味
2. runtime patch / attach / detach の想定
3. source sample の実行・自動テスト・LLVM timing
4. 型体系と lambda cube の位置づけ

に対し、current repo の正本文書と current 実装状況に沿った FAQ を
`faq_002.md` として追加する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `faq_001.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/examples/121-shared-space-authoritative-room-baseline.md`
- `specs/examples/125-shared-space-control-plane-carrier-threshold.md`
- `plan/04-core-semantics-current-l2.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `docs/research_abstract/phase1-current-l2-semantics-stabilization.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `docs/reports/0628-phase6-source-sample-verification-ladder-wiring.md`
- `docs/reports/0629-phase6-source-sample-authoring-bless-regression-policy.md`
- `docs/reports/0630-phase6-theorem-first-concrete-tool-pilot.md`
- `samples/current-l2/README.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_verification_ladder.rs`
- `crates/mir-semantics/src/lib.rs`
- `scripts/current_l2_source_sample_regression.py`

## 3. Actions taken

1. AGENTS 指定順に基礎文書と current snapshot を再読した。
2. 既存 `faq_001.md` を確認し、FAQ 文書の語り口と「決定済み / 未決 / 現時点の読み」の分け方を踏襲した。
3. subagent 2 本で
   - `place / patch / dynamic attachment`
   - `sample execution / tests / LLVM timing / type system`
   を並列確認した。
4. subagent 結果を local 側の current code / test / policy anchor で照合し、Q&A を `faq_002.md` に整理した。
5. regression helper の current CLI に `--overwrite` が無いことを確認し、`.docs/current-l2-source-sample-authoring-policy.md` の 1 行だけ drift 修正した。
6. FAQ が supplemental Q&A であり、既存 `faq_001.md` も index mirror に含めていないことを確認したうえで、`Documentation.md` / `progress.md` / `tasks.md` / `plan/` / `specs/00-document-map.md` は今回更新しない判断を採った。

## 4. Files changed

- Added: `faq_002.md`
- Added: `docs/reports/0634-faq-current-state-place-patch-samples-and-type-system.md`
- Updated: `.docs/current-l2-source-sample-authoring-policy.md`

## 5. Commands run and exact outputs

- `df -h .`
  - output:
    - `Filesystem      Size  Used Avail Use% Mounted on`
    - `/dev/vda2        99G   76G   19G  81% /`
- `free -h`
  - output:
    - `total        used        free      shared  buff/cache   available`
    - `Mem:           960Mi       557Mi        92Mi       132Ki       463Mi       402Mi`
    - `Swap:           19Gi       1.2Gi        18Gi`
- `rg --files | rg 'faq_00[12]\\.md|faq'`
  - output:
    - `faq_001.md`
    - `docs/reports/0111-faq-unresolved-issues-current-state.md`
- `date '+%Y-%m-%d %H:%M:%S %Z'`
  - output:
    - `2026-04-12 15:53:02 JST`
- `python3 scripts/current_l2_source_sample_regression.py inventory`
  - output:
    - `current L2 fixed-subset first-cluster inventory`
    - `e2-try-fallback | source-authored | valid | success | runtime_try_cut_cluster | present | first authored trio runtime path`
    - `e4-malformed-lineage | source-authored | malformed | not_evaluated | fixture_static_cluster | present | first authored trio static stop`
    - `e23-malformed-try-fallback-missing-fallback-body | source-authored | malformed | not_evaluated | fixture_static_cluster | present | first authored trio static stop`
    - `e1-place-atomic-cut | source-target-only | not_yet_authored | not_yet_authored | not_yet_authored | absent | deferred authored row`
    - `e3-option-admit-chain | source-target-only | not_yet_authored | not_yet_authored | not_yet_authored | absent | deferred authored row`
    - `e21-try-atomic-cut-frontier | source-target-only | not_yet_authored | not_yet_authored | not_yet_authored | absent | deferred authored row`
- `python3 scripts/current_l2_source_sample_regression.py regression --help`
  - output:
    - `usage: current_l2_source_sample_regression.py regression [-h] [--artifact-root ARTIFACT_ROOT] [--run-label RUN_LABEL]`
- `python3 scripts/current_l2_source_sample_regression.py regression --run-label faq002-status --artifact-root target/current-l2-source-sample-regression-faq002`
  - output summary:
    - `[1/7] runtime lowering test ... ok`
    - `[2/7] source sample runner test ... ok`
    - `[3/7] verification ladder test ... ok`
    - `[4/7] formal hook support test ... ok`
    - `[5/7] runtime formal hook smoke for e2-try-fallback`
    - `[6/7] static formal hook smoke for e4-malformed-lineage`
    - `[7/7] static formal hook smoke for e23-malformed-try-fallback-missing-fallback-body`
    - `all regression commands passed`
- `python3 scripts/validate_docs.py`
  - output:
    - `Documentation scaffold looks complete.`
    - `Found 633 numbered report(s).`
- `git diff --check`
  - output:
    - no output
- `git status --short`
  - output:
    - ` M .docs/current-l2-source-sample-authoring-policy.md`
    - `?? docs/reports/0634-faq-current-state-place-patch-samples-and-type-system.md`
    - `?? faq_002.md`

## 6. Evidence / findings

- `place` は current repo では deployment 単位ではなく、「local state / rollback scope / ownership transfer を解釈する最小の意味論的 locus」である。
- `atomic_cut` も place-local finalizing cut であり、process 全体・分散合意・永続化完了ではない。
- runtime patch / overlay / rebinding は方向性として強く想定されているが、full operational semantics や in-flight task rule は未決である。
- source sample は current fixed subset で actual に走るが、authored 済みなのは `e2` / `e4` / `e23` の first trio に留まる。
- source sample の inventory / regression helper と cargo test 群は存在し、repo-local policy も `.docs/current-l2-source-sample-authoring-policy.md` にある。
- local regression helper 実行では 7/7 command が passed し、sample 実行・ladder・formal-hook smoke の current cut を再確認した。
- LLVM-family backend / external codegen binding は near-term mainline に入っておらず、later / reserve として扱われている。
- current 実装の executable core は typed lambda calculus として cube 上に素直に置ける形ではなく、full Mir theory 全体も type system だけではないため cube の 1 点に還元しづらい。
- subagent `Halley` は `place` / `patch` 系の根拠として `specs/04`、`05`、`09`、shared-space baseline を抽出した。
- subagent `Poincare` は sample execution / regression / LLVM timing / lambda-cube 非適合の根拠として `samples/current-l2/README.md`、`crates/mir-runtime`、`plan/10..12`、phase abstract 群を抽出した。

## 7. Changes in understanding

- user の `place = server / process / executable` 直感は、repo の current semantics とはズレていることを明示的に FAQ 化する必要があると再確認した。
- user の `runtime patch attach/detach` 方向は repo の safe evolution / Mirrorea / shared-space baseline と整合するが、repo が fix 済みなのは operational detail ではなく layering と invariants までであると整理し直した。
- 型体系については「System F より強そう」という直感をそのまま肯定するより、lambda cube が扱う軸と Mir が扱う軸の違いを先に説明する方が誤解が少ないと判断した。

## 8. Open questions

- `place` と actual process / executable packaging の最終対応をどこで固定するか。
- in-flight coroutine / task が route change / patch activation をまたぐときの exact rule をどう切るか。
- final type system をどの形式体系として整理するか。
- type subsystem 単体を将来 lambda-cube 系で説明するのか、ownership / effect / contract を含む別整理にするのか。

## 9. Suggested next prompt

`faq_002.md` を踏まえて、次は「source sample を増やす順序」と「type subsystem を切り出して将来どう整理したいか」を、current docs に沿って別 FAQ または plan memo として整理してください。

## 補足

- `Documentation.md` 更新不要
- `progress.md` 更新不要
- `tasks.md` 更新不要
- `plan/` 更新不要
- `specs/00-document-map.md` 更新不要
