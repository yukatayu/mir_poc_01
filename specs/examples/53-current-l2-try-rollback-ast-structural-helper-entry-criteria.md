# 53 — current L2 try/rollback AST structural helper entry criteria

## 目的

この文書は、current L2 parser-free PoC、first parser cut inventory、first checker cut inventory、
`TryFallback` / `AtomicCut` の runtime representative を前提に、
**`try` / rollback locality を dedicated AST structural helper として actualize してよい最小 entry criteria**
を narrow に整理する。

ここで固定するのは final checker API ではない。
固定するのは、

- どの条件が揃えば docs/runtime representative から helper-local actualization に進めるか
- その helper が何を読んでよく、何を読んではいけないか
- 既存 reason-row family helper とどこで分けるべきか

という current docs-only judgment だけである。

## current 前提

current repo では次が source-backed に揃っている。

- `specs/examples/29-current-l2-first-parser-subset-inventory.md` により、`try { ... } fallback { ... }` と `atomic_cut` は first parser cut 候補 cluster に入れてよい
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md` により、`TryFallback` / `AtomicCut` の **structural floor** は first checker cut 候補 cluster に入れてよい
- `specs/examples/51-current-l2-try-rollback-structural-floor-and-restore-scope.md` により、`place_anchor == current_place` gate と restore scope の exact shape は runtime / proof boundary に残す
- `specs/examples/52-current-l2-try-rollback-fourth-checker-spike-comparison.md` により、current phase では existing reason-row family helper の fourth spike に actualize しない
- representative runtime evidence として `E2` / `E21` / `E22` がある

したがって current 問いは、
**existing reason-row family へは載せないと決めた `try` / rollback locality を、どの条件が揃った時点で dedicated AST structural helper に actualize してよいか**
である。

## 比較観点

1. parser boundary / checker boundary / runtime boundary を混ぜないか
2. dynamic gate や restore scope を hidden に helper へ持ち込まないか
3. existing reason-row family と責務競合しないか
4. malformed static family と runtime representative の役割分担が明確か
5. theorem prover / model checker に残すべき global property を premature に checker helper へ落とさないか

## 比較対象

### 案 1. parser inventory があるので、runtime valid fixture だけで今すぐ dedicated AST helper を切る

- `E2` / `E21` / `E22` だけを入力にして helper-local structural check を actualize する

#### 利点

- runtime representative と並走して executable helper を早く持てる
- reason-row family に新 kind を足さずに済む

#### 欠点

- malformed static family がまだ無く、helper が何を reject / accept するかの cut が曖昧なままになる
- parser/loader malformed source をどこに置くか未決のまま helper shape だけ先行する
- current docs が runtime representative として扱っている `E2` / `E21` / `E22` を、そのまま checker helper family の evidence へ昇格させると static/runtime の境界を誤読しやすい

### 案 2. parser/loader source、AST-only floor、malformed family の最小核が揃ってから dedicated helper を切る

- helper は AST structural floor だけを読む
- parser/loader malformed source を明示する
- runtime gate / restore scope は読まない
- runtime representative と malformed static family を分ける

#### 利点

- parser boundary / checker boundary / runtime boundary を最もきれいに分離できる
- existing reason-row family と競合しない
- dynamic gate と structural floor を混ぜずに済む
- theorem prover / model checker に残すべき global property を premature に落とし込まない

#### 欠点

- 現時点ではまだ entry criteria comparison に留まり、helper 実装は次段になる
- malformed static family を本当に増やすかの追加判断が必要になる

### 案 3. current phase では docs/runtime representative に留め、entry criteria 自体も切らない

#### 利点

- current split を最も保守的に維持できる

#### 欠点

- future actualization を切る基準が残らず、next narrow step が曖昧なままになる
- parser boundary と first checker cut inventory が見えてきた current phase では、比較判断を 1 段具体化する価値を捨てることになる

## dedicated AST structural helper に進める最小 entry criteria

current judgment として、actualization に進める最小 entry criteria は次である。

### 1. malformed source の位置が explicit であること

少なくとも次のどちらかを先に決める必要がある。

- parser / loader が structural malformed を reject するのか
- parser-free fixture / static gate 側で malformed AST family として受けるのか

ここが未決のまま helper だけを切ると、
「これは parser error なのか」「loader error なのか」「checker error なのか」
が drift source になる。

### 2. helper 入力が AST structural floor に限定されていること

helper が読んでよいのは、少なくとも次までに限る。

- `TryFallback` region shape
- `fallback_body` の有無
- `AtomicCut` が statement node として現れていること
- allowed nesting の最小 structural rule

helper がここから先、

- runtime event 列
- `place_anchor == current_place`
- restore 後 store shape

を読むようになると、dedicated AST structural helper ではなく runtime helper になる。

### 3. helper 出力が reason-row family と別 carrier であること

current dedicated helper は、少なくとも次をしてはいけない。

- `expected_static.checked_reason_codes` family を流用する
- detached static gate artifact `reason_codes` row family に無理に合わせる

理由は、`try` / rollback locality の current docs-only floor が
**same-lineage / missing-option / capability と同じ static reason family としてはまだ source-backed に切れていない**
からである。

したがって dedicated helper を actualize するなら、
row-family compare とは別の structural helper contract にするのが最小である。

### 4. dynamic gate と restore scope を helper の non-goal として明示すること

helper は、次を proof/runtime boundary に残す non-goal でなければならない。

- `place_anchor == current_place` の dynamic gate
- frontier update success / failure
- restore scope の exact shape
- rollback / cut non-interference の一般証明

これを明示しないと、`specs/examples/51` で残した split を helper が壊す。

### 5. runtime representative とは別に、最低 2 例以上の structural family を持つこと

current dedicated helper を actualize するなら、singleton ではなく少なくとも family を持つ方が自然である。
最小でも、

- malformed `TryFallback` shape
- malformed `AtomicCut` placement / statement shape

のように、複数例で compare できる必要がある。

これが無いと、helper が `E2` / `E21` / `E22` という runtime valid fixture の読み替えに留まり、
structural checker helper としての cut が弱い。

## current judgment

current L2 の next narrow step として最も自然なのは、
**案 2. parser/loader source、AST-only floor、malformed family の最小核が揃ってから dedicated helper を切る**
である。

理由は次の通り。

1. parser boundary と first checker cut inventory は見えているが、malformed source と helper contract はまだ actualize されていない
2. `TryFallback` / `AtomicCut` は current では structural floor と dynamic gate を明示的に分けて読むべきであり、helper が runtime gate を読んではならない
3. existing reason-row family helper へ無理に寄せるより、future dedicated AST structural helper として比較し直す方が current docs と整合する
4. entry criteria を docs-only で先に固定すれば、future actualization で premature な helper 実装を避けやすい

## current cut

この task では次を行わない。

- dedicated AST structural helper の actual code 追加
- `TryFallback` / `AtomicCut` malformed static fixture family の actual追加
- parser / loader / checker の actual error source finalization
- `checked_reason_codes` family への new kind 追加
- `place_anchor == current_place` gate や restore scope を checker helper へ昇格

## OPEN に残すもの

- structural malformed source を parser / loader / static gate のどこに置くか
- malformed static fixture family を本当に追加するか
- dedicated AST structural helper の future artifact shape をどこに置くか
- parser cut と future checker API cut をどの文書で handoff するか
- rollback locality を theorem prover / model checker 側へ送る最小 carrier は何か
