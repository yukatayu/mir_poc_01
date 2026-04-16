# 433 — current L2 request/predicate/try cluster typed-surface reserve note

## 目的

`specs/examples/418-current-l2-first-source-visible-typed-surface-comparison.md`
と
`specs/examples/425-current-l2-checker-attachment-to-handoff-row-migration-note.md`
で fixed した

- checker-adjacent semantic carrier first
- source-visible structural marker family first
- checker attachment principal / handoff row derived reserve

を前提に、

- request cluster
- predicate fragment cluster
- `try` / rollback structural cluster

までを **typed-surface reserve** としてどこまで grouped reading に含めるかを docs-first に整理する。

ここで fixed するのは reserve line だけであり、
new typed block、full type calculus、final typed syntax は fixed しない。

## source-backed floor

- principal typed source は checker-adjacent semantic carrier にある。
- source-visible marker family として
  - `capability`
  - `lease`
  - `admit`
  - lineage annotation
  - request-side `require` / `ensure`
  が already ある。
- parser / source bridge 側には minimal predicate fragment と `try` / rollback structural cluster の narrow carrier がある。
- theorem / model-check artifact は derived consumer であり、principal typed source ではない。

## cluster comparison

| cluster | current reading | strengths | current risk |
|---|---|---|---|
| request-side clause cluster | `require` / `ensure` を obligation-side typed cue として grouped reading する | source-visible typed cue を増やせる | request clause を final typed syntax と誤読しやすい |
| predicate fragment cluster | predicate fragment を guarded attachment candidate として読む | checker / theorem / model-check bridge の shared cue になる | predicate language を premature に typed calculus 側へ引き込む |
| `try` structural cluster | `try` / rollback structural cue を effect / obligation reserve reading に留める | rollback / cut / theorem-side cue と接続しやすい | `try` syntax を effect typing surface と誤読しやすい |
| dedicated typed block | request / predicate / `try` を独立 block に寄せる | long-run 統一には見通しがある | final grammar と namespace を先取りしやすい |

## current judgment

current L2 で最も自然なのは、
**request / predicate / `try` cluster を grouped reserve reading に留め、
checker attachment principal を維持する**
ことである。

理由は次の通り。

1. repo は already `checker attachment principal / handoff row derived reserve` を fixed しており、request/predicate/`try` をここで principal source に繰り上げる理由がない。
2. request-side clause、predicate fragment、`try` structural cluster は source-visible cue としては有益だが、いずれも final typed syntax として読むには早い。
3. dedicated typed block は current parser-open rule と conflict しやすい。

## reserve bundle floor

current reserve note が持つ minimum は次である。

```text
typed_surface_cluster_reserve = {
  principal_attachment_ref,
  request_cluster_refs[],
  predicate_cluster_refs[],
  try_cluster_refs[],
  guard_refs[],
  kept_later_refs[]
}
```

## keep / drop line

### keep

- principal typed source は checker attachment に残す
- request-side clause cluster は grouped reserve cue に留める
- predicate fragment cluster は guarded reserve cue に留める
- `try` structural cluster は rollback / obligation reserve cue に留める

### drop from current package

- request / predicate / `try` を shared typed block に統合すること
- predicate fragment を principal typed source にすること
- theorem / model-check artifact を typed source of truth に繰り上げること

## next promoted line

next promoted line は、
**typed-surface family unification keep/drop note**
に置く。

## what is not decided here

- `family_refs[]` namespace
- request / predicate / `try` の shared attachment shape
- full type calculus
- inference / annotation design
- final typed syntax
- public typed API
