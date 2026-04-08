# 92 — current L2 stage 3 predicate fragment reopen sequencing

## 目的

この文書は、`specs/examples/91-current-l2-stage3-request-local-clause-spillover-first-tranche-actualization.md`
までで

- declaration-side `admit` attached slot の success-side first tranche
- request head spillover
- bare request-local `require` / `ensure` spillover

が揃ったことを前提に、
**stage 3 later branch の次段として `request head + clause attachment multiline shape` と `predicate fragment boundary` のどちらを先に reopen すべきか**
を narrow に比較する。

ここで固定するのは final parser grammar でも final checker API でもない。
固定するのは、declaration-side `admit` line と request-local clause line の両方に効く
**next narrow sequencing judgment** だけである。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- stage 3 helper は private / test-only のまま維持する。
- fixture-side `OptionDecl.admit` handoff は current phase では docs-only deferred に留める。
- request-local bare clause spillover pair は actualize 済みであり、current helper は clause attachment rule をまだ持たない。
- `specs/examples/01-current-l2-surface-syntax-candidates.md` では、
  - statement-local `require` / `ensure`
  - option-local `admit`
  - multi-line predicate block
  の minimal companion syntax candidate が既に整理されている。
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md` では、
  predicate fragment の最小 well-formedness は first checker cut 候補 cluster に入れてよいと整理済みである。

## current source anchor

- `specs/examples/01-current-l2-surface-syntax-candidates.md`
  - `require:` / `ensure:` / `admit:` の multi-line predicate block 候補
  - minimal predicate fragment として bare atom / application-like form / explicit `and` / 括弧 grouping
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
  - minimal predicate fragment well-formedness は first checker cut に入れてよい
- `specs/examples/89-current-l2-stage3-admit-node-handoff-comparison.md`
  - fixture-side `OptionDecl.admit` handoff は predicate fragment boundary が見えるまで docs-only deferred
- `specs/examples/90-current-l2-stage3-request-local-clause-spillover-comparison.md`
  - bare request-local `require` / `ensure` は later-stage clause boundary として helper-local malformed-source pair に留める
- `specs/examples/91-current-l2-stage3-request-local-clause-spillover-first-tranche-actualization.md`
  - bare request-local clause pair は actualize 済み
- `crates/mir-ast/tests/support/current_l2_stage3_admit_slot_spike_support.rs`
  - current helper は `perform` / `require` / `ensure` line を fail-closed に reject するが、predicate fragment や multiline attachment はまだ parse しない

current issue は、次に actualize しうる later branch を

1. request head + clause attachment multiline shape
2. predicate fragment boundary

のどちらから開くべきかである。

## 比較する 3 案

### 案 1. request head + clause attachment multiline shape を先に比較する

まず request head と indented clause suite の multiline shape を扱う。

例:

```text
perform write_profile via profile_ref
  require:
    (
      write
      and owner_is(session_user)
    )
```

#### 利点

- request cluster の見た目に早く近づく。
- `require:` / `ensure:` の block attachment を docs-only で先に整理できる。

#### 欠点

- request head attachment と predicate payload の 2 論点を同時に持ち込む。
- declaration-side `admit:` multiline と shared floor を持ちにくい。
- fixture-side `OptionDecl.admit` handoff deferred line と直接つながらない。

### 案 2. predicate fragment boundary の reopen 条件を先に比較する

まず declaration-side `admit` と request-local `require` / `ensure` の両方が共有する
**minimal predicate fragment** の reopen 条件を比較する。

例:

```text
option owner_writer on profile_doc capability write lease live
  admit:
    (
      owner_is(session_user)
      and well_formed(profile_draft)
    )
```

```text
perform write_profile via profile_ref
  ensure:
    (
      owner_is(session_user)
      and write
    )
```

#### 利点

- declaration-side branch と request-local branch の shared floor を先に切れる。
- fixture-side `OptionDecl.admit` handoff reopen 条件と直接つながる。
- `specs/examples/30` の first checker cut inventory と整合する。
- request head attachment rule をまだ hidden に持ち込まない。

#### 欠点

- request cluster の multiline shape 自体はまだ進まない。
- opaque slot / parsed fragment / canonicalization の境界を丁寧に分ける必要がある。

### 案 3. multiline shape と predicate fragment を同時に reopen する

#### 利点

- request-local clause line と declaration-side `admit` line を一気に進められる。

#### 欠点

- clause attachment、predicate fragment、fixture-side handoff の 3 論点が同時に混ざる。
- current stage では hidden parse / hidden attachment / hidden canonicalization pressure が最も強い。
- stage 3 line の narrow progression に合わない。

## 比較

### declaration-side / request-local の shared floor

- 案 1 は request-local branch だけを先に開くので、declaration-side `admit` line との shared floor を作りにくい。
- 案 2 は `admit` と `require` / `ensure` の両方に使う predicate fragment を先に扱える。
- 案 3 は shared floor 自体は見えるが、同時に attachment と handoff も混ざる。

### fixture-side `OptionDecl.admit` handoff deferred line との接続

- `specs/examples/89` で deferred にした理由は、fixture-side `OptionDecl.admit` が already elaborated predicate node だからである。
- したがって reopen 条件として自然なのは request head multiline shape ではなく、predicate fragment boundary 側である。
- この観点では案 2 だけが直接つながる。

### hidden attachment rule の回避

- 案 1 は multiline clause suite を扱い始める時点で、request head + clause attachment の structural rule を暗黙に持ち込みやすい。
- 案 2 は predicate payload 側の reopen 条件だけを比較するので、attachment rule をまだ外に残せる。
- 案 3 は最も危険である。

### first checker cut との整合

- `specs/examples/30` は minimal predicate fragment well-formedness を first checker cut に入れてよいと整理している。
- 一方 request head + clause attachment multiline shape は structural candidate ではあるが、request-local suite と block nesting の surface judgmentを先に増やす。
- current phase の parser/checker staging としては、案 2 の方が small decidable core の line に合う。

## current judgment

current repo の next narrow step としては、
**案 2. predicate fragment boundary の reopen 条件を先に比較する**
のが最も自然である。

つまり、

1. stage 3 later branch の次段は request head + clause attachment multiline shape ではなく、predicate fragment boundary の reopen 条件を先に扱う
2. reopen の対象は、declaration-side `admit` と request-local `require` / `ensure` に共有される minimal predicate fragment floor である
3. request head + clause attachment multiline shape は、その shared fragment floor が見えた後の later step に残す

## なぜこれが最小か

- fixture-side `OptionDecl.admit` handoff deferred line を reopen する条件は predicate fragment 側にある。
- bare clause spillover pair は既に actualize 済みなので、request-local branch は fail-closed evidence を最低限持っている。
- この時点で multiline attachment shape を先に開くと、request-local branch だけが先に太り、declaration-side branch と first checker cut inventory の shared floor が見えにくくなる。

## current stage でまだやらないこと

- request head + clause attachment multiline malformed-source shape の actualization
- request-local clause suite の structural parser
- fixture-side request contract node compare
- fixture-side `OptionDecl.admit` への direct lowering
- predicate fragment canonicalization / pretty-print

## next narrow step

次は
**minimal predicate fragment boundary を current stage でどこまで reopen してよいか**
を docs-only で比較するのが自然である。

その comparison では少なくとも次を分けて扱う。

1. opaque slot のまま surface retention だけ増やす案
2. minimal parsed fragment を helper-local compare に入れる案
3. declaration-side `admit` と request-local clause の両方へ shared floor を与えるが、request head attachment は still later stage に残す案
