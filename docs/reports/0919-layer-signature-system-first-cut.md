# Report 0919 — LayerSignature system first cut

## 1. Title and identifier

- Identifier: `0919`
- Title: `LayerSignature system first cut`

## 2. Objective

current promoted package `LayerSignature system` を close し、
helper-local / report-local の minimal layer carrier を
Sugoroku world vertical slice と clean near-end current layer に first cut として追加する。

## 3. Scope and assumptions

- この package は final public layer law schema ではない。
- current helper / sample / report を横断する evidence-oriented metadata と debug surface だけを置く。
- final public auth / visualization / projection / hot-plug API、real network transport、packaging は deferred とする。
- helper 側と runtime 側の layer 名は first cut では厳密一致を要求しない。

## 4. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/05-mirrorea-fabric.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/reports/0918-term-signature-registry-debug-output.md`

## 5. Actions taken

- `TermSignature registry / debug output` close 後の next package として `LayerSignature system` を promoted line に昇格した。
- helper/runtime/docs seams を並列調査し、current stop line と minimal insertion point を回収した。
- `scripts/tests/test_sugoroku_world_samples.py` に `layer_signatures`、`--debug layers`、closeout `layer_signature_kinds` を要求する test を追加した。
- `scripts/sugoroku_world_samples.py` に `LayerSignature` carrier、`verification` / `runtime_trace` / `membership` layer inventory、`--debug layers`、closeout `layer_signature_kinds` / `reserved_layer_signature_kinds` を追加した。
- `crates/mir-runtime/src/clean_near_end.rs` と `crates/mir-runtime/src/bin/mir-clean-near-end.rs` に runtime report-local `layer_signatures` inventory と closeout carrier を追加した。
- `crates/mir-runtime/tests/clean_near_end_samples.rs` に runtime report / closeout 側の `LayerSignature` inventory test を追加した。
- `progress.md`、`tasks.md`、`samples_progress.md`、`plan/11-roadmap-near-term.md`、`docs/research_abstract/mirrorea_future_axis_01.md` を同期し、next promoted line を `MessageEnvelope / Auth seam` に進めた。

### Files changed

- `scripts/sugoroku_world_samples.py`
- `scripts/tests/test_sugoroku_world_samples.py`
- `crates/mir-runtime/src/clean_near_end.rs`
- `crates/mir-runtime/src/bin/mir-clean-near-end.rs`
- `crates/mir-runtime/tests/clean_near_end_samples.rs`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `plan/11-roadmap-near-term.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`

### Commands run

- `python3 -m unittest scripts.tests.test_sugoroku_world_samples`
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug layers`
- `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
- `cargo test -p mir-runtime --test clean_near_end_samples`
- `cargo test -p mir-runtime`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

### Dirty state at task start

- この report は `0920` task 中に retroactive に整えた。
- report 整備開始時点の worktree は dirty で、`LayerSignature` first cut の code/docs diff はすでに入っていた。
- 少なくとも次が modified だった:
  - `scripts/sugoroku_world_samples.py`
  - `scripts/tests/test_sugoroku_world_samples.py`
  - `crates/mir-runtime/src/clean_near_end.rs`
  - `crates/mir-runtime/src/bin/mir-clean-near-end.rs`
  - `crates/mir-runtime/tests/clean_near_end_samples.rs`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `plan/11-roadmap-near-term.md`
  - `docs/research_abstract/mirrorea_future_axis_01.md`
- untracked だった principal file:
  - `docs/reports/0919-layer-signature-system-first-cut.md`

### Reviewer completion status

- reviewer completion: combined-diff review は `0920` task 中に実施済み。
- reviewer は docs/report drift を指摘し、それらは `0920` task 内で修正した。
- final quick reviewer pass でも no findings だった。
- `0919` code-side `LayerSignature` change に対する新たな failing finding は残らなかった。

## 6. Evidence / outputs / test results

- helper-side tests:
  - `python3 -m unittest scripts.tests.test_sugoroku_world_samples`
  - pass
  - evidence:
    - `Ran 17 tests`
    - `OK`
- helper-side focused debug:
  - `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug layers`
  - pass
  - evidence:
    - `LAYER SIGNATURES`
    - active helper layers:
      - `verification`
      - `runtime_trace`
- current-L2 floor regression:
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - pass
  - evidence:
    - `matrix.total_samples = 16`
    - families:
      - `typing = 5`
      - `order-handoff = 6`
      - `model-check = 3`
      - `modal = 2`
- runtime-side focused sample:
  - `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  - pass
  - evidence:
    - `sample = 05_delegated_rng_service`
    - layer signature name:
      - `transport_provider_boundary`
- runtime-side closeout:
  - `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
  - pass
  - evidence:
    - `layer_signatures` contains:
      - `auth_authority_witness`
      - `transport_provider_boundary`
      - `verification_model_check`
- runtime tests:
  - `cargo test -p mir-runtime --test clean_near_end_samples`
  - pass
  - evidence:
    - `running 18 tests`
    - `clean_near_end_closeout_records_layer_signature_inventory ... ok`
- broader runtime regression:
  - `cargo test -p mir-runtime`
  - pass
- docs / hierarchy:
  - `python3 scripts/check_source_hierarchy.py`
  - `python3 scripts/validate_docs.py`
  - `git diff --check`
  - all pass

## 7. What changed in understanding

- `LayerSignature` は `TermSignature` の上位互換ではなく、object inventory とは別軸の processing / ownership / evidence lane metadata として置くのが自然である。
- current helper first cut では `verification`、`runtime_trace`、`membership` を active layer とし、`auth`、`transport`、`telemetry`、`projection`、`hot-plug`、`visualization` は reserve name に留めるのが stop line に合う。
- runtime report-local lane では `transport_provider_boundary`、`auth_authority_witness`、`verification_model_check` のように sample family 寄りの名前を使う方が report evidence と結びつきやすい。helper/runtime naming の完全統一は後段 package でよい。

## 8. Open questions

- UNRESOLVED: helper 側と runtime 側の `LayerSignature` naming / lane law をどこまで共有 schema に寄せるか。
- UNRESOLVED: `laws` field を string list のまま保つか、later package で theorem/runtime/visualization anchor に分けるか。
- UNRESOLVED: `auth` / `transport` / `telemetry` / `projection` / `hot-plug` を active layer に上げる最小条件をどの package で固定するか。

## 9. Suggested next prompt

`MessageEnvelope / Auth seam` package を進めてください。transport / authentication / authorization / membership / capability / witness を潰さない最小 carrier を、Sugoroku helper と clean near-end runtime report の両方で読める narrow first cut として actualize してください。
