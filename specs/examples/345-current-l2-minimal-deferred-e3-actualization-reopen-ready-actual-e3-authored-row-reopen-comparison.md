# 345 — current L2 minimal-deferred-`e3`-actualization-reopen-ready actual-`e3`-authored-row reopen comparison

## 目的

`specs/examples/344-current-l2-deferred-e3-actualization-reopen-timing-ready-minimal-deferred-e3-actualization-reopen-threshold.md`
で deferred `e3` actualization reopen timing の minimum を fixed した次段として、

- `e3-option-admit-chain` をどこまで source-authored row として actualize するか
- current theorem-side consumer と current formal-hook top をどこまで保ったまま widen するか
- proof / model-check first concrete tool pilot と second source-sample cluster sequencing をどこまで後段に残すか

を比較する。

ここで固定するのは
**current L2 minimal-deferred-`e3`-actualization-reopen-ready actual-`e3`-authored-row reopen comparison**
であり、

- new formal-hook family
- proof / model-check first concrete tool pilot
- second source-sample cluster sequencing
- public surface inventory

はまだ固定しない。

## scope

- entry criteria は `specs/examples/337...344` で fixed 済みの `e3` guard comparison、plain / compare-ready bridge sketch、deferred `e3` reopen timing に置く。
- actualize するのは source row / runner / authored inventory / verification ladder までである。
- current theorem-side consumer `proof_notebook_review_unit` と current formal-hook top `runtime_try_cut_cluster` は保つ。

## current 前提

current repo では次が成立している。

1. `e3-option-admit-chain` は fixture / host-plan / mapping matrix target を already 持つ。
2. current lowerer first cut は inline `admit` fragment を受ける。
3. current sample runner first cut は accepted sample set 内の explicit path / shorthand を受ける。
4. current formal-hook support は runtime detached artifact の top を `runtime_try_cut_cluster` に保っており、`rollback` / `atomic-cut` を持たない `e3` runtime bundle は reject する。
5. current theorem-side concrete pilot は tool-neutral formal hook を入力にする row-local `proof_notebook_review_unit` に留める。

したがって current 問いは、
**`e3` を source-authored row / runner / inventory / ladder へ narrow actualize しつつ、formal hook は `not reached (guarded)` に留めるのが自然か**
である。

## 比較観点

1. source-authored row widening と formal-hook family widening を分離できるか
2. current theorem-side consumer / formal-hook top を壊さないか
3. regression helper と README ladder に machine-check 可能な row を残せるか
4. next line を proof / model-check first concrete tool pilot に素直に handoff できるか

## 比較対象

### 案 1. `e3` を source-authored row へ actualize し、formal hook は `not reached (guarded)` に留める

#### 読み

- `samples/current-l2/e3-option-admit-chain.txt` を actual source row として置く
- current runner accepted set と regression inventory に `e3` を入れる
- verification ladder では `static gate = reached(valid)`、`interpreter = reached(success)`、`formal hook = not reached (guarded)` とする
- current theorem-side consumer と current formal-hook top は変えない

#### 利点

- source-sample widening と formal-hook family widening を分離できる。
- current guard comparison と deferred reopen timing の判断を lossless に受け継げる。
- next line を proof / model-check first concrete tool pilot に素直に進められる。

#### 欠点

- inventory / README / progress mirror で guarded non-reached wording を揃える必要がある。

### 案 2. `e3` を source-authored row と同時に new formal-hook family へ上げる

#### 利点

- `e3` runtime bundle まで formal hook reached row にできる。

#### 欠点

- current formal-hook top widening を premature に混ぜる。
- theorem-side current consumer との切り分けが弱くなる。

### 案 3. `e3` は source-target-only のまま残し、concrete tool pilot を先に reopen する

#### 利点

- theorem/model-check side pressureを先に見られる。

#### 欠点

- deferred `e3` reopen timing を fixed した直後の next line と矛盾する。
- current executable subset widening が不必要に遅れる。

## current judgment

current L2 で最も自然なのは、
**案 1. `e3` を source-authored row へ actualize し、formal hook は `not reached (guarded)` に留める**
である。

理由は次の通り。

1. `specs/examples/343...344` の reopen timing judgmentを素直に受け継ぐ。
2. current theorem-side consumer / formal-hook top を保ったまま source-backed executable subset を厚くできる。
3. proof / model-check first concrete tool pilot を次 package に押し分けやすい。

## current first choice details

- `e3-option-admit-chain` は current authored source sample に昇格させる。
- current lowerer / runner / regression inventory / verification ladder までを package scope に含める。
- current ladder row は `static gate = reached(valid)`、`interpreter = reached(success)`、`formal hook = not reached (guarded)` とする。
- `admit-miss` は explicit failure ではなく non-admissible skip として trace metadata に残す。
- proof / model-check first concrete tool pilot、新 formal-hook family、second source-sample cluster sequencingは still later に残す。

## next promoted line

next promoted line は、
**actual-`e3`-authored-row-reopen-ready proof-model-check-first-concrete-tool-pilot**
に置く。

## open questions

- proof notebook current cut の後で model-check side をどう narrow reopen するか
- `e3` guarded non-reached row を concrete tool pilot の evidence policyへどこまで渡すか
- second source-sample cluster sequencing を theorem/model-check pilot の後でどの family に置くか
