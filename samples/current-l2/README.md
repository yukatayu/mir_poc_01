# current-l2 source samples

この directory は、current L2 fixed subset の **source text sample corpus** を置く repo-root layer である。

## 位置づけ

- representative prose (`specs/examples/00...01`) でもない。
- machine-readable fixture corpus (`crates/mir-ast/tests/fixtures/current-l2/`) でもない。
- fixed subset の syntax-backed regression / lowering / runner / verification ladder に進むための第 3 層である。

## current policy

- directory は `samples/current-l2/` に固定する。
- 初期 corpus は flat layout にする。
- extension は final grammar を意味しない neutral text file として `.txt` を使う。
- sample stem は fixture stem / `fixture_id` と揃える。
- filename に verdict、parser tranche、reached stage は埋め込まない。

## initial cluster

`specs/examples/317...318` で fixed した initial cluster は次である。

- `e1-place-atomic-cut.txt`
- `e2-try-fallback.txt`
- `e3-option-admit-chain.txt`
- `e4-malformed-lineage.txt`
- `e21-try-atomic-cut-frontier.txt`
- `e23-malformed-try-fallback-missing-fallback-body.txt`

current authored corpus は、この initial sextet に

- `e22-try-atomic-cut-place-mismatch.txt`
- `e19-malformed-target-mismatch.txt`
- `e16-malformed-missing-chain-head-option.txt`
- `e18-malformed-missing-successor-option.txt`

を順に加えた decet に進んでいる。

## current mapping matrix

current first matrix は、representative prose / fixture corpus / source target を次の row で結ぶ。

| order | sample stem | representative anchor | status | fixture id | mode | source target | coverage focus | expected static | expected runtime |
|---|---|---|---|---|---|---|---|---|---|
| 1 | `e1-place-atomic-cut` | `E1` | `direct` | `e1_place_atomic_cut` | `runtime_fixture` | `samples/current-l2/e1-place-atomic-cut.txt` | post-cut failure でも pre-cut mutation を rollback しない | `valid` | `explicit_failure` |
| 2 | `e2-try-fallback` | `E2` | `direct` | `e2_try_fallback` | `runtime_fixture` | `samples/current-l2/e2-try-fallback.txt` | local rollback 後に explicit fallback branch が走る | `valid` | `success` |
| 3 | `e3-option-admit-chain` | `E3-variant` | `variant` | `e3_option_admit_chain` | `runtime_fixture` | `samples/current-l2/e3-option-admit-chain.txt` | option-local `admit` miss を non-admissible skip に留める | `valid` | `success` |
| 4 | `e4-malformed-lineage` | `E4` | `direct` | `e4_malformed_lineage` | `static_only_fixture` | `samples/current-l2/e4-malformed-lineage.txt` | edge-local lineage mismatch を static stop に留める | `malformed` | `not_evaluated` |
| 5 | `e16-malformed-missing-chain-head-option` | `E16` | `direct` | `e16_malformed_missing_chain_head_option` | `static_only_fixture` | `samples/current-l2/e16-malformed-missing-chain-head-option.txt` | missing chain head option を source-backed static stop pair へ上げる | `malformed` | `not_evaluated` |
| 6 | `e19-malformed-target-mismatch` | `E19` | `direct` | `e19_malformed_target_mismatch` | `static_only_fixture` | `samples/current-l2/e19-malformed-target-mismatch.txt` | declared target mismatch を source-backed static stop pair へ戻す | `malformed` | `not_evaluated` |
| 7 | `e21-try-atomic-cut-frontier` | `E21` | `direct` | `e21_try_atomic_cut_frontier` | `runtime_fixture` | `samples/current-l2/e21-try-atomic-cut-frontier.txt` | try body 内 `atomic_cut` の frontier update | `valid` | `success` |
| 8 | `e22-try-atomic-cut-place-mismatch` | `E22` | `direct` | `e22_try_atomic_cut_place_mismatch` | `runtime_fixture` | `samples/current-l2/e22-try-atomic-cut-place-mismatch.txt` | nested place 内 `atomic_cut` の place-mismatch contrast | `valid` | `success` |
| 9 | `e18-malformed-missing-successor-option` | `E18` | `direct` | `e18_malformed_missing_successor_option` | `static_only_fixture` | `samples/current-l2/e18-malformed-missing-successor-option.txt` | missing successor option を source-backed static stop pair へ上げる | `malformed` | `not_evaluated` |
| 10 | `e23-malformed-try-fallback-missing-fallback-body` | unresolved representative anchor | `unresolved` | `e23_malformed_try_fallback_missing_fallback_body` | `static_only_fixture` | `samples/current-l2/e23-malformed-try-fallback-missing-fallback-body.txt` | empty `fallback_body` structural malformed floor | `malformed` | `not_evaluated` |

## current notes

- `e3` は plain `E3` ではなく fixture-side `source_example_id = E3-variant` を mirror する。
- `e23` は fixture-side `source_example_id = E23` を already 持つが、current representative prose row はまだない。
- current matrix 自体は source target path までを固定し、reached stage inventory は下の ladder row に分けて持つ。

