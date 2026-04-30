# 522 — current L2 IFC secret valid/invalid foundation and Japanese Lean corpus sync

## 目的

Package 56 の reopened self-driven line を、`LabelModel` の抽象断片だけで止めず、

- secret-key valid/invalid
- explicit authority declassification
- `samples/lean/` 説明文の日本語同期

まで actualize して、current repo の mechanization-ready floor と human-facing sample docs の読みを揃える。

ここでの主眼は final typed surface や final IFC syntax ではない。
helper-local Lean foundation と committed sample docs を current recommendation に合わせて一段具体化することである。

## current recommendation

### IFC foundation の current floor

current repo は `samples/lean/foundations/CurrentL2IfcSecretExamples.lean` を追加し、
Package 56 の first fragment を次まで actualize してよい。

- secret key は `High` label に留まる
- explicit authority が無い `High -> Low` release は不可能
- explicit authority がある fingerprint release は可能
- authorized release は payload を保ったまま `Low` へ落ちる

これは checker-adjacent principal / layered stack の current first line と整合する。
source principal を stronger typed surface へ早期昇格するものではない。

### Lean corpus explanation wording

current repo は `samples/lean/README.md`、
`samples/lean/clean-near-end/*/README.md`、
`samples/lean/foundations/*.md`
を日本語で保持してよい。
pre-clean-near-end representative corpus の説明は
`samples/lean/old/2026-04-22-pre-clean-near-end/current-l2/*/README.md`
に historical appendix として残してよい。

重要なのは、次の読み分けを文面で崩さないことである。

- `foundations/`
  - actual small proof を含む mechanization-ready core
- `clean-near-end/`
  - active clean sample suite から生成した theorem stub corpus
  - `sorry` を含むため、artifact well-formedness / bridge alignment evidence として読む
- `old/2026-04-22-pre-clean-near-end/current-l2/`
  - pre-clean-near-end representative sample corpus の archive
  - current active generated front door ではない

日本語化は wording refresh であって、新しい規範判断ではない。
ただし current explanation と sample-facing guidance の drift を抑える package としては actual value がある。

## actualized evidence

- `samples/lean/foundations/CurrentL2IfcSecretExamples.lean`
  - secret-key valid/invalid と explicit authority declassification の actual small proof fragment
- `scripts/current_l2_lean_sample_sync.py`
  - 日本語 README / explanation 生成と新しい IFC foundation を committed corpus へ同期
- `scripts/tests/test_current_l2_lean_sample_sync.py`
  - 日本語 explanation と foundation inventory を failing test -> passing test で固定

## stop line

この package は次までを close する。

- Lean-side IFC concrete example first slice
- secret-key valid/invalid の mechanization-ready reading
- sample-facing Lean corpus explanation の日本語同期

この package は次を意味しない。

- final typed source principal
- final IFC syntax
- declassification / endorsement public contract
- source-side exhaustive IFC sample corpus
- final public verifier contract

## retained later line

- explicit authority / label-flow negative の source-side checker fragment integration
- richer declassification / endorsement authority carrier
- theorem/model-check public contract
- final label API
- final typed calculus / final parser grammar
