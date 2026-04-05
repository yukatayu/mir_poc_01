# 67 — current L2 try/rollback malformed pattern slot selection

## 目的

この文書は、current L2 parser-free PoC、future dedicated AST structural helper の
first tranche cut、malformed static family timing、malformed static tranche size を前提に、
**two-fixture first tranche の `TryFallback` slot と `AtomicCut` slot に最初に入れる malformed pattern をどこまで narrow に切るか**
を整理する。

ここで固定するのは actual helper 実装ではない。
固定するのは、

- `TryFallback` slot の first malformed pattern
- `AtomicCut` slot の first malformed pattern
- first tranche working assumption として何を選び、何を後段へ残すか

という current docs-only judgment だけである。

## current 前提

current repo では次が source-backed に揃っている。

- malformed static family の first tranche は `TryFallback` 1 件 + `AtomicCut` 1 件の two-fixture pair を最小とする
- structural malformed source は loader ではなく static gate / dedicated AST structural helper 側に置く
- `TryFallback` / `AtomicCut` runtime representative `E2` / `E21` / `E22` は current evidence として維持する
- future dedicated AST structural helper の first tranche は
  - helper code
  - fixture-side fields
  - minimal malformed static family
  - static gate smoke path
  を一体で切る
- current typed fixture schema では `TryFallback` は `body` / `fallback_body` を持つ statement node、`AtomicCut` は ordinary statement node として decode される

したがって current 問いは、
**first tranche の各 slot に割り当てる pattern が、current schema で decode 可能で、loader/source placement judgement と衝突せず、runtime representative とも競合しない最小 pair になっているか**
である。

## 比較観点

1. current typed fixture schema で decode 可能か
2. loader/source placement judgement と衝突しないか
3. runtime representative `E2` / `E21` / `E22` を static malformed で上書きしないか
4. dedicated helper-local compare を first tranche で fail-closed に観察できるか
5. malformed wording / finding kind を premature に広げすぎないか

## `TryFallback` slot の比較

### 案 1. empty `fallback_body` を `missing_fallback_body` と読む

具体的には、AST としては

- `TryFallback`
- `body` は decode できる
- `fallback_body = []`

を許し、future dedicated AST structural helper が
`missing_fallback_body` finding を返す working assumption である。

#### 利点

- current typed fixture schema で decode できる
- loader / carrier malformed に落ちない
- `TryFallback` structural floor を helper-local dedicated contract に寄せやすい
- finding kind は既存 docs example とも整合する

#### 欠点

- `missing` という語は field absence ではなく semantic floor violation を指す、と prose で明示する必要がある

### 案 2. field absence そのものを `missing_fallback_body` と読む

#### 利点

- wording は直感的である

#### 欠点

- current typed fixture schema では loader / decode failure になり、semantic structural malformed ではなく carrier/schema malformed になる
- structural malformed source placement judgement と衝突する

### 案 3. `TryFallback` slot は今回は保留する

#### 利点

- wording invent をさらに避けられる

#### 欠点

- two-fixture first tranche を current docs-only で actualization-ready に近づける価値を失う
- `TryFallback` side の working assumption が残らず、first tranche へ進みにくい

## current judgment: `TryFallback` slot

current L2 の next narrow step として最も自然なのは、
**案 1. empty `fallback_body` を `missing_fallback_body` と読む**
である。

理由は次の通り。

1. current typed fixture schema で decode できる
2. field absence を loader error に押し戻さず、semantic structural malformed を static helper 側に置ける
3. `missing_fallback_body` は current docs example にも既に出ており、finding kind family を増やしすぎない

## `AtomicCut` slot の比較

### 案 A. nested place placement を `disallowed_nesting` と読む

#### 利点

- `subject_kind = AtomicCut` の dedicated finding を作りやすい

#### 欠点

- runtime representative `E22` が current phase では **valid** かつ event-only contrast として固定済みである
- nested place mismatch を static malformed にすると、runtime / proof boundary に残した dynamic gate の読みと衝突する

### 案 B. non-statement shape を first slot にする

#### 利点

- `AtomicCut` structural floor の raw shape には触れられる

