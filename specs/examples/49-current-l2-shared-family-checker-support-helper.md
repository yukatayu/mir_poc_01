# 49 — current L2 shared family checker support helper

## 目的

この文書は、same-lineage / missing-option / capability の 3 checker spike が揃った current 段階で、
**family ごとの facade script は残したまま、共通 compare ロジックだけを shared support helper へ薄く寄せてよいか**
を narrow に整理する。

ここで固定するのは final checker API ではない。
固定するのは、

- helper-local third spike まで揃った family compare の重複をどこまで束ねてよいか
- `scripts/` 内の non-production support module へ落としてよい最小責務
- facade script / detached loop wrapper / docs mirror を壊さない cut

という最小境界だけである。

## current 前提

current L2 では次が成立している。

- same-lineage floor は helper-local first checker spike として smoke 済みである
- missing-option structure floor は helper-local second checker spike として smoke 済みである
- capability strengthening floor は `e13` / `e20` を根拠に helper-local third checker spike として smoke 済みである
- 3 family script はいずれも
  - fixture-side `expected_static.checked_reason_codes`
  - detached static gate artifact `detached_noncore.reason_codes`
  を読み、family-specific kind set だけを filter して exact compare する
- current detached validation loop には
  - `smoke-same-lineage-checker`
  - `smoke-missing-option-checker`
  - `smoke-capability-checker`
  があり、family-specific entry は人間から見て分かりやすい

したがって current 問いは、
**3 family facade を保ったまま shared support helper へ寄せるべきか、それとも helper-local duplication をそのまま維持すべきか**
である。

## 比較観点

1. helper-local / non-production boundary を壊さないか
2. current family-specific entry の readability を落とさないか
3. detached validation loop wrapper と public checker API の誤読を増やさないか
4. future shared family compare entry を決める前の refactor として十分 narrow か
5. tests / docs / plan / progress mirror の drift を減らせるか

## 比較対象

### 案 1. family ごとの helper-local compare をそのまま維持する

- `scripts/current_l2_same_lineage_checker.py`
- `scripts/current_l2_missing_option_checker.py`
- `scripts/current_l2_capability_checker.py`

の重複をそのまま残す。

#### 利点

- 各 script が独立しており、読み手に cluster が直接見える
- support module という新しい helper layer を追加しなくて済む

#### 欠点

- parser / status / row filtering / stdout rendering が 3 script で重複する
- family が増えたとき drift source が増える
- review で cluster wording と core contract を毎回 3 箇所見る必要がある

### 案 2. shared support helper を追加し、family facade script は残す

- `scripts/current_l2_family_checker_support.py`
  - parser、row filter、status 決定、stdout rendering を共通化する
- family script は
  - kind set
  - cluster 名
  - description
  - missing status 名
  だけを持つ thin facade として残す

#### 利点

- duplicated core contract を 1 箇所へ寄せられる
- family-specific command name / wrapper / docs はそのまま残せる
- public checker API や generic checker CLI を既成事実化しない
- future に checker-side shared family compare entry を比較する前段 refactor として narrow である

#### 欠点

- `scripts/` に helper layer が 1 枚増える
- facade と support の責務切り分けを docs mirror へ明記しないと、generic checker entry と誤読される余地がある

### 案 3. checker-side shared family compare entry を新設する

- たとえば `scripts/current_l2_family_checker.py --family same-lineage`
  のような generic entry を直接追加する

#### 利点

- 実行 entry も 1 箇所に揃う
- family 追加時の command naming は統一しやすい

#### 欠点

- current detached loop の family-specific readability を落とす
- helper-local support refactor を越えて、新しい checker-side public-looking entry を増やすことになる
- family facade を置くか generic entry に寄せるかの判断を早く固定しすぎる

## current judgment

current L2 で最も自然なのは、
**案 2. shared support helper を追加し、family facade script は残す**
である。

理由は次の通り。

1. current 3 family spike の duplicated core contract は narrow に共通化できる
2. detached validation loop では family-specific command name がそのまま残るため、ユーザ向けの導線が崩れない
3. `scripts/` 内の helper-local refactor に留まるため、public checker API や generic checker CLI を早く固定しなくて済む
4. future に generic shared family compare entry を切るかどうかは、support helper を導入した後でも別 task で比較できる

## current cut

current task で actualize してよいのは次である。

- `scripts/current_l2_family_checker_support.py`
  - non-production support module
  - `checked_reason_codes` と detached static gate `reason_codes` を読み、kind set で filter して compare する最小 contract だけを持つ
- family facade script は残す
  - `scripts/current_l2_same_lineage_checker.py`
  - `scripts/current_l2_missing_option_checker.py`
  - `scripts/current_l2_capability_checker.py`
- detached loop wrapper command 名は変えない

この cut では次を行わない。

- `lib.rs` / `harness.rs` public checker API への昇格
- generic checker-side shared family compare CLI の導入
- fixture schema field の追加
- theorem-level invariant proof

## smoke / contract evidence

current cut で最低限確認したいのは次である。

- support module の generic contract が
  - `matched`
  - `fixture_*_rows_missing`
  - `out_of_scope`
  を正しく返す
- existing family-specific tests が facade 経由でも green を維持する
- detached loop wrapper の smoke contract が変わらない

## 未決事項

- future に checker-side shared family compare entry を新設するか
- family facade をいつまで残すか
- capability floor をさらに厚くした後に generic family compare の必要性が変わるか
- final checker API をどこへ置くか
