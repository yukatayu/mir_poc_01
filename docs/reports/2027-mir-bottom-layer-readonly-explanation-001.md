# Report 2027 — Mir bottom layer readonly explanation 001

## Objective

Mir の最下層としての位置づけ、sample の所在、実行コマンド、native binary / binary split の現状、型システム・model check・Lean / dependent-type 境界、I/O と計算能力の current-scope explanation を repo evidence に基づいて整理する。

## Scope and assumptions

- 説明専用の read-only inspection を scope とする。
- 新機能実装、仕様変更、public API / ABI freeze、sample 昇格は行わない。
- user question は Mir semantic core / current-L2 / clean-near-end active suite の current repo state を聞いているものとして読む。

## Start state / dirty state

- branch: `docs/layered-repro-guide-001`
- start dirty state:
  - `?? docs/reports/1177-layered-repro-guide-001-readonly-repro-audit.md`
- この task では既存 untracked report を revert しない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/09-invariants-and-constraints.md`
- `specs/13-type-system-lifetime-fallback.md`
- `specs/14-contract-subtyping-layer-compatibility.md`
- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/06-current-l2-interpreter-skeleton.md`
- `plan/00-index.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/23-compiler-backend-llvm-guardrail-roadmap.md`
- `samples/current-l2/README.md`
- `samples/clean-near-end/README.md`
- `samples/lean/README.md`
- `samples/current-l2/e2-try-fallback.txt`
- `samples/clean-near-end/model-check/01_peterson_sc_pass.mir`
- `samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean`
- `samples/lean/clean-near-end/01_peterson_sc_pass/01_peterson_sc_pass.lean`
- `samples/lean/manifest.json`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `scripts/current_l2_source_sample_regression.py`

## Actions taken

1. Mir / current-L2 / clean-near-end / Lean / codegen roadmap の関連 docs を順に読んだ。
2. active sample root と current-L2 source corpus の違いを確認した。
3. `mir-current-l2` operational CLI と `clean_near_end_samples.py` の current command surface を確認した。
4. current-L2 runtime sample 1 本と clean-near-end model-check family を実行した。
5. native binary / packaging / projection/codegen の stop line を roadmap から確認した。
6. Lean foundations と generated theorem stub の差を確認した。

## Files changed

- `docs/reports/2027-mir-bottom-layer-readonly-explanation-001.md` を追加

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
sed -n '1,220p' /home/yukatayu/.codex/skills/superpowers/skills/using-superpowers/SKILL.md
sed -n '1,220p' .agents/skills/discord-report/SKILL.md
sed -n '1,220p' README.md
sed -n '1,220p' Documentation.md
sed -n '1,220p' specs/00-document-map.md
sed -n '1,260p' specs/01-charter-and-decision-levels.md
sed -n '1,260p' specs/02-system-overview.md
sed -n '1,260p' specs/03-layer-model.md
sed -n '1,260p' specs/04-mir-core.md
sed -n '1,260p' specs/07-typed-effects-wiring-platform.md
sed -n '1,220p' specs/09-invariants-and-constraints.md
sed -n '1,520p' specs/13-type-system-lifetime-fallback.md
sed -n '1,460p' specs/14-contract-subtyping-layer-compatibility.md
sed -n '1,240p' specs/examples/00-representative-mir-programs.md
sed -n '1,220p' specs/examples/06-current-l2-interpreter-skeleton.md
sed -n '1,260p' plan/00-index.md
sed -n '1,260p' plan/07-parser-free-poc-stack.md
sed -n '1,260p' plan/23-compiler-backend-llvm-guardrail-roadmap.md
sed -n '1,240p' samples/current-l2/README.md
sed -n '1,220p' samples/clean-near-end/README.md
sed -n '1,180p' samples/lean/README.md
sed -n '1,220p' samples/current-l2/e2-try-fallback.txt
sed -n '1,220p' samples/clean-near-end/model-check/01_peterson_sc_pass.mir
sed -n '1,200p' samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean
sed -n '1,60p' samples/lean/clean-near-end/01_peterson_sc_pass/01_peterson_sc_pass.lean
sed -n '1,220p' samples/lean/manifest.json
sed -n '1,260p' crates/mir-runtime/src/current_l2_cli.rs
sed -n '1,220p' crates/mir-runtime/src/current_l2.rs
sed -n '1,260p' crates/mir-runtime/tests/current_l2_operational_cli.rs
sed -n '1,220p' crates/mir-semantics/tests/current_l2_minimal_interpreter.rs
sed -n '1,260p' scripts/current_l2_source_sample_regression.py
find samples/current-l2 -maxdepth 2 \( -name '*.host-plan.json' -o -name '*.json' \)
find crates/mir-ast/tests/fixtures/current-l2 -maxdepth 1 \( -name 'e2-try-fallback.json' -o -name 'e2-try-fallback.host-plan.json' -o -name 'e21-try-atomic-cut-frontier.json' -o -name 'e21-try-atomic-cut-frontier.host-plan.json' \) | sort
rg -n "projection/codegen|emitted executable|emitted-program ABI|installed binary|packaging|place-program|placement optimizer|deployment planner|native" README.md Documentation.md plan specs crates scripts -g '!target'
rg -n "\b(add|sub|mul|div|print|stdout|stdin|stderr|hello|numeric|nat|int|bool)\b" samples/current-l2 samples/clean-near-end specs/examples/00-representative-mir-programs.md crates/mir-ast/src crates/mir-semantics/src -g '!target'
cargo run -q -p mir-runtime --example mir_current_l2 -- --help
cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/clean-near-end/typing/01_authorized_declassification.mir --format json
cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/clean-near-end/model-check/01_peterson_sc_pass.mir --format json
cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample e2-try-fallback --host-plan crates/mir-ast/tests/fixtures/current-l2/e2-try-fallback.host-plan.json --format json
python3 scripts/clean_near_end_samples.py run model-check --format json
python3 - <<'PY'
from pathlib import Path
import re
nums=[]
for p in Path('docs/reports').glob('*.md'):
    m=re.match(r'(\\d+)-', p.name)
    if m:
        nums.append(int(m.group(1)))
