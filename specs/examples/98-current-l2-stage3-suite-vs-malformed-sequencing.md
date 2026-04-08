# 98 — current L2 stage 3 suite vs malformed sequencing

## 目的

この文書は、`specs/examples/97-current-l2-stage3-multiline-attachment-first-tranche-actualization.md` までで
shared single attachment frame の helper-local first tranche が actualize 済みになったことを前提に、
**次段として request-local clause suite completion と multiline attachment malformed-source pair extension のどちらを先に扱うべきか**
を narrow に比較する。

ここで固定するのは final parser grammar でも final diagnostics contract でもない。
固定するのは、stage 3 later branch の **next-step sequencing judgment** だけである。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- shared single attachment frame の helper-local extraction bridge は actualize 済みである。
- declaration-side `admit:` は current stage では at-most-one attachment として読み、clause suite へ広げない。
- request-local `require:` / `ensure:` は current companion notation では statement-local clause suite を形成する候補である。
- helper-local multiline malformed smoke は
  - missing block
  - nested header over-acceptance 防止
  - blank line fail-closed
  まで actualize 済みである。

## current issue

現在残っている主要 gap は 2 系統ある。

1. request-local clause suite completion
   - `perform` head 配下で `require:` / `ensure:` を sibling attachment line としてどう読むか
   - ordering / multiplicity / suite termination をどこまで narrow に切るか
2. multiline attachment malformed-source pair extension
   - `ensure:` missing block
   - duplicate clause header
   - clause-between blank line
   - generic continuation confusion

次段では、この 2 系統のどちらを先に比較 / actualize するかを決める必要がある。

## 比較する 3 案

### 案 1. request-local clause suite completion を先に比較する

shared single attachment frame の次段として、
`perform` head 配下の request-local clause suite を narrow に比較する。

たとえば次のような question を先に切る。

```text
perform update_authority on profile_authority
  require:
    write
  ensure:
    owner_is(session_user)
```

- `require:` と `ensure:` は sibling attachment line か
- clause suite owner は `perform` だけか
- duplicate `require:` は current stage で malformed か
- `require` / `ensure` の current preferred ordering を structural floor に含めるか

#### 利点

- current remaining gap のうち structural side を先に埋められる。
- shared single attachment frame actualization の次として自然である。
- malformed diagnostics を増やす前に、何が suite completion の対象かを明確にできる。

#### 欠点

- diagnostics wording の coverage 自体はすぐには増えない。

### 案 2. multiline attachment malformed-source pair extension を先に actualize する

shared single attachment frame はそのままにして、
`ensure:` missing block や duplicate clause の malformed pair を先に増やす。

#### 利点

- helper-local fail-closed evidence を短期で増やせる。
- implementation は一見 narrow に見える。

#### 欠点

- suite completion がまだ曖昧なまま diagnostics contract が先行しやすい。
- duplicate clause や clause-between blank line は suite ownership / ordering / termination を前提にしやすく、structural judgment より先に diagnostics wording が膨らむ。

### 案 3. suite completion と malformed extension を同時に開く

#### 利点

- stage 3 later branch を一気に進めているように見える。

#### 欠点

- structural floor と diagnostics contract が再び混ざる。
- current repo の narrow progression に反する。

## 比較

### current gap との近さ

- shared single attachment frame は既に actualize 済みなので、次の gap は request-local suite completion 側に寄っている。
- malformed extension だけを先に増やしても、duplicate clause や suite termination の読みが still implicit になりやすい。

### staged line との整合

- current line は、fragment malformed より先に attachment structural floor を見る、という順で進んでいる。
- その流れを保つなら、次も diagnostics expansion より structural suite completion を先に比べる方が自然である。

### declaration-side / request-local の責務分離

- declaration-side `admit:` は still at-most-one attachment であり、suite 問題は request-local 固有である。
- したがって next step を request-local suite completion comparison に絞っても、shared single attachment frame の line を壊さない。

## current judgment

current repo の next narrow step としては、
**案 1. request-local clause suite completion を先に比較する**
のが最も自然である。

つまり、

1. shared single attachment frame は current floor として据え置く
2. 次は request-local `require:` / `ensure:` の sibling suite としての structural floor を docs-only で比較する
3. multiline attachment malformed-source pair extension は、その後段に残す

## なぜこれが最小か

- shared attachment frame actualization の次段として、最も不足が大きい structural line を先に埋められる。
- duplicate clause や clause-between blank line などの malformed family を、suite ownership / ordering / termination の比較より先に確定しなくて済む。
- declaration-side `admit:` と request-local `require:` / `ensure:` の役割差を保ったまま、request-local 側だけを 1 段深く進められる。

## current stage でまだやらないこと

- suite completion の actual parser 実装
- duplicate clause malformed pair の actualization
- `ensure:` missing block malformed pair の actualization
- declaration-side `admit:` を suite へ広げること
- public parser API 化

## next narrow step

次は、
**request-local `require:` / `ensure:` を sibling clause suite としてどこまで structural floor に入れてよいか**
を docs-only で整理するのが自然である。
