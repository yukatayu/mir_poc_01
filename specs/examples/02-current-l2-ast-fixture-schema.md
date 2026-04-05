# examples/02 — current L2 AST fixture schema

この文書は、current L2 の representative Mir programs を、**parser なしで machine-readable に扱うための最小 AST fixture schema** を整理する補助文書である。
ここで定めるのは final parser grammar ではなく、`specs/examples/00-representative-mir-programs.md` と `specs/examples/01-current-l2-surface-syntax-candidates.md` の current reading を、そのまま interpreter 前段の fixture に落とすための companion carrier である。

## この文書の位置づけ

- current L2 の representative examples を parser 実装なしで fixture 化する。
- syntax 未決部分を固定せず、意味側の構造だけを machine-readable に保つ。
- static verdict、runtime outcome、trace / audit expectation を example fixture 側でも持てるようにする。
- parser なし最小 interpreter に必要な runtime carrier は `specs/examples/03-current-l2-evaluation-state-schema.md` を参照する。
- parser なし最小 interpreter の 1-step semantics は `specs/examples/04-current-l2-step-semantics.md` を参照する。
- parser なし最小 interpreter の predicate / effect oracle boundary は `specs/examples/05-current-l2-oracle-api.md` を参照する。

## ここで固定すること / しないこと

- 固定すること:
  - current L2 examples を lossless に近い形で落とせる最小 node set
  - `valid` / `malformed` / `underdeclared` の static verdict carrier
  - `success` / `explicit_failure` / `Reject` などの runtime outcome carrier
  - request-local trace / audit expectation の最小 carrier
- 固定しないこと:
  - final parser punctuation
  - final reserved keyword 集合
  - JSON field 名の長期固定
  - trace / audit metadata の serialization
  - future interpreter が内部で使う完全な runtime object model

## carrier 方針

- current L2 fixture は JSON carrier を使ってよい。
- ただし JSON は final wire format ではなく、examples を parser なしで読むための companion carrier に留める。
- field 名は current L2 の説明用名であり、最終 API 名を固定しない。

## 最小 top-level shape

各 fixture は、概念的には次の 4 つだけを持てばよい。

1. `program`
   - current L2 program 断片を AST として持つ。
2. `expected_static`
   - `valid` / `malformed` / `underdeclared` と、その最小理由を持つ。
3. `expected_runtime`
   - runtime evaluation に入るかどうか、入るなら final outcome が何かを持つ。
4. `expected_trace_audit`
   - event surface に残るべき abstract event と、audit 側に残すべき metadata / explanation を持つ。

## 最小 AST node set

current L2 representative examples を落とす最小 node set は、少なくとも次で足りる。

| node | 役割 |
|---|---|
| `Program` | top-level statement list |
| `PlaceBlock` | `place name { ... }` の意味構造 |
| `PerformOn` | `perform op on target` |
| `PerformVia` | `perform op via chain_ref` |
| `OptionDecl` | option declaration と option-local `admit` |
| `ChainDecl` | canonical chain declaration |
| `TryFallback` | local rollback を伴う `try { ... } fallback { ... }` |
| `AtomicCut` | place-local `atomic_cut` |

### `contract` の保持

- surface では `contract` keyword を使わない current L2 policy を維持する。
- ただし fixture AST では、`require` / `ensure` を単なる行列挙ではなく semantic role として保持するため、`PerformOn` / `PerformVia` の下に `contract` object を持たせてよい。
- これにより、`require pred` と `require:` block は同じ意味構造へ正規化できる。

### `admit` の保持

- option-local `admit` は separate statement にせず、`OptionDecl` の admission-side metadata として保持する。
- current L2 では option-local `ensure` や outcome guarantee はまだ持たせない。

## syntax 依存情報と意味構造の切り分け

fixture では、次の情報を**保持しない**。

- `perform` / `option` / `chain` / `try` / `fallback` / `admit` の最終 token 形
- `require pred` と `require:` block の見た目の差
- multi-line predicate の改行位置
- blank line
- `lineage(A -> B)` の punctuation

代わりに、次の意味構造だけを保持する。

- `perform` が direct target か chain 経由か
- `require` / `ensure` がどの request に属するか
- option declaration ごとの target / capability / lease / admit
- chain edge ごとの predecessor / successor / lineage assertion
- `try` body と fallback body
- `atomic_cut` の位置

## predicate の最小 carrier

current L2 では、predicate は最小 companion fragment に合わせて次の構造だけを持てばよい。

