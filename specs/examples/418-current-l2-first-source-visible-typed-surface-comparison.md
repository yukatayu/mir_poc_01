# 418 — current L2 first source-visible typed-surface comparison

## 目的

`specs/examples/413-current-l2-typed-core-attachment-inventory-and-obligation-allocation-refresh.md`
で fixed した checker-adjacent semantic carrier first cut を前提に、

- どの source-visible marker family を first typed-surface candidate として扱うか
- checker-adjacent principal source をどこまで維持するか
- 何を first comparison から drop するか

を docs-first に整理する。

ここで fixed するのは **first source-visible typed-surface comparison** であり、
full type calculus や final typed syntax を fixed しない。

## source-backed floor

- current first typed attachment candidate は checker-adjacent semantic carrier にある。
- theorem/model-check consumer artifact は derived attachment と読む。
- source-side already-visible structural marker として、
  - `capability`
  - `lease`
  - `admit`
  - lineage edge annotation
  - request-side `require` / `ensure`
  がある。

## comparison family

| candidate | reading | strengths | current risk |
|---|---|---|---|
| explicit structural marker family | 既存 source row に already ある `capability` / `lease` / `admit` / lineage / request clause を typed source anchor として読む | syntax open を保ちつつ source-visible comparison を始められる | marker をそのまま final typed syntax と誤読しやすい |
| dedicated attachment block | chain / request の近傍に typed attachment block を新設する | typed carrier を見やすく切れる | final grammar と final attachment shape を先取りしやすい |
| unified obligation block | request / chain / proof handoff を単一 source block へ寄せる | long-run unification には見通しがある | current checker floor と syntax open を壊しやすい |

## current judgment

current L2 で最も自然なのは、
**explicit structural marker family を first source-visible typed-surface candidate に置く**
ことである。

理由は次の通り。

1. current repo は checker-adjacent semantic carrier を principal source に固定済みであり、source-visible comparison はその mirror に留める方が自然である。
2. `capability` / `lease` / `admit` / lineage / `require` / `ensure` は already source-backed floor に存在し、new block syntax を導入しなくても comparison ができる。
3. dedicated attachment block や unified obligation block は final parser grammar と final typed namespace を premature に拘束しやすい。

## keep / drop line

### keep

- principal typed source は checker-adjacent semantic carrier に維持する
- explicit structural marker family は **source-visible candidate** として comparison に使う
- obligation owner matrix は `core_static_checker / theorem_prover_boundary / protocol_verifier_boundary / runtime_policy_boundary` に維持する

### drop from current package

- dedicated attachment block を first cut に採ること
- theorem/model-check consumer artifact を principal typed source に繰り上げること
- unified obligation block を source-level first surface に採ること

## current first choice details

- source-visible typed candidate は、new typed keyword family ではなく、existing structural marker family の grouped reading に留める。
- `require` / `ensure`、option-side marker、lineage annotation を 1 grammar へ統一するかは still OPEN に残す。
- current package では namespace、field naming、public typed schema を固定しない。

## next promoted line

next promoted line は、
**checker attachment から handoff row への migration note**
に置く。

## what is not decided here

- full strong type system
- inference / annotation design
- final typed syntax
- dedicated attachment block の採否
- unified obligation block の採否
