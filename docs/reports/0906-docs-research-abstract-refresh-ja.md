# Report 0906 — docs research abstract refresh ja

- Date: 2026-04-23T08:57:00+09:00
- Author / agent: Codex
- Scope: active docs の日本語全面刷新、`docs/research_abstract/` の再構成、detail 文書への actual sample code / output / built-in boundary の反映
- Decision levels touched: 規範意味論の変更なし。L2 current snapshot / explanation / traceability を更新。`specs/00-document-map.md` は文書導線の更新のみ。

## 1. Objective

- `README.md`、`Documentation.md`、`progress.md`、`tasks.md`、`docs/research_abstract/` を、2026-04-23 時点の current active line に合わせて日本語で新規書き直しする。
- old information が active explanation に残り続けないように、旧 active research_abstract summary を current root から外し、clean near-end family 中心の summary / `_detail` 構成へ再編する。
- `_detail` 文書には、実際に実行可能な sample code 全文、共有前提、actual JSON / Lean verification output、built-in / user-defined boundary を掲載する。

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
- `specs/12-decision-register.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `samples/clean-near-end/00_index_theories.mir`
- `samples/clean-near-end/typing/*.mir`
- `samples/clean-near-end/order-handoff/*.mir`
- `samples/clean-near-end/model-check/*.mir`
- `samples/clean-near-end/modal/*.mir`
- `samples/lean/foundations/*.lean`
- `samples/lean/foundations/*.md`
- `samples/lean/manifest.json`
- representative generated stub in `samples/lean/clean-near-end/`
- `docs/reports/0904-clean-near-end-alpha-release-completion.md`
- `faq_007.md`
- `faq_010.md`
- `faq_011.md`

補足:

- `plan/` は参照のみで、今回の request では更新不要と判断した。

## 3. Actions taken

- top-level current snapshot documents を全面的に書き直した。
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
- `docs/research_abstract/` を current active line に合わせて再構成した。
  - phase 0..6 summary を current reading に合わせて書き直した
  - clean near-end typing / order-model / modal / Lean summary を新規作成した
  - corresponding `_detail.md` を新規作成した
- `_detail` 文書では次を掲載した。
  - shared index theory
  - actual sample code 全文
  - actual JSON output
  - Lean foundation code 全文
  - representative generated stub
  - `samples/lean/manifest.json` の verification result
- 旧 active research_abstract summary を current root から外した。
  - `static_analysis_01.md`
  - `static_analysis_01_detail.md`
  - `order_01.md`
  - `order_01_detail.md`
  - `lean_01.md`
  - `lean_01_detail.md`
- `specs/00-document-map.md` を更新し、new summary / detail structure を document map に反映した。
- active docs に対して stale-reference scan を行い、旧 predicate 名や旧 sample path が active explanation に残っていないことを確認した。

## 4. Files changed

- Updated:
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `specs/00-document-map.md`
  - `samples/clean-near-end/README.md`
  - `samples/lean/README.md`
  - `docs/research_abstract/README.md`
  - `docs/research_abstract/phase0-repository-memory-and-decision-boundary.md`
  - `docs/research_abstract/phase1-current-l2-semantics-stabilization.md`
  - `docs/research_abstract/phase2-parser-free-poc-and-detached-validation-loop.md`
  - `docs/research_abstract/phase3-parser-boundary-and-first-checker-cut.md`
  - `docs/research_abstract/phase4-shared-space-membership-and-practical-room-boundary.md`
  - `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
  - `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- Added:
  - `docs/research_abstract/clean_near_end_typing_01.md`
  - `docs/research_abstract/clean_near_end_typing_01_detail.md`
  - `docs/research_abstract/clean_near_end_order_model_01.md`
  - `docs/research_abstract/clean_near_end_order_model_01_detail.md`
  - `docs/research_abstract/clean_near_end_modal_01.md`
  - `docs/research_abstract/clean_near_end_modal_01_detail.md`
  - `docs/research_abstract/clean_near_end_lean_01.md`
  - `docs/research_abstract/clean_near_end_lean_01_detail.md`
  - `docs/reports/0906-docs-research-abstract-refresh-ja.md`
- Removed from current active `docs/research_abstract/` root:
  - `docs/research_abstract/static_analysis_01.md`
  - `docs/research_abstract/static_analysis_01_detail.md`
  - `docs/research_abstract/order_01.md`
  - `docs/research_abstract/order_01_detail.md`
  - `docs/research_abstract/lean_01.md`
  - `docs/research_abstract/lean_01_detail.md`

## 5. Commands run and exact outputs

### 1. resource check

```bash
df -h .
free -h
```

Exact outputs:

- `df -h .`
  - `/dev/vda2 99G size, 85G used, 9.3G avail, 91%`
- `free -h`
  - `Mem: 960Mi total, 642Mi used, 66Mi free, 414Mi buff/cache, 317Mi available`
  - `Swap: 19Gi total, 1.1Gi used, 18Gi free`

### 2. clean suite family outputs

```bash
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-family typing --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-family order-handoff --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-family model-check --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-family modal --format json
```

Exact result summary:

- typing:
  - `01_authorized_declassification` = `valid`, `success`
  - `02_unauthorized_declassification_rejected` = `malformed`, `authority_preorder_constraint_failed`
  - `03_label_flow_rejected` = `malformed`, `label_flow_constraint_failed`
  - `04_capture_escape_rejected` = `malformed`, `capture_escape`
  - `05_cost_bound_rejected` = `malformed`, `cost_bound_exceeded`
- order-handoff:
  - `01_authorized_roll_publish_handoff` = `valid`, `success`
  - `02_missing_witness_rejected` = `malformed`, `missing_handoff_witness`
  - `03_handoff_before_publication_rejected` = `malformed`, `handoff_before_publication`
  - `04_stage_block_authorized_handoff` = `valid`, `success`
  - `05_delegated_rng_service` = `valid`, `success`
  - `06_auditable_authority_witness` = `valid`, `success`
- model-check:
  - `01_peterson_sc_pass` = `pass`
  - `02_peterson_relaxed_counterexample` = `counterexample`
  - `03_broken_mutex_counterexample` = `counterexample`
- modal:
  - `01_stage_stable_later_minimal` = `valid`
  - `02_published_witnessed_mode_bridge` = `valid`

### 3. Lean sync

```bash
python3 scripts/current_l2_lean_sample_sync.py
```

Exact output:

```text
/home/yukatayu/dev/mir_poc_01/samples/lean/manifest.json
```

Manifest exact result summary:

- foundations 4 files all `success: true`
- generated clean-near-end stubs 16 samples all `success: true`
- Lean version:
  `Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)`

### 4. suite-level smoke / closeout

```bash
python3 scripts/current_l2_guided_samples.py smoke-all --format json
python3 scripts/current_l2_guided_samples.py closeout --format json
```

Exact result summary:

- `smoke-all`:
  - `total_samples = 16`
  - valid samples = 7
  - malformed samples = 6
  - model-check pass = 1
  - model-check counterexample = 2
- `closeout`:
  - active sample root = `samples/clean-near-end`
  - archive sample root = `samples/old/2026-04-22-pre-clean-near-end`
  - built-in vocabulary = 24 terms
  - proof sample count = 16

### 5. docs validation

```bash
python3 scripts/validate_docs.py
```

Exact output:

```text
Documentation scaffold looks complete.
Found 904 numbered report(s).
```

### 6. stale-reference scan on active docs

```bash
rg -n 'declassify_authority\(|observer_role\(|fingerprint_bound\(|fingerprint_visible\(' README.md Documentation.md progress.md tasks.md docs/research_abstract/*.md samples/clean-near-end/README.md samples/lean/README.md
rg -n 'samples/prototype|samples/not_implemented|p07-dice|p08-dice|p10-typed|p11-typed|p12-typed|p13-dice|p14-dice' README.md Documentation.md progress.md tasks.md docs/research_abstract/*.md samples/clean-near-end/README.md samples/lean/README.md
```

Exact outputs:

- both commands exited with code `1`
- no match in active docs

## 6. Evidence / findings

- active docs now describe the current active line as `samples/clean-near-end/` centered, not the old `p..` / prototype corpus.
- `docs/research_abstract/` now has a clear split between summary and `_detail`.
- `_detail` files include:
  - actual clean sample code
  - actual JSON outputs for valid and intentionally rejected / counterexample cases
  - explicit built-in / user-defined boundary
- Lean detail now distinguishes:
  - small actual proof fragment in `samples/lean/foundations/`
  - generated theorem stub in `samples/lean/clean-near-end/`
- no active doc file references old predicate placeholders or old prototype sample IDs anymore.

## 7. Changes in understanding

- current repo state is strong enough that documentation can now be organized around the clean near-end family itself, instead of retrospective package chains.
- the most important doc boundary is no longer “old sample explanation vs new sample explanation”, but rather:
  - current active summary
  - evidence-bearing detail
  - archive / historical comparison
- modal family needed its own summary / detail. Without it, active suite coverage in `research_abstract` was incomplete even though the code and validation already existed.

## 8. Open questions

- Should `docs/research_abstract/old/` itself be further compressed into a single archive index, or is the current archive granularity still useful?
- When public parser / checker surface is revisited, should `research_abstract` gain a dedicated parser/API current-state summary file separate from phase summaries?
- For Lean, should future `_detail` continue embedding the full manifest, or move to a generated appendix once the generated stub corpus grows further?

## 9. Suggested next prompt

- `public-seam residual を詰めたいので、parser / checker / verifier API に関する current mixed gate を docs と code anchor 付きで整理してください。`
- `memory_order 再解釈 line の public wording residual を、current high-level relation family を保ったまま docs-first で詰めてください。`
- `Lean foundations の次の hardening package として、generated stub ではなく actual reusable lemma をどこに追加すべきか提案し、必要なら実装してください。`