## current authored files

- actual source file として current repo にあるのは `e1-place-atomic-cut.txt`、`e2-try-fallback.txt`、`e3-option-admit-chain.txt`、`e4-malformed-lineage.txt`、`e16-malformed-missing-chain-head-option.txt`、`e19-malformed-target-mismatch.txt`、`e21-try-atomic-cut-frontier.txt`、`e22-try-atomic-cut-place-mismatch.txt`、`e18-malformed-missing-successor-option.txt`、`e23-malformed-try-fallback-missing-fallback-body.txt` の current authored decet である。
- `e3` は source row / runner / inventory / ladder までは actualize 済みだが、formal hook stage は current top guard により `not reached (guarded)` に留める。
- current lowerer first cut は single-line `require` / `ensure` と inline `admit` fragment を受け、multiline clause suite は fail-closed に止める。
- current runner first cut は accepted sample set 内の explicit path と sample stem shorthand を受け、host plan は explicit input に留める。
- current stage 2 bridge floor は nested `place` block を top-level `Other` statement head として畳み、`e22` の place-mismatch contrast を current helper-local surface で受ける。

## current verification ladder

current reached-stage inventory は current authored decet `e1` / `e2` / `e3` / `e4` / `e16` / `e19` / `e21` / `e22` / `e18` / `e23` に付ける。

| sample stem | authored status | static gate | interpreter | formal hook | evidence route |
|---|---|---|---|---|---|
| `e1-place-atomic-cut` | `source-authored` | `reached(valid)` | `reached(explicit_failure)` | `reached(runtime_try_cut_cluster)` | `current_l2_source_sample_runner` + runtime detached bundle formal-hook smoke |
| `e2-try-fallback` | `source-authored` | `reached(valid)` | `reached(success)` | `reached(runtime_try_cut_cluster)` | `current_l2_source_sample_runner` + runtime detached bundle formal-hook smoke |
| `e3-option-admit-chain` | `source-authored` | `reached(valid)` | `reached(success)` | `not reached (guarded)` | `current_l2_source_sample_runner` + guarded detached-bundle rejection evidence |
| `e4-malformed-lineage` | `source-authored` | `reached(malformed)` | `not reached (static stop)` | `reached(fixture_static_cluster)` | `current_l2_source_sample_runner` + static-gate detached formal-hook smoke |
| `e16-malformed-missing-chain-head-option` | `source-authored` | `reached(malformed)` | `not reached (static stop)` | `reached(fixture_static_cluster)` | `current_l2_source_sample_runner` + static-gate detached formal-hook smoke |
| `e19-malformed-target-mismatch` | `source-authored` | `reached(malformed)` | `not reached (static stop)` | `reached(fixture_static_cluster)` | `current_l2_source_sample_runner` + static-gate detached formal-hook smoke |
| `e21-try-atomic-cut-frontier` | `source-authored` | `reached(valid)` | `reached(success)` | `reached(runtime_try_cut_cluster)` | `current_l2_source_sample_runner` + runtime detached bundle formal-hook smoke |
| `e22-try-atomic-cut-place-mismatch` | `source-authored` | `reached(valid)` | `reached(success)` | `reached(runtime_try_cut_cluster)` | `current_l2_source_sample_runner` + runtime detached bundle formal-hook smoke |
| `e18-malformed-missing-successor-option` | `source-authored` | `reached(malformed)` | `not reached (static stop)` | `reached(fixture_static_cluster)` | `current_l2_source_sample_runner` + static-gate detached formal-hook smoke |
| `e23-malformed-try-fallback-missing-fallback-body` | `source-authored` | `reached(malformed)` | `not reached (static stop)` | `reached(fixture_static_cluster)` | `current_l2_source_sample_runner` + static-gate detached formal-hook smoke |

- `formal hook` reached は current package では source-runner-native artifact ではなく、fixture-aligned detached artifact route を使う。
- `runtime_try_cut_cluster` と `fixture_static_cluster` は tool-neutral formal hook の current top であり、concrete theorem/model-check tool choice はまだ混ぜない。
- `not reached (guarded)` は failure ではなく、`e3` runtime bundle が current `runtime_try_cut_cluster` family の外にあるため、formal hook widening を still later に残していることを意味する。
- current authoring / bless / regression flow は [.docs/current-l2-source-sample-authoring-policy.md](/home/yukatayu/dev/mir_poc_01/.docs/current-l2-source-sample-authoring-policy.md) と `python3 scripts/current_l2_source_sample_regression.py inventory|regression` を基準にする。
- theorem-first concrete pilot current cut では、tool-neutral formal hook artifact を入力にする `proof_notebook_review_unit` consumer を non-production helper/example に留める。
- post-sextet first cluster current cut では、`e21` frontier row と `e22` place-mismatch row を first runtime contrast pair として扱う。
- stable-static edge-pair first reopen では、existing `e4` row と new `e19` row を source-backed static-stop pair として actualize 済みである。

## non-goals

- final parser grammar の固定
- fixture JSON からの逆生成
- representative prose のそのまま昇格
- public CLI / exporter / backend path の先取り

