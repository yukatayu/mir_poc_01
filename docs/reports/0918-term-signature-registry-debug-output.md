# Report 0918 — TermSignature registry / debug output

## 1. Title and identifier

- Identifier: `0918`
- Title: `TermSignature registry / debug output`

## 2. Objective

current promoted package `TermSignature registry / debug output` を close し、
helper-local / report-local の minimal signature carrier を
Sugoroku world vertical slice と clean near-end current layer に first cut として追加する。

## 3. Scope and assumptions

- この package は final public signature schema ではない。
- current helper / sample / report を横断する evidence-oriented carrier と debug surface だけを置く。
- `LayerSignature system`、final public auth / visualization / projection / hot-plug API、
  real network transport、packaging は引き続き deferred とする。

## 4. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/05-mirrorea-fabric.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `samples/clean-near-end/README.md`
- `samples/clean-near-end/sugoroku-world/README.md`
- `docs/research_abstract/hands_on_sugoroku_sample_matrix.md`
- `docs/research_abstract/hands_on_sugoroku_detail.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/reports/0916-sugoroku-sample-progress-alignment.md`
- `docs/reports/0917-avatar-fairy-follow-sample-plan.md`

## 5. Actions taken

- AGENTS 順に current docs / roadmap / sample docs / plan を再読し、current promoted package が
  `TermSignature registry / debug output` であることを再確認した。
- RED を再現した。
  `python3 -m unittest scripts.tests.test_sugoroku_world_samples`
  と
  `cargo test -p mir-runtime --test clean_near_end_samples`
  で、Sugoroku payload の `term_signatures` 欠落、`--debug signatures` 欠落、
  Rust report / closeout field 欠落を確認した。
- `scripts/sugoroku_world_samples.py` に helper-local `TermSignature` carrier、
  sample source + runtime result 由来の signature inventory、
  `--debug signatures`、closeout の `signature_kinds` / `reserved_signature_kinds` を追加した。
- `crates/mir-runtime/src/clean_near_end.rs` に report-local `TermSignature` struct、
  `CleanNearEndSampleReport.term_signatures`、
  `CleanNearEndCloseout.signature_kinds` /
  `reserved_signature_kinds` を追加し、
  sample source と existing spec metadata から minimal signature inventory を導出する helper を追加した。
- snapshot / repository memory を同期した。
  `progress.md`、`tasks.md`、`samples_progress.md`、`plan/01-status-at-a-glance.md`、
  `plan/09-helper-stack-and-responsibility-map.md`、`plan/11-roadmap-near-term.md`
  を更新し、`TermSignature` package close と next promoted package `LayerSignature system`
  を反映した。
- reader-facing docs / specs を同期した。
  `README.md`、`Documentation.md`、`samples/clean-near-end/sugoroku-world/README.md`、
  `docs/research_abstract/hands_on_sugoroku_sample_matrix.md`、
  `docs/research_abstract/hands_on_sugoroku_detail.md`、
  `docs/research_abstract/mirrorea_future_axis_01.md`、
  `specs/10-open-questions.md`、`specs/11-roadmap-and-workstreams.md`、
  `plan/12-open-problems-and-risks.md`、`plan/17-research-phases-and-autonomy-gates.md`
  に `TermSignature` first cut と next package line を反映した。
- reviewer の medium finding を反映した。
  Sugoroku helper は `(kind, name)` ではなく `(kind, name, evidence_role)` 単位で provenance を保持するようにし、
  sample source declaration と runtime-exercised transition を両方残すようにした。
  clean near-end closeout は sample source read failure を silent に無視せず、
  `CleanNearEndError` として返すようにした。
  `docs/research_abstract/hands_on_sugoroku_detail.md` の `--debug signatures` snippet も actual formatter に合わせて修正した。
- Files changed:
  `scripts/sugoroku_world_samples.py`,
  `crates/mir-runtime/src/clean_near_end.rs`,
  `scripts/tests/test_sugoroku_world_samples.py`,
  `crates/mir-runtime/tests/clean_near_end_samples.rs`,
  `README.md`,
  `Documentation.md`,
  `progress.md`,
  `tasks.md`,
  `samples_progress.md`,
  `plan/01-status-at-a-glance.md`,
  `plan/09-helper-stack-and-responsibility-map.md`,
  `plan/11-roadmap-near-term.md`,
  `plan/12-open-problems-and-risks.md`,
  `plan/17-research-phases-and-autonomy-gates.md`,
  `samples/clean-near-end/sugoroku-world/README.md`,
  `docs/research_abstract/hands_on_sugoroku_sample_matrix.md`,
  `docs/research_abstract/hands_on_sugoroku_detail.md`,
  `docs/research_abstract/mirrorea_future_axis_01.md`,
  `specs/10-open-questions.md`,
  `specs/11-roadmap-and-workstreams.md`,
  `docs/reports/0918-term-signature-registry-debug-output.md`
