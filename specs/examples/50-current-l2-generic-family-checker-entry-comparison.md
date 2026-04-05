# 50 — current L2 generic family checker entry comparison

## 目的

この文書は、shared family checker support helper 導入後の次段として、
**generic checker-side shared family compare entry を追加する価値があるか**
を narrow に比較する。

ここで固定するのは final checker API ではない。
固定するのは、

- current family facade script を維持したまま十分か
- `scripts/current_l2_family_checker_support.py` の次に generic entry を足すべきか
- detached validation loop / human-facing command surface / public checker API 誤読に対してどこで止めるか

という current docs-only judgment だけである。

## current 前提

current L2 では次が成立している。

- same-lineage / missing-option / capability の 3 checker spike は helper-local smoke 済みである
- `scripts/current_l2_family_checker_support.py` により、3 family facade が共有する parser / filter / status / stdout contract は support module に寄せられている
- detached validation loop には
  - `smoke-same-lineage-checker`
  - `smoke-missing-option-checker`
  - `smoke-capability-checker`
  があり、family-specific command 名は current user-facing convenience として読める
- generic checker-side shared entry はまだ存在しない

したがって current 問いは、
**shared support helper の次に generic checker-side shared family compare entry を追加すべきか、それとも family facade 維持で止めるべきか**
である。

## 比較観点

1. current detached validation loop の readability を壊さないか
2. public checker API と誤読される risk を増やさないか
3. shared support helper 導入だけで十分に duplication が減っているか
4. family-specific smoke contract を docs / tests / progress で維持しやすいか
5. future public checker cut を早く既成事実化しすぎないか

## 比較対象

### 案 1. family facade 維持で止める

- `scripts/current_l2_same_lineage_checker.py`
- `scripts/current_l2_missing_option_checker.py`
- `scripts/current_l2_capability_checker.py`

を current command surface として残し、
generic checker-side shared entry は追加しない。

#### 利点

- detached validation loop の family-specific readability を保てる
- support module によって duplication はすでに大きく減っている
- public checker API と誤読される risk が低い
- family ごとの smoke evidence と representative fixture がそのまま結びつく

#### 欠点

- execution entry は 3 つのままで揃わない
- family が増えたとき facade script 追加は必要になる

### 案 2. generic checker-side shared entry を追加する

- たとえば `scripts/current_l2_family_checker.py --family same-lineage`
  のような command を追加する
- facade script は残すか、thin alias にする

#### 利点

- execution entry を 1 箇所にまとめられる
- future family 追加時の command pattern は統一しやすい

#### 欠点

- current phase では user-facing convenience というより checker-like surface に見えやすい
- family-specific readability が下がる
- support module から一段進んで、新しい command surface を既成事実化することになる

### 案 3. generic entry を actual public checker cut の比較まで保留する

- current family facade 維持で止めつつ、
  generic entry の議論は later public checker API comparison と同時に扱う

#### 利点

- current helper-local phase に余計な surface を増やさない
- public checker API、family taxonomy、future verifier handoff をまとめて比較できる

#### 欠点

- generic entry の使い勝手自体を early に試すことはしない
- support module の次段が docs-only judgment に留まる

## current judgment

current L2 で最も自然なのは、
**案 1. family facade 維持で止める**
である。

ただし、その意味は「generic entry を永久に不要と決める」ではない。
current judgment は、

- shared support helper 導入だけで duplicated core contract の問題は十分に下がった
- その時点で generic checker-side shared entry を足すと、public-looking command surface を増やす副作用の方が大きい
- generic entry の比較は、later public checker cut / external verifier handoff の比較と一緒に行う方が自然である

というものである。

## current cut

current task で fixed とみなしてよいのは次である。

- `scripts/current_l2_family_checker_support.py` は current support layer の終点とする
- family facade script は current command surface として維持する
- detached loop wrapper の `smoke-*` command 名は変えない
- generic checker-side shared family compare entry はまだ追加しない

この cut では次を行わない。

- new generic CLI の actualization
- facade script の廃止
- `lib.rs` / `harness.rs` public checker API への昇格
- theorem-level invariant proof

## 未決事項

- generic entry を public checker cut comparison と同時に扱うか
- family taxonomy が今後増えたとき facade 維持コストがどう変わるか
- final public checker API をどこへ置くか
