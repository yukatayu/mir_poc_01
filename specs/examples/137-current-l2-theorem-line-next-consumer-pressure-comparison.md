# 137 — current L2 theorem line next consumer pressure comparison

## 目的

`specs/examples/134-current-l2-theorem-line-consumer-class-comparison.md`
から
`specs/examples/136-current-l2-theorem-line-notebook-bridge-artifact-threshold.md`
までを前提に、

- `proof_notebook` first bridge の次段 reopen を
  **concrete notebook workflow pressure**
  から始めるべきか
- それとも **`proof_assistant_adapter` consumer pressure**
  から始めるべきか
- あるいは両者を同時 compare に送るべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line next practical reopen order** であり、

- actual theorem handoff emitter
- public checker API
- proof assistant specific schema
- notebook artifact path / bless / retention policy

は固定しない。

## scope

- theorem-side minimum contract row core は `obligation_kind + evidence_refs` のまま維持する。
- `proof_notebook` first bridge の current lightweight attachment は `goal_text` に留める。
- notebook bridge artifact は current phase では docs-only derived view に留める。
- shared-space / protocol verifier / runtime policy line は巻き込まない。

## current 前提

current repo では、少なくとも次が成立している。

1. theorem-side minimum contract row core は `obligation_kind + evidence_refs` に留める。
2. first practical consumer class は `proof_notebook` に置く。
3. `proof_notebook` first bridge の current lightweight attachment は `goal_text` に留める。
4. current phase では notebook bridge artifact を named family に昇格させず、docs-only derived view に留める。
5. `proof_assistant_adapter` は first consumer ではなく second practical candidate に留める。

したがって current 問いは、
**next practical reopen を人間向け notebook workflow 側から始めるか、
それとも machine-facing `proof_assistant_adapter` 側から始めるか**
である。

## 比較観点

1. minimum contract row core と `goal_text` attachment の current cutを壊さないか
2. public-looking stable contract や actual emitter を premature に強めないか
3. docs-only derived view の延長として手戻りが小さいか
4. theorem consumer pressure を source-backed に示しやすいか
5. later に両 line を統合するとき、片方の仮定が他方を不必要に拘束しないか

## 比較対象

### 案 1. concrete notebook workflow pressure を先に扱う

#### 読み

next reopen は、

- notebook-oriented review checklist
- obligation row の human walkthrough
- compare / bless-like flow

のような **concrete notebook workflow** が repo 内で必要になったときに、
stable notebook bridge sketch comparison を先に切る。

#### 利点

- `proof_notebook` first consumer と最も連続的である
- current `goal_text` attachment の延長として説明しやすい
- machine-facing schema を premature に固定しにくい
- docs-only derived view から stable sketch への一段階だけを比較すればよい

#### 欠点

- notebook workflow 自体が concrete に現れない限り、比較根拠が弱い
- `proof_assistant_adapter` pressure が先に concrete になった場合は順番を入れ替える必要がある

### 案 2. `proof_assistant_adapter` consumer pressure を先に扱う

#### 読み

next reopen は、

- proof assistant import
- theorem assistant side adapter schema
- machine-readable proof obligation projection

のような **`proof_assistant_adapter` consumer** を first practical reopen に置く。

#### 利点

- external theorem workflow への橋を早く見られる
- stable row contract や adapter-local schema を先に比較しやすい

#### 欠点

- current docs-only derived view からの跳躍が大きい
- `goal_text` only bridge では不足しやすく、`proof_hint` / consumer-local fields を早く要求しやすい
- public-looking contract や actual emitter pressure を premature に強めやすい

### 案 3. notebook workflow と `proof_assistant_adapter` を同時 compare に送る

#### 読み

next reopen で両者を同時に扱い、common envelope と divergence point を一気に比較する。

#### 利点

- 二者の差を 1 回で見やすい

#### 欠点

- current phase では scope が広い
- notebook-sideの lightweight bridge と assistant-sideの machine-facing bridge が混線しやすい
- actual schema pressure を不要に増やしやすい

## current judgment

current L2 で最も自然なのは、
**案 1. concrete notebook workflow pressure を先に扱う**
である。

理由は次の通り。

1. `proof_notebook` はすでに first practical consumer class であり、current docs-only derived view と最も連続的である
2. `goal_text` attachment は notebook-oriented walkthrough と相性がよく、current row core を壊さずに pressure を説明できる
3. `proof_assistant_adapter` を先に扱うと、consumer-local machine schema、stable contract、actual emitter の pressure が一気に強まりやすい
4. notebook workflow pressure を先に切っても、`proof_assistant_adapter` を second practical candidate に残す current order は壊れない

## notebook workflow pressure とみなしてよい最小条件

次の条件が揃ったときだけ、
notebook workflow pressure を concrete reopen candidate とみなしてよい。

1. repo 内で obligation row の human walkthrough を独立 artifact / checklist / compare input として再利用したい
2. current docs-only prose だけでは review / bless / compare の手順説明が不足する
3. stable notebook bridge sketch を導入しても public checker API や actual emitter と混同しない cut を維持できる

## `proof_assistant_adapter` pressure を先に扱ってよい条件

次の条件が notebook workflow pressure より先に concrete になった場合だけ、
案 2 を reopen 候補へ繰り上げてよい。

1. actual theorem consumer が machine-facing row contract を必要とする
2. `goal_text` だけでは不足し、consumer-local field や adapter schema の比較が unavoidable である
3. notebook-side stable sketch を飛ばしても、minimum contract row core と public checker defer threshold を壊さない説明が書ける

## practical examples

### example A — notebook workflow pressure が先に concrete になる case

```text
proof_notebook review = {
  subject_ref = e12-lineage-edge-mismatch,
  checklist = [
    "row ごとに obligation_kind を確認する",
    "evidence_refs から fixture と static gate artifact を開く",
    "goal_text を人間 reviewer が prose と照合する"
  ]
}
```

この case では必要なのは review / checklist / compare flow であり、
machine-facing adapter schema はまだ要らない。

### example B — `proof_assistant_adapter` pressure が先に concrete になる case

```text
assistant_adapter_request = {
  row = {
    obligation_kind = rollback_cut_non_interference,
    evidence_refs = [...],
    goal_text = "..."
  },
  target = "assistant-X"
}
```

この case では row を machine-facing consumer へ渡す schema comparison が必要になり、
notebook workflow より強い contract pressure が発生する。

## current docs-only cut

current task で fixed にしてよいのは次である。

1. next practical reopen は concrete notebook workflow pressure comparison を first choice に置く
2. `proof_assistant_adapter` pressure comparison は second practical candidate に残す
3. notebook workflow pressure が concrete にならない限り、current theorem-line package は checkpoint close を維持する
4. actual emitted notebook artifact と proof assistant specific schema は still later reopen に残す
