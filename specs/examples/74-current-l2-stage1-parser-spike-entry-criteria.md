# 74 — current L2 stage 1 parser spike entry criteria

## 目的

この文書は、`specs/examples/73-current-l2-first-parser-spike-staging.md` で current next step とした
checker-led staged spike のうち、
**stage 1 の chain / declaration structural floor を actual parser spike として切るなら、
何を accepted parse cluster に含め、何を non-goal として later stage に残すべきか**
を narrow に比較する。

ここで固定するのは final parser grammar ではない。
固定するのは、

- stage 1 で parser に持ち込んでよい structural floor
- stage 1 ではまだ parser / checker に持ち込まないもの
- declaration-side guard slot をどこまで parse 対象に含めるか

という entry criteria だけである。

## 前提

- current L2 の core semantics は変更しない。
- `specs/examples/29-current-l2-first-parser-subset-inventory.md` の full inventory は維持する。
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md` の first checker cut inventory も維持する。
- `specs/examples/73-current-l2-first-parser-spike-staging.md` の stage order
  - stage 1: chain / declaration structural floor
  - stage 2: `try` / rollback structural floor
  - stage 3: request / admissibility cluster
  は維持する。
- final token、final punctuation、A2 / A1 の exact grammar choice、predicate sublanguage の completion は引き続き OPEN である。

## current source anchor

stage 1 を source-backed に支えている current anchor は次である。

- static gate anchor:
  - `crates/mir-semantics/src/lib.rs` の `static_gate_detailed`
- current AST fixture carrier:
  - `specs/examples/02-current-l2-ast-fixture-schema.md`
- representative malformed / underdeclared fixture:
  - `e4-malformed-lineage`
  - `e12-underdeclared-target-missing`
  - `e13-malformed-capability-strengthening`
  - `e16-malformed-missing-chain-head-option`
  - `e17-malformed-missing-predecessor-option`
  - `e18-malformed-missing-successor-option`
  - `e19-malformed-target-mismatch`
  - `e20-malformed-late-capability-strengthening`
- current helper family:
  - `scripts/current_l2_same_lineage_checker.py`
  - `scripts/current_l2_missing_option_checker.py`
  - `scripts/current_l2_capability_checker.py`
  - `scripts/current_l2_detached_loop.py`

この anchor が現在静的に見ているのは、

- option declaration の visible existence
- chain head / predecessor / successor の structural resolution
- edge-local lineage annotation の edge attachment
- declared access target の presence / equality
- capability strengthening prohibition

であり、request-local clause や predicate fragment parse は main source ではない。

## stage 1 で比較する 3 案

### 案 1. declaration-side guard slot を stage 1 から外す

stage 1 では option declaration core を

- option name
- target
- capability

までに限り、
`lease <guard>` は stage 3 以降へ deferred にする。

#### 利点

- stage 1 の parse 対象を最も小さくできる。
- predicate fragment と lexical pressure をさらに避けられる。

#### 欠点

- `specs/examples/29-current-l2-first-parser-subset-inventory.md` で
  option declaration core に含めた `lease <guard>` を stage 1 の structural floor から外すことになる。
- current fixture carrier では `OptionDecl` がすでに `lease` を持つため、
  parser spike と parser-free AST carrier の対応がかえって見えにくくなる。
- stage 1 が declaration structural floor を見ると言いながら、
  declaration-side guard attachment の存在確認すら later stage に送ることになる。

### 案 2. declaration-side guard slot を opaque attached slot として含める

stage 1 では option declaration core を

- option name
- target
- capability
- declaration-side guard slot

まで parse 対象に含める。

ただし guard slot の中身は
**minimal predicate fragment として parse しない**。
stage 1 が保証するのは、

- `lease` slot が option declaration に属していること
- slot が request-local clause や chain row と混ざっていないこと
- parser / checker handoff 上、slot の存在と attachment を保持できること

までに留める。

#### 利点

- `specs/examples/29-current-l2-first-parser-subset-inventory.md` の option declaration core を崩さない。
- current AST fixture carrier の `OptionDecl.target / capability / lease / admit` という shape に最も近い。
- stage 1 の structural floor を same-lineage / target / capability checker family に素直につなげられる。
- predicate fragment parse / well-formedness を stage 3 に残せる。

#### 欠点

- stage 1 に slot carrier を 1 つ置く必要がある。
- stage 1 だけ見ると、guard slot の内部意味はまだ弱い。

### 案 3. declaration-side guard slot を minimal predicate fragment として parse する

stage 1 で `lease <guard>` の guard を、すでに
`specs/examples/30-current-l2-first-checker-cut-entry-criteria.md` にある

- bare atom
- application-like form
- explicit `and`
- 括弧 grouping

まで parse 対象に含める。

#### 利点

- parser spike と first checker cut の predicate fragment floor を早く近づけられる。
- stage 3 で扱う request / admissibility cluster へ一部先行できる。

#### 欠点

- `specs/examples/73-current-l2-first-parser-spike-staging.md` で stage 3 に残した
  request / admissibility cluster の lexical pressureを stage 1 に持ち込む。
- option declaration structural floor と predicate fragment parse が混ざる。
- `perform` / `require` / `ensure` / option-local `admit` に関する later stage の判断を先食いしやすい。

## 比較

### current static gate anchor との整合

- 案 1:
  declaration floor から `lease` slot 自体を外すぶん、current fixture carrier と距離がある。
- 案 2:
  current static gate が必要としている declaration existence / attachment floor と最も整合する。
- 案 3:
  checker cut inventory とは整合するが、stage 1 の structural floor としては一段広い。

### parser-free AST carrier との整合

- 案 1:
  current `OptionDecl.lease` を later stage 専用扱いにしてしまう。
- 案 2:
  current `OptionDecl` shape を崩さず、guard 内部だけ later stage に残せる。
- 案 3:
  current AST carrier の predicate node family と早く接続できるが、stage split の利点が薄れる。

### first checker cut との handoff

- 案 1:
  same-lineage / target / capability floor へはつながるが、declaration-side guard attachment floor が抜ける。
- 案 2:
  stage 1 では structural attachment を押さえ、predicate well-formedness は later stage へ送れる。
- 案 3:
  stage 1 から predicate fragment floor を抱えるため、checker-led staged spike の切り分けが弱くなる。

### lexical freeze pressure

- 案 1:
  最も低い。
- 案 2:
  slot presence だけを見るので低いまま保てる。
- 案 3:
  predicate fragment parse を含むぶん高い。

## stage 1 の minimal accepted parse cluster

current judgment として、stage 1 に含めてよいのは次である。

1. option declaration の visible header
   - option name
   - declared access target
   - declared capability
2. declaration-side guard slot
   - current stage 1 では **opaque attached slot** として扱う
   - slot の existence / attachment / boundary だけを parse する
3. chain declaration の core
   - chain name
   - head option reference
   - ordered successor row
4. edge-local lineage metadata
   - row-local metadata として predecessor / successor edge に属すること

ここでいう opaque attached slot は、
**guard fragment の内部 grammar を stage 1 で決める**という意味ではない。
意味は次の 2 点に限る。

- `lease` は option declaration 側の slot である
- その slot は request / admissibility / chain metadata と混ざらない

## stage 1 の non-goal

stage 1 では、次をまだ扱わない。

1. minimal predicate fragment の parse / well-formedness
2. option-local `admit`
3. statement-local `require` / `ensure`
4. `perform on` / `perform via`
5. richer predicate grammar
   - `or`
   - `not`
   - precedence table
   - comparison operator completion
6. final token / reserved keyword / punctuation choice
7. A2 / A1 の exact accepted concrete grammar set
8. runtime / proof property
   - canonical normalization の一般証明
   - no re-promotion
   - rollback / cut non-interference

## stage 1 の output contract をどう読むか

stage 1 が与えるのは、current docs-only judgment では
**declaration structural floor に必要な parse boundary**だけである。

したがって stage 1 の output は、

- option declaration に declaration-side guard slot が attached している
- chain row に lineage metadata が row-local に attached している

という structural fact を落とさなければよい。

ここでまだ決めないものがある。

- slot の concrete carrier 名
- slot を raw token slice、raw text、opaque leaf のどれで持つか
- parser cut と parser-free AST fixture schema を actual API でどう接続するか

これらは actual parser spike / checker handoff task まで OPEN に残す。

## current judgment

current repo の next narrow step としては、
**案 2. declaration-side guard slot を opaque attached slot として含める**
のが最も自然である。

つまり、

1. stage 1 は chain / declaration structural floor に留める
2. option declaration core から `lease` slot 自体は外さない
3. ただし guard fragment 自体の parse / well-formedness は stage 3 以降へ残す

という cut を current docs-only judgment とする。

その actual handoff を parser-free AST fixture schema へどう接続するかは、
`specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md`
で parser-side opaque slot carrier と fixture-side narrow lowering bridge を分ける cut として比較する。

## OPEN に残すもの

- declaration-side guard slot の actual carrier 名
- opaque slot を parser API / checker API にどう受け渡すか
- stage 1 の actual parser spike を fixture corpus のどこまで使って smoke するか
- stage 3 に入る predicate fragment floor と option-local `admit` を同じ parser spike に載せるか
- final parser grammar と final visual polish