#### 欠点

- current typed fixture schema では loader / decode malformed に寄りやすく、semantic structural malformed source placement judgement と衝突する
- dedicated AST structural helper の first tranche より loader malformed に近い

### 案 C. `fallback_body` placement を `disallowed_fallback_placement` と読む

具体的には、

- AST としては ordinary `AtomicCut` statement として decode できる
- `TryFallback.fallback_body` 内に置いた場合だけ、future dedicated AST structural helper が
  `disallowed_fallback_placement` finding を返す working assumption

と読む。

#### 利点

- current typed fixture schema で decode できる
- loader/source placement judgement と衝突しない
- current runtime representative `E21` / `E22` と競合しない
- `AtomicCut` slot を dedicated helper-local placement rule として narrow に切れる

#### 欠点

- current runtime semantics はこの placement 自体をまだ静的 malformed としては扱っていないため、first tranche working assumption であることを明記する必要がある

### 案 D. `AtomicCut` slot は未決のまま actualization を止める

#### 利点

- wording invent を最小化できる

#### 欠点

- two-fixture first tranche を current docs-only で आगे actualization-ready に近づけられない
- first tranche actual helper 着手の concrete write set が曖昧に残る

## current judgment: `AtomicCut` slot

current L2 の next narrow step として最も自然なのは、
**案 C. `fallback_body` placement を `disallowed_fallback_placement` と読む**
である。

理由は次の通り。

1. current typed fixture schema で decode できる
2. raw schema / non-statement shape を loader 側へ残す current malformed source placement と整合する
3. nested place mismatch は `E22` の runtime-valid representative と衝突するため、first tranche には使えない
4. `TryFallback` body 内 frontier update / event-only mismatch の runtime evidence を壊さずに、`AtomicCut` side の dedicated placement rule を narrow に切れる

## first tranche working pair

current docs-only minimum として、two-fixture first tranche の working pair は次と読むのが自然である。

1. `TryFallback` slot
   - pattern: empty `fallback_body`
   - finding kind: `missing_fallback_body`
2. `AtomicCut` slot
   - pattern: `fallback_body` placement
   - finding kind: `disallowed_fallback_placement`

ここで重要なのは、これは **future dedicated AST structural helper の first tranche working assumption** であり、

- final parser malformed taxonomy
- loader malformed taxonomy
- generic structural checker family
- detached shared carrier

をまだ固定しないことである。

## first tranche から外に残すもの

次は current phase の first tranche から外に残す。

- nested place `AtomicCut` mismatch
  - runtime representative `E22` として維持する
- non-statement / raw schema malformed
  - loader / decode malformed に残す
- additional `TryFallback` malformed variants
  - later tranche に残す
- additional `AtomicCut` placement variants
  - later tranche に残す
- exact human-facing wording の長文化
  - helper-local compare と smoke を回してから詰める

## current guidance

current helper stack と fixture authoring template では、次を守る。

1. `TryFallback` slot は field absence ではなく、decode 後の empty `fallback_body` を semantic structural malformed として扱う
2. `AtomicCut` slot は nested place mismatch を使わず、first tranche working assumption として `fallback_body` placement を使う
3. non-statement / raw schema malformed は引き続き loader side に残す
4. `disallowed_fallback_placement` は current first tranche の working assumption であり、later generic structural family まで先回りしない

## current cut

この task では次を行わない。

- dedicated AST structural helper の actual実装
- malformed static fixture の actual追加
- fixture schema actual field 追加
- detached artifact shared carrier actualization
- public checker API comparison actualization

## next narrow step

current docs-only judgment の次段として自然なのは、
**first tranche working pair を前提に、fixture-side expected wording / `finding_kind` をどこまで narrow に固定して actual helper 実装へ進めてよいか**
を比較することである。

## OPEN に残すもの

- `missing_fallback_body` の exact prose wording を empty list case にどう揃えるか
- `disallowed_fallback_placement` を later generic structural family でどう generalize するか
- dedicated helper actualization task の exact wrapper / subcommand 名
- shared carrier / public checker API へ上げる threshold の再確認
