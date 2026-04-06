# 89 — current L2 stage 3 admit node handoff comparison

## 目的

この文書は、`specs/examples/88-current-l2-stage3-admit-next-step-sequencing.md` で
stage 3 declaration-side `admit` branch の次段は request-local clause spillover ではなく
fixture-side `OptionDecl.admit` handoff comparison を先に扱うと整理したことを前提に、
**parser-side `decl_admit_slot` と fixture-side `OptionDecl.admit` predicate node の間を later stage にどう残すのが最小か**
を比較する。

ここで固定するのは final parser API ではない。
固定するのは、stage 3 line が hidden predicate parse や hidden canonicalization を持ち込まない
**docs-only handoff boundary** だけである。

## 前提

- current L2 の core semantics は変更しない。
- parser-side carrier は `decl_admit_slot` を第一候補とする。
- current first tranche では fixture-side `OptionDecl.admit` へ direct lowering しない。
- fixture-side `OptionDecl.admit` は already elaborated predicate node である。
- predicate fragment parse / normalization は still later stage に残す。

## current source anchor

- `specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md`
  - stage 1 では parser-side opaque slot carrier と fixture-side compatibility anchor を thin lowering で分ける
- `specs/examples/84-current-l2-stage3-admit-slot-carrier-and-compare-surface.md`
  - stage 3 first tranche では fixture-side `OptionDecl.admit` へ direct lower しない
- `specs/examples/85-current-l2-stage3-admit-slot-first-tranche-actualization.md`
  - `decl_admit_slot.surface_text` retention smoke までは actualize 済み
- `crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json`
  - `OptionDecl.admit` は predicate node carrier

current issue は、stage 3 first tranche の次で
fixture-side `OptionDecl.admit` node とどう接続するかを決めないまま request cluster へ進むと、
declaration-side branch 自体の handoff line が曖昧なままになることである。

## 比較する 3 案

### 案 1. parser-side `decl_admit_slot` を fixture-side `OptionDecl.admit` へ direct lower する

#### 利点

- parser-side output と fixture-side predicate node を直接比較できる。

#### 欠点

- predicate fragment parse / normalization を hidden に先食いしやすい。
- current stage で already elaborated node へ合わせる pressure が強すぎる。
- stage 1 handoff の thin bridge line と対称でなくなる。

### 案 2. fixture-side `OptionDecl.admit` から helper-local canonical surface を再構成して compare する

#### 利点

- direct lowering よりは narrow に見える。
- parser-side `surface_text` と fixture-side reference surface を比べられる。

#### 欠点

- helper-local canonicalization rule を新設する必要がある。
- pretty-print / normalization が hidden parse point になりやすい。
- later predicate fragment parser と責務競合しやすい。

### 案 3. current stage では handoff comparison を docs-only に留め、actual compare は predicate fragment boundary が見えてから reopen する

parser-side `decl_admit_slot.surface_text` と fixture-side `OptionDecl.admit` node の関係は、
「later stage で接続すべき boundary」であることだけを明示し、
current stage では actual compare / canonicalization / lowering をまだ入れない。

#### 利点

- hidden predicate parse を最も避けやすい。
- stage 1 と同様、opaque slot line を守ったまま later-stage handoff を先送りできる。
- request-local clause line と独立に整理できる。

#### 欠点

- current repo に actual compare evidence はまだ増えない。
- handoff comparison は docs-only understanding に留まる。

## 比較

### hidden predicate parse / normalization の回避

- 案 1 は最も危険である。
- 案 2 も canonical surface reconstruction が hidden normalization になる。
- 案 3 は actual compare を deferred にするため最も安全である。

### stage 1 line との対称性

- stage 1 は thin lowering bridge を current fixture compatibility anchor にだけ使った。
- stage 3 では fixture-side `OptionDecl.admit` が string-like carrier ではなく predicate node なので、
  stage 1 と同じ immediate bridge はまだ作れない。
- したがって current phase では、案 3 のように docs-only handoff boundary として残す方が stage 1 line と整合する。

### request cluster との分離

- 案 1 と案 2 は declaration-side handoff を actualize し始めるぶん、
  request cluster へ進む前に predicate fragment boundary まで触りやすい。
- 案 3 は declaration-side branch の remaining issue を明文化しつつ、
  request-local clause line と混ぜない。

## current judgment

current repo の next narrow step としては、
**案 3. current stage では handoff comparison を docs-only に留め、actual compare は predicate fragment boundary が見えてから reopen する**
のが最も自然である。

つまり、

1. parser-side `decl_admit_slot` と fixture-side `OptionDecl.admit` node の接続は、current stage では actualize しない
2. direct lowering も helper-local canonical surface compare も current phase では導入しない
3. later stage で predicate fragment boundary が source-backed に見えた時点で reopen する

## なぜこれが最小か

- fixture-side `OptionDecl.admit` は already elaborated predicate node なので、stage 1 の `lease` handoff と違って string-like compatibility anchor がない。
- current first tranche の役割は declaration-side attached slot の existence / attachment / retained surface を示すことであり、predicate fragment semantics を先食いすることではない。
- request-local clause line も still later stage に残しているため、ここで declaration-side handoff まで actualize する pressureはまだ弱い。

## current stage でまだやらないこと

- parser-side `decl_admit_slot` から fixture-side predicate node への lowering
- helper-local canonical surface reconstruction
- `OptionDecl.admit` node を compare target に含める lowered fixture-subset compare
- request-local `require` / `ensure` spillover actualization

## reopen 条件

この handoff line を reopen してよいのは、少なくとも次のいずれかが source-backed に見えたときである。

1. predicate fragment boundary の first parser cut 候補が docs-only で切れたとき
2. fixture-side `OptionDecl.admit` compare が actual checker / diff loop に必要になったとき
3. request-local clause line と declaration-side admit line を同時に比較しないと drift を防げない concrete pressure が出たとき

## next narrow step

current mainline としては、
request-local `require` / `ensure` spillover を stage 3 later branch としてどこまで docs-only に持つかを比較する前に、
まずこの handoff line は docs-only deferred であることを `plan/` と `progress.md` に mirror しておくのが自然である。
