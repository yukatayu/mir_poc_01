# hands-on: lean

この文書は、Mir current-L2 の Lean material を初めて読む人向けに説明するハンズオンです。

Lean は定理証明支援系です。
ここでは「プログラムが動くか」ではなく、「小さな数学的主張を機械的に確認できるか」を見ます。

## 最初に結論

現在 repo で確認できる Lean material は 2 種類あります。

- `samples/lean/foundations/`
  小さな実証明です。finite index、lifetime、capture、cost の基礎が実際に Lean で確認できます。
- `samples/lean/clean-near-end/`
  clean near-end sample suite から生成した theorem stub corpus です。sample と theorem path を結ぶ足場ですが、全 domain proof が完成したという意味ではありません。

この handson では、まず foundation file を読みます。

## 使うコマンド

Lean toolchain が入っている環境では、次を実行します。

```bash
source "$HOME/.elan/env"
lean samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean
```

repo に `lakefile.lean` がある環境では `lake build` を使うこともありますが、この repo の current handson では単一 file の `lean` 実行を基本にします。

generated sample stub を同期したい場合は次です。
これは generated stub / manifest を同期する maintenance command です。
読むだけなら不要で、実行すると生成物が更新される場合があります。

```bash
python3 scripts/current_l2_lean_sample_sync.py
```

## 言葉の準備

`theorem` は、証明したい主張です。
Lean では `theorem name : proposition := proof` の形で書きます。

`Prop` は、真か偽かを問う命題の種類です。
たとえば「Session は Step より長い」は命題です。

`inductive` は、値の選択肢を列挙して新しい型を作る書き方です。
ここでは `Lifetime.step` と `Lifetime.session` のような有限の世界を作ります。

`def` は、関数や定義を作る書き方です。

`abbrev` は、長い型名に短い別名を付ける書き方です。

`by` は、ここから証明を書くという合図です。

`cases` は、有限の選択肢を場合分けする証明 tactic です。

`simp` は、定義を展開し、簡単な式を自動で整理する tactic です。

`contradiction` は、仮定が矛盾しているときに証明を閉じる tactic です。

`decide` は、有限で決定可能な命題を計算で確かめる tactic です。

## Step 1: foundation file の全体を見る

対象 file は `samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean` です。

まずは lifetime の部分から見ます。

```lean
namespace CurrentL2FiniteIndexFirstLayer

inductive Lifetime where
  | step
  | session
  deriving DecidableEq, Repr

def outlives : Lifetime -> Lifetime -> Prop
  | Lifetime.session, _ => True
  | Lifetime.step, Lifetime.step => True
  | Lifetime.step, Lifetime.session => False

theorem outlives_refl (l : Lifetime) : outlives l l := by
  cases l <;> simp [outlives]

theorem session_outlives_step : outlives Lifetime.session Lifetime.step := by
  simp [outlives]

theorem step_does_not_outlive_session : ¬ outlives Lifetime.step Lifetime.session := by
  simp [outlives]
```

行ごとの読み方です。

- `namespace CurrentL2FiniteIndexFirstLayer` は、この file の名前空間を作ります。
- `inductive Lifetime where` は、lifetime の有限な世界を作ります。
- `| step` は、短い寿命を表す値です。
- `| session` は、長い寿命を表す値です。
- `deriving DecidableEq, Repr` は、等しいかどうかを判定でき、表示もできるようにする補助指定です。
- `def outlives : Lifetime -> Lifetime -> Prop` は、2 つの lifetime を受け取り、「左が右より長く生きるか」という命題を返す定義です。
- `| Lifetime.session, _ => True` は、`session` は任意の lifetime を outlive すると定義します。
- `| Lifetime.step, Lifetime.step => True` は、`step` は自分自身なら outlive すると定義します。
- `| Lifetime.step, Lifetime.session => False` は、`step` は `session` を outlive しないと定義します。
- `theorem outlives_refl` は、どの lifetime も自分自身を outlive するという定理です。
- `(l : Lifetime)` は、任意の lifetime `l` を受け取るという意味です。
- `: outlives l l` は、証明したい命題です。
- `:= by` は、ここから tactic で証明するという意味です。
- `cases l` は、`l = step` と `l = session` の場合分けをします。
- `<;>` は、場合分けでできた各 goal に同じ tactic を適用します。
- `simp [outlives]` は、`outlives` の定義を展開して簡単にします。
- `theorem session_outlives_step` は、`session` が `step` を outlive するという定理です。
- `theorem step_does_not_outlive_session` は、`step` は `session` を outlive しないという定理です。
- `¬` は「ではない」を表します。

この部分は、Mir の `Region` / lifetime preorder の最小模型です。

## Step 2: capture set の部分を見る

```lean
inductive Capability where
  | roomHistory
  | ephemeralToken
  deriving DecidableEq, Repr

abbrev CaptureSet := Capability -> Bool

def captureSubset (a b : CaptureSet) : Prop :=
  ∀ c, a c = true -> b c = true

def emptyCapture : CaptureSet := fun _ => false

def roomHistoryOnly : CaptureSet
  | Capability.roomHistory => true
  | Capability.ephemeralToken => false

def tokenOnly : CaptureSet
  | Capability.roomHistory => false
  | Capability.ephemeralToken => true

theorem empty_capture_subset_room_history : captureSubset emptyCapture roomHistoryOnly := by
  intro c h
  simp [emptyCapture] at h

theorem token_not_subset_room_history : ¬ captureSubset tokenOnly roomHistoryOnly := by
  intro h
  have token_in_source : tokenOnly Capability.ephemeralToken = true := by
    simp [tokenOnly]
  have token_in_target : roomHistoryOnly Capability.ephemeralToken = true :=
    h Capability.ephemeralToken token_in_source
  simp [roomHistoryOnly] at token_in_target
```

