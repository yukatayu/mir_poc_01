# 64 — current L2 try/rollback malformed static family timing

## 目的

この文書は、current L2 parser-free PoC、future dedicated AST structural helper の
public checker entry criteria を前提に、
**future dedicated AST structural helper の malformed static family を actual corpus に増やす timing をどこに置くか**
を narrow に整理する。

ここで固定するのは actual malformed fixture 追加ではない。
固定するのは、

- malformed static family を今すぐ足すべきか
- dedicated helper actualization のどの段で足すのが自然か
- public checker comparison まで待つべきか

という docs-only judgment だけである。

## current 前提

current repo では次が成立している。

- runtime representative `E2` / `E21` / `E22` が current phase の evidence として十分強い
- malformed static family は current phase ではまだ actual corpus に増やさない judgment がある
- dedicated AST structural helper の compare contract、future field 候補、static gate loop 挿入、structural verdict carrier、shared carrier threshold、wrapper family、generic family boundary、public checker entry criteria は docs-only で整理済みである
- later public checker API comparison に進む前提として、malformed static family か同等の AST-only structural corpus が actual fixtures として複数件あることが求められる

したがって current 問いは、
**malformed static family の actual corpus 追加は、current phase の今ではなく、dedicated helper actualization と later public checker comparison のどの間に置くのが自然か**
である。

## 比較観点

1. runtime representative 優先の current evidence を壊さないか
2. dedicated helper actualization に必要な AST-only corpus を遅らせすぎないか
3. malformed wording や fixture-side expected field を premature に固定しないか
4. public checker comparison のための corpus requirement を満たせるか
5. helper-local progression と public-looking comparison を混ぜないか

## 比較対象

### 案 1. current phase の今すぐ actual corpus に足す

#### 利点

- dedicated helper actualization 前に static corpus を先に用意できる
- malformed family 候補の wording を早く concrete にできる

#### 欠点

- current runtime representative 優先 judgment と衝突しやすい
- helper actualization や fixture-side field actualization が無いまま malformed fixture だけ増える
- wording / expected carrier / compare path を premature に固定しやすい

### 案 2. dedicated helper actualization の first tranche と同時に actual corpus へ足す

ここでいう first tranche は、少なくとも次を含む。

1. dedicated helper-local 実装の着手
2. fixture-side fields
   - `checked_try_rollback_structural_verdict`
   - `checked_try_rollback_structural_findings`
   の actual compare 化
3. static gate artifact loop の family-specific smoke path actualization

#### 利点

- malformed static family が dedicated helper actualization の直接 evidence になる
- wording / expected field / compare path を同じ tranche で narrow に整えられる
- public checker comparison に必要な AST-only corpus を、public cut より前に揃えられる

#### 欠点

- dedicated helper actualization tranche は一段重くなる

### 案 3. later public checker API comparison の直前まで actual corpus に足さない

#### 利点

- current docs-only phaseを長く保てる

#### 欠点

- public checker comparison の entry criteria に必要な corpus を後段へ押し込みすぎる
- dedicated helper actualization 後の実地反復が runtime representative だけに偏る
- public comparison の前に malformed wording / compare path の磨き込みができない

## current judgment

current L2 の next narrow step として最も自然なのは、
**案 2. dedicated helper actualization の first tranche と同時に malformed static family を actual corpus へ足す**
である。

理由は次の通り。

1. current phase の今すぐ actualize すると、runtime representative 優先 judgment と helper未actualize状態が衝突する
2. public checker comparison の直前まで待つと、dedicated helper actualization の実地反復に必要な AST-only structural corpus が不足する
3. dedicated helper actualization の first tranche と同時に足せば、wording / fixture-side field / static gate loop compare を同じ narrow tranche で揃えられる

## minimum timing cut

current docs-only minimum は次である。

1. **今はまだ actual corpus に増やさない**
2. **dedicated helper actualization を始める task で、最小 malformed static family tranche を同時に切る**
3. **public checker API comparison より前に、その tranche で数回の authoring / compare 反復を済ませる**

この timing cut では、

- runtime representative `E2` / `E21` / `E22` は current evidence として維持する
- malformed static family は dedicated helper actualization の proof-of-use として扱う
- public checker comparison は、その後段で corpus と helper contract が落ち着いてから始める

## current guidance

current helper stack と roadmap では、次を守る。

1. current phase では malformed static family をまだ actual corpus に足さない
2. dedicated helper actualization に着手する task で、最小 malformed static family tranche を同時に設計する
3. public checker comparison の requirement を満たすために malformed static family を後から慌てて足す構図は避ける

## current cut

この task では次を行わない。

- malformed static fixture の actual追加
- dedicated AST structural helper の actual実装
- fixture schema actual field 追加
- public checker API comparison actualization

## next narrow step

current docs-only judgment の次段として自然なのは、
**future dedicated AST structural helper を actualize するときの first tranche を、helper code / fixture fields / malformed static family / smoke path のどこまで一体で切るか**
を比較することである。

## OPEN に残すもの

- malformed static family の最小 tranche size
- actual helper actualization 時の exact subcommand 名
- malformed wording family をどこまで fixture-side expected に載せるか
