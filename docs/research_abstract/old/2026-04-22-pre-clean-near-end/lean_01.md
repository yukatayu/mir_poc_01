# lean_01

## この文書の目的

この文書は、repo にある Lean ファイルについて、次を初心者向けに整理するための guide である。

- 今、Lean で何を証明できているのか
- それぞれの証明が何の役に立つのか
- その性質が満たされないと、どんな場面で困るのか
- Lean をどう実行するのか
- Lean の出力をどう読むのか

この repo にある Lean は、**全部を 1 つの完成した型理論として mechanize している**わけではない。
現時点の読みは、次の 2 層を明確に分けることにある。

- `samples/lean/foundations/`
  - 実際に小さな証明が通る foundation
- `samples/lean/current-l2/`
  - representative sample から生成された theorem stub
  - Lean には受理されるが、まだ `sorry` を含む

つまり、今の Lean は

- **proof fragment として本当に通る部分**
- **theorem bridge が Lean artifact を正しく出していると確認する部分**

の 2 つを分けて扱っている。

## まず結論

2026-04-21 時点で、この repo では少なくとも次を Lean で確認できる。

- label flow と authority の基本法則
- secret 由来データの declassification が許可される条件 / 許可されない条件
- capture / lifetime / budget に関する first-layer fact
- review-unit から Lean stub への theorem bridge が subject / obligation を取り違えていないこと

一方で、まだ次はやっていない。

- final public theorem contract の完全 mechanization
- generated current-l2 stub から `sorry` を完全に除くこと
- concrete theorem prover binding を public contract として固定すること

## どの Lean ファイルを見ればよいか

最初に見るべきなのは `samples/lean/foundations/` の 4 本である。

| ファイル | 何を証明しているか | それが無いと何に困るか |
|---|---|---|
| `CurrentL2LabelModel.lean` | label flow と authority の基本法則 | label の合成判断がぶれる。例: 一度 high に乗った情報を low に戻してよいか一貫して判定できない |
| `CurrentL2IfcSecretExamples.lean` | secret fingerprint の valid / invalid release | authority のない公開を禁止できない。例: 権限のない room member が secret 由来 fingerprint を publish できてしまう |
| `CurrentL2FiniteIndexFirstLayer.lean` | capture / lifetime / remote-call budget | ephemeral token や quota の制約を破れないと困る。例: session token が room 全体へ漏れる、quota 0 でも外部 API を叩ける |
| `CurrentL2ProofSkeleton.lean` | review-unit と emitted Lean stub の整合 | proof obligation の取り違えを検出できない。例: `no_re_promotion` の review を rollback proof と誤って結び付ける |

そのあとで `samples/lean/current-l2/` を見ると、「今の theorem bridge がどんな Lean artifact を出しているか」が分かる。

## 1. Lean を実行する準備

repo root で次を実行する。

```bash
source "$HOME/.elan/env"
lean --version
```

2026-04-21 の再実行結果:

```text
Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)
```

以後、この文書の Lean command はすべて repo root でそのままコピペすればよい。

## 2. 一番大事な読み方

Lean の出力は、初心者向けにはまず次の 3 種類に分けて読むとよい。

### 2-1. 何も出ずに成功する

これは「証明が通った」という意味である。

```bash
source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2IfcSecretExamples.lean
```

2026-04-21 の再実行では、**出力なし**で終了した。

意味:

- syntax は正しい
- theorem / example が全部通った
- この file の mechanization は、少なくとも Lean 上では整合している

### 2-2. warning は出るが成功する

これは「Lean file としては受理されたが、まだ placeholder が残っている」という意味である。

```bash
source "$HOME/.elan/env" && \
lean samples/lean/current-l2/p06-typed-proof-owner-handoff/p06-typed-proof-owner-handoff.lean
```

2026-04-21 の再実行結果:

```text
samples/lean/current-l2/p06-typed-proof-owner-handoff/p06-typed-proof-owner-handoff.lean:12:8: warning: declaration uses `sorry`
```

意味:

- Lean file 自体は読めた
- theorem 名や module 構造は整っている
- ただし theorem body の本体はまだ placeholder

つまり、これは **bridge artifact の成功**であって、**完全証明の成功**ではない。

### 2-3. error で止まる

これは「その主張は今の仮定では証明できない」という意味である。

```bash
bash -lc 'cat > /tmp/lean_beginner_unsolved.lean <<\"EOF\"
inductive SecurityLabel where
  | low
  | high

def flowsTo : SecurityLabel → SecurityLabel → Prop
  | .low, _ => True
  | .high, .high => True
  | .high, .low => False

def CanDeclassify (hasAuthority : Bool) (fromLabel toLabel : SecurityLabel) : Prop :=
  hasAuthority = true ∨ flowsTo fromLabel toLabel

example : CanDeclassify false SecurityLabel.high SecurityLabel.low := by
  simp [CanDeclassify, flowsTo]
EOF
source \"$HOME/.elan/env\" && lean /tmp/lean_beginner_unsolved.lean'
```

