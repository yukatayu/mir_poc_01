# 577 — current L2 parser-side companion surface bundle actualization

## 目的

`specs/examples/472-current-l2-minimal-companion-experimental-order-handoff-surface.md`、
`564-current-l2-phase6-perform-head-request-clause-bundle-attachment-comparison.md`、
`565-current-l2-phase6-perform-head-request-clause-bundle-thin-wrapper-threshold-helper-mirror.md`、
`575-current-l2-problem1-theorem-first-pilot-bundle-actualization.md`、
`576-current-l2-problem2-authoritative-room-scenario-bundle-actualization.md`
を前提に、

- representative sample `p06`
- representative pair `p07 / p08`
- parser-side companion surface sample
- `Stage3RequestHeadClauseBundle`

を、`samples/prototype/current-l2-parser-companion/` と `mir-ast` test によって
**thin experimental companion surface bundle**
として actualize する current cut を 1 本に束ねる。

ここで actualize するのは、
**representative theorem-first / authoritative-room slice を final grammar へ上げずに parser-side carrier へ戻す current cut**
であり、

- final grammar
- final public parser / checker / runtime API
- final public theorem contract
- final public witness/provider/artifact contract

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed にある。

1. minimal companion experimental surface
   - helper-local companion wording
   - semantic honesty first
   - final grammar later
2. parser-side non-production carrier
   - `Stage3RequestHeadClauseBundle`
   - request-local `require` / `ensure`
   - `perform ... on ...` / `perform ... via ...`
3. guided problem bundle
   - Problem 1 bundle `p06`
   - Problem 2 bundle `p07 / p08`

したがって current open problem は、
new source principal を採ることではなく、
**representative sample を parser-side carrier に戻す最小 companion slice をどこまで actualize するか**
である。

## current actualization cut

current package では、次を採る。

1. `samples/prototype/current-l2-parser-companion/`
   を追加する
2. first slice は
   - `p06-typed-proof-owner-handoff.request.txt`
   - `p07-dice-late-join-visible-history.request.txt`
   - `p08-dice-stale-reconnect-refresh.request.txt`
   に限定する
3. それぞれを `Stage3RequestHeadClauseBundle` へ parse する `mir-ast` test を置く
4. README で original prototype / bundle guide / parser-side companion の対応を日本語で説明する

## actual runnable evidence

| evidence | current reading |
|---|---|
| `samples/prototype/current-l2-parser-companion/README.md` | parser-side companion surface の位置づけと original sample との対応を日本語で辿れる |
| `cargo test -p mir-ast --test current_l2_stage3_request_head_clause_bundle_sample_bundle` | `p06 / p07 / p08` companion sample が `Stage3RequestHeadClauseBundle` へ parse できる |
| `crates/mir-ast/tests/current_l2_stage3_request_head_clause_bundle_sample_bundle.rs` | parser-side companion sample と expected carrier shape の対応を machine-check する |

## current recommendation

1. parser-side companion surface は representative slice に限って actualize してよい。
2. companion sample は helper-local / non-production の reader aid であり、public source principal ではない。
3. first slice は `p06`、`p07`、`p08` に限定し、reserve / negative sample への widening は still later に残す。

## retained alternatives

- final grammar adoption を先に行う
- full prototype set を parser-side companion sample に widen する
- parser-side carrier を省いて docs-only wording に留める

## stop line

current package は次で止める。

- final grammar
- final public parser / checker / runtime API
- final public theorem contract
- final public witness/provider/artifact contract

## next self-driven line

current package を close した後の next self-driven line は、
parser-side carrier を current guided bundle / helper summary と一本道で結ぶ thin bridge に置くのが自然である。
