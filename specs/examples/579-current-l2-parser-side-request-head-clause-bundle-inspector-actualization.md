# 579 — current L2 parser-side request-head / clause-bundle inspector actualization

## 目的

`specs/examples/577-current-l2-parser-side-companion-surface-bundle-actualization.md` と
`578-current-l2-parser-side-bundle-to-helper-bridge-actualization.md`
を前提に、

- `Stage3RequestHeadClauseBundle`
- representative parser companion sample `p06 / p07 / p08`
- repo-local command で読める JSON / pretty inspection

を 1 本に束ねる
**parser-side inspector**
として actualize する current cut を固定する。

ここで actualize するのは、
**parser-side carrier の parse result を final public parser API に上げずに repo-local command で inspectable にする**
ための narrow package であり、

- final grammar
- final public parser / checker / runtime API
- full `Program` lowering
- final diagnostics / span-rich contract

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed にある。

1. parser-side carrier
   - `Stage3RequestHeadClauseBundle`
   - `perform ... on ...`
   - `perform ... via ...`
   - request-local `require` / `ensure`
2. representative parser companion sample
   - `p06`
   - `p07`
   - `p08`
3. guided bundle bridge
   - original prototype
   - parser companion path
   - Lean artifact path

したがって current open problem は、
new parser carrier を作ることではなく、
**representative parser companion sample の parse result をどこまで repo-local command で inspectable にするか**
である。

## current actualization cut

current package では、次を採る。

1. `CurrentL2RequestHeadClauseBundleInspection`
   を追加する
2. `inspect_stage3_request_head_clause_bundle(...)`
   で parse result を
   - `op`
   - `target_kind`
   - `target_ref`
   - `require_fragment_text`
   - `ensure_fragment_text`
   - `attachment_frame_kind`
   に圧縮する
3. JSON / pretty renderer を用意する
4. `cargo run -q -p mir-ast --example current_l2_inspect_request_head_clause_bundle -- <path> --format pretty|json`
   を repo-local command として actualize する
5. representative sample `p06 / p07 / p08` の focused test で current cut を machine-check する

## actual runnable evidence

| evidence | current reading |
|---|---|
| `cargo test -p mir-ast --test current_l2_request_head_clause_bundle_inspector_manifest` | inspector manifest と representative sample `p06 / p07 / p08` の JSON / pretty rendering を machine-check する |
| `cargo run -q -p mir-ast --example current_l2_inspect_request_head_clause_bundle -- samples/prototype/current-l2-parser-companion/p06-typed-proof-owner-handoff.request.txt --format json` | Problem 1 representative slice の parse result を JSON で inspect できる |
| `cargo run -q -p mir-ast --example current_l2_inspect_request_head_clause_bundle -- samples/prototype/current-l2-parser-companion/p07-dice-late-join-visible-history.request.txt --format pretty` | Problem 2 representative slice の parse result を pretty summary で inspect できる |

## current recommendation

1. parser-side carrier の inspection は repo-local command として actualize してよい。
2. inspection target は representative slice `p06 / p07 / p08` に留め、reserve / negative sample への widening は still later に残す。
3. inspection summary は parse result の readable cut であり、final public parser API ではない。

## retained alternatives

- parse result inspection を docs-only explanation に留める
- final public parser API を先に作る
- sample id catalog / matrix を先に作る

## stop line

current package は次で止める。

- final public parser / checker / runtime API
- full `Program` lowering
- final diagnostics / span-rich contract
- final public verifier contract

## next self-driven line

current package を close した後の next self-driven line は、
original prototype / parser companion / guided bundle / Lean artifact / anchor spec-report の mapping matrix を drift なく readable にする package に置くのが自然である。
