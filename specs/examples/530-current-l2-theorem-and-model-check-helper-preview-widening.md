# 530 — current L2 theorem / model-check helper preview widening

## 目的

`specs/examples/491`、
`492`、
`500`、
`501`、
`529`
と
`e5-underdeclared-lineage`、
`p06-typed-proof-owner-handoff`、
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`、
`p09-dice-delegated-rng-provider-placement`
を前提に、

- theorem result-object preview
- model-check public-checker artifact preview
- helper-local operational CLI diagnostics
- representative reached / guard split
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**`run-source-sample` helper summary に theorem-side result-object preview と model-check side public-checker preview を narrow diagnostics として actualize する current cut**
であり、

- final public theorem result object
- consumer-shaped theorem payload public contract
- concrete theorem prover brand
- proof object public schema
- first settled property language
- concrete model-check tool brand
- final public checker artifact
- actual public checker migration
- actual emitted verifier handoff artifact
- final public verifier contract

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. theorem result-object preview and proof-object-schema reserve actualization
   - notebook-consumer object first
   - consumer-shaped payload preview only
   - proof-object-schema reserve keep
2. theorem result-object route actual adoption
   - review-unit transport first
   - notebook-consumer object first
   - payload preview keep
3. model-check public-checker artifact preview and verifier-handoff reserve actualization
   - consumer-shaped artifact preview only
   - verifier-handoff reserve keep
   - brand-neutral tool-binding reserve keep
4. model-check checker-artifact route actual adoption
   - row-local property route first
   - checker-boundary contract anchor
   - consumer-shaped checker-artifact candidate only
5. IFC checker-hint preview narrow actualization
   - current `run-source-sample` helper summary widening precedent

したがって current open problem は、
theorem/model-check mixed gate をいきなり final public contract に送ることではなく、
**already source-backed な helper-local actualization manifest を representative sample 上で operational CLI に見せること**
である。

## current actualization cut

current package では、次を採る。

1. theorem result-object preview は live subject `e5`、clean-near-end `05_delegated_rng_service` runtime-adjacent compare floor、historical `p05 / p06 / p07 / p08` compare anchors の split に置く。
2. model-check public-checker preview は current clean-near-end model-check family
   `01_peterson_sc_pass / 02_peterson_relaxed_counterexample / 03_broken_mutex_counterexample`
   runtime floor、historical `e5 / p06 / p07 / p09` compare anchors、`p05` guard-only reading の split に置く。
3. `p08` は theorem preview reached / model-check preview guarded の historical mixed-helper asymmetry anchor に留め、theorem/model-check live compare floor へ戻さない。
4. `p09` は theorem preview guarded / model-check preview reached の historical mixed-helper asymmetry anchor に留め、theorem side current live floor と collapse しない。
5. field は helper-local actualization manifest を mirror し、
   repo-local emitted artifact refs first / reserve keep / kept-later を collapse しない。

## helper preview shape

theorem result-object preview は少なくとも次を持つ。

- `status`
- `preview_kind`
- `subject_kind` / `subject_ref`
- `result_object_route_refs`
- `notebook_payload_preview_refs`
- `proof_object_schema_reserve_refs`
- `actual_adoption_default_refs`
- `evidence_refs`
- `compare_floor_refs`
- `guard_refs`
- `kept_later_refs`
- `guard_reason`

model-check public-checker preview は少なくとも次を持つ。

- `status`
- `preview_kind`
- `subject_kind` / `subject_ref`
- `checker_artifact_preview_refs`
- `verifier_handoff_reserve_refs`
- `tool_binding_reserve_refs`
- `actual_adoption_default_refs`
- `evidence_refs`
- `compare_floor_refs`
- `guard_refs`
- `kept_later_refs`
- `guard_reason`

## current evidence

| evidence | current reading |
|---|---|
| `cargo test -q -p mir-runtime --test current_l2_operational_cli` | CLI summary が theorem/model-check helper preview widening 後も green であることを確認する |
| `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/clean-near-end/order-handoff/05_delegated_rng_service.mir --format json` | theorem-side helper preview mirror が live theorem compare floor と切り離されていないことを adjacent runtime evidence として再確認する |
| `python3 scripts/clean_near_end_samples.py run model-check --format json` | model-check-side helper preview mirror が current clean-near-end model-check family の上に残っていることを再確認する |
| `docs/reports/0811-package58-theorem-model-check-helper-preview-widening.md` | `p08 / p09` の historical mixed-helper asymmetry anchor は Package 58 close 時の prototype-side evidence として残す。current accepted sample set では再実行対象に戻さない |

## current recommendation

1. representative Lean sample set actual Lean execution が reached した後の next widening は、new corpus 追加よりも helper-local diagnostics widening を優先してよい。
2. theorem line は result-object preview actualization を final public theorem result object に上げず、helper-local actualization manifest として CLI に mirror するのが自然である。
3. model-check line は public-checker preview actualization を final public checker artifact に上げず、helper-local actualization manifest として CLI に mirror するのが自然である。
4. `p08` と `p09` の reached / guard 非対称を visible にすることで、この widening が theorem/model-check live compare floorそのものではなく historical mixed-helper asymmetry anchor であることを誤読しにくくできる。

## retained alternatives

- theorem result-object preview を final public theorem result object first adoption へ進める
- model-check public-checker preview を final public checker artifact first adoption へ進める
- theorem/model-check helper preview を unified public verifier payload に collapse する
- CLI widening を行わず docs-only helper actualization のまま止める

## stop line

current package は次で止める。

- final public theorem result object
- consumer-shaped theorem payload public contract
- concrete theorem prover brand
- proof object public schema
- first settled property language
- concrete model-check tool brand
- final public checker artifact
- actual public checker migration
- actual emitted verifier handoff artifact
- production checker/runtime-policy contract
- final public verifier contract

## next line

Package 58 の helper / CLI hardening and broader coverage では、

- theorem result-object preview helper mirror
- model-check public-checker preview helper mirror

まで actualize したという historical queue anchor が残る。
current queue authority は `progress.md` / `tasks.md` に置き、`Package 59` near-end closeout sync と later mixed/user-spec residual compression は repository-memory 上の historical reopen reading としてのみ扱う。
