# static_analysis_01

## この文書の目的

この文書は、repo にある **Problem 1**

- 型検査に相当する checker-adjacent line
- theorem-first の Lean bridge
- model-check second line reserve

を、実際の sample と実行結果を順に見ながら理解するための入門ガイドである。

特に、次の 2 点を混ぜないために書いている。

- **今この repo で実際に動くもの**
- **まだ final public contract には上げていないもの**

ここで扱う current line は `repo-local near-end` であり、final public theorem contract や final public verifier contract を確定したわけではない。

## ここで扱う範囲

この文書で扱うのは、主に次の sample と Lean foundation である。

- `samples/prototype/current-l2-typed-proof-model-check/p06-typed-proof-owner-handoff.txt`
- `samples/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.txt`
- `samples/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.txt`
- `samples/prototype/current-l2-typed-proof-model-check/p12-typed-classified-fingerprint-publication-block.txt`
- `samples/prototype/current-l2-typed-proof-model-check/p15-typed-capture-escape-rejected.txt`
- `samples/prototype/current-l2-typed-proof-model-check/p16-typed-remote-call-budget-exceeded.txt`
- `samples/lean/foundations/CurrentL2IfcSecretExamples.lean`
- `samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean`

扱わないものも先に明記しておく。

- full dependent core の public calculus
- concrete theorem prover brand の固定
- concrete model-check tool brand の固定
- final public checker artifact / verifier contract

## 事前準備

この repo の root で次を使う。

```bash
python3 --version
cargo --version
source "$HOME/.elan/env"
lean --version
```

Lean を使う command は、以後も `source "$HOME/.elan/env"` を付けて実行するとよい。

## 最短確認

まず bundle 全体が崩れていないことを一度に確認する。

```bash
python3 scripts/current_l2_guided_samples.py smoke-all --format json
```

実際に 2026-04-21 に再実行した結果では、`problem1` / `problem2` の両方が `status: "passed"` だった。
Problem 1 だけを追うなら、以後の section を順番に実行すればよい。

## 1. まず typed / IFC の成功例と失敗例を見る

最初に見るべきなのは `p10` から `p16` の typed sample 群である。
これらは「一見もっともらしいが実はダメ」を自動で弾く current first line を、一番分かりやすく示す。

### 実行コマンド

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- \
  check-source-sample \
  samples/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.txt \
  --format pretty

cargo run -q -p mir-runtime --example mir_current_l2 -- \
  check-source-sample \
  samples/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.txt \
  --format pretty

cargo run -q -p mir-runtime --example mir_current_l2 -- \
  check-source-sample \
  samples/prototype/current-l2-typed-proof-model-check/p12-typed-classified-fingerprint-publication-block.txt \
  --format pretty

cargo run -q -p mir-runtime --example mir_current_l2 -- \
  check-source-sample \
  samples/prototype/current-l2-typed-proof-model-check/p15-typed-capture-escape-rejected.txt \
  --format pretty

cargo run -q -p mir-runtime --example mir_current_l2 -- \
  check-source-sample \
  samples/prototype/current-l2-typed-proof-model-check/p16-typed-remote-call-budget-exceeded.txt \
  --format pretty
```

### それぞれの意味

- `p10`
  - authority を持つ主体だけが secret 由来 fingerprint を公開できる success 例
- `p11`
  - holder ではあるが release authority を持たないので reject される例
- `p12`
  - authority の有無ではなく、label-flow mismatch で止まる例
- `p15`
  - ephemeral token を capture scope の外へ逃がそうとして reject される例
- `p16`
  - remote call budget が 0 のまま follow-up call を試みて reject される例

### 実際の出力例

`p10` は success になる。

```text
static_gate_verdict: valid
terminal_outcome: success
typed_checker_hint_status: reached
```

`p11` は parse や static gate では落ちず、**意味上の条件違反**として reject される。

```text
static_gate_verdict: valid
terminal_outcome: Reject
typed_checker_hint_status: reached
```

`p12`, `p15`, `p16` も同じく `static_gate_verdict: valid` だが `terminal_outcome: Reject` になる。
これは「構文としては読めるが、型・label・capture・budget の条件を満たしていない」という意味である。

## 2. theorem-first bridge を見る

次に、typed sample が theorem-first bridge にどう繋がっているかを確認する。
ここで重要なのは、**foundation 側の Lean proof** と **generated stub 側の Lean artifact** を分けて読むことだ。

### 実行コマンド

```bash
python3 scripts/current_l2_guided_samples.py emit-theorem problem1
```

### 実際の出力例

```text
Problem 1 theorem-first emitted artifact loop
output dir: target/current-l2-guided/problem1-theorem-pilot
pilot summary markdown: target/current-l2-guided/problem1-theorem-pilot/pilot-summary.md

