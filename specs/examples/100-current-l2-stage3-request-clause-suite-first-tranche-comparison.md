# 100 — current L2 stage 3 request clause suite first tranche comparison

## 目的

この文書は、`specs/examples/99-current-l2-stage3-request-clause-suite-structural-floor.md` で
request-local `require:` / `ensure:` の structural floor を
`perform` owner の fixed two-slot suite に留めると整理したことを前提に、
**その floor を helper-local / test-only actual evidence にどこまで actualize するのが最小か**
を narrow に比較する。

ここで固定するのは final parser grammar でも public parser API でもない。
固定するのは、

- current multiline attachment bridge の次に
- request-local fixed two-slot suite を
- どの compare surface まで helper-local に actualize してよいか

という **first tranche cut** だけである。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- shared single attachment frame は actualize 済みである。
- request-local clause suite floor は `perform` owner の fixed two-slot suite に留める。
- stage 3 helper は private / test-only に留める。
- `require:` / `ensure:` の predicate fragment parse 自体は、既存 isolated predicate fragment helper を流用してよい。
- duplicate clause malformed family や public diagnostics contract は still later stage に残す。

## 比較する 3 案

### 案 1. clause presence / order summary only

helper-local actualization は、
`perform` head 配下に

- `require:` があるか
- `ensure:` があるか
- その出現順が current preferred ordering に合うか

だけを返す summary に留める。

```text
perform write_profile on profile_doc
  require:
    owner_is(session_user)
  ensure:
    owner_is(session_user)
```

この案では、predicate block の extracted fragment は suite helper 自体からは返さない。

#### 利点

- helper surface は最も小さい。
- request-local clause suite owner / ordering の確認だけに絞れる。

#### 欠点

- current multiline attachment bridge と isolated predicate fragment helper の接続が切れる。
- suite helper が structural summary と fragment extraction の 2 段構えになりやすい。
- next stage で clause slot ごとの fragment compare を再度足す必要がある。

### 案 2. fixed two-slot suite bridge

helper-local actualization は、
`perform` head 配下の fixed two-slot suite を次の carrier として返す。

```text
Stage3RequestClauseSuite {
  require_fragment_text: Option<String>,
  ensure_fragment_text: Option<String>,
}
```

この案では helper-local floor として次を fail-closed に扱ってよい。

- suite owner は `perform`
- `require:` / `ensure:` は immediate child attachment line
- current preferred ordering は `require` の後に `ensure`
- `require` / `ensure` は各 at-most-one
- suite termination は dedent / 次 statement head
- clause-between blank line は reject

ただし compare は still narrow に留める。

- helper output は clause slot ごとの dedented fragment text
- fragment parse は existing isolated predicate fragment helper を使う
- full request node compare や public diagnostics row には上げない

#### 利点

- shared single attachment frame の上に、request-local suite floor をそのまま 1 段積める。
- `require` / `ensure` の slot ごとに fixture-side predicate subset compare を再利用できる。
- request suite ownership / ordering / multiplicity / termination を hidden にしないまま、full request parse を避けられる。

#### 欠点

- helper-local fail-closed error wording は少し増える。
- multiline attachment bridge と似た logic をもう 1 段持つため、support helper の責務整理が要る。

### 案 3. full request-local clause suite compare

request-local clause suite を helper-local request node として reconstruct し、
fixture-side `PerformOn` / `PerformVia` contract subset と直接 compare する。

#### 利点

- request-local suite の compare と fixture-side structure が近く見える。

#### 欠点

- request head parse と clause suite compare が同時に太る。
- stage 3 current phase の narrow lineを越える。
- public parser / checker API へ寄る pressure が強い。

## 比較

### shared single attachment frame との接続

- 案 1 は shared attachment frame actualization と clause slot compare が分断される。
- 案 2 は shared attachment frame を clause slot pair へ束ねるだけなので、current line と最も連続的である。
- 案 3 は request node reconstruction が先行しすぎる。

### fixture-side compare の再利用

- 案 1 は clause slot summary しか持たないため、fixture-side predicate subset compare を直接再利用しにくい。
- 案 2 は `require` / `ensure` の fragment text を isolated predicate fragment helper に渡せるので、`e2` / `e10` anchor をそのまま使える。
- 案 3 は compare は強いが、request node reconstruction と一体化して narrow cut ではなくなる。

### malformed family との分離

- 案 2 でも helper-local fail-closed check は必要だが、これは fixed two-slot suite floor の structural guard であり、duplicate clause diagnostics family の public actualizationとは分けて保てる。
- 案 3 は request suite malformed/source family と request node compare が同時に膨らみやすい。

## current judgment

current repo の next narrow actualization としては、
**案 2. fixed two-slot suite bridge**
を採るのが最も自然である。

つまり current stage では、

1. helper input は inline test string
2. helper output は
   - `require_fragment_text: Option<String>`
   - `ensure_fragment_text: Option<String>`
   の two-slot suite carrier
3. helper-local structural guard として
   - owner
   - sibling line
   - ordering
   - at-most-one
   - suite termination
   - clause-between blank line reject
   を fail-closed に扱う
4. clause payload compare 自体は existing isolated predicate fragment helper を流用する

までを first tranche に入れてよい。

## current stage でまだやらないこと

- full request head parse
- fixture-side full request contract node compare
- public parser API 化
- typed diagnostics carrier
- duplicate clause malformed pair を独立 family として docs / tests に広げること

## next narrow step

次は、
**fixed two-slot suite bridge の helper-local / test-only first tranche を actualize する**
のが自然である。

current first tranche では、

- success-side suite extraction
- slot ごとの predicate subset compare
- helper-local structural fail-closed smoke

までを通し、request-local clause malformed family の public widening は still later に残す。