2026-04-21 の再実行結果:

```text
/tmp/lean_beginner_unsolved.lean:13:70: error: unsolved goals
⊢ False
```

意味:

- 「authority なしで high -> low を許したい」という主張は通らない
- `⊢ False` は、最終的に偽を示さないと goal が閉じないという意味

一言で言うと、**その主張は今のルールでは無理**ということである。

## 3. Foundation 1: `CurrentL2LabelModel.lean`

```bash
source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2LabelModel.lean
```

### ここで証明していること

この file は、two-point lattice に近い最小 label model を置いている。

主な theorem:

- `flowsTo_refl`
  - どの label も自分自身には流せる
- `low_flows_to_any`
  - `low` はどの label にも流せる
- `flowsTo_trans`
  - flow 判断は推移する
- `secret_to_public_requires_authority`
  - `high -> low` は flow だけでは通らない
- `no_declassify_without_authority`
  - authority なし declassification は不可能
- `authority_enables_secret_to_public`
  - authority があれば `high -> low` を許せる

### 何の役に立つか

この file は「秘密データをそのまま public に出してよいか」を判断する最小ルールを支えている。

例えば `flowsTo_trans` がないと、複数段階の label 推論を一貫して扱いにくい。
一言の具体例:

- `low -> high` はよい
- `high -> high` もよい
- なのに合成した `low -> high` を一般則として扱えない

という状態になり、checker の説明が不安定になる。

### 初心者向けの見方

ここで大事なのは、**value そのものではなく「label 間の関係」**を Lean で固めている点である。
後で出てくる `CurrentL2IfcSecretExamples.lean` は、この基礎の上に具体的な payload を乗せている。

## 4. Foundation 2: `CurrentL2IfcSecretExamples.lean`

```bash
source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2IfcSecretExamples.lean
```

### ここで証明していること

この file は、secret key から作る fingerprint を例にして、valid / invalid な release を具体的に表現している。

主な theorem:

- `declassify_preserves_value`
  - declassify は payload を勝手に書き換えない
- `low_to_low_release_without_authority_is_available`
  - public な情報を public のまま扱うのに authority は要らない
- `authorized_public_fingerprint_keeps_payload`
  - authority があるとき、secret 由来 fingerprint を public 側へ下ろせる
- `authorized_live_fingerprint_release_has_witness`
  - その release には実際に witness が存在する
- `unauthorized_live_fingerprint_release_is_impossible`
  - authority なしでは、その release を支える witness を作れない

### 何の役に立つか

この file がないと、「authority があるときは通す」「ないときは止める」という current IFC line を、具体例つきで説明しにくい。

一言の具体例:

- 困る例: 権限のない player が secret key 由来 fingerprint を room_members に publish できてしまう

`unauthorized_live_fingerprint_release_is_impossible` は、これを Lean 側で明示的に否定している。

### その場で通る最小例

次は authorized な例で、何も出ずに成功する。

```bash
bash -lc 'cat > /tmp/lean_beginner_ok.lean <<\"EOF\"
inductive SecurityLabel where
  | low
  | high

def flowsTo : SecurityLabel → SecurityLabel → Prop
  | .low, _ => True
  | .high, .high => True
  | .high, .low => False

def CanDeclassify (hasAuthority : Bool) (fromLabel toLabel : SecurityLabel) : Prop :=
  hasAuthority = true ∨ flowsTo fromLabel toLabel

example : CanDeclassify true SecurityLabel.high SecurityLabel.low := by
  simp [CanDeclassify, flowsTo]
EOF
source \"$HOME/.elan/env\" && lean /tmp/lean_beginner_ok.lean'
```

2026-04-21 の再実行では **出力なしで成功**した。

## 5. Foundation 3: `CurrentL2FiniteIndexFirstLayer.lean`

```bash
source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean
```

### ここで証明していること

この file は、strong typing の full dependent calculus までは行かずに、

- lifetime
- capture set
- remote-call budget

の first-layer fact を mechanize している。

主な theorem:

- `outlives_refl`
- `session_outlives_step`
- `step_does_not_outlive_session`
- `outlives_trans`
- `capture_subset_trans`
- `ephemeral_only_not_subset_of_empty`
- `room_history_only_not_subset_of_ephemeral_only`
- `single_budget_is_exhausted_after_one_call`
- `two_budget_still_allows_after_one_call`

### 何の役に立つか

この file がないと、`p15` や `p16` の rejection を「ただの個別ルール」としてしか説明できない。

一言の具体例:

- lifetime/capture が弱いと、ephemeral session token を room 全体に publish できてしまう
- budget の減少則が弱いと、quota 1 の API call を 2 回通せてしまう

この repo では、そうした bad pattern を first-layer の fact として押さえている。

### 初心者向けの読み方

ここで見ているのは「一般の依存型言語を全部 mechanize した」という話ではない。
そうではなく、**今の sample を支える最小限の decidable fragment を Lean で固めた**と読むのが正確である。

