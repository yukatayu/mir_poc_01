# current-l2 source samples

この directory は、current L2 fixed subset の **source text sample corpus** を置く repo-root layer である。

## 位置づけ

- representative prose (`specs/examples/00...01`) でもない。
- machine-readable fixture corpus (`crates/mir-ast/tests/fixtures/current-l2/`) でもない。
- fixed subset の syntax-backed regression / lowering / runner / verification ladder に進むための第 3 層である。

## current policy

- directory は `samples/current-l2/` に固定する。
- 初期 corpus は flat layout にする。
- extension は final grammar を意味しない neutral text file として `.txt` を使う。
- sample stem は fixture stem / `fixture_id` と揃える。
- filename に verdict、parser tranche、reached stage は埋め込まない。

## initial cluster

current first cluster は次を想定する。

- `e1-place-atomic-cut.txt`
- `e2-try-fallback.txt`
- `e3-option-admit-chain.txt`
- `e4-malformed-lineage.txt`
- `e21-try-atomic-cut-frontier.txt`
- `e23-malformed-try-fallback-missing-fallback-body.txt`

## non-goals

- final parser grammar の固定
- fixture JSON からの逆生成
- representative prose のそのまま昇格
- public CLI / exporter / backend path の先取り

## next steps

- representative / fixture / source mapping matrix を作る
- parser-to-`Program` lowering first cut を fail-closed に定義する
- syntax-backed runner と verification ladder へ接続する
