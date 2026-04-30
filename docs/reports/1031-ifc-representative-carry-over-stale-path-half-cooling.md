# 1031. ifc representative carry-over stale-path half cooling

## Objective

`specs/examples/522/523/524/529` と mirror docs に残っていた `samples/lean/current-l2/` / `samples/prototype/current-l2-typed-proof-model-check/` stale wording を冷やし、current front door を clean-near-end typing `01/02/03` と active Lean docs、historical compare anchor を archived `p10/p11/p12` prototype / old Lean corpus に揃える。

## Scope and assumptions

- docs-first maintenance package として扱い、final typed source principal、final IFC syntax、final public checker artifact、final public verifier contract は claim しない。
- current sample-side compare floor は `samples/clean-near-end/typing/01_authorized_declassification.mir`、`02_unauthorized_declassification_rejected.mir`、`03_label_flow_rejected.mir` に置く。
- `p10/p11/p12` は archived compare anchors として保持し、active entrypoint へ戻さない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `AGENTS.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/522-current-l2-ifc-secret-valid-invalid-foundation-and-japanese-lean-corpus-sync.md`
- `specs/examples/523-current-l2-source-side-ifc-authority-prototype-pair-and-representative-lean-sample-set-widening.md`
- `specs/examples/524-current-l2-ifc-label-flow-negative-prototype-closeout-and-representative-lean-sample-set-widening.md`
- `specs/examples/529-current-l2-ifc-typed-checker-hint-preview-actualization.md`
- `samples/lean/README.md`
- `scripts/current_l2_lean_sample_sync.py`

## Actions taken

1. `specs/examples/522` の Lean corpus explanation を `foundations/` / active `clean-near-end/` / archived old corpus split に更新した。
2. `specs/examples/523` を historical `p10/p11` authority compare-anchor memory と current clean-near-end typing `01/02` compare floor の split に書き換えた。
3. `specs/examples/524` を historical `p12` label-flow compare-anchor memory と current clean-near-end typing `03` compare floor の split に書き換えた。
4. `specs/examples/529` の helper preview actualization を active clean-near-end typing trio `01/02/03` front door に寄せ、archived `p10/p11/p12` は compare anchor に留めた。
5. `specs/12 D-111/D-112/D-117`、`specs/11`、`specs/00` を同じ archived/current split に mirror した。
6. `progress.md` と `tasks.md` を更新し、remaining stale-docs line を order-handoff representative carry-over cluster `specs/examples/525/527/528` に狭めた。
7. reviewer 指摘を受けて `529` の intro wording を active trio 基準へ揃え、`523/524/529` に archived Lean appendix evidence を戻し、`tasks.md` timestamp と report の `rg` scope wording を補正した。

## Files changed

- `specs/examples/522-current-l2-ifc-secret-valid-invalid-foundation-and-japanese-lean-corpus-sync.md`
- `specs/examples/523-current-l2-source-side-ifc-authority-prototype-pair-and-representative-lean-sample-set-widening.md`
- `specs/examples/524-current-l2-ifc-label-flow-negative-prototype-closeout-and-representative-lean-sample-set-widening.md`
- `specs/examples/529-current-l2-ifc-typed-checker-hint-preview-actualization.md`
- `specs/12-decision-register.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/00-document-map.md`
- `progress.md`
- `tasks.md`

## Evidence / outputs / test results

- docs floor:
  - `python3 scripts/check_source_hierarchy.py`
  - `python3 scripts/validate_docs.py`
- typing compare floor:
  - `python3 scripts/clean_near_end_samples.py run typing --format json`
  - pass。`01_authorized_declassification` は `valid` / `success`、`02_unauthorized_declassification_rejected` と `03_label_flow_rejected` は expected malformed/static-stop families で返った。
- Lean sync:
  - `python3 scripts/current_l2_lean_sample_sync.py`
  - pass。`samples/lean/manifest.json` を出力し、working tree に追加 diff を残さなかった。
- targeted stale-path recheck:
  - `rg -n "samples/lean/current-l2|samples/prototype/current-l2-typed-proof-model-check|samples/prototype/current-l2-order-handoff" specs/examples/522-current-l2-ifc-secret-valid-invalid-foundation-and-japanese-lean-corpus-sync.md specs/examples/523-current-l2-source-side-ifc-authority-prototype-pair-and-representative-lean-sample-set-widening.md specs/examples/524-current-l2-ifc-label-flow-negative-prototype-closeout-and-representative-lean-sample-set-widening.md specs/examples/529-current-l2-ifc-typed-checker-hint-preview-actualization.md specs/12-decision-register.md specs/11-roadmap-and-workstreams.md specs/00-document-map.md`
  - exit `1`。example/spec mirror set で stale token zero match を確認した。
- formatting / whitespace:
  - `git diff --check`

## What changed in understanding

- IFC helper/carry-over cluster は一括 historical 化ではなく、`522` と `529` に current front door が残り、`523/524` に archived compare-anchor memory が残る二層 split として扱うほうが正確だった。
- active clean-near-end typing trio `01/02/03` が current sample-side authority / label-flow compare floor を担っており、`p10/p11/p12` は archived pre-clean-near-end compare anchor に留まる。
- `samples/lean/README.md` と `current_l2_lean_sample_sync.py` の current behavior から見ても、`samples/lean/current-l2/` wording は current front door として残せなかった。

## Open questions

- order-handoff representative carry-over cluster `specs/examples/525/527/528` を、どこまで active clean-near-end `order-handoff/02/03/05` compare floor と archived `p09/p13/p14` anchor split に寄せるか。
- `specs/examples/526` は path drift 自体は無いが、`525/527/528` を冷やした後に summary wording の parity 調整が必要になるか。

## Suggested next prompt

`specs/examples/525/527/528` を対象に、delegated RNG carry-over / late-join negative pair docs に残る `samples/prototype/current-l2-order-handoff` と `samples/lean/current-l2/` stale wording を、active clean-near-end `order-handoff/05/02/03` compare floor + archived `p09/p13/p14` anchor split に揃え、必要な mirror docs と progress/tasks を同期してほしい。

## plan/ update status

- `plan/ 更新不要`

## progress.md update status

- 更新した。

## tasks.md update status

- 更新した。

## samples_progress.md update status

- 更新不要。active runnable sample dashboard は変えていない。

## Skipped validations and reasons

- full validation floor は未実行。今回の変更は docs-first stale-path cooling と active/archived path authority repair に限定し、implementation や sample taxonomy 自体は変更していないため、typing floor + Lean sync + docs floor に留めた。

## Commit / push status

- report authoring 時点では pending。
- package close commit / push はこの report 生成後に実施する。

## Sub-agent session close status

- reviewer session (`019dde7b-858d-7032-b534-4bff8bedcd12`): completed。4 件の wording / evidence-scope findings を回収したうえで close 済み。
