# 63 — current L2 try/rollback AST helper public checker entry criteria

## 目的

この文書は、current L2 parser-free PoC、future dedicated AST structural helper の
generic family boundary judgement を前提に、
**later public checker API comparison に future dedicated AST structural helper family を載せる entry criteria をどこまで narrow に切るか**
を整理する。

ここで固定するのは actual public checker API ではない。
固定するのは、

- dedicated helper family を later public checker API comparison に載せてよい条件
- generic family 合流 threshold と public checker API comparison threshold の違い
- current state でまだ足りていないもの

という docs-only judgment だけである。

## current 前提

current repo では次が成立している。

- same-lineage / missing-option / capability の 3 checker spike は helper-local spike と family facade script を current command surface に保っている
- generic checker-side shared family entry は still OPEN であり、later public checker API comparison と同時に扱う
- future dedicated AST structural helper は
  - static gate artifact loop の family-specific wrapper に留める
  - shared detached carrier へは threshold 充足後にだけ上げる
  - generic structural checker family 合流も later public checker API comparison と同時に扱う
- dedicated AST structural helper 自体は current state ではまだ actualize していない

したがって current 問いは、
**future dedicated AST structural helper family を later public checker API comparison に載せ始めるには、generic family 合流 threshold 以外に何を揃えるべきか**
である。

## 比較観点

1. current family facade pattern と衝突しないか
2. helper-local dedicated contract と public-looking checker API を premature に混ぜないか
3. parser / verifier / host boundary の未決事項を public checker API comparison に持ち込みすぎないか
4. dedicated helper family を later public table に載せる実利があるか
5. actual helper / corpus / compare loop の evidence を欠いたまま public cut を始めないか

## 比較対象

### 案 1. generic family 合流 threshold が揃ったら、そのまま public checker API comparison に載せる

#### 利点

- threshold の段数が少ない
- generic family 合流と public checker comparison を一気に進められる

#### 欠点

- dedicated helper family の internal stabilization と public table への載せ替えを同時にやることになる
- API naming、return shape、verifier handoff まで一緒に前倒ししやすい

### 案 2. generic family 合流 threshold に加えて、public checker comparison 専用の entry criteria を別に要求する

ここでいう additional entry criteria は、少なくとも次を指す。

1. dedicated AST structural helper が actual helper-local 実装として存在する
2. fixture-side fields
   - `checked_try_rollback_structural_verdict`
   - `checked_try_rollback_structural_findings`
   が actual compare contract として安定している
3. static gate artifact loop の family-specific wrapper で複数回の authoring / compare 反復が済んでいる
4. malformed static family か同等の AST-only structural corpus が actual fixtures として揃っている
5. same-lineage / missing-option / capability と並べた public comparison table に載せる具体的理由
   - command surface 整理
   - verifier handoff boundary
   - shared output contract
   のいずれかが source-backed に見えている

#### 利点

- generic family 合流と public checker API comparison を別段に分けられる
- public-looking checker API を evidence-backed に始めやすい
- existing 3 family checker spike の progression と整合する

#### 欠点

- threshold が一段増える

### 案 3. final parser cut や theorem prover boundary が固まるまで public checker API comparison 自体を始めない

#### 利点

- public checker API をかなり遅くまで凍結できる

#### 欠点

- checker-side public cut の比較が必要になった時に、dedicated helper family だけ準備不足になりやすい
- parser / proof boundary を待ちすぎて、checker-side ergonomics comparison が止まりやすい

## current judgment

current L2 の next narrow step として最も自然なのは、
**案 2. generic family 合流 threshold に加えて、public checker comparison 専用の entry criteria を別に要求する**
である。

理由は次の通り。

1. generic family 合流は abstraction boundary の問いであり、public checker API comparison は command surface / output contract / verifier handoff の問いである
2. 既存 3 family checker spike でも、helper-local smoke と family facade を積んだ後に public-looking generic entry を later comparison へ送っている
3. dedicated AST structural helper family も、actual helper 実装、fixture-side field actualization、static corpus、loop stabilizationが揃う前に public checker table へ載せるべきではない
4. ただし final parser cut や theorem prover boundary の完全確定まで待つ必要はなく、checker-side evidence が揃えば docs-only public comparison は先に始めてよい

## minimum public-checker entry criteria

later public checker API comparison に dedicated AST structural helper family を載せる minimum は次である。

1. **helper actualization**
   - dedicated helper-local 実装が存在する
2. **fixture contract stabilization**
   - `checked_try_rollback_structural_verdict`
   - `checked_try_rollback_structural_findings`
   が actual compare contract として運用されている
3. **corpus stabilization**
   - malformed static family か同等の AST-only structural corpus が actual fixtures として複数件ある
4. **loop stabilization**
   - static gate artifact loop の family-specific wrapper で複数回の実地反復が済んでいる
5. **public comparison pressure**
   - existing 3 family と並べた shared table、shared output contract、external verifier handoff、command surface整理のいずれかに具体的 pressure がある

この 5 条件が揃うまでは、later public checker API comparison に載せず、dedicated family の docs-only judgment と helper-local progressionに留めるのが自然である。

## current guidance

current helper stack と roadmap では、次を守る。

1. generic family 合流 threshold と public checker API comparison threshold を別に扱う
2. actual helper / corpus / loop stabilizationが無い段階で public checker API comparison を始めない
3. shared detached carrier actualization は public checker API comparison の前提にしない
4. parser finalization や theorem prover boundary の完全確定を待つことも current minimum には含めない

## current cut

この task では次を行わない。

- dedicated AST structural helper の actual実装
- public checker API actualization
- generic checker-side shared entry actualization
- parser / verifier boundary の finalization

## next narrow step

current docs-only judgment の次段として自然なのは、
**future dedicated AST structural helper の malformed static family を actual corpus に増やす timing をどこに置くか**
を比較することである。

## OPEN に残すもの

- malformed static family を actual corpus に増やす timing
- actual helper 実装開始時の exact subcommand 名
- public checker API comparison の shared output contract をどの family まで共通化するか
