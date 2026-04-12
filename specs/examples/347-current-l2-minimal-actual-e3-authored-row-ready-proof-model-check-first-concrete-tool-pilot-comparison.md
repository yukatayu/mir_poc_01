# 347 — current L2 minimal-actual-`e3`-authored-row-ready proof-model-check-first-concrete-tool-pilot comparison

## 目的

`specs/examples/346-current-l2-actual-e3-authored-row-reopen-ready-minimal-actual-e3-authored-row-threshold.md`
で actual `e3` authored-row actualization の minimum を fixed した次段として、

- proof notebook side と model-check side のどちらを first concrete pilot に置くか
- current row-local review-unit code path をどこまで current first choice として保つか
- second source-sample cluster sequencing と public checker migration をどこまで後段に残すか

を比較する。

ここで固定するのは
**current L2 minimal-actual-`e3`-authored-row-ready proof-model-check-first-concrete-tool-pilot comparison**
であり、

- second source-sample cluster sequencing
- model-check concrete pilot
- public checker migration
- bless / review-session metadata

はまだ固定しない。

## scope

- entry criteria は `specs/examples/327...328` の theorem-first concrete tool pilot cut と `specs/examples/339...346` の theorem-side bridge / `e3` guarded actualization line に置く。
- current package は concrete carrier の順序づけを扱う docs-only package である。
- existing proof notebook review-unit helper / tests を巻き戻さない。

## current 前提

current repo では次が成立している。

1. tool-neutral formal hook artifact は current code path にある。
2. `proof_notebook_review_unit` helper / example / focused tests も current code path にある。
3. `e3` は source-authored row まで actualize 済みだが、formal hook stage は guarded に留める。
4. model-check side や public checker migration は current code path にまだ actualize していない。

したがって current 問いは、
**first concrete tool pilot を current proof-notebook review-unit cut に置き、model-check side は second reserve に残すのが自然か**
である。

## 比較観点

1. current row-local review-unit cut をそのまま first concrete carrier として読めるか
2. `e3` guarded non-reached row を premature な formal-hook wideningへ押し込まないか
3. second source-sample cluster sequencing と public checker migration を後段に分離できるか

## 比較対象

### 案 1. proof notebook review-unit を first concrete pilot にし、model-check side は second reserve に残す

#### 利点

- current helper / tests / report line と連続的である。
- theorem-side bridge の human-facing walkthrough pressure を narrow に保てる。
- `e3` guarded row を machine-facing schema wideningに巻き込まない。

#### 欠点

- model-check side の concrete carrier はなお後段で詰める必要がある。

### 案 2. model-check side を first concrete pilot にする

#### 利点

- machine-facing consumer pressure を先に見られる。

#### 欠点

- current proof-notebook helper line を飛び越える。
- public checker / solver-specific schema を premature に要求しやすい。

### 案 3. proof notebook と model-check を dual-first pilot にする

#### 利点

- human-facing / machine-facing pressure を同時に見られる。

#### 欠点

- current package が太りすぎる。
- `e3` guarded row と second source-sample cluster sequencing の切り分けが弱くなる。

## current judgment

current L2 で最も自然なのは、
**案 1. proof notebook review-unit を first concrete pilot にし、model-check side は second reserve に残す**
である。

理由は次の通り。

1. current code path で actualize 済みの concrete carrier は proof notebook review unit である。
2. `e3` actualization直後の package としては、formal-hook family wideningや solver-specific schema導入を避けるのが自然である。
3. second source-sample cluster sequencing を next mainline に押し分けやすい。

## current first choice details

- first concrete pilot は tool-neutral formal hook を入力にする row-local `proof_notebook_review_unit` に留める。
- current code anchor は helper / example / focused tests で十分であり、新しい code path はこの package では要求しない。
- model-check side と public checker migration は second reserve / later reopen に残す。

## next promoted line

next promoted line は、
**proof-model-check-first-concrete-tool-pilot-ready second-source-sample-cluster-sequencing**
に置く。

## open questions

- second source-sample cluster を `e22` contrast、expiry family、static malformed familyのどれから reopen するか
- proof notebook の後で model-check side をどの abstraction から開くか
- public checker migration を source-sample widening とどこで切り分けるか