- Commands run:
  `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`;
  `git status --short`;
  `python3 -m unittest scripts.tests.test_sugoroku_world_samples`;
  `cargo test -p mir-runtime --test clean_near_end_samples`;
  `date '+%Y-%m-%d %H:%M %Z'`;
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug signatures`;
  `python3 scripts/current_l2_guided_samples.py smoke-all --format json`;
  `cargo test -p mir-runtime`;
  `python3 scripts/check_source_hierarchy.py`;
  `python3 scripts/validate_docs.py`;
  `git diff --check`;
  `df -h .`;
  `free -h`

## 6. Evidence / outputs / test results

- resource audit:

```text
$ df -h .
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        99G   63G   32G  67% /

$ free -h
               total        used        free      shared  buff/cache   available
Mem:           960Mi       620Mi        79Mi       104Ki       413Mi       339Mi
Swap:           19Gi       1.0Gi        18Gi
```

- RED evidence:

```text
$ python3 -m unittest scripts.tests.test_sugoroku_world_samples
...F....E..F.
KeyError: 'term_signatures'
AssertionError: '--debug signatures' not found in ['--debug summary', '--debug turn-trace', '--debug membership', '--debug verification', '--format json']
AssertionError: 'TERM SIGNATURES' not found in pretty output
```

```text
$ cargo test -p mir-runtime --test clean_near_end_samples
error[E0609]: no field `term_signatures` on type `CleanNearEndSampleReport`
error[E0609]: no field `signature_kinds` on type `CleanNearEndCloseout`
error[E0609]: no field `reserved_signature_kinds` on type `CleanNearEndCloseout`
```

- `--debug signatures` preview after implementation:

```text
$ python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug signatures
TERM SIGNATURES
  - effect: roll_dice [source_decl]
  - transition: take_turn_alice [source_decl]
  - transition: take_turn_alice [sample_transition]
  - witness: draw_pub [source_decl]
  - witness: draw_pub#1 [runtime_witness]
  - relation: publication_order [derived_relation]
  ...
```

- verification summary:
  Python unittest は 13/13 pass。
  `cargo test -p mir-runtime --test clean_near_end_samples` は 14/14 pass。
  `python3 scripts/current_l2_guided_samples.py smoke-all --format json` は pass し、
  clean near-end JSON に `term_signatures`、closeout に `signature_kinds` を出した。
  `cargo test -p mir-runtime` は pass した。
  `python3 scripts/check_source_hierarchy.py` は required `23` / missing `0` で pass した。
  `python3 scripts/validate_docs.py` は
  `Documentation scaffold looks complete.`
  と
  `Found 916 numbered report(s).`
  を返した。
  `git diff --check` は clean。
  reviewer sub-agent は、Sugoroku provenance 欠落、Rust closeout の silent read fallback、
  docs snippet drift を指摘し、修正後に上記 validation を取り直した。

## 7. What changed in understanding

- `TermSignature` の first cut は「final schema」ではなく、existing sample metadata、
  witness field、relation family、effect-row hint、proof obligation を束ねる
  helper-local / report-local inventory として置くのが正しい。
- `current_l2_guided_samples.py` は semantic source ではなく compatibility forwarder なので、
  clean near-end 側は Rust runtime-owned carrier に寄せる方が自然である。
- Sugoroku 側は richer な place/runtime/message-ish helper data を持つが、それを core semantics と
  既成事実化せず、helper runtime extension bucket として扱う必要がある。
- reserved kind は live support を偽装せず、
  closeout の `reserved_signature_kinds` にだけ `message` / `adapter` / `layer`
  を置くのが current stop line に合う。

## 8. Open questions

- UNRESOLVED: `TermSignature` の exact granularity を将来 `LayerSignature` や `MessageEnvelope`
  とどこまで共有するか。
- UNRESOLVED: `message` / `adapter` reserved kind を runtime helper inventory に actualize する時点で、
  auth / transport / projection のどの carrier から first-class に起こすか。
- UNRESOLVED: phase 11 umbrella progress を `25%` より高く読む根拠はまだ弱く、
  `LayerSignature` 未着手のまま過大評価しない方がよい。

## 9. Suggested next prompt

`LayerSignature system` を進めてください。`TermSignature` first cut を public schema に凍らせず、
auth / verification / visualization / telemetry / transport を typed layer として合成する
metadata と law surface を docs-first / repo-local に actualize し、`progress.md` / `tasks.md` /
`samples_progress.md` / new report を同期してください。