print(max(nums) if nums else 0)
PY
```

## Evidence / outputs / test results

- `mir-current-l2 --help` は current docs-only shell を
  `run-source-sample` / `check-source-sample` に限定していた。
- `cargo run ... run-source-sample e2-try-fallback --host-plan ... --format json` は
  `static_verdict = valid`、
  `entered_evaluation = true`、
  `terminal_outcome = success`、
  `steps_executed = 8`、
  `events = ["performsuccess", "performfailure", "rollback", "performsuccess"]`
  を返した。
- `cargo run ... check-source-sample samples/clean-near-end/typing/01_authorized_declassification.mir --format json`
  は `static_verdict = valid`、`terminal_outcome = success`、authority / declassify constraint solved を返した。
- `cargo run ... check-source-sample samples/clean-near-end/model-check/01_peterson_sc_pass.mir --format json`
  は `model_check_result = pass`、`property = mutual_exclusion`、`checked_under = sequential_consistency` を返した。
- `python3 scripts/clean_near_end_samples.py run model-check --format json` は
  - `01_peterson_sc_pass`: `pass`
  - `02_peterson_relaxed_counterexample`: `counterexample`
  - `03_broken_mutex_counterexample`: `counterexample`
  を返した。
- `samples/lean/manifest.json` は foundations と generated clean-near-end stub の verification `success = true` を保持していた。
- `samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean` は actual Lean proof fragment を持つ。
- `samples/lean/clean-near-end/01_peterson_sc_pass/01_peterson_sc_pass.lean` は `True := by trivial` の generated theorem stub であり、domain proof discharge completion ではない。
- `plan/23-compiler-backend-llvm-guardrail-roadmap.md` は actual LLVM build、final backend choice、final installed-binary packaging success criteria は fixed しないと明記していた。
- `README.md` / `Documentation.md` は projection/codegen current evidence を final emitted executable family / final public emitted-program ABI と読まないよう明記していた。

## What changed in understanding

- 「一番下のレイヤー」は repo の設計層としては Mir core でよいが、`specs/03-layer-model.md` 上はその下に前提として OS / network substrate `L0` がある。
- current repo で実際に runnable な canonical sample root は `samples/clean-near-end/` であり、`samples/current-l2/` は source corpus であって active canonical executable suite ではない。
- current-L2 `.txt` sample 実行は host plan sidecar 前提であり、単独 `.txt` から final language runtime が動くわけではない。
- native binary compilation / binary splitting / final packaging は current repo scope では未完であり、roadmap でも stop line のままである。
- Lean は mechanization spine だが、current generated stub は theorem bridge floor であって Isabelle/Lean の fully discharged language ではない。

## Open questions

- Mir source principal surface を最終的にどこまで textual grammar として public 化するかは unresolved。
- emitted executable family / emitted-program ABI / installed binary packaging adoption target は unresolved。
- full dependent type theory / full dependent refinement subtyping / concrete theorem prover / model-checker production binding は deferred。
- Mir core と Typed-Effect Wiring Platform の境界で external I/O port をどう public 命名するかは later gate。

## Suggested next prompt

`Layer 1 の Mir core だけに絞って、event DAG / effect / atomic_cut / fallback / ownership / lifetime を current sample と合わせて詳しく説明してください`

## plan/ update status

- `plan/ 更新不要`

## Documentation.md update status

- `Documentation.md 更新不要`

## progress.md update status

- `progress.md 更新不要`

## tasks.md update status

- `tasks.md 更新不要`

## samples_progress.md update status

- `samples_progress.md 更新不要`

## reviewer findings and follow-up

- reviewer 未実施。説明専用 read-only inspection のため local evidence を優先した。

## skipped validations and reasons

- `python3 scripts/current_l2_source_sample_regression.py regression --run-label ... --artifact-root ...`
  は未実行。今回の質問は Mir bottom-layer explanation と command surface の確認であり、full regression rerun までは不要と判断した。
- `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  はこの turn では未再実行。前 task の closeout 実行はあるが、今回の fresh evidence は direct CLI / model-check family に絞った。
- `lake` / full Lean workspace rerun は未実行。`samples/lean/manifest.json` と source inspection までに留めた。

## commit / push status

- commit していない
- push していない

## sub-agent session close status

- sub-agent 不使用
