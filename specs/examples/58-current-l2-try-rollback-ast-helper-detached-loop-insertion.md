# 58 — current L2 try/rollback AST helper detached-loop insertion

## 目的

この文書は、current L2 parser-free PoC、detached validation loop、family checker support helper、
future dedicated AST structural helper の expected field / focused compare shape を前提に、
**future dedicated AST structural helper を detached validation loop のどこへ差し込むのが最小か**
を narrow に整理する。

ここで固定するのは actual helper 実装ではない。
固定するのは、

- bundle-first runtime loop
- static gate artifact loop
- generic checker-side shared entry

のどこへ future dedicated helper を最初に接続するのが current helper boundary に最も自然か、
という docs-only judgment だけである。

## current 前提

current repo では次が成立している。

- dedicated AST structural helper は future option であり、current phase では actualize しない
- compare contract は helper-local dedicated contract から始める
- optional expected field 名は `expected_static.checked_try_rollback_structural_findings` が最小候補である
- focused compare shape は `subject_kind` / `finding_kind` の helper-local row listに留める
- same-lineage / missing-option / capability の 3 checker spike は
  - static gate artifact emit
  - family facade script
  - detached loop wrapper の `smoke-*` subcommand
  という三層で current non-production loop に接続されている
- `TryFallback` / `AtomicCut` runtime representative としては `smoke-try-rollback-locality` が bundle-first path に既にある

したがって current 問いは、
**future dedicated AST structural helper を actualize するとしても、それを runtime representative の bundle path に混ぜるべきか、それとも static gate 側の helper-local smoke family に置くべきか**
である。

## 比較観点

1. AST-only structural floor と runtime representative contrast を混ぜないか
2. helper-local dedicated contract を shared detached carrier や public API へ premature に寄せないか
3. current detached validation loop の command surface を不必要に generic 化しないか
4. current static gate artifact path と family facade pattern を再利用できるか
5. malformed static family 未actualizeの current phase でも boundary を先に整理できるか

## 比較対象

### 案 1. bundle-first runtime loop へ差し込む

- `smoke-fixture`
- `smoke-try-rollback-locality`

の bundle artifact path へ future dedicated helper compare を混ぜる。

#### 利点

- current `TryFallback` / `AtomicCut` representative runtime pair と同じ command familyに見える
- detached loop の主線にすぐ載せられるように見える

#### 欠点

- dedicated AST helper は AST-only structural floor を読むべきであり、bundle/runtime artifact path は責務が違う
- `E2` / `E21` / `E22` runtime representative と malformed / structural helper compare を同じ command family に入れると、static/runtime boundary が濁る
- helper-local compare contract より bundle-first payload core の方が強く見え、future shared carrier や runtime event surface へ寄せる pressure が強い

### 案 2. static gate artifact loop に dedicated smoke subcommand を足す

- `emit-static-gate`
- future dedicated helper-local compare script
- `smoke-try-rollback-structural-helper`

のように、static gate artifact path の wrapper family へ差し込む。

#### 利点

- AST structural floor と runtime representative contrast を分けたまま loop に接続できる
- current same-lineage / missing-option / capability の checker spike と同じ helper-local progression に乗る
- static gate artifact emit、overwrite policy、artifact root、wrapper ergonomics を既存 detached loop から再利用できる
- shared detached carrier や public checker API を増やさずに済む

#### 欠点

- dedicated helper 実装が actualize されるまで、command 名だけ先に決めすぎる risk がある
- malformed static family actualization と command actualization の timing はまだ分けて考える必要がある

### 案 3. generic checker-side shared entry を先に足す

- たとえば `smoke-structural-checker --family try-rollback`
- あるいは `current_l2_family_checker_support.py` を generic 化する

#### 利点

- future generic structural checker family には広げやすい

#### 欠点

- current docs-only judgment では generic checker-side shared entry 自体をまだ切らない
- `TryFallback` / `AtomicCut` dedicated helper を generic CLI へ載せるのは、public-looking command surface を増やしすぎる
- malformed family actualization も generic structural checker family も未決の current phase では premature である

## current judgment

current L2 の next narrow step として最も自然なのは、
**案 2. static gate artifact loop に dedicated smoke subcommand を足す**
である。

ただしその意味は、
**いま actual command を追加する**
ということではない。

current docs-only judgment は次である。

1. future dedicated AST structural helper は runtime bundle path ではなく static gate path に接続するのが自然である
2. detached validation loop へ差し込むとしても、最初は helper-local dedicated smoke family に留める
3. `smoke-fixture` / `smoke-try-rollback-locality` の bundle-first runtime path へは混ぜない
4. generic checker-side shared entry は still OPEN であり、later public checker API comparison と同時に扱う

## minimal future call shape

future actualization を narrow に書くなら、最小 call shape は次で十分である。

```text
scripts/current_l2_detached_loop.py smoke-try-rollback-structural-helper
  -> emit-static-gate 相当で static gate artifact を保存
  -> future dedicated helper-local compare script を呼ぶ
  -> fixture-side optional expected field を fail-closed compare する
```

ここで helper-local compare script は、少なくとも conceptually 次の入力だけを読めばよい。

- fixture path
- emitted static gate artifact path
- optional `expected_static.checked_try_rollback_structural_findings`

ここでは bundle artifact path、aggregate path、runtime event 列を読む必要はない。

## current cut

この task では次を行わない。

- dedicated AST structural helper の actual code 追加
- `scripts/current_l2_detached_loop.py` への actual subcommand 追加
- `smoke-static-gate` の generic option 化
- bundle-first runtime wrapper への helper compare 混入
- generic checker-side shared entry の actualization

## current guidance

current helper stack と fixture authoring では、次を守る。

1. `TryFallback` / `AtomicCut` runtime representative は引き続き `smoke-try-rollback-locality` で bundle-first に扱う
2. future dedicated AST helper を loop へ載せるとしても、第一候補は static gate artifact loop である
3. helper actualization 前に fixture schema や detached artifact shared carrier を広げない
4. command surface は family-specific / helper-local に留め、generic checker-side shared entry へ急がない

## next narrow step

current docs-only judgment の次段として自然なのは、
**future dedicated AST structural helper の structural verdict carrier / name をどこまで narrow に切るか**
を比較することである。

## OPEN に残すもの

- dedicated helper の structural verdict carrier / name
- actual subcommand 名をいつ決めるか
- malformed static family を actual corpus に増やす必要が本当にあるか
- detached artifact shared carrier へ上げる閾値
