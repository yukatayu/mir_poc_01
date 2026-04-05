# 70 — current L2 try/rollback first-tranche wording stability

## 目的

この文書は、current L2 parser-free PoC と `specs/examples/68`〜`69` を前提に、
dedicated `TryFallback` / `AtomicCut` AST structural helper first tranche の

- static gate wording
- helper-local `subject_kind`
- helper-local `finding_kind`

を、**いまどこまで固定してよいか**を narrow に整理する。

ここで扱うのは current first tranche の wording / row family stability だけである。
second malformed static tranche の actualization、shared detached carrier 昇格、generic structural checker family 合流、
public checker API 比較は扱わない。

## current source-backed anchor

current repo では、first-tranche wording / row family の source-backed anchor は次である。

- `crates/mir-semantics/src/lib.rs`
  - `collect_try_rollback_structural_reasons()`
  - `ExpectedStatic.checked_try_rollback_structural_verdict`
  - `ExpectedStatic.checked_try_rollback_structural_findings`
- `scripts/current_l2_try_rollback_structural_checker.py`
  - static gate artifact `checker_core.reasons` を helper-local row family へ写す bridge
- `crates/mir-ast/tests/fixtures/current-l2/e23-malformed-try-fallback-missing-fallback-body.json`
- `crates/mir-ast/tests/fixtures/current-l2/e24-malformed-atomic-cut-fallback-placement.json`
- `specs/examples/67-current-l2-try-rollback-malformed-pattern-slot-selection.md`
- `specs/examples/68-current-l2-try-rollback-ast-helper-first-tranche-actualization.md`
- `specs/examples/69-current-l2-try-rollback-second-malformed-static-tranche-comparison.md`

current actual wording と helper-local row family は次である。

### `TryFallback` side

- static gate wording
  - `try fallback body must not be empty`
- helper-local row
  - `subject_kind = TryFallback`
  - `finding_kind = missing_fallback_body`

### `AtomicCut` side

- static gate wording
  - `atomic cut may not appear inside fallback body`
- helper-local row
  - `subject_kind = AtomicCut`
  - `finding_kind = disallowed_fallback_placement`

## 比較する選択肢

### 案A. current exact wording / row family をそのまま固定する

current first tranche の reason wording と helper-local row family を、
少なくとも next narrow step の間はそのまま保つ。

これは次を意味する。

- fixture-side `checked_try_rollback_structural_findings` の row は current exact spelling を使う
- helper-local checker は current reason wording だけを accepted bridge とする
- docs / plan / fixture authoring guidance でも current exact names を mirror する
- alias、synonym、genericized family 名はまだ足さない

### 案B. helper-local `finding_kind` だけ先に generic 化する

reason wording は current wording を残したまま、
helper-local row family だけを、より generic に見える名前へ先に変更する。

例:

- `missing_fallback_body` -> `empty_fallback_body`
- `disallowed_fallback_placement` -> `atomic_cut_in_fallback_body`

この案では fixture-side row、helper mapping、spec mirror、plan mirror が先に動く。

### 案C. alias / synonym layer を先に入れる

current wording / row family は一旦残しつつ、
helper 側で複数 wording / 複数 `finding_kind` を accepted にする。

例:

- `missing_fallback_body` と `empty_fallback_body` を同義として扱う
- `disallowed_fallback_placement` と `atomic_cut_in_fallback_body` を同義として扱う

この案は rename を先送りできるが、helper が複数表現を受け入れることになる。

## 比較軸

### 1. current source-backed 契約との近さ

current repo で actualize 済みなのは、`e23` / `e24`、fixture-side expected row、
static gate wording、helper-local bridge の 4 点セットである。

この 4 点を最小変更で維持できる案ほど、current actual contract に近い。

### 2. hidden acceptance を増やさないか

current L2 では hidden acceptance を避ける必要がある。
alias / synonym を helper に入れると、later drift が生じても helper が通ってしまい、
wording / row family のズレを見逃しやすい。

### 3. shared carrier / public checker comparison を早く既成事実化しないか

current first tranche は helper-local dedicated family に留めている。
generic 化や alias を先に入れすぎると、
shared detached carrier や generic structural checker family へ寄せる判断を先食いしやすい。

### 4. fixture authoring guidance の素直さ

current authoring template では、first tranche の fixture を hand-edit で足すときに
current exact row family をそのまま書けることが利点である。
generic 名や alias が混じると、review でどれを書けばよいかが曖昧になりやすい。

## 比較結果

### 案A の利点

- current actual contract をそのまま保てる
- `e23` / `e24`、helper bridge、schema mirror、authoring template がそのまま整合する
- wording drift を helper が隠さない
- shared carrier / generic family / public checker API の比較と混ざらない

### 案A の欠点

- wording が later generic family 名として最適かどうかはまだ分からない
- `disallowed_fallback_placement` は first tranche の working family であり、長期 taxonomy と一致する保証はまだ無い

### 案B の利点

- 将来 generic family へ寄せやすそうに見える
- reason wording と helper-local family を概念上は分離できる

### 案B の欠点

- current actual contract を source-backed に広げる前に rename を先食いする
- `e23` / `e24`、helper bridge、schema mirror、plan mirror をまとめて変更する割に、新しい evidence は増えない
- second concrete family がまだ無い current state では、generic 名が requirement invent に寄りやすい

### 案C の利点

- later rename への移行は滑らかに見える
- docs と code のどちらか一方だけを先に動かせる

### 案C の欠点

- helper が複数表現を accepted にするので hidden acceptance に寄りやすい
- wording drift を compare helper が検出しにくくなる
- helper-local dedicated family を current phase より広く見せてしまう

## current judgment

current L2 の next narrow step としては、**案A を採る**のが自然である。

つまり current repo では、少なくとも次の 2 段までは
current exact wording / row family を fixed working set として扱う。

1. first-tranche wording / row family stability comparison
2. saved artifact compare need が shared carrier threshold を本当に満たすかの再比較

この current judgment では、次を固定する。

- `TryFallback` side の wording は `try fallback body must not be empty`
- `AtomicCut` side の wording は `atomic cut may not appear inside fallback body`
- helper-local row family は
  - `TryFallback` / `missing_fallback_body`
  - `AtomicCut` / `disallowed_fallback_placement`
- current helper / fixture / docs / plan では alias / synonym を足さない
- current helper は reason wording の drift を hidden acceptance で吸収しない

## 何をまだ固定しないか

current judgment は、これらをまだ固定しない。

- later generic structural checker family の final taxonomy
- `disallowed_fallback_placement` を long-term に維持するかどうか
- shared detached carrier に mirror したときの row naming
- public checker API comparison 時の exported field 名

これらは、少なくとも

- second concrete decode-valid family が source-backed に見える
- shared carrier threshold を再比較する
- generic structural checker family 合流 comparison に入る

のいずれかが起きた時点で reopen する。

## next narrow step

current wording / row family stability comparison の次段として自然なのは、
**saved artifact compare need が shared carrier threshold を本当に満たすか**
を、current first-tranche fixed working set のまま再比較することである。
