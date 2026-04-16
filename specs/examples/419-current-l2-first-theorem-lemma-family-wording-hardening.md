# 419 — current L2 first theorem lemma family wording hardening

## 目的

`specs/examples/414-current-l2-semantic-core-theorem-pilot-planning.md`
で fixed した theorem pilot order を前提に、

- lemma statement wording
- admissible evidence floor
- review artifact と theorem discharge の stop line

を docs-first に harden する。

## source-backed floor

- first lemma order は
  1. `canonical_normalization_law`
  2. `no_re_promotion`
  3. `rollback_cut_non_interference`
  である。
- subject kind は `fixture_static_cluster` / `runtime_try_cut_cluster` に留める。
- tool-neutral formal hook と row-local `proof_notebook_review_unit` は actual source-backed anchor を already 持つ。

## lemma wording floor

| lemma family | current wording floor | admissible evidence floor |
|---|---|---|
| `canonical_normalization_law` | checker floor で admitted な same-lineage chain は canonical flattening 後も candidate order / guard / target-capability consistency を変えない | fixture ref、static gate artifact ref、relevant contract row refs |
| `no_re_promotion` | same-lineage degradation が始まった evaluation は earlier option へ hidden re-promotion しない | fixture ref、static cluster subject ref、relevant contract row refs |
| `rollback_cut_non_interference` | current `place` の local rollback は prior `atomic_cut` 以前の committed prefix を巻き戻さない | fixture ref、runtime cluster ref、relevant contract row refs |

## review vs discharge wording

| layer | current reading | must not be called |
|---|---|---|
| formal hook | tool-neutral obligation row bundle | proof object |
| proof notebook review unit | row-local human-review artifact | theorem discharge result |
| theorem discharge | still later concrete proof-side result | current pilot |

## current judgment

1. theorem pilot wording は **semantic-core law の statement floor** に留める。
2. admissible evidence floor は fixture ref / subject ref / contract row refs を first class にし、narrative prose や review checklist を theorem evidence へ混ぜない。
3. `proof_notebook_review_unit` は review artifact に留め、proof object や discharge receipt と呼ばない。

## current first choice details

- statement wording は semantic law を直接述べ、implementation detail や concrete prover tactic を含めない。
- evidence floor は symbolic ref family を維持し、concrete serialized artifact schema は要求しない。
- review checklist / `goal_text` / walkthrough note は theorem discharge の代用品として読まない。

## next promoted line

next promoted line は、
**proof artifact / bridge stop-line refresh**
に置く。

## what is not decided here

- concrete theorem prover brand
- proof object public contract
- actual theorem discharge transport
- public checker migration timing