行ごとの読み方です。

- `inductive Capability where` は、capability の有限な世界を作ります。
- `roomHistory` は、部屋の履歴を見る能力です。
- `ephemeralToken` は、短命 token の能力です。
- `abbrev CaptureSet := Capability -> Bool` は、capture set を「capability を受け取って、その capability を含むなら true を返す関数」として表します。
- `def captureSubset (a b : CaptureSet) : Prop` は、capture set `a` が `b` に含まれるかを定義します。
- `∀ c` は、任意の capability `c` について、という意味です。
- `a c = true -> b c = true` は、`a` に `c` が入っているなら `b` にも入っている、という意味です。
- `emptyCapture` は、何も捕まえない capture set です。
- `roomHistoryOnly` は、`roomHistory` だけを含む capture set です。
- `tokenOnly` は、`ephemeralToken` だけを含む capture set です。
- `theorem empty_capture_subset_room_history` は、空集合は `roomHistoryOnly` に含まれるという定理です。
- `intro c h` は、任意の capability `c` と、`emptyCapture c = true` という仮定 `h` を導入します。
- `simp [emptyCapture] at h` は、空集合に何かが入っているという仮定を簡約し、矛盾として扱える状態にします。
- `theorem token_not_subset_room_history` は、`tokenOnly` は `roomHistoryOnly` に含まれないという定理です。
- `intro h` は、反対に「含まれる」と仮定して矛盾を導く証明の始まりです。
- `have token_in_source` は、`ephemeralToken` が source 側に入っていることを示します。
- `have token_in_target` は、もし subset なら target 側にも入っているはずだと導きます。
- `simp [roomHistoryOnly] at token_in_target` は、しかし target 側では false なので矛盾する、というところまで簡約します。

この部分は、Mir sample の `capture_escape_rejected` と対応します。

## Step 3: cost bound の部分を見る

```lean
def remoteCallAllowed (remainingCalls : Nat) : Prop :=
  0 < remainingCalls

def spendRemoteCall (remainingCalls : Nat) : Nat :=
  remainingCalls - 1

theorem zero_budget_rejects_remote_call : ¬ remoteCallAllowed 0 := by
  simp [remoteCallAllowed]

theorem positive_budget_allows_remote_call : remoteCallAllowed 1 := by
  simp [remoteCallAllowed]

theorem single_budget_is_exhausted_after_one_call : spendRemoteCall 1 = 0 := by
  decide
```

行ごとの読み方です。

- `def remoteCallAllowed` は、remote call の残り回数があるかを定義します。
- `(remainingCalls : Nat)` は、残り回数が自然数であることを示します。
- `0 < remainingCalls` は、残り回数が 0 より大きいなら許可するという意味です。
- `def spendRemoteCall` は、remote call を 1 回使った後の残り回数を定義します。
- `remainingCalls - 1` は、1 回分減らすという意味です。
- `zero_budget_rejects_remote_call` は、残り回数 0 では remote call が許可されないという定理です。
- `positive_budget_allows_remote_call` は、残り回数 1 なら remote call が許可されるという定理です。
- `single_budget_is_exhausted_after_one_call` は、1 回分の予算を 1 回使うと 0 になるという定理です。
- `decide` は、この有限で計算可能な主張を Lean に計算させる tactic です。

この部分は、Mir sample の `cost_bound_rejected` と対応します。

## Step 4: Lean を実行する

```bash
source "$HOME/.elan/env"
lean samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean
```

成功した場合、Lean は何も出力せず終了することがあります。
Unix 系の command では、出力がないこと自体は失敗を意味しません。
終了 code が 0 なら成功です。

確認しやすくするには次のように実行します。

```bash
source "$HOME/.elan/env"
lean samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean
echo $?
```

期待される終了 code は次です。

```text
0
```

## Step 5: generated theorem stub との違いを見る

`samples/lean/clean-near-end/` には、active `.mir` sample に対応する Lean file と bundle JSON があります。

たとえば次です。

```text
samples/lean/clean-near-end/01_authorized_declassification/01_authorized_declassification.lean
samples/lean/clean-near-end/01_authorized_declassification/01_authorized_declassification.bundle.json
```

これらは「sample と theorem path を接続する足場」です。
重要なのは、stub があることを「全ての theorem proof が完成した」と読まないことです。

現在の repo-local alpha では、次のように分けて読みます。

- foundation file:
  小さな finite fragment の実証明。
- generated stub:
  sample corpus と theorem pipeline の接続証跡。
- future proof work:
  各 domain sample の full proof obligation をどこまで Lean で discharge するか。

## この handson で確認できたこと

Lean material は、以下を現在の repo で確認できます。

- lifetime preorder の小さな theorem。
- capture subset の positive / negative theorem。
- cost bound の小さな theorem。
- clean near-end sample と Lean stub corpus の接続。
- 「Lean-first proof skeleton」はあるが、「全 final theorem が完成」ではないこと。

## まだ final product ではないこと

次はまだ deferred です。

- full dependent type theory の mechanization。
- all typing / order / modal / model-check obligation の complete proof discharge。
- theorem artifact の final public schema。
- verifier API。
- production proof pipeline。

この handson の到達点は、「repo-local alpha-ready current layer で、small finite-index proof fragment を Lean で確認し、generated theorem stub と実証明の違いを理解できる」です。
