# 439 — current L2 typed-surface family unification keep/drop note

## 目的

`specs/examples/418-current-l2-first-source-visible-typed-surface-comparison.md`
と
`specs/examples/433-current-l2-request-predicate-try-cluster-typed-surface-reserve-note.md`
で fixed した

- checker-adjacent semantic carrier principal
- source-visible structural marker family first
- request / predicate / `try` cluster grouped reserve cue

を前提に、
current L2 で **どの family までを shared reading として寄せ、どこからを keep/drop で分けるか**
を docs-first に整理する。

ここで fixed するのは family split の current cut であり、
shared attachment shape、full type calculus、final typed syntax は fixed しない。

## source-backed floor

- principal typed source は checker-adjacent semantic carrier にある。
- first source-visible candidate は
  `capability` / `lease` / `admit` / lineage annotation / `require` / `ensure`
  をまとめた explicit structural marker family である。
- request-side clause cluster、predicate fragment cluster、`try` / rollback structural cluster は grouped reserve cue に留める。
- theorem/model-check artifact、handoff row、unified obligation block は principal typed source ではない。

## family comparison

| candidate | reading | strengths | current risk |
|---|---|---|---|
| fully separate families | structural marker family と reserve cluster family を完全に別 reading に保つ | safest | shared reading が弱すぎて current next question が進まない |
| structural marker only unified | `capability` / `lease` / `admit` / lineage / `require` / `ensure` だけを 1 family として読む | source-backed で軽い | request / predicate / `try` reserve 側との境界が曖昧なまま残る |
| two-tier split | principal checker attachment の下に、source-visible structural marker family と reserve cluster family を 2 層で置く | current repo memory と最も整合的 | 共有 attachment shape と誤読される余地がある |
| single shared attachment schema | structural marker と request / predicate / `try` reserve を 1 schema に寄せる | long-run unification の見通しはある | final grammar / namespace / attachment shape を premature に拘束する |
| downstream artifact promoted family | theorem/model-check artifact や handoff row を typed family に繰り上げる | cross-boundary integration は見やすい | principal-vs-derived split を壊す |

## current judgment

current L2 で最も自然なのは、
**two-tier split**
を current first choice に置くことである。

理由は次の通り。

1. checker attachment principal は already fixed しており、ここを shared source family に dissolve する理由がない。
2. explicit structural marker family は source-visible candidate として source-backed である。
3. request / predicate / `try` cluster は useful reserve cue だが、shared attachment shape に上げるには早い。
4. handoff row、theorem/model-check artifact、unified obligation block を typed source of truth に繰り上げると boundary split が崩れる。

## current family split

```text
typed_surface_family_split = {
  principal_checker_attachment_refs,
  source_visible_structural_marker_family_refs[],
  reserve_cluster_family_refs[],
  derived_handoff_family_refs[],
  guard_refs[],
  kept_later_refs[]
}
```

### keep

- principal typed source は checker attachment に残す
- source-visible structural marker family は grouped family reading にしてよい
- request / predicate / `try` cluster は reserve-side family に留める
- derived handoff family は checker attachment からの downstream reserve に留める

### drop from current package

- structural marker family と reserve cluster family を 1 attachment schema に統合すること
- request / predicate / `try` cluster を principal typed source にすること
- theorem/model-check artifact と handoff row を typed source of truth に繰り上げること
- unified obligation block を first source-visible family にすること

## next promoted line

next promoted line は、
**notebook-consumer threshold and discharge reserve note**
に置く。

## what is not decided here

- `family_refs[]` の exact namespace
- shared attachment shape
- stronger typed surface promotion threshold
- full type calculus
- inference / annotation design
- final typed syntax
- public typed API
