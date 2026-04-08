# 99 — current L2 stage 3 request clause suite structural floor

## 目的

この文書は、`specs/examples/98-current-l2-stage3-suite-vs-malformed-sequencing.md` で
multiline attachment malformed-source pair extension より先に
**request-local `require:` / `ensure:` の sibling clause suite structural floor を比較する**
と整理したことを前提に、
**current stage で suite structural floor をどこまで切ってよいか**
を narrow に比較する。

ここで固定するのは final parser grammar でも final diagnostics contract でもない。
固定するのは、shared single attachment frame の上に積む
**request-local clause suite の最小 structural floor** だけである。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- declaration-side `admit:` は current stage でも at-most-one attachment に留める。
- shared single attachment frame は actualize 済みである。
- current companion notation では `perform` head だけが request-local clause suite owner である。
- `specs/examples/01-current-l2-surface-syntax-candidates.md` では、current preferred ordering を `require` の後に `ensure` としている。

## 比較する 3 案

### 案 1. fixed two-slot suite floor

current stage の suite structural floor を、
`perform` head 配下の **optional `require` + optional `ensure`** に留める。

```text
perform update_authority on profile_authority
  require:
    write
  ensure:
    owner_is(session_user)
```

この案で floor に含めるものは次である。

- clause suite owner は `perform` だけ
- child clause line は sibling attachment line
- current preferred ordering は `require` の後に `ensure`
- `require` / `ensure` は各 at-most-one
- suite は dedent / 次 statement head で終わる
- clause-between blank line は current stage では不可

#### 利点

- current companion notation と最も整合する。
- suite ownership / ordering / multiplicity / termination を最小で切れる。
- duplicate clause malformed family を still later stage に残しても、structural floor 自体は十分明確になる。

#### 欠点

- future に richer contract clause family を入れるなら later extension が要る。

### 案 2. ordered clause list floor

`perform` head 配下を ordered clause list として扱い、
`require:` / `ensure:` の duplicate も current stage では構造上許す。

#### 利点

- future extension には柔らかい。

#### 欠点

- current examples には過剰である。
- duplicate clause や ordering の malformed/source question を structural floor から切り離しにくい。

### 案 3. generic keyed clause map floor

request-local clause suite を unordered keyed map として読む。

#### 利点

- surface から離れた abstract model としては簡潔に見える。

#### 欠点

- current line-based companion notation と距離がある。
- sibling order / dedent / suite termination の surface questionを hidden にしやすい。

## 比較

### current companion candidate との整合

- `specs/examples/01...` は current preferred ordering を `require` の後に `ensure` としている。
- この current phase では、案 1 が最も自然である。

### staged line との整合

- shared single attachment frame は already actualize 済みであり、次に必要なのは request-local clause suite の最小 structural floor である。
- ここで generic list / map を先に入れると、surface companion line より abstract shape が先行しやすい。
- この観点でも案 1 が最小である。

### malformed family との分離

- 案 1 は duplicate clause や clause-between blank line の malformed family を later に残しやすい。
- 案 2 と案 3 は malformed family の切り方を structural floor へ巻き込みやすい。

## current judgment

current repo の next narrow floor としては、
**案 1. fixed two-slot suite floor**
を採るのが最も自然である。

つまり current stage では、

- clause suite owner は `perform`
- child clause line は sibling attachment line
- `require` / `ensure` の current preferred ordering は `require` の後に `ensure`
- `require` / `ensure` は各 at-most-one
- suite termination は dedent / 次 statement head
- clause-between blank line は不可

までを request-local clause suite structural floor に入れてよい。

## current stage でまだやらないこと

- duplicate clause malformed pair の actualization
- `ensure:` missing-block malformed pair の actualization
- richer clause family (`contract` block、extra clause keyword)
- declaration-side `admit:` を suite へ広げること
- public parser API 化

## next narrow step

次は、
**fixed two-slot suite floor を helper-local / test-only structural compare にどこまで actualize するか**
を整理するのが自然である。
