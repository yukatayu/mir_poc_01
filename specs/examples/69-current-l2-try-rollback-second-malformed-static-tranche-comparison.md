# 69 — current L2 try/rollback second malformed static tranche comparison

## 目的

この文書は、current L2 parser-free PoC と `specs/examples/65`〜`68` の first-tranche chain を前提に、
**dedicated `TryFallback` / `AtomicCut` AST structural helper の second malformed static tranche を今すぐ足すべきか**
を narrow に比較する。

ここで固定するのは second tranche の actual fixture 追加ではない。
固定するのは、

- current repo が second tranche comparison を先に行うべきか
- second tranche actualization を今すぐ始めてよいだけの候補 family が既に source-backed か
- それとも current phase では wording / finding family stability comparison へ進むべきか

という current docs-only judgment である。

## current 前提

current repo では次が成立している。

- dedicated helper first tranche は actualize 済みである
  - `e23`
  - `e24`
  - `checked_try_rollback_structural_verdict`
  - `checked_try_rollback_structural_findings`
  - `smoke-try-rollback-structural-checker`
- first tranche working pair は次である
  - `TryFallback` slot
    - empty `fallback_body`
    - `missing_fallback_body`
  - `AtomicCut` slot
    - `fallback_body` placement
    - `disallowed_fallback_placement`
- malformed source placement では、次は dedicated helper 側ではなく loader / decode 側に残す
  - required field absence
  - raw schema / non-statement malformed
- runtime representative `E21` / `E22` は current evidence として維持し、
  nested place `AtomicCut` mismatch は runtime-valid contrast として残す
- near-term sequencing では、first tranche actualization の次に
  second malformed static tranche comparison
  を置くのが current plan である

したがって current 問いは、
**second tranche comparison を今ここで閉じるとして、その結果は actual tranche 追加になるのか、それとも candidate inventory 不足を確認して wording stability へ進むのか**
である。

## 比較観点

1. current specs / reports / code に decode-valid な second-tranche candidate が明示されているか
2. loader malformed / runtime-valid representative と衝突しないか
3. first tranche の helper-local compare contract を不用意に広げないか
4. generic family / shared carrier / public checker API を premature に巻き込まないか
5. next self-drivable step を曖昧にせず、plan sequencing を前へ進められるか

## 比較対象

### 案 1. second tranche を今すぐ actualize する

ここでは、first tranche の次として additional `TryFallback` malformed variant と
additional `AtomicCut` placement variant を current actual corpus へ足す。

#### 利点

- helper-local evidence は増える
- later wording comparison を additional examples つきで始められる

#### 欠点

- current repo では、second tranche 候補として source-backed な decode-valid family がまだ具体化されていない
- `field absence` は loader malformed に残る
- nested place `AtomicCut` mismatch は `E22` runtime-valid contrast と衝突する
- source-backed でない variant を今ここで actualize すると、current phase では要件 invent に寄りやすい

### 案 2. second tranche comparison は今ここで閉じるが、actual tranche 追加はまだ行わない

ここでは、次を current judgment とする。

1. second tranche comparison は first tranche の後で先に行う
2. ただし current specs / reports / code だけでは、actualize-ready な second-tranche pair はまだ不足している
3. したがって next self-drivable step は wording / finding family stability comparison である

#### 利点

- current plan sequencing を前へ進めつつ、要件 invent を避けられる
- first tranche pair と helper-local compare contract を stable baseline にできる
- runtime-valid representative / loader malformed source placement との衝突を避けられる

#### 欠点

- helper-local evidence の件数自体はまだ増えない

### 案 3. second tranche comparison 自体を飛ばして、先に wording stability へ進む

#### 利点

- 近い実務論点だけを見るなら速い

#### 欠点

- near-term sequencing とずれる
- second tranche を later に送る理由が source-backed に整理されない
- actual tranche を先送りした理由が曖昧に残る

## candidate inventory の current state

current repo で source-backed に確認できるのは次である。

### dedicated helper 側で明示済みのもの

- `missing_fallback_body`
- `disallowed_fallback_placement`

### second tranche 候補として明示されるが、まだ具体 family が source-backed でないもの

- additional `TryFallback` malformed variants
- additional `AtomicCut` placement variants

### current phase で second tranche 候補から外すもの

- `field absence`
  - loader / decode malformed に残す
- raw schema / non-statement malformed
  - loader / decode malformed に残す
- nested place `AtomicCut` mismatch
  - `E22` runtime-valid contrast として残す

したがって current state では、
**second tranche を actualize するための concrete decode-valid family が docs chain 上まだ足りていない**
と読むのが自然である。

## current judgment

current L2 の next narrow step として最も自然なのは、
**案 2. second tranche comparison は今ここで閉じるが、actual tranche 追加はまだ行わない**
である。

理由は次の通り。

1. near-term sequencing 上、second tranche comparison 自体は先に済ませる価値がある
2. しかし current source だけでは、first tranche を越える decode-valid second-tranche family がまだ具体化されていない
3. `field absence` と raw schema malformed は loader 側に残す current judgment と衝突する
4. nested place `AtomicCut` mismatch は `E22` runtime-valid contrast と衝突する
5. したがって今すぐ second tranche を actualize すると、要件 invent の危険が高い

## current sequencing refinement

この文書により、current near-term sequencing は次と読む。

1. first tranche actualization
2. second tranche comparison
3. wording / finding family stability comparison
4. shared carrier threshold 再比較

ここで second tranche comparison の結論は、
**現時点では actual tranche を増やさず、wording / finding family stability へ進む**
である。

## current cut

この task では次を行わない。

- second tranche fixture の actual追加
- new `finding_kind` の actual追加
- static gate wording の拡張
- shared carrier / generic family / public checker API comparison

## next narrow step

current docs-only judgment の次段として自然なのは、
**first tranche の `missing_fallback_body` / `disallowed_fallback_placement` wording と helper-local row family が、数回の反復を経ても drift しにくいか**
を narrow に比較することである。

## OPEN に残すもの

- second tranche の concrete decode-valid family
- `missing_fallback_body` の長期 wording 固定
- `disallowed_fallback_placement` の長期 wording 固定
- second tranche を later generic family へどう接続するか