| predicate node | 役割 |
|---|---|
| `atom` | bare atom (`write`, `read`, `append`) |
| `call` | application-like form (`owner_is(session_user)`) |
| `and` | explicit conjunction |

- surface の括弧は AST には残さず、grouping された predicate tree に正規化してよい。
- 改行による implicit conjunction は current L2 では導入しないため、AST にも implicit connective node は要らない。

## chain edge の最小保持

`ChainDecl` は、最低限次を持てばよい。

- `head`
- ordered `edges`
- 各 edge の `predecessor`
- 各 edge の `successor`
- optional な `lineage_assertion`

この shape にしておくと、

- `valid`
- `malformed`
- `underdeclared`

の差を parser token に頼らず表せる。

## expected behavior の最小 carrier

### `expected_static`

- `verdict`
  - `valid`
  - `malformed`
  - `underdeclared`
- `reasons`
  - human-facing な補助説明を含みうる最小文字列配列
- optional `checked_reasons`
  - actual static gate compare に使う fail-closed machine-check carrier
  - current L2 では additive optional field としてのみ導入してよい
  - absent のときは current harness が `verdict` だけを machine-check し、`reasons` は explanation に残す

### `expected_runtime`

- `enters_evaluation`
  - runtime evaluation に入るか
- `final_outcome`
  - `success`
  - `explicit_failure`
  - `Reject`
  - `not_evaluated`
- `notes`
  - final outcome だけでは落ちる説明を補う最小文字列配列

### `expected_trace_audit`

- `event_kinds`
  - event surface に残るべき abstract event の並び
- `non_admissible_metadata`
  - current request evaluation にぶら下がる option-local miss metadata
- `narrative_explanations`
  - formal subreason に上げていない説明
- `must_explain`
  - trace / audit が最低限説明できるべきこと

## non-admissible metadata の最小 shape

current L2 では、non-admissible metadata は次だけ読めればよい。

- `option_ref`
- `subreason`

ここで `subreason` に current L2 で formal に残すのは、

- `admit-miss`
- `lease-expired`

だけである。

capability mismatch は current L2 では narrative explanation に留めるため、formal subreason にしない。

## current fixture set

current L2 の最小 fixture set は次の 13 本とする。

| fixture | source example | 主題 |
|---|---|---|
| `e1-place-atomic-cut.json` | E1 | place 入れ子 + authority update + `atomic_cut` |
| `e2-try-fallback.json` | E2 | local `try` + `fallback` |
| `e3-option-admit-chain.json` | E3 比較用 variant | option-local `admit` を伴う valid chain |
| `e4-malformed-lineage.json` | E4 | malformed fallback branch |
| `e5-underdeclared-lineage.json` | E5 | underdeclared fallback case |
| `e12-underdeclared-target-missing.json` | E12 | declared access target 欠落による underdeclared static stop |
| `e13-malformed-capability-strengthening.json` | E13 | same-lineage edge で capability が強化される malformed static stop |
| `e6-write-after-expiry.json` | E6 | write-after-expiry と runtime `Reject` |
| `e7-write-fallback-after-expiry.json` | E6 補完 variant | `lease-expired` の後に後段 write-capable option へ degrade して success |
| `e8-monotone-degradation-reject.json` | E3 / E6 補完 regression | `admit-miss` と explicit failure を挟んだ left-to-right degradation の末に runtime `Reject` |
| `e9-monotone-degradation-success.json` | E3 / E6 success-side 補完 regression | `admit-miss` と middle explicit failure を挟んでも later write-capable option へ left-to-right degradation して runtime `success` |
| `e10-perform-on-ensure-failure.json` | E1 direct-target ensure failure variant | direct `PerformOn` の request-local `ensure` unsatisfied を runtime `explicit_failure` として固定し、`Reject` や non-admissible metadata を混ぜない |
| `e11-perform-via-ensure-then-success.json` | E3 via-chain ensure continuation variant | via-chain の earlier option が request-local `ensure` で失敗しても、later same-lineage option があれば tentative commit を破棄して runtime `success` まで継続できる |

fixture 本体は `crates/mir-ast/tests/fixtures/current-l2/` に置く。

## ここであえて決めていないこと

- field 名の最終固定
- detached trace serialization
- `request ref` を独立 field にするかどうか
- `reason kind` を独立 field にするかどうか
- final parser syntax と AST の対応表
- fixture から直接 interpreter 実装へ進む際の evaluation state shape

これらは **未決定** とする。current L2 で固定するのは、representative examples を parser なしで評価準備できる最小 schema だけである。
