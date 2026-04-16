# 413 — current L2 typed-core attachment inventory and obligation allocation refresh

## 目的

current repo の typed-core line について、

- first typed attachment candidate
- obligation owner matrix
- stop line

を docs-first に整理する。

ここで固定するのは **full type calculus** ではなく、
current source-backed floor から見て何を first typed attachment と呼べるかの inventory である。

## source-backed floor

- current repo は、checker-side cluster row と 4-way split を already 持つ。
- typed reason family hint は row core ではなく optional attachment として比較済みである。
- theorem/model-check side には `formal_hook -> proof_notebook_review_unit / model_check_concrete_carrier` の derived artifact line がある。
- current Track A は first attachment を semantic carrier / checker boundary から選び、source-visible typed syntax を immediate target にしない。

## attachment candidate ladder

| candidate | reading | strengths | current risk |
|---|---|---|---|
| source-visible clause attachment | `require` / `ensure` / `admit` / `lease` / lineage などの source-side marker を typed surface の first cut にする | user-facing には見えやすい | final parser grammar と final typed syntax を先取りしやすい |
| checker-adjacent semantic carrier | checker cluster row / family hint / source anchor を typed attachment の first cut にする | current source-backed floor と連続的、syntax open を保てる | user-facing syntax にはまだ見えにくい |
| verifier handoff row | theorem / model-check handoff row を typed attachment の principal source にする | proof / protocol line と繋がる | checker floor と source anchor から遠くなりやすい |
| consumer-side review artifact | `proof_notebook_review_unit` や model-check carrier を typed source と見なす | actual downstream artifact は already ある | derived consumer shape を source of truth と誤読しやすい |

## obligation owner matrix

| obligation family | first owner | current reason |
|---|---|---|
| declared target / lineage / capability structural floor | `core_static_checker` | local / structural / decidable floor として already 読める |
| checker cluster row への family hint attachment | `core_static_checker` | checker cluster inventory の近傍で additive に扱える |
| canonical normalization / no re-promotion / rollback-cut non-interference | `theorem_prover_boundary` | global semantic law と proof obligation の line に自然に乗る |
| publication / witness / replay / provider receipt / room seriality | `protocol_verifier_boundary` | room protocol / handoff family と強く結びつく |
| activation / rejoin / retry / host-policy drift | `runtime_policy_boundary` | runtime control-plane / operational policy 側の責務が大きい |

## current judgment

1. current first typed attachment candidate は **checker-adjacent semantic carrier** に置く。
2. source-visible typed surface は next comparison に残し、current package では fixed しない。
3. theorem/model-check consumer artifact は **derived attachment** として扱い、principal typed source にしない。
4. typed work の current package は、obligation owner matrix と attachment ladder を揃えたところで閉じる。

## current first choice details

- first attachment cut は
  - source anchor
  - checker cluster row
  - optional family hint
  - downstream handoff ref
  を分けて読める inventory に留める。
- current package では、`family_refs[]` の exact namespace や public schema naming は固定しない。
- capability singleton、lineage pair、request predicate attachment を同一 shape に統一するかは still OPEN に残す。

## what is not decided here

- full strong type system
- inference / annotation design
- final typed syntax
- public typed API
- `family_refs[]` の final namespace
- request / predicate / `try` cluster まで同じ attachment shape を流用するか