## 6. Foundation 4: `CurrentL2ProofSkeleton.lean`

```bash
source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2ProofSkeleton.lean
```

### ここで証明していること

この file は domain theorem を証明するのではなく、**review-unit から Lean stub を作る橋渡しの構造**を証明している。

主な theorem:

- `mkLeanStub_preserves_subject`
  - subject ref を取り違えない
- `mkLeanStub_preserves_obligation`
  - obligation kind を取り違えない
- `mkLeanStub_names_theorem`
  - theorem name の組み立てが決まっている
- `emitStubs_length`
  - review unit の数と emitted stub の数が合う
- `e5_first_stub_subject`
  - `e5` の 1 本目の stub が正しい subject を持つ
- `e5_second_stub_obligation`
  - `e5` の 2 本目の stub が `noRePromotion` obligation に対応する

### 何の役に立つか

この file がないと、generated Lean artifact は作れても、
「本当にその sample と proof obligation に対応しているのか」を Lean 側で確認しにくい。

一言の具体例:

- 困る例: `no_re_promotion` の review unit を誤って `rollback_cut_non_interference` として出力してしまう

`e5_second_stub_obligation` は、少なくともこの代表例で obligation の並びが崩れていないことを確認している。

## 7. generated current-L2 stub は何を意味するのか

`samples/lean/current-l2/` は、foundation と違って theorem bridge の artifact である。

代表例として `p06` を見る。

```bash
source "$HOME/.elan/env" && \
lean samples/lean/current-l2/p06-typed-proof-owner-handoff/p06-typed-proof-owner-handoff.lean
```

実際の出力:

```text
samples/lean/current-l2/p06-typed-proof-owner-handoff/p06-typed-proof-owner-handoff.lean:12:8: warning: declaration uses `sorry`
```

対応する file の中身は次のようになっている。

```lean
theorem p06_typed_proof_owner_handoff__rollback_cut_non_interference : True := by
  sorry
```

ここで分かるのは次の 2 点である。

- theorem 名や module 構造は出せている
- しかし proof body はまだ空である

したがって、この warning を見たときは

- 「Lean file として受理された」
- 「でも theorem discharge はまだ終わっていない」

と読むのが正しい。

## 8. corpus 全体を同期して確認する

foundation と generated stub をまとめて確認したいときは、次を使う。

```bash
source "$HOME/.elan/env" && python3 scripts/current_l2_lean_sample_sync.py
```

2026-04-21 の再実行では、foundation 側はすべて `success: true` で、generated stub 側には `warning: declaration uses \`sorry\`` が残っていた。

初心者向けには、ここを次のように読むとよい。

- foundation `success: true`
  - 実際の小さな証明が通っている
- generated stub `success: true` + `warning`
  - artifact は生成できているが、本証明は後段

## 9. test で何を固定しているのか

Lean の source text と説明文が drift していないかは、Python unit test でも確認している。

```bash
python3 -m unittest scripts/tests/test_current_l2_lean_sample_sync.py
```

2026-04-21 の再実行結果:

```text
........
----------------------------------------------------------------------
Ran 9 tests in 0.001s

OK
```

この test は、例えば次を見ている。

- representative sample set が欠けていないか
- foundation file 一覧が欠けていないか
- generated stub explanation が `sorry` と scope を明示しているか
- added theorem 名が generator source に残っているか

## 10. Lean 初心者向けの入出力の読み方

Lean の出力は、次の順番で読むと混乱しにくい。

1. `error` があるか
   - あればまず失敗
2. `warning` があるか
   - success だが注意が必要
3. 何も出ないか
   - 多くの場合は成功

この repo では特に次を覚えておくとよい。

- `error: unsolved goals`
  - 証明が閉じていない
- `⊢ False`
  - 今の仮定ではその主張は成り立たない
- `warning: declaration uses 'sorry'`
  - theorem body に placeholder がある

## 11. 何が本質で、何が目的か

この repo の Lean の本質は、「最終理論を全部 mechanize した」と主張することではない。
本質は、current-L2 の current line について、

- 本当に通る小さな proof fragment を foundation として置く
- その上で theorem bridge がどんな artifact を出すかを inspectable にする
- 両者を混同しない

ことにある。

目的意識を一言で言うと、

- **foundation 側で、今必要な不変条件を Lean で確かめる**
- **generated stub 側で、proof artifact の受け渡し経路を壊さない**

である。

## 12. 次に読むとよい文書

- `docs/research_abstract/static_analysis_01.md`
  - Problem 1 の typed / theorem / model-check 全体の流れ
- `samples/lean/README.md`
  - Lean corpus 全体の役割整理
- `samples/lean/foundations/*.md`
  - 各 foundation file の短い説明

Lean だけを最短で再確認したいときは、次の 4 本から入るとよい。

```bash
source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2LabelModel.lean
source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2IfcSecretExamples.lean
source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean
source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2ProofSkeleton.lean
```