- p06-typed-proof-owner-handoff: representative theorem-first sample
  pilot_status: reached
  lean_stub_artifact_count: 1
```

この command は、`p06` を中心に theorem-first pilot の emitted artifact を materialize する。
ここで得られる `samples/lean/current-l2/...` の Lean file は **Lean に受理される generated artifact** だが、まだ `sorry` を含む。

つまり、

- foundation 側
  - 小さいが実証済みの proof fragment
- generated current-l2 側
  - bridge が Lean text を正しく出せることの確認

という役割分担になっている。

## 3. model-check second line reserve を見る

次に、Problem 1 の rejection 群が model-check second line reserve でも整理されることを見る。

### 実行コマンド

```bash
python3 scripts/current_l2_guided_samples.py emit-reserve model-check-second-line
```

### 実際の出力例

```text
model-check second-line reserve package
output dir: target/current-l2-guided/reserve-packages/model-check-second-line

- p10-typed-authorized-fingerprint-declassification: authority release positive carrier
  static_gate: valid
  terminal_outcome: success

- p11-typed-unauthorized-fingerprint-release: authority miss rejection
  static_gate: valid
  terminal_outcome: reject

- p12-typed-classified-fingerprint-publication-block: label-flow rejection
  static_gate: valid
  terminal_outcome: reject

- p15-typed-capture-escape-rejected: capture/lifetime rejection
  static_gate: valid
  terminal_outcome: reject

- p16-typed-remote-call-budget-exceeded: simple cost rejection
  static_gate: valid
  terminal_outcome: reject
```

ここでの読み方は単純である。

- `success`
  - current first line では許可される
- `reject`
  - current first line では禁止される

まだ concrete model-check tool brand や final public checker artifact を出しているわけではないが、**「どの bad pattern を今の line で弾けるか」**は自動で確認できる。

## 4. Lean foundation の proof fragment を実行する

次に、foundation 側の Lean file が実際に通ることを確かめる。
ここが、generated stub より本質的な部分である。

### 実行コマンド

```bash
source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2IfcSecretExamples.lean
source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean
```

2026-04-21 の再実行では、どちらも **出力なしで終了コード 0** だった。
Lean では、proof file が通ると通常は何も出ない。

### 何が入っているか

`CurrentL2IfcSecretExamples.lean` では、今回の hardening により次の補題群を追加している。

- `declassify_preserves_value`
  - declassify が payload 自体を書き換えていないことを示す
- `low_to_low_release_without_authority_is_available`
  - public 情報の素通しは authority なしでも通ることを示す
- `authorized_live_fingerprint_release_has_witness`
  - authority がある high -> low release には実際に witness があることを示す
- `unauthorized_live_fingerprint_release_is_impossible`
  - authority なし high -> low release は witness を作れないことを示す

`CurrentL2FiniteIndexFirstLayer.lean` では、次の補題群を追加している。

- `outlives_trans`
  - lifetime の順序が推移すること
- `capture_subset_trans`
  - capture set inclusion も推移すること
- `single_budget_is_exhausted_after_one_call`
  - 1 回分の budget は 1 回使うと尽きること
- `two_budget_still_allows_after_one_call`
  - 2 回分なら 1 回使ってもまだ許されること

これにより、sample の success / reject を「その sample だけの話」で終わらせず、**再利用可能な補題**として読めるようになった。

## 5. 依存型の雰囲気をその場で確かめる

`CurrentL2IfcSecretExamples.lean` では `Labeled (label : SecurityLabel) α` の形で、label を型引数として持つ。
これは full dependent calculus ではないが、**security label を型にぶら下げる**読みを体験するには十分である。

### 通る最小例

次の command は、その場で型と relation を定義し、authorized な高->低 declassification が通ることを示す。

```bash
bash -lc 'cat > /tmp/current_l2_ifc_valid.lean <<\"EOF\"
inductive SecurityLabel where
  | low
  | high

def flowsTo : SecurityLabel → SecurityLabel → Prop
  | .low, _ => True
  | .high, .high => True
  | .high, .low => False

