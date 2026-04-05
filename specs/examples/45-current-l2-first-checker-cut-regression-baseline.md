# 45 — current L2 first checker cut regression baseline

## 目的

この文書は、current static reason carrier の additive coexistence を維持したまま、
**mainline を first checker cut 側へ戻してよいだけの regression baseline が current corpus にあるか**
を narrow に整理する。

ここで固定するのは final checker API ではない。
固定するのは、

- current static-only corpus が first checker cut のどの cluster をどこまでカバーしているか
- carrier migration の細部より先に checker workstream へ戻してよいか

という readiness judgment だけである。

## current 前提

current L2 では次が成立している。

- `checked_reasons` は additive machine-check bridge として残る
- `checked_reason_codes` は stable cluster 8 kind に対する additive typed companion として残る
- detached static gate artifact の `reason_codes` は helper-local / reference-only mirror に留まる
- readiness scan では stable cluster 8 fixture が zero follow-up で aligned している
- duplicate declaration cluster は current typed carrier / stable mirror に昇格していない

したがって current 問いは、
**static reason carrier migration をさらに細かく詰める前に、first checker cut の local / structural floor へ主線を戻してよいか**
である。

## 比較観点

1. current static-only corpus が first checker cut 候補 cluster をどこまで覆っているか
2. checker workstream に戻しても helper-local migration task と責務競合しないか
3. duplicate cluster / explanation-only cluster を無理に checker cut へ押し込まないで済むか
4. parser boundary / first checker cut / theorem prover boundary の順序が自然に保てるか

## current corpus baseline

current `scan-reason-code-readiness` の checker cluster coverage は次である。

- `same_lineage_evidence_floor`: `4`
  - `e4`
  - `e5`
  - `e12`
  - `e19`
- `capability_strengthening_floor`: `2`
  - `e13`
  - `e20`
- `missing_option_structure_floor`: `3`
  - `e16`
  - `e17`
  - `e18`

補助 evidence:

- stable coexistence anchors: `8`
- `checked_reason_codes` but missing `checked_reasons`: `0`
- `checked_reason_codes` mismatch vs actual suggestion: `0`
- duplicate cluster `e14` / `e15` は current typed carrier の外に残る

## 比較対象

### 案 1. mainline を first checker cut へ戻す

- current carrier coexistence は維持する
- 次の主線を first checker cut の local / structural floor へ戻す

#### 利点

- current corpus が same-lineage / capability / missing-option の 3 cluster を実地に覆っている
- static reason carrier migration をこれ以上先に掘るより、checker boundary の整理へ戻る方が全体計画に沿う
- duplicate cluster を explanation-only のまま保ちやすい

#### 欠点

- capability floor は same-lineage / missing-option よりは still thinner で、public checker API に寄せるにはもう一段 helper-local evidence を積みたい
- malformed / underdeclared rejection の cluster summary は別の axis で整理し直す必要がある

### 案 2. carrier migration の deprecation 条件をもう一段詰める

- `checked_reasons` / `checked_reason_codes` shrink 条件をさらに比較する

#### 利点

- static reason carrier story はさらに滑らかになる

#### 欠点

- current zero follow-up 以上の新しい evidence は増えにくい
- first checker cut の主線が遅れる

### 案 3. duplicate cluster まで typed carrier 候補に広げる

#### 利点

- cluster story は一見そろう

#### 欠点

- helper 内部構造に近い cluster を急いで core 側へ寄せることになる
- current non-promotion judgment と衝突する

## current judgment

current L2 の次の主線として最も自然なのは
**案 1. mainline を first checker cut へ戻す**
である。

理由は次の通り。

1. same-lineage evidence floor / capability strengthening / missing-option structure の 3 cluster は current corpus で実地 coverage がある
2. coexistence scan は zero follow-up まで収束しており、carrier migration は current cut として十分安定している
3. duplicate cluster を無理に typed carrier / checker core に寄せずに済む
4. parser boundary → first checker cut → theorem prover boundary という順序に戻す方が全体 roadmap と整合する

## checker workstream に戻すときの current floor

current task で first checker cut readiness として認めてよいのは次である。

- same-lineage static evidence floor
- malformed / underdeclared rejection
- minimal capability strengthening prohibition
- missing-option structure floor

ただし capability floor は `e13` / `e20` の 2 fixture まで actual corpus に入った段階であり、
same-lineage / missing-option と比べると helper-local spike を 1 段挟んでから public checker cut を比較する方が自然である。

一方で、次はまだ checker cut の外に残す。

- duplicate declaration cluster
- `checked_reasons` deprecation
- detached `reason_codes` の first-class 昇格
- theorem-level invariant proof

## 未決事項

- first checker cut を actual helper / API としてどこへ置くか
- malformed / underdeclared split を cluster summary にどう反映するか
- capability floor の corpus coverage `2` で enough とみなせるか、追加 fixture を要するか
- parser subset inventory と first checker cut inventory の handoff をどの文書で一本化するか
