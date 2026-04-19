# plan/06 — surface notation の現在地

## 位置づけ

current L2 の surface notation は **companion notation** である。
つまり、examples と PoC を安定して読めるようにするための説明用記法であって、final parser grammar ではない。

## current L2 companion notation に含まれるもの

### `perform`

- direct target: `perform op on target`
- chain reference: `perform op via chain_ref`
- `perform` は convenience 表記であり、final reserved keyword では未固定

### option chain

- option declaration: `option name on target capability cap lease guard`
- same-lineage の証拠は edge-local lineage annotation
- current polished first choice は explicit edge-row family

### `try { ... } fallback { ... }`

- local rollback を伴う block form
- option chain の `fallback successor` と構文形で区別する

### `contract` policy

- `contract` は semantic role 名に留める
- surface では statement-local `require` / `ensure` を使う
- `contract { ... }` block form は current companion notation では採らない

### `require` / `ensure`

- single-line: `require pred` / `ensure pred`
- multi-line のときだけ `require:` / `ensure:` block
- predicate block 内では implicit conjunction を入れない

### predicate sublanguage

- bare atom
- application-like form
- explicit `and`
- 括弧 grouping

`or` / `not` / precedence table を final grammar としてどうするかは未決である。

### option-local `admit`

- option declaration の下に indented で置く
- admission-side metadata であり request-local clause と混ぜない
- outcome-side guarantee は current L2 ではまだ持ち込まない

## fallback notation の現在地

### current L2 で維持している family

**explicit edge-row family** を current L2 companion notation として維持している。

理由:

- guarded option chain を最も semantically honest に表せる
- edge-local lineage と request-evaluation boundary を保ちやすい
- outer / inner wrapper 読みを最も抑制しやすい

### current polished first choice

current L2 では、explicit edge-row family の中で **A2: hanging lineage continuation** を polished first choice に置く。

```text
chain profile_ref = writer
  fallback delegated_writer
    @ lineage(writer -> delegated_writer)
  fallback readonly
    @ lineage(delegated_writer -> readonly)
```

- rollout は docs-only に留めず、representative examples のうち fallback / preference chain を主題にする code block まで広げてよい。
- 一方で fixture schema、interpreter、tests は machine-check surface が主目的なので、A2 rendering へ同期させない。
- hanging continuation の追加 indent は outer / inner wrapper の入れ子ではなく、直前 edge row にだけ属する edge-local metadata continuation として読む。

### companion-equivalent shorthand

短い row では **A1 inline row** も companion-equivalent shorthand として残してよい。

```text
chain profile_ref = writer
  fallback delegated_writer @ lineage(writer -> delegated_writer)
```

### 採らない current L2 候補

- line-leading `>` ladder
  - 比較はしたが current L2 companion notation には昇格させていない
- packed metadata row
  - option-local 情報と edge-local 情報を混ぜやすい

## order / handoff companion narrowing の現在地

### current principal

- order / handoff companion surface でも **explicit edge-row / vertical continuation family** を principal に置く。
- source-facing vocabulary は low-level `memory_order` exact token ではなく、
  - `publish`
  - `observe`
  - `handoff`
  - `witness`
  のような readable high-level family を優先する。
- `after ...` continuation のように edge-local order / obligation を見せる line は current recommendation に含めてよい。

### retained secondary candidate

- `stage` / `after` / `witness` family は strong secondary experimental candidate に残す。
- これは final grammar ではなく、helper-local compare floor で `p07/p08` を読む secondary surface として扱う。
- `stage-block + witness` を current principal に上げるのは still 早い。

### reserve sugar

- `serial on <owner>` family は authoritative-room-specific high-level sugar として reserve に残す。
- shared-memory lock のような source principal へ寄せず、authoritative serial transition scope の room-specific sugar として読む。

### still not principal

- low-level `memory_order` / fence exact surface
- packed metadata line
- hidden precedence
- final modal token import

## line-leading `>` ladder の位置づけ

- 視覚的 compactness と vertical reading では魅力がある
- しかし current L2 では operator / expression に見えやすい
- outer/inner 誤読は減らせても、別の形で edge-locality と request-evaluation 境界を薄めやすい
- したがって **比較候補には残すが current companion notation には採らない**

## final parser grammar について

### まだ決めていないもの

- final parser syntax
- reserved keyword 集合
- punctuation の確定
- inline / hanging continuation の最終 grammar rule
- predicate grammar 完成形

## first parser cut inventory の current understanding

current L2 では、final grammar を先に決めずに、
**first parser cut に入れてよい semantic cluster**だけを先に棚卸しするのが自然である。

### first parser cut に入れてよいもの

- `place { ... }`
- `try { ... } fallback { ... }`
- `atomic_cut`
- `perform ... on ...`
- `perform ... via ...`
- statement-local `require` / `ensure`
- `option ... on ... capability ... lease ...`
- option-local `admit`
- chain declaration の explicit edge-row family

### まだ companion に残すもの

- A2 と A1 の exact lexical choice
- line-leading `>` ladder
- `contract { ... }` block sugar
- richer predicate sublanguage
- option-local outcome metadata

### current judgment

- parser cut では exact visual polish より semantic cluster を先に固定する
- explicit edge-row family 自体は first parser cut 候補に入れてよい
- ただし A2 hanging continuation を唯一の final lexical surface として固定するのはまだ早い
- actual parser spike を切る順序は別問題であり、current next narrow step は
  1. chain / declaration structural floor
  2. `try` / rollback structural floor
  3. request / admissibility cluster
  の checker-led staged spike として読むのが自然である

### 目標だけは明示しているもの

- C 的でないこと
- AST dump 的でないこと
- 本質的に簡潔であること
- 縦方向に読めること
- インデントと相性がよいこと
- outer/inner wrapper 誤読を誘発しないこと

## 今後の syntax workstream で比較するもの

- syntax candidate は、少なくとも
  - semantic honesty
  - checker legibility
  - modal adequacy
  - misreading resistance
  - lowering friendliness
  の 5 軸で比較する。
- 「自然に書けるものは自然な挙動をするべき」「理論的に破綻した動作は書けない / 書きにくいべき」を、
  final grammar ではなく companion notation / theory comparison の判断軸として先に維持する。
- explicit edge-row family のさらに良い polishing があるか
- `perform` / `on` / `via` / `admit` / `require` / `ensure` の final token をどうするか
- `try` / `fallback` block form と option-chain `fallback` の最終 grammar separation
- predicate sublanguage をどこまで core に入れるか
- syntax を compact にしても semantics honesty を落とさないか
- order / handoff で edge-row principal と stage-block secondary candidate をどこまで helper-local に保つか
- authoritative-room `serial` sugar を room-specific reserve のまま保つか

## current L2 settled / OPEN

### current L2 settled

- companion notation は examples 用の安定記法
- explicit edge-row family を維持
- A2 を polished first choice
- A1 を shorthand として許容
- A2 の rollout は representative examples の chain-heavy code block まで

### OPEN

- final parser grammar
- line-leading `>` ladder を将来再比較する閾値
- `contract` block sugar の復活可能性
- option-local outcome metadata