def CanDeclassify (hasAuthority : Bool) (fromLabel toLabel : SecurityLabel) : Prop :=
  hasAuthority = true ∨ flowsTo fromLabel toLabel

structure Labeled (label : SecurityLabel) (α : Type) where
  value : α

example : CanDeclassify true SecurityLabel.high SecurityLabel.low := by
  simp [CanDeclassify, flowsTo]
EOF
source \"$HOME/.elan/env\" && lean /tmp/current_l2_ifc_valid.lean'
```

2026-04-21 の再実行では、これは **出力なしで成功**した。

### 弾かれる最小例

次の command は、authority なしで高->低 release を証明しようとして失敗する。

```bash
bash -lc 'cat > /tmp/current_l2_ifc_invalid.lean <<\"EOF\"
inductive SecurityLabel where
  | low
  | high

def flowsTo : SecurityLabel → SecurityLabel → Prop
  | .low, _ => True
  | .high, .high => True
  | .high, .low => False

def CanDeclassify (hasAuthority : Bool) (fromLabel toLabel : SecurityLabel) : Prop :=
  hasAuthority = true ∨ flowsTo fromLabel toLabel

structure Labeled (label : SecurityLabel) (α : Type) where
  value : α

example : CanDeclassify false SecurityLabel.high SecurityLabel.low := by
  simp [CanDeclassify, flowsTo]
EOF
source \"$HOME/.elan/env\" && lean /tmp/current_l2_ifc_invalid.lean'
```

実際の出力はこうなる。

```text
/tmp/current_l2_ifc_invalid.lean:16:70: error: unsolved goals
⊢ False
```

この `⊢ False` は、「その条件のもとでは証明義務を満たせない」という意味である。
つまり、見た目には `declassify` を呼べそうでも、**authority なし高->低**は型・論理の側で弾かれている。

## 6. foundation と generated stub の違いを一度に確認する

次の command は foundation と generated current-l2 の両方を再生成し、Lean で検証する。

```bash
source "$HOME/.elan/env" && python3 scripts/current_l2_lean_sample_sync.py
```

2026-04-21 の再実行では、JSON の中で foundation 側はすべて `success: true` だった。
一方で generated current-l2 側には、次のような warning も出ている。

```text
"filename": "CurrentL2IfcSecretExamples.lean"
"verification": {
  "success": true
}

"sample_id": "p15-typed-capture-escape-rejected"
"verification": {
  "success": true,
  "stdout": "... warning: declaration uses `sorry`"
}
```

ここでの読み方は重要である。

- foundation 側
  - actual small proof
- generated stub 側
  - Lean acceptance はしているが theorem body は placeholder を含む

つまり、今回「`sorry` を減らした」というときの主語は **foundation 側の本質部分**である。

## 7. 出力の読み方

Problem 1 の command を読むときは、まず次の 3 つを見るとよい。

- `static_gate_verdict`
  - `valid` なら構文と基本 shape は通った
- `terminal_outcome`
  - `success` なら current line で許可
  - `Reject` / `reject` なら current line で禁止
- `typed_checker_hint_status`
  - `reached` なら typed checker-adjacent line まで到達している

補助的に次も役に立つ。

- `foundation_evidence_ref`
  - どの Lean foundation が根拠としてぶら下がっているか
- `theorem_preview_status`
  - theorem-first bridge でどこまで materialize されているか
- `model_check_preview_status`
  - model-check second line reserve でどこまで materialize されているか

## 8. 一見大丈夫そうだが実はダメな例

初心者が最初に勘違いしやすいのは、`p11` である。

- `derive_secret_fingerprint` まではそれらしく見える
- `publish_without_authority` も字面だけ見ると perform できそうに見える

しかし current line では、**fingerprint を保持していること**と
**public release authority を持つこと**を分けている。

そのため、実行するとこうなる。

```text
static_gate_verdict: valid
terminal_outcome: Reject
typed_checker_hint_status: reached
```

これは「parser ではなく、意味づけの line で落としている」ことを示すので、むしろ望ましい。

## まとめ

Problem 1 の current line は、すでに次のことを repo 内で再確認できる。

- success / reject の typed sample 群が実行できる
- theorem-first bridge を artifact として materialize できる
- model-check second line reserve で rejection 群をまとめ直せる
- Lean foundation では `sorry` なしの小さな補題群が実際に通る

ただし、これはまだ final public theorem contract や final public verifier contract ではない。
現在の読みは、**foundation proof を本質部分として厚くし、generated stub は bridge artifact として明示的に切り分ける**ところにある。