## current authoring flow

- source sample を更新するときは source text / fixture mapping / matrix / ladder / snapshot docs を同じ task で揃える。
- `python3 scripts/current_l2_source_sample_regression.py inventory` は current authored decet の file presence と guarded row status を確認する。
- `python3 scripts/current_l2_source_sample_regression.py regression --run-label <label> --artifact-root <root>` は current authored decet の lowering / runner / ladder bundle を流し、formal-hook smoke sub-bundleは current top に入る 9 row だけへ留める。
- current `bless` は retained artifact archive ではなく、review 済み repo-local sync、`inventory` / `regression` success、必要なら emitted review-unit / model-check carrier helper output の inspection を意味する。

## next steps

- model-check / public-checker second reserve inventory は fixed 済みであり、`proof_notebook_review_unit` を current first concrete pilot に保ったまま machine-facing reserve line を整理した
- model-check concrete carrier first actualization gate も fixed 済みであり、`tool_neutral_formal_hook_only_input + compare_ready_docs_only_bridge_sketch` を entry にする narrow gate を current first choice に昇格した
- public operational surface actualization gate も fixed 済みであり、`run_current_l2_source_sample` を later public-pressure の first docs-only candidate、`run_current_l2_runtime_skeleton` / `lower_current_l2_fixed_source_text` を tranche-internal support、`resolve_current_l2_source_sample_path` と repo-local script/example surface を excluded bucket に留める current cut を採った
- shared-space identity / auth layering reopen も fixed 済みであり、membership identity core と auth/admission/projection side carriers の split を docs-first boundary に残した
- stable malformed broader follow-up inventory も fixed 済みであり、broader stable malformed next reopen order は missing-option family first、capability family second、duplicate cluster と `TryFallback` / `AtomicCut` malformed-static family は later に残す
- docs-first I/O / host-facing port boundary も fixed 済みであり、language core に privileged `stdin/stdout` を入れず、capability-scoped input/output port / adapter boundary を first docs-only cut に置き、visualizer / host substrate / host runtime を consumer/provider 側、FFI / game engine adapter と final naming を later gate に残す
- stable malformed missing-option first reopen actualization comparison も fixed 済みであり、helper-local compare を entry evidence に再利用しつつ、first reopen family は `e16/e17/e18` triplet、current next actualization mode は source-backed widening first に置く
- stable malformed missing-option first source-backed widening actualization も fixed 済みであり、`e16` / `e18` は source-authored static-stop pair として actualize 済み、`e17` は same-family staged guard に留める
- public operational CLI / final public contract later gate も fixed 済みであり、public-side later ordering は final public parser / checker / runtime API first、public operational CLI second に残す
- model-check concrete carrier actualization comparison も fixed 済みであり、sample-visible theorem/model-check line の順序は actual carrier first、source-sample emitted verification artifact wiring second、sample-facing summary third に fixed 済みである
- model-check concrete carrier first actualization も fixed 済みであり、tool-neutral formal hook only hard input から row-local machine-facing carrier list を actualize 済みである
- source-sample emitted verification artifact wiring も fixed 済みであり、runtime test/support helper-local route として `source report -> formal hook reached/guarded split -> proof_notebook_review_units / model_check_concrete_carriers` fan-out を actualize 済みである
- sample-facing theorem/model-check evidence summary and bless/review flow も fixed 済みであり、README / `.docs` / snapshot docs を sample-facing surface、reviewed repo-local sync + inventory/regression success を current bless に置く docs-first cut を採っている
- final public parser / checker / runtime first later gate actualization comparison も fixed 済みであり、current first later cut は `run_current_l2_source_sample` と `CurrentL2SourceSampleRunReport` を public entry / report に置く runtime-led thin library facade に留め、`CurrentL2LoweredSourceProgram` / `CurrentL2RuntimeSkeletonReport` / `CurrentL2CheckerFloorReport` / `RunReport` を nested carrier として扱う
- public operational CLI second later gate actualization comparison も fixed 済みであり、current first cut は runtime-led thin facade を巻き戻さない Rust-side operational wrapper over `run_current_l2_source_sample` に留める
- final public parser/checker/runtime thin-facade later support actualization も fixed 済みであり、later support cut は `run_current_l2_runtime_skeleton` + `CurrentL2RuntimeSkeletonReport` に置く
- capability second reopen actualization comparison も fixed 済みであり、current family judgment は `e13/e20` pair、next malformed-side actualization mode は source-backed widening first に置く
- public operational CLI concrete shell naming comparison も fixed 済みであり、current docs-only shell family は `mir-current-l2 run-source-sample`、shell concern は `<sample>` / `--host-plan` / `--format pretty|json` に留める
- capability second source-backed widening actualization comparison も fixed 済みであり、current actualization family は `e13/e20` source-authored static-stop pair、actualized surface は source sample / lowerer / runner / ladder / regression helper / fixture-static formal-hook smoke に留める
- repo-level current line は public operational CLI concrete shell actualization comparison であり、その後に stable malformed capability second source-backed widening actualization を reserve に置く
