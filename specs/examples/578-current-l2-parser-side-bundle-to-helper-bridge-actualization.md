# 578 — current L2 parser-side bundle-to-helper bridge actualization

## 目的

`specs/examples/575-current-l2-problem1-theorem-first-pilot-bundle-actualization.md`、
`576-current-l2-problem2-authoritative-room-scenario-bundle-actualization.md`、
`577-current-l2-parser-side-companion-surface-bundle-actualization.md`
を前提に、

- guided bundle helper
- parser-side companion sample
- representative prototype
- Lean artifact

を、`scripts/current_l2_guided_samples.py bundle problem1|problem2`
で 1 本に束ねる
**parser-side bundle-to-helper bridge**
として actualize する current cut を 1 本に束ねる。

ここで actualize するのは、
**docs / helper / parser-side companion surface の 3 点を一本道で辿れるようにする current cut**
であり、

- final public parser / checker / runtime API
- full `Program` lowering
- final public verifier contract

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed にある。

1. Problem 1 / Problem 2 bundle helper
   - representative prototype
   - Lean artifact
   - anchor spec / report
2. parser-side companion surface sample
   - `p06`
   - `p07`
   - `p08`
3. non-production parser-side carrier
   - `Stage3RequestHeadClauseBundle`

したがって current open problem は、
parser-side sample を separate に増やすことではなく、
**current bundle helper の中で parser-side companion への橋をどこまで visible にするか**
である。

## current actualization cut

current package では、次を採る。

1. `scripts/current_l2_guided_samples.py`
   の bundle manifest に `parser_companion_path` を追加する
2. pretty bundle output に
   - original prototype
   - parser companion
   - Lean artifact
   を並べて出す
3. representative slice `p06 / p07 / p08` だけが parser companion path を持つ current cut に留める
4. README / sample guide は parser-side companion sample が final grammar でないことを保ったまま bundle へ接続する

## actual runnable evidence

| evidence | current reading |
|---|---|
| `python3 scripts/current_l2_guided_samples.py bundle problem1` | `p06` representative slice の original prototype / parser companion / Lean artifact を 1 画面で辿れる |
| `python3 scripts/current_l2_guided_samples.py bundle problem2` | `p07 / p08` representative pair の original prototype / parser companion / Lean artifact を 1 画面で辿れる |
| `python3 scripts/current_l2_guided_samples.py bundle problem1 --format json` | bundle manifest に `parser_companion_path` が入る |
| `python3 -m unittest scripts.tests.test_current_l2_guided_samples` | bundle helper が parser companion path を落とさないことを machine-check する |

## current recommendation

1. parser-side companion sample への bridge は guided bundle helper に actualize してよい。
2. bridge は representative slice だけに留め、reserve / negative line の parser companion widening は still later に残す。
3. helper bridge は final parser/public API ではなく、repo-local readability cut として扱う。

## retained alternatives

- parser-side sample と helper bundle を separate のままに保つ
- final parser/public API を先に作る
- reserve / negative line まで parser companion を先に widen する

## stop line

current package は次で止める。

- final public parser / checker / runtime API
- full `Program` lowering
- final public verifier contract

## next self-driven line

current package を close した後の next self-driven line は、
parser-side companion sample の parse result を inspectable JSON / CLI summary として出す small inspector line に置くのが自然である。
