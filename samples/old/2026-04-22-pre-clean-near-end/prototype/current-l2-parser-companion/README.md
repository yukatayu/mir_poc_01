# current-l2 parser-side companion surface サンプル

## 要約

- final grammar ではなく、`mir-ast` の non-production parser-side carrier を representative sample に結び付けるための tiny companion surface。
- current first slice は `p06`、`p07`、`p08` にだけ置き、Problem 1 / Problem 2 の representative line を parser-side に戻す。

## このディレクトリの位置づけ

- ここに置く `.request.txt` は helper-local / non-production の companion surface であり、public source principal ではない。
- 目的は、既存の corrected prototype と bundle guide の読みを壊さずに、
  `Stage3RequestHeadClauseBundle` へ落ちる最小 surface を inspectable sample として残すことにある。
- `atomic_cut`、final public verifier contract、final public witness/provider/artifact contract、final grammar はここでは fixed しない。

## representative slice

### `p06-typed-proof-owner-handoff.request.txt`

- Problem 1 の representative sample `p06-typed-proof-owner-handoff` に対応する parser-side companion surface。
- theorem-first pilot / typed bridge の representative sliceを、`perform ... via ...` と request-local `require` / `ensure` で最小表現した。

### `p07-dice-late-join-visible-history.request.txt`

- Problem 2 の representative sample `p07-dice-late-join-visible-history` に対応する parser-side companion surface。
- authoritative-room first line の late join visible history を、`perform ... on ...` と single-line clause で最小表現した。

### `p08-dice-stale-reconnect-refresh.request.txt`

- Problem 2 の representative sample `p08-dice-stale-reconnect-refresh` に対応する parser-side companion surface。
- stale reconnect fail-then-refresh を、`perform ... via ...` と request-local clause で最小表現した。

## 読み方

- まず original prototype / bundle guide を見る:
  - `samples/prototype/current-l2-typed-proof-model-check/README.md`
  - `samples/prototype/current-l2-order-handoff/README.md`
- 次にこの companion sample を `mir-ast` test で parse する:

```bash
cargo test -p mir-ast --test current_l2_stage3_request_head_clause_bundle_sample_bundle
```

- parse result 自体を repo-local inspector で見る:

```bash
cargo run -q -p mir-ast --example current_l2_inspect_request_head_clause_bundle -- \
  samples/prototype/current-l2-parser-companion/p06-typed-proof-owner-handoff.request.txt \
  --format json
```

- original prototype / parser companion / guided bundle / Lean artifact / anchor spec-report の対応をまとめて見る:

```bash
python3 scripts/current_l2_guided_samples.py mapping
```

- companion sample 自体は meaning-preserving な最小 reader aid であり、final grammar の草案ではない。

## original sample との対応

- `p06-typed-proof-owner-handoff.request.txt`
  - original: `samples/prototype/current-l2-typed-proof-model-check/p06-typed-proof-owner-handoff.txt`
- `p07-dice-late-join-visible-history.request.txt`
  - original: `samples/prototype/current-l2-order-handoff/p07-dice-late-join-visible-history.txt`
- `p08-dice-stale-reconnect-refresh.request.txt`
  - original: `samples/prototype/current-l2-order-handoff/p08-dice-stale-reconnect-refresh.txt`

## representative mapping

| representative sample | original prototype | parser companion | guided bundle | Lean artifact | anchor |
|---|---|---|---|---|---|
| `p06-typed-proof-owner-handoff` | `samples/prototype/current-l2-typed-proof-model-check/p06-typed-proof-owner-handoff.txt` | `samples/prototype/current-l2-parser-companion/p06-typed-proof-owner-handoff.request.txt` | `python3 scripts/current_l2_guided_samples.py bundle problem1` | `samples/lean/current-l2/p06-typed-proof-owner-handoff/` | `specs/examples/575` / `577` / `578` / `579` |
| `p07-dice-late-join-visible-history` | `samples/prototype/current-l2-order-handoff/p07-dice-late-join-visible-history.txt` | `samples/prototype/current-l2-parser-companion/p07-dice-late-join-visible-history.request.txt` | `python3 scripts/current_l2_guided_samples.py bundle problem2` | `samples/lean/current-l2/p07-dice-late-join-visible-history/` | `specs/examples/576` / `577` / `578` / `579` |
| `p08-dice-stale-reconnect-refresh` | `samples/prototype/current-l2-order-handoff/p08-dice-stale-reconnect-refresh.txt` | `samples/prototype/current-l2-parser-companion/p08-dice-stale-reconnect-refresh.request.txt` | `python3 scripts/current_l2_guided_samples.py bundle problem2` | `samples/lean/current-l2/p08-dice-stale-reconnect-refresh/` | `specs/examples/576` / `577` / `578` / `579` |

- この表は representative slice の current cut だけをまとめたものであり、reserve / negative sample や exhaustive catalog まで広げるものではない。
