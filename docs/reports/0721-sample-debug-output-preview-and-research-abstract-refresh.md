# Report 0721 — sample debug output preview and research abstract refresh

- Date: 2026-04-17T02:27:04.454439Z
- Author / agent: Codex (GPT-5)
- Scope: helper-local sample debug output preview、prototype sidecar update、research_abstract 全面刷新、snapshot / FAQ / traceability 同期
- Decision levels touched: L2 / L3

## 1. Objective

- sample / prototype 実行時に debug 用の可視化を見やすくする helper-local cut を追加する。
- ただし final host-I/O / stdio / FFI / public shell contract を premature に固定しない。
- `docs/research_abstract/` を累積 drift から切り離し、current repo の本質だけが読める concise summary に全面書き直す。
- 関連 docs / snapshot / FAQ / traceability を current reading に揃える。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `specs/examples/451-current-l2-runnable-prototype-and-not-implemented-sample-buckets.md`

## 3. Actions taken

1. `mir-semantics::RunReport` に `final_place_store` を追加し、interpreter 実行後の place-store snapshot を helper-local runtime summary へ渡せるようにした。
2. `mir-runtime` operational CLI に `debug_outputs` 集約を追加し、`debug_*` または `_debug_` を含み `_output` / `_pipe` で終わる target へ append-record された内容を pretty summary に表示する cut を入れた。
3. prototype sidecar 3 本に debug preview record を追加した。
   - `dice_debug_text_output`
   - `dice_recovery_debug_output`
   - `avatar_controller_debug_output`
4. operational CLI / sample runner test を拡張し、debug preview record が actual execution から見えることを ratchet した。
5. `specs/examples/452` を新設し、helper-local debug output preview cut を docs-first current judgment として整理した。
6. `docs/research_abstract/README.md` と phase0〜6 を全面書き直し、累積ログではなく concise current reading だけを保つ構成へ移した。
7. `Documentation.md`、`progress.md`、`tasks.md`、`faq_005.md`、`samples/prototype/README.md`、`plan/01`、`08`、`09`、`10`、`11`、`18`、`90`、`specs/00-document-map.md` を current reading に同期した。
8. reviewer 指摘を受け、debug preview target 判定を `debug_*` または `_debug_` を含み `_output` / `_pipe` で終わる target に 좁め、generic `*_output` / `*_pipe` を誤って helper-local preview と見なさないようにした。
9. narrow re-review で no actionable findings を確認した。

## 4. Files changed

- code
  - `crates/mir-semantics/src/lib.rs`
  - `crates/mir-runtime/src/current_l2_cli.rs`
  - `crates/mir-runtime/tests/current_l2_operational_cli.rs`
  - `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- samples
  - `samples/prototype/current-l2-order-handoff/p01-dice-publication-handoff.host-plan.json`
  - `samples/prototype/current-l2-order-handoff/p02-dice-publication-fallback.host-plan.json`
  - `samples/prototype/current-l2-dynamic-attach-detach/p03-avatar-controller-attach-detach.host-plan.json`
  - `samples/prototype/README.md`
- specs / plan / snapshot / FAQ
  - `specs/examples/452-current-l2-debug-output-preview-helper-cut.md`
  - `specs/00-document-map.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `faq_005.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/08-representative-programs-and-fixtures.md`
  - `plan/09-helper-stack-and-responsibility-map.md`
  - `plan/10-roadmap-overall.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
  - `plan/90-source-traceability.md`
- research abstract
  - `docs/research_abstract/README.md`
  - `docs/research_abstract/phase0-repository-memory-and-decision-boundary.md`
  - `docs/research_abstract/phase1-current-l2-semantics-stabilization.md`
  - `docs/research_abstract/phase2-parser-free-poc-and-detached-validation-loop.md`
  - `docs/research_abstract/phase3-parser-boundary-and-first-checker-cut.md`
  - `docs/research_abstract/phase4-shared-space-membership-and-practical-room-boundary.md`
  - `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
  - `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`

## 5. Commands run and exact outputs

```bash
$ df -h .
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        99G   77G   18G  82% /

$ free -h
               total        used        free      shared  buff/cache   available
Mem:           960Mi       496Mi       139Mi       2.0Mi       325Mi       315Mi
Swap:             0B          0B          0B

$ python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .

$ cargo test -p mir-runtime --test current_l2_source_sample_runner
test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test -p mir-runtime --test current_l2_operational_cli
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

最終検証は §6 に追記する。

## 6. Evidence / findings

- sample / prototype 実行後の final place-store から debug preview target を拾い、CLI summary に `debug_outputs` として表示できることを確認した。
- rollback で消えた record は preview に残さないため、helper-local convenience でありつつ semantic honesty を壊していない。
- reviewer 初回指摘だった「generic `*_output` / `*_pipe` まで debug preview 扱いする drift」は naming rule narrowing で解消した。
- current cut は helper-local convenience に留まり、final host-I/O / stdio 導入を意味しない。
- prototype sidecar の debug record により、
  - publication / handoff
  - fallback / recovery
  - attach / detach
  の意図を sample summary 上で読み取りやすくした。
- `research_abstract` は current repo の本質を追える concise summary に置き換え、old cumulative wording を除去した。

最終確認で実施した command:

```bash
$ cargo test -p mir-runtime --test current_l2_source_lowering
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test -p mir-runtime --test current_l2_source_sample_runner
test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test -p mir-runtime --test current_l2_operational_cli
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p01-dice-publication-handoff.txt --format pretty
sample: p01-dice-publication-handoff
...
debug_outputs:
- dice_debug_text_output:
  - roll_dice: player_a -> visible
  - handoff_dice_authority: player_a -> player_b

$ cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p02-dice-publication-fallback.txt --format pretty
sample: p02-dice-publication-fallback
...
debug_outputs:
- dice_recovery_debug_output:
  - fallback: loaded last confirmed roll

$ cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-dynamic-attach-detach/p03-avatar-controller-attach-detach.txt --format pretty
sample: p03-avatar-controller-attach-detach
...
debug_outputs:
- avatar_controller_debug_output:
  - attach_avatar_controller: avatar_a
  - detach_avatar_controller: avatar_a

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 720 numbered report(s).

$ git diff --check
# no output
```

## 7. Changes in understanding

- sample 実行時の可視化は、final host-I/O を待たなくても helper-local preview として切り出せる。
- その際、debug output target naming を sample family ごとの仮名に留めれば、final shell / FFI / transcript contract を premature に固定せずに済む。
- helper-local preview だと明示するなら、naming rule も helper-local intent を反映した narrow rule に保つ方が drift を避けやすい。
- `research_abstract` は phase memory の再利用ではなく、phase role / fixed reading / source-backed evidence / open / handoff を短く保つ方が repo current state と整合する。

## 8. Open questions

- debug output preview を json summary でどこまで強く見せるかは current helper cut のままで十分か。
- typed / theorem / model-check が sample-visible になる prototype 群を、`samples/prototype/` の second tranche としてどう並べるか。
- final host-I/O / adapter / shell contract をどの macro phase で mixed gate に昇格させるか。

## 9. Suggested next prompt

`samples/prototype/` の second tranche として、typed / theorem / model-check が sample-visible になる corrected prototype 群を追加し、debug output preview と emitted verification artifact を並べて見られるようにしてください。
