# lean_01_detail

## この文書の目的

この文書は [`docs/research_abstract/lean_01.md`](./lean_01.md) を、**紹介順を保ったまま**、Lean 初学者でも単体で読めるように詳説した standalone 文書である。

主眼は次の 3 点にある。

1. この repo で Lean が今どこまで使われているかを、foundation と generated stub に分けて正確に説明する。
2. `samples/lean/foundations/` の 4 本と、generated stub の代表例、最小 success / error 例について、**コード全文**と**実行例**をその場で読めるようにする。
3. Lean の出力に出てくる `warning: declaration uses \`sorry\``、`error: unsolved goals`、`⊢ False` が何を意味するのかを、初学者向けに丁寧に説明する。

この repo の Lean は、**全部を 1 つの完成済み型理論として mechanize したものではない**。現在の読みは、次の 2 層を明確に分けることにある。

- `samples/lean/foundations/`
  - 実際に小さな定理や例が通る foundation。
- `samples/lean/current-l2/`
  - representative sample から生成された theorem stub。
  - Lean には受理されるが、まだ `sorry` を含む。

したがって、現在の Lean で確認しているのは次の 2 種類である。

- **proof fragment として本当に通る部分**
- **theorem bridge が Lean artifact を正しく出していると確認する部分**

## まず結論

2026-04-21 時点で、この repo では少なくとも次を Lean で再確認できる。

- label flow と authority の最小法則
- secret 由来データの valid / invalid な release 条件
- capture / lifetime / remote-call budget の first-layer fact
- review-unit から Lean stub への theorem bridge が、subject と obligation kind を取り違えていないこと

一方で、まだ次は終わっていない。

- final public theorem contract の完全 mechanization
- generated current-L2 stub から `sorry` を完全に除くこと
- concrete theorem prover binding を public contract として固定すること

## どの Lean ファイルを見るのか

最初に見るべき 4 本は `samples/lean/foundations/` の次のファイルである。

| ファイル | 何を証明しているか | それが無いと何に困るか |
|---|---|---|
| `CurrentL2LabelModel.lean` | label flow と authority の最小法則 | 「一度 secret に上がった情報を public に戻してよいか」を一貫して判定できなくなる |
| `CurrentL2IfcSecretExamples.lean` | secret fingerprint の valid / invalid release | authority のない公開を禁止できなくなる |
| `CurrentL2FiniteIndexFirstLayer.lean` | lifetime / capture / budget の first-layer fact | ephemeral token の漏えい、quota 超過呼び出しの拒否を説明しにくくなる |
| `CurrentL2ProofSkeleton.lean` | review-unit と emitted Lean stub の整合 | proof obligation の取り違えを検出しにくくなる |

そのあとで `samples/lean/current-l2/` の generated stub を見ると、**現在の theorem bridge がどんな Lean artifact を出しているか**が分かる。

## Lean のキーワードとタクティクの超入門

コードを読む前に、この文書でよく出る Lean の言葉をまとめておく。

- `inductive`
  - 列挙型や帰納的データ型を作る宣言である。今回の `SecurityLabel` や `Lifetime` はこれで作っている。
- `structure`
  - フィールドを持つレコード型を作る宣言である。`Labeled`、`ReviewUnit`、`LeanStub` がそうである。
- `abbrev`
  - 既存の型に短い別名を付ける。`SecretKey := Labeled high String` は「長い型名を読みやすくするための別名」である。
- `def`
  - 関数や定義を置く。`flowsTo`、`join`、`fingerprint`、`mkLeanStub` などがこれで定義されている。
- `theorem`
  - 証明したい命題に名前を付ける。定理や補題を宣言するときに使う。
- `example`
  - 名前を残さない小さな例を示す。最小 success / error 例ではこれを使う。
- `by`
  - ここから証明を書く、という合図である。`:= by` のあとに tactic を並べる。
- `simp`
  - 定義を展開し、`True` / `False` や簡単な等式を自動で整理する。
- `simpa`
  - `simp` した結果として「これでちょうど示したい形になる」ときに使う。
- `rfl`
  - 定義どおりに見れば明らかな等式であることを示す。reflexivity の略と考えるとよい。
- `cases`
  - 場合分けをする。`SecurityLabel` なら `low` と `high` に分ける。
- `intro`
  - 含意や否定の証明で、仮定を 1 つ受け取る。
- `exact`
  - いま手元にある証拠を、そのまま goal に渡す。
- `refine`
  - 証明の骨組みを先に置き、残りを `?_` で後回しにする。
- `rcases`
  - 存在証明や積の仮定を分解して、中身を取り出す。
- `constructor`
  - ある型のコンストラクタを順番に埋める tactic である。今回の対象コードでは使っていないが、`∃` や `And` を作るときによく使う。
- `contradiction`
  - 仮定の中に矛盾があるとき、自動で goal を閉じようとする tactic である。今回の対象コードでは使っていない。

## 1. Lean を実行する準備

repo root で次を実行する。

```bash
source "$HOME/.elan/env"
lean --version
```

2026-04-21 の再実行結果は次のとおり。

```text
Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)
```

このコマンドで確認しているのは次の点である。

- `elan` による Lean 環境が現在の shell に入っているか
- `lean` コマンドが見つかるか
- どのバージョンで以下のサンプルを再実行したか

初心者向けには、「以後の結果はこの Lean 4.29.1 環境で確認した」と読むとよい。

## 2. 一番大事な読み方

Lean の出力は、まず次の 3 種類に分けて読むと混乱しにくい。

1. **何も出ずに成功する**
   - 証明が通っている。
2. **warning は出るが成功する**
   - Lean file としては受理されたが、未完成な箇所が残っている。
3. **error で止まる**
   - その主張は今の仮定では証明できないか、証明を書き切れていない。

### 2-1. 何も出ずに成功する例

コマンド:

```bash
source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2IfcSecretExamples.lean
```

2026-04-21 の再実行結果:

実行結果全文は **出力なし** である。

これは次を意味する。

- syntax が正しい
- file 内の `theorem` と `example` が全部通った
- 少なくとも Lean 上では、この proof fragment は整合している

### 2-2. warning は出るが成功する例

コマンド:

```bash
source "$HOME/.elan/env" && lean samples/lean/current-l2/p06-typed-proof-owner-handoff/p06-typed-proof-owner-handoff.lean
```

2026-04-21 の再実行結果:

```text
samples/lean/current-l2/p06-typed-proof-owner-handoff/p06-typed-proof-owner-handoff.lean:12:8: warning: declaration uses `sorry`
```

これは次を意味する。

- Lean file 自体は読めた
- theorem 名、namespace、module 相当の構造は整っている
- ただし、その theorem 本体はまだ `sorry` という placeholder で埋めている

`declaration uses sorry` を初学者向けに言い換えると、**「この定理は、いまは『あとでちゃんと証明する』と約束して通している」** である。したがって、これは **artifact acceptance の成功**であって、**完全証明の成功**ではない。

### 2-3. error で止まる最小例

この例は「authority なしで `high -> low` の declassification を通そうとすると失敗する」ことを示す、最小の error 例である。

#### コード全文

```lean
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
```

#### 行ごとの解説

- 行 1
  - `inductive SecurityLabel where` は、`SecurityLabel` という新しい型を作り始める宣言である。
- 行 2
  - `low` というコンストラクタを定義している。公開側ラベルと思えばよい。
- 行 3
  - `high` というコンストラクタを定義している。秘密側ラベルと思えばよい。
- 行 5
  - `flowsTo` という関係を定義している。型は「ラベル 2 個を受け取って命題 `Prop` を返す」である。
- 行 6
  - `.low, _ => True` は「`low` からはどこへでも流せる」と定めている。`_` は「何でもよい」の意味である。
- 行 7
  - `.high, .high => True` は「`high` から `high` へは流せる」と定めている。
- 行 8
  - `.high, .low => False` は「`high` から `low` へは流せない」と定めている。
- 行 10-11
  - `CanDeclassify` は declassification の条件である。
  - `hasAuthority = true ∨ flowsTo fromLabel toLabel` なので、「authority がある」か「そもそも普通の flow として許される」かのどちらかで通る。
- 行 13
  - `example` は名前を付けない小さな証明である。
  - ここでは `CanDeclassify false high low`、つまり「authority がないのに `high -> low` を通したい」と主張している。
- 行 13 の `:= by`
  - ここから tactic で証明を書く、という意味である。
- 行 14
  - `simp [CanDeclassify, flowsTo]` は、定義を展開して機械的に簡約しようとしている。
  - しかし今回は `false = true ∨ False` になり、最終的に `False` を示さないと閉じないので失敗する。

#### 実行コマンド

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
source "$HOME/.elan/env" && lean /tmp/lean_beginner_unsolved.lean'
```

#### 実行結果全文

```text
/tmp/lean_beginner_unsolved.lean:13:70: error: unsolved goals
⊢ False
```

#### この出力をどう読むか

- `error: unsolved goals`
  - goal、つまり「まだ示すべきこと」が残ったまま終了した、という意味である。
  - 今回は `simp` を 1 回かけても証明が完了しなかった。
- `⊢ False`
  - いま Lean の目の前に残っている目標が `False` だという意味である。
  - `False` は「偽」を表す命題なので、これを証明できるのは仮定が矛盾しているときだけである。
  - 今回は矛盾の材料がないので、当然閉じない。

一言で言うと、この error は **「authority なしで secret を public に落とす主張は、今のルールでは証明できない」** と読めばよい。

## 3. Foundation 1: `CurrentL2LabelModel.lean`

この file は、label flow と authority の最小法則を置く foundation である。後続の IFC 具体例は、この土台の上に乗る。

### コード全文

```lean
/-!
current-l2 first IFC / label-model fragment

This file is intentionally small and self-contained. It is not the final public type
system. It fixes only the minimal lattice and authority-sensitive lemmas needed for the
current checker-adjacent reading.
-/

namespace CurrentL2

inductive SecurityLabel where
  | low
  | high
deriving DecidableEq, Repr

open SecurityLabel

def flowsTo : SecurityLabel → SecurityLabel → Prop
  | low, _ => True
  | high, high => True
  | high, low => False

def join : SecurityLabel → SecurityLabel → SecurityLabel
  | high, _ => high
  | _, high => high
  | low, low => low

def CanDeclassify (hasAuthority : Bool) (fromLabel toLabel : SecurityLabel) : Prop :=
  hasAuthority = true ∨ flowsTo fromLabel toLabel

theorem flowsTo_refl (label : SecurityLabel) : flowsTo label label := by
  cases label <;> simp [flowsTo]

theorem low_flows_to_any (label : SecurityLabel) : flowsTo low label := by
  cases label <;> simp [flowsTo]

theorem flowsTo_trans
    {a b c : SecurityLabel}
    (hab : flowsTo a b)
    (hbc : flowsTo b c) :
    flowsTo a c := by
  cases a <;> cases b <;> cases c <;> simp [flowsTo] at hab hbc ⊢

theorem flowsTo_join_left (a b : SecurityLabel) : flowsTo a (join a b) := by
  cases a <;> cases b <;> simp [flowsTo, join]

theorem flowsTo_join_right (a b : SecurityLabel) : flowsTo b (join a b) := by
  cases a <;> cases b <;> simp [flowsTo, join]

theorem secret_to_public_requires_authority :
    ¬ flowsTo high low := by
  simp [flowsTo]

theorem no_declassify_without_authority :
    ¬ CanDeclassify false high low := by
  simp [CanDeclassify, flowsTo]

theorem authority_enables_secret_to_public :
    CanDeclassify true high low := by
  simp [CanDeclassify]

end CurrentL2
```

### 行ごとの解説

- 行 1-7
  - 先頭の `/-! ... -/` はドキュメントコメントである。
  - 「この file は最終 public type system ではなく、current checker-adjacent reading に必要な最小 fragment だけを固定する」と宣言している。
- 行 9
  - `namespace CurrentL2` は名前の衝突を避けるための入れ物である。
- 行 11-14
  - `SecurityLabel` を `low` と `high` の 2 値で定義する。
  - `deriving DecidableEq, Repr` により、等値判定と表示用の仕組みを自動導出する。
- 行 16
  - `open SecurityLabel` により、`SecurityLabel.low` ではなく単に `low` と書ける。
- 行 18-21
  - `flowsTo` は「この label からその label へ安全に流してよいか」を決める規則である。
  - `low` はどこへでも流せる。
  - `high` は `high` へは流せるが、`low` へは流せない。
- 行 23-26
  - `join` は 2 つの label を合わせたときの上限側ラベルである。
  - どちらかが `high` なら結果は `high`、両方 `low` のときだけ `low` になる。
- 行 28-29
  - `CanDeclassify` は declassification の条件である。
  - authority があれば通るし、authority がなくても通常の flow が許すなら通る。
- 行 31-32
  - `flowsTo_refl` は反射律である。どの label も自分自身には流せる。
  - `cases label` で `low` / `high` の 2 場合に分け、`simp` で片付けている。
- 行 34-35
  - `low_flows_to_any` は `low` がどこへでも流せることを示す。
  - これがないと「public 情報を public や secret 側に使う」基本操作を一般則として扱いにくい。
- 行 37-42
  - `flowsTo_trans` は推移律である。
  - `a` から `b`、`b` から `c` に流せるなら、`a` から `c` にも流せる。
  - `cases a <;> cases b <;> cases c` は 3 変数全部を場合分けしている。
  - 末尾の `⊢` は「現在の goal に対しても `simp` を使う」という記法である。
- 行 44-45
  - `flowsTo_join_left` は、左の label から `join a b` には流せることを示す。
  - join が上側の label なので、元の情報をそこへ持ち上げるのは安全だ、という意味である。
- 行 47-48
  - `flowsTo_join_right` は右側について同じことを示す。
- 行 50-52
  - `secret_to_public_requires_authority` は `high -> low` が plain な flow としては不可能だと示す。
  - これがないと secret を authority なしで public に落としてもよいことになってしまう。
- 行 54-56
  - `no_declassify_without_authority` は、authority なし declassification を直接否定している。
  - IFC 説明の中心になる補題である。
- 行 58-60
  - `authority_enables_secret_to_public` は、authority があるときだけ `high -> low` を許せることを示す。
  - 「全面禁止」ではなく、「明示 authority による解除」だと分かるのが重要である。
- 行 62
  - namespace を閉じる。

### 実行コマンド

```bash
source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2LabelModel.lean
```

### 実行結果全文

実行結果全文は **出力なし** である。

### 何を確認しているのか

このコマンドは、上の最小 label model とその定理群が Lean 上で全部通ることを確認している。

特に重要なのは次の 3 点である。

- `flowsTo_trans`
  - label 関係を複数段にまたがって使っても破綻しない。
- `no_declassify_without_authority`
  - authority なし declassification を禁止できる。
- `authority_enables_secret_to_public`
  - authority あり解除という経路は残している。

この foundation がないと、「なぜ `p10` は通り、`p11` は止まるのか」を Lean 側の最小法則として説明しにくくなる。

## 4. Foundation 2: `CurrentL2IfcSecretExamples.lean`

この file は、前節の label model を使って、secret key 由来 fingerprint の valid / invalid release を具体例で示す。

### コード全文

```lean
/-!
current-l2 IFC secret examples fragment

This file keeps the first concrete IFC examples for Package 56 in one self-contained
Lean artifact. The goal is not the final public typed surface. The goal is to make the
secret-key valid/invalid reading executable at the proof-fragment level.
-/

namespace CurrentL2IfcSecretExamples

inductive SecurityLabel where
  | low
  | high
deriving DecidableEq, Repr

open SecurityLabel

def flowsTo : SecurityLabel → SecurityLabel → Prop
  | low, _ => True
  | high, high => True
  | high, low => False

def CanDeclassify (hasAuthority : Bool) (fromLabel toLabel : SecurityLabel) : Prop :=
  hasAuthority = true ∨ flowsTo fromLabel toLabel

structure Labeled (label : SecurityLabel) (α : Type) where
  value : α

abbrev SecretKey := Labeled high String
abbrev SecretFingerprint := Labeled high String
abbrev PublicFingerprint := Labeled low String

def fingerprint (key : SecretKey) : SecretFingerprint :=
  { value := "fp:" ++ key.value }

def declassify
    (hasAuthority : Bool)
    {fromLabel toLabel : SecurityLabel}
    (_proof : CanDeclassify hasAuthority fromLabel toLabel)
    (value : Labeled fromLabel α) :
    Labeled toLabel α :=
  { value := value.value }

theorem declassify_preserves_value
    (hasAuthority : Bool)
    {fromLabel toLabel : SecurityLabel}
    (proof : CanDeclassify hasAuthority fromLabel toLabel)
    (value : Labeled fromLabel α) :
    (declassify hasAuthority proof value).value = value.value := by
  rfl

theorem no_secret_release_without_authority :
    ¬ CanDeclassify false high low := by
  simp [CanDeclassify, flowsTo]

theorem authorized_secret_release_is_available :
    CanDeclassify true high low := by
  simp [CanDeclassify]

theorem low_to_low_release_without_authority_is_available :
    CanDeclassify false low low := by
  simp [CanDeclassify, flowsTo]

def publicAuditNote : Labeled low String :=
  { value := "audit-ok" }

def unchangedPublicAuditNote : Labeled low String :=
  declassify false low_to_low_release_without_authority_is_available publicAuditNote

theorem unchanged_public_audit_note_keeps_payload :
    unchangedPublicAuditNote.value = "audit-ok" := by
  simpa [unchangedPublicAuditNote, publicAuditNote] using
    declassify_preserves_value
      false
      low_to_low_release_without_authority_is_available
      publicAuditNote

def liveSecretKey : SecretKey :=
  { value := "sk_live" }

theorem fingerprint_keeps_secret_payload :
    (fingerprint liveSecretKey).value = "fp:sk_live" := by
  rfl

def authorizedPublicFingerprint : PublicFingerprint :=
  declassify true authorized_secret_release_is_available (fingerprint liveSecretKey)

theorem authorized_public_fingerprint_keeps_payload :
    authorizedPublicFingerprint.value = "fp:sk_live" := by
  simpa [authorizedPublicFingerprint, fingerprint, liveSecretKey] using
    declassify_preserves_value
      true
      authorized_secret_release_is_available
      (fingerprint liveSecretKey)

theorem invalid_release_has_no_authority_proof :
    ¬ ∃ _proof : CanDeclassify false high low, True := by
  intro h
  rcases h with ⟨proof, _⟩
  exact no_secret_release_without_authority proof

theorem valid_release_has_authority_proof :
    ∃ _proof : CanDeclassify true high low, True := by
  exact ⟨authorized_secret_release_is_available, trivial⟩

theorem authorized_live_fingerprint_release_has_witness :
    ∃ proof : CanDeclassify true high low,
      (declassify true proof (fingerprint liveSecretKey)).value = "fp:sk_live" := by
  refine ⟨authorized_secret_release_is_available, ?_⟩
  simpa [fingerprint, liveSecretKey] using
    declassify_preserves_value
      true
      authorized_secret_release_is_available
      (fingerprint liveSecretKey)

theorem unauthorized_live_fingerprint_release_is_impossible :
    ¬ ∃ proof : CanDeclassify false high low,
      (declassify false proof (fingerprint liveSecretKey)).value = "fp:sk_live" := by
  intro h
  rcases h with ⟨proof, _⟩
  exact no_secret_release_without_authority proof

end CurrentL2IfcSecretExamples
```

### 行ごとの解説

- 行 1-7
  - 先頭コメントで「これは final public typed surface ではなく、secret-key の valid / invalid 読みを proof-fragment level で実行可能にする file だ」と明示している。
- 行 9-24
  - `SecurityLabel`、`flowsTo`、`CanDeclassify` を再び定義している。
  - これは file 単体で読める self-contained artifact にするためである。
- 行 26-27
  - `structure Labeled` は「値 `value` に label が付いている」ことを型で表すレコードである。
  - `α : Type` の `α` は値本体の型で、ここでは `String` を入れて使っている。
- 行 29-31
  - `SecretKey`、`SecretFingerprint`、`PublicFingerprint` は読みやすさのための別名である。
  - 実際には全部 `Labeled ... String` だが、label が違う。
- 行 33-34
  - `fingerprint` は secret key から secret fingerprint を作る。
  - ここでは単純に `"fp:" ++ key.value` という文字列結合で表している。
- 行 36-42
  - `declassify` は、authority などの証拠 `_proof` があるときに label を変えて値を運ぶ関数である。
  - 本体 `{ value := value.value }` は payload をそのまま渡しており、label だけが変わる。
  - 引数名が `_proof` なのは、「証拠は使うが、本体では直接参照しない」という Lean の慣習である。
- 行 44-50
  - `declassify_preserves_value` は declassification が payload を壊さないことを示す。
  - `rfl` で終わるのは、関数本体を見れば定義どおり明らかだからである。
- 行 52-54
  - `no_secret_release_without_authority` は authority なしの `high -> low` が不可能だと示す。
- 行 56-58
  - `authorized_secret_release_is_available` は authority ありなら同じ遷移が可能だと示す。
- 行 60-62
  - `low_to_low_release_without_authority_is_available` は、public 情報を public のまま扱うときに authority が要らないことを示す。
- 行 64-68
  - `publicAuditNote` と `unchangedPublicAuditNote` は、もともと `low` の値を `low` のまま運ぶ具体例である。
- 行 70-76
  - `unchanged_public_audit_note_keeps_payload` は、その public な例で payload が変わらないことを確認している。
  - `simpa ... using ...` は、既存の補題 `declassify_preserves_value` を、いま欲しい具体形へ簡約して再利用している。
- 行 78-83
  - `liveSecretKey` を具体的な secret key として置き、その fingerprint が `"fp:sk_live"` になることを示す。
- 行 85-94
  - `authorizedPublicFingerprint` は authority あり declassification の具体例である。
  - その下の theorem は、公開側へ落としても payload が `"fp:sk_live"` のままだと示す。
- 行 96-100
  - `invalid_release_has_no_authority_proof` は「authority なし release を支える proof は存在しない」と示す。
  - `intro h` で「もし存在すると仮定しよう」と始め、`rcases` で中身の `proof` を取り出し、既存補題で矛盾に落としている。
- 行 102-104
  - `valid_release_has_authority_proof` は逆に「authority があれば証拠を 1 つ作れる」と示す。
  - `trivial` は `True` の証明を即座に与える。
- 行 106-114
  - `authorized_live_fingerprint_release_has_witness` は、authority あり release の witness が具体的に存在することを示す。
  - `refine ⟨..., ?_⟩` は「存在証明の 1 つ目の要素だけ先に埋め、残りは後で示す」という書き方である。
- 行 116-121
  - `unauthorized_live_fingerprint_release_is_impossible` は、authority なし witness が存在しないことを示す。
  - current IFC line の禁止側を Lean で固定する最重要部分の 1 つである。
- 行 123
  - namespace を閉じる。

### 実行コマンド

```bash
source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2IfcSecretExamples.lean
```

### 実行結果全文

実行結果全文は **出力なし** である。

### 何を確認しているのか

このコマンドは、secret fingerprint の valid / invalid release に関する上の具体例がすべて Lean で通ることを確認している。

特に重要なのは次である。

- `authorized_live_fingerprint_release_has_witness`
  - authority があるときは valid release の witness を作れる。
- `unauthorized_live_fingerprint_release_is_impossible`
  - authority がないときは witness 自体が存在しない。

これがないと、「権限のない room member が secret 由来 fingerprint を publish できない」という current IFC line を、具体例つきで示しにくい。

### その場で通る最小 success 例

次は、authority があるときの最小 success 例である。

#### コード全文

```lean
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
```

#### 行ごとの解説

- 行 1-3
  - `SecurityLabel` を `low` / `high` で定義している。
- 行 5-8
  - `flowsTo` を定義している。
  - `high -> low` だけを `False` にしているのが核心である。
- 行 10-11
  - `CanDeclassify` は authority か通常 flow のどちらかで通る、という条件である。
- 行 13
  - 今回の `example` は `CanDeclassify true high low`、つまり「authority ありなら secret を public に落としてよい」と主張している。
- 行 14
  - `simp` が `hasAuthority = true` 側を使って goal を閉じる。

#### 実行コマンド

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
source "$HOME/.elan/env" && lean /tmp/lean_beginner_ok.lean'
```

#### 実行結果全文

実行結果全文は **出力なし** である。

#### この出力をどう読むか

出力がないのは、「authority ありなら `high -> low` を通してよい」という主張が、現在の最小ルールに一致しているからである。

## 5. Foundation 3: `CurrentL2FiniteIndexFirstLayer.lean`

この file は、full dependent calculus 全体ではなく、current sample を支える最小の decidable fragment として、lifetime / capture set / remote-call budget を mechanize している。

### コード全文

```lean
/-!
current-l2 finite-index first-layer fragment

This file keeps the smallest self-contained Lean facts for the finite-index first layer:
capture-set inclusion, lifetime preorder, and simple remote-call budget.
It is not the final public typed calculus.
-/

namespace CurrentL2FiniteIndexFirstLayer

inductive Lifetime where
  | step
  | session
deriving DecidableEq, Repr

open Lifetime

def outlives : Lifetime → Lifetime → Prop
  | session, _ => True
  | step, step => True
  | step, session => False

theorem outlives_refl (lifetime : Lifetime) : outlives lifetime lifetime := by
  cases lifetime <;> simp [outlives]

theorem session_outlives_step : outlives session step := by
  simp [outlives]

theorem step_does_not_outlive_session : ¬ outlives step session := by
  simp [outlives]

theorem outlives_trans
    {a b c : Lifetime}
    (hab : outlives a b)
    (hbc : outlives b c) :
    outlives a c := by
  cases a <;> cases b <;> cases c <;> simp [outlives] at hab hbc ⊢

inductive Capability where
  | roomHistory
  | ephemeralToken
deriving DecidableEq, Repr

open Capability

abbrev CaptureSet := Capability → Bool

def captureSubset (lhs rhs : CaptureSet) : Prop :=
  ∀ capability, lhs capability = true → rhs capability = true

def emptyCapture : CaptureSet := fun _ => false

def fullCapture : CaptureSet := fun _ => true

def ephemeralOnly : CaptureSet
  | ephemeralToken => true
  | roomHistory => false

def roomHistoryOnly : CaptureSet
  | roomHistory => true
  | ephemeralToken => false

theorem capture_subset_refl (captures : CaptureSet) : captureSubset captures captures := by
  intro capability h
  exact h

theorem empty_capture_subset (captures : CaptureSet) :
    captureSubset emptyCapture captures := by
  intro capability h
  simp [emptyCapture] at h

theorem capture_subset_trans
    {capturesA capturesB capturesC : CaptureSet}
    (hab : captureSubset capturesA capturesB)
    (hbc : captureSubset capturesB capturesC) :
    captureSubset capturesA capturesC := by
  intro capability h
  exact hbc capability (hab capability h)

theorem ephemeral_only_subset_of_full_capture :
    captureSubset ephemeralOnly fullCapture := by
  intro capability h
  simp [fullCapture]

theorem ephemeral_only_not_subset_of_empty :
    ¬ captureSubset ephemeralOnly emptyCapture := by
  intro h
  have hToken := h ephemeralToken rfl
  simp [emptyCapture] at hToken

theorem room_history_only_not_subset_of_ephemeral_only :
    ¬ captureSubset roomHistoryOnly ephemeralOnly := by
  intro h
  have hHistory := h roomHistory rfl
  simp [ephemeralOnly] at hHistory

def remoteCallAllowed (remainingCalls : Nat) : Prop :=
  0 < remainingCalls

def spendRemoteCall : Nat → Nat
  | 0 => 0
  | remainingCalls + 1 => remainingCalls

theorem zero_budget_rejects_remote_call :
    ¬ remoteCallAllowed 0 := by
  simp [remoteCallAllowed]

theorem positive_budget_allows_remote_call :
    remoteCallAllowed 1 := by
  simp [remoteCallAllowed]

theorem succ_budget_allows_remote_call (remainingCalls : Nat) :
    remoteCallAllowed (Nat.succ remainingCalls) := by
  simp [remoteCallAllowed]

theorem single_budget_is_exhausted_after_one_call :
    ¬ remoteCallAllowed (spendRemoteCall 1) := by
  simp [spendRemoteCall, remoteCallAllowed]

theorem two_budget_still_allows_after_one_call :
    remoteCallAllowed (spendRemoteCall 2) := by
  simp [spendRemoteCall, remoteCallAllowed]

end CurrentL2FiniteIndexFirstLayer
```

### 行ごとの解説

- 行 1-7
  - 先頭コメントで、この file が finite-index first layer の最小 self-contained fact を置くことを明示している。
- 行 9-14
  - `Lifetime` を `step` と `session` の 2 種類で定義する。
  - ここでは「短い寿命」と「長い寿命」の最小モデルと思えばよい。
- 行 16-21
  - `outlives` は lifetime の前順序を表す。
  - `session` は何より長生きする。
  - `step` は `step` は越えられるが `session` は越えられない。
- 行 23-37
  - `outlives_refl`、`session_outlives_step`、`step_does_not_outlive_session`、`outlives_trans` は lifetime 側の基本法則である。
  - 特に `outlives_trans` がないと、複数段の lifetime 判定を一般則として扱いにくい。
- 行 39-42
  - `Capability` を `roomHistory` と `ephemeralToken` の 2 種類で定義する。
  - ここでは「長く残してよい履歴」と「短命トークン」の対比が主眼である。
- 行 46-49
  - `CaptureSet` は capability ごとに `true / false` を返す関数として表現する。
  - `captureSubset` は「左で捕まえる capability が、右にもすべて含まれる」を意味する。
- 行 51-61
  - `emptyCapture`、`fullCapture`、`ephemeralOnly`、`roomHistoryOnly` は具体的な capture set の例である。
- 行 63-78
  - `capture_subset_refl`、`empty_capture_subset`、`capture_subset_trans` は capture inclusion の基本法則である。
  - `intro capability h` は「任意の capability と、その capability が左に含まれるという仮定」を受け取る書き方である。
- 行 80-83
  - `ephemeral_only_subset_of_full_capture` は、`ephemeralOnly` が `fullCapture` の部分集合であることを示す。
- 行 85-89
  - `ephemeral_only_not_subset_of_empty` は、ephemeral token を含む集合が空集合には入らないことを示す。
  - `have hToken := ...` は途中結果に名前を付ける書き方である。
- 行 91-95
  - `room_history_only_not_subset_of_ephemeral_only` は、`roomHistory` を含む集合が `ephemeralOnly` には入らないことを示す。
  - これにより、履歴と短命トークンを同じ capture として雑に扱えないことが分かる。
- 行 97-102
  - `remoteCallAllowed` は残り回数が正なら呼べる、`spendRemoteCall` は 1 回使うと残りを 1 減らす、という最小 budget model である。
- 行 104-114
  - `zero_budget_rejects_remote_call`、`positive_budget_allows_remote_call`、`succ_budget_allows_remote_call` は budget の基本法則である。
- 行 116-122
  - `single_budget_is_exhausted_after_one_call` は quota 1 を 1 回使ったらもう呼べないことを示す。
  - `two_budget_still_allows_after_one_call` は quota 2 なら 1 回後もまだ呼べることを示す。
- 行 124
  - namespace を閉じる。

### 実行コマンド

```bash
source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean
```

### 実行結果全文

実行結果全文は **出力なし** である。

### 何を確認しているのか

このコマンドは、capture / lifetime / budget の first-layer fact が全部通ることを確認している。

これが必要な理由は具体的である。

- `ephemeral_only_not_subset_of_empty` がないと
  - ephemeral token を「何も capture していない」と偽装できてしまう。
- `room_history_only_not_subset_of_ephemeral_only` がないと
  - room 全体の履歴を ephemeral token と同じ軽い扱いにしてしまう。
- `single_budget_is_exhausted_after_one_call` がないと
  - quota 1 の API 呼び出しを 2 回通せてしまう。

つまり、この foundation がないと `p15` や `p16` の rejection を「単なる個別ルール」ではなく「最小一般則」として支えるのが難しくなる。

## 6. Foundation 4: `CurrentL2ProofSkeleton.lean`

この file は domain theorem そのものを証明するのではなく、**review-unit から Lean stub を作る橋渡しの構造**を証明する。

### コード全文

```lean
/-!
current-l2 first proof-skeleton fragment

This file captures the structural part of the theorem-side bridge: review units, emitted
Lean stubs, and the invariant that emission preserves subject and obligation identity.
-/

namespace CurrentL2

inductive ObligationKind where
  | rollbackCutNonInterference
  | noRePromotion
deriving DecidableEq, Repr

open ObligationKind

def obligationName : ObligationKind → String
  | rollbackCutNonInterference => "rollback_cut_non_interference"
  | noRePromotion => "no_re_promotion"

structure ReviewUnit where
  subjectRef : String
  obligationKind : ObligationKind
deriving Repr

structure LeanStub where
  subjectRef : String
  obligationKind : ObligationKind
  theoremName : String
deriving Repr

def mkLeanStub (unit : ReviewUnit) : LeanStub :=
  {
    subjectRef := unit.subjectRef
    obligationKind := unit.obligationKind
    theoremName := unit.subjectRef ++ "__" ++ obligationName unit.obligationKind
  }

def emitStubs (units : List ReviewUnit) : List LeanStub :=
  units.map mkLeanStub

theorem mkLeanStub_preserves_subject (unit : ReviewUnit) :
    (mkLeanStub unit).subjectRef = unit.subjectRef := by
  simp [mkLeanStub]

theorem mkLeanStub_preserves_obligation (unit : ReviewUnit) :
    (mkLeanStub unit).obligationKind = unit.obligationKind := by
  simp [mkLeanStub]

theorem mkLeanStub_names_theorem (unit : ReviewUnit) :
    (mkLeanStub unit).theoremName = unit.subjectRef ++ "__" ++ obligationName unit.obligationKind := by
  simp [mkLeanStub]

theorem emitStubs_length (units : List ReviewUnit) :
    (emitStubs units).length = units.length := by
  simp [emitStubs]

def e5ReviewUnits : List ReviewUnit :=
  [
    { subjectRef := "e5-underdeclared-lineage", obligationKind := rollbackCutNonInterference },
    { subjectRef := "e5-underdeclared-lineage", obligationKind := noRePromotion }
  ]

theorem e5_emission_preserves_count :
    (emitStubs e5ReviewUnits).length = 2 := by
  simp [emitStubs, e5ReviewUnits]

theorem e5_first_stub_subject :
    match emitStubs e5ReviewUnits with
    | stub :: _ => stub.subjectRef = "e5-underdeclared-lineage"
    | _ => False := by
  simp [emitStubs, e5ReviewUnits, mkLeanStub]

theorem e5_second_stub_obligation :
    match emitStubs e5ReviewUnits with
    | _ :: stub :: _ => stub.obligationKind = noRePromotion
    | _ => False := by
  simp [emitStubs, e5ReviewUnits, mkLeanStub]

end CurrentL2
```

### 行ごとの解説

- 行 1-6
  - 先頭コメントで「この file は theorem-side bridge の構造部分、つまり review unit と emitted Lean stub の対応を扱う」と宣言している。
- 行 8-13
  - `ObligationKind` を `rollbackCutNonInterference` と `noRePromotion` の 2 種類で定義する。
  - ここでは obligation の種類を明示的な列挙にしている。
- 行 15-19
  - `obligationName` は obligation kind を theorem 名用の文字列へ変換する。
  - 生成される theorem 名が決まった規則に従うことを支える関数である。
- 行 21-24
  - `ReviewUnit` は theorem bridge の入力側レコードで、`subjectRef` と `obligationKind` を持つ。
- 行 26-30
  - `LeanStub` は出力側レコードで、subject、obligation、theorem 名を持つ。
- 行 32-37
  - `mkLeanStub` は 1 個の review unit から 1 個の Lean stub を作る関数である。
  - theorem 名は `subjectRef ++ "__" ++ obligationName ...` で組み立てている。
- 行 39-40
  - `emitStubs` は review unit のリスト全部に `mkLeanStub` を適用する。
  - `List.map` を使っているので、要素数が変わらないのが直感的にも分かる。
- 行 42-52
  - `mkLeanStub_preserves_subject`、`mkLeanStub_preserves_obligation`、`mkLeanStub_names_theorem` は、生成関数が subject / obligation / theorem 名規則を壊していないことを示す基本補題である。
- 行 54-56
  - `emitStubs_length` は review unit 数と stub 数が一致することを示す。
  - bridge がこっそり要素を落としたり増やしたりしていないことを確かめる。
- 行 58-62
  - `e5ReviewUnits` は `e5-underdeclared-lineage` 用の具体的 review unit 列である。
  - 同じ subject に対して 2 個の obligation を並べている。
- 行 64-66
  - `e5_emission_preserves_count` は `e5` で本当に 2 本の stub が出ることを示す。
- 行 68-72
  - `e5_first_stub_subject` は、1 本目の stub の subject が正しく `e5-underdeclared-lineage` になっていることを示す。
  - `match ... with` はリスト先頭をパターンマッチで取り出して検査する書き方である。
  - `| _ => False` は「その形でなければ偽」という fallback である。
- 行 74-78
  - `e5_second_stub_obligation` は、2 本目の stub の obligation が `noRePromotion` であることを示す。
  - これにより、obligation の並びや紐付けの取り違えを防ぐ。
- 行 80
  - namespace を閉じる。

### 実行コマンド

```bash
source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2ProofSkeleton.lean
```

### 実行結果全文

実行結果全文は **出力なし** である。

### 何を確認しているのか

このコマンドは、review-unit から Lean stub を作る構造部分が Lean 上で整合していることを確認している。

これが必要な理由は具体的である。たとえば、もし theorem bridge が review unit の obligation を取り違えたら、

- 本来 `no_re_promotion` としてレビューされるべきものを
- `rollback_cut_non_interference` の theorem 名で出してしまう

という事故が起きうる。

`e5_second_stub_obligation` は、その代表例で「少なくともこの具体例では obligation の対応が崩れていない」ことを Lean 側で押さえている。

## 7. generated current-L2 stub は何を意味するのか

ここからは foundation ではなく、**theorem bridge の出力物**を見る。最初に `p06` を代表例として、そのあと `e5` を補足例として扱う。

### 7-1. 代表例 `p06-typed-proof-owner-handoff`

#### コード全文

```lean
/- current-l2 Lean-first theorem stub (non-production)
subject_kind = runtime_try_cut_cluster
subject_ref = p06-typed-proof-owner-handoff
obligation_kind = rollback_cut_non_interference
module_name = CurrentL2.P06TypedProofOwnerHandoff
-/
namespace CurrentL2

-- goal: Review that rollback / atomic-cut evidence does not interfere with surviving runtime behavior for `p06-typed-proof-owner-handoff`.
-- evidence fixture:p06-typed-proof-owner-handoff
-- evidence runtime_cluster:p06-typed-proof-owner-handoff
theorem p06_typed_proof_owner_handoff__rollback_cut_non_interference : True := by
sorry

end CurrentL2
```

#### 行ごとの解説

- 行 1-6
  - 先頭コメントで、この file が **non-production の Lean-first theorem stub** だと明示している。
  - `subject_kind`、`subject_ref`、`obligation_kind`、`module_name` は、bridge が何を出そうとしているかを人間が追えるようにしたメタ情報である。
- 行 7
  - `namespace CurrentL2` に入る。
- 行 9-11
  - ここは生成時に埋め込まれた説明コメントである。
  - `goal` はこの theorem が何をレビューしたいのかを自然文で示す。
  - `evidence ...` はどの fixture / runtime cluster に対応しているかを示す。
- 行 12
  - theorem 宣言そのものはきちんと生成されている。
  - theorem 名に sample 名と obligation 名が入っているので、対応関係を機械的に追える。
- 行 13
  - `sorry` は未完成 proof の placeholder である。
  - Lean は「いまは仮置きで通す」が、完全証明としては数えない。
- 行 15
  - namespace を閉じる。

#### 実行コマンド

```bash
source "$HOME/.elan/env" && lean samples/lean/current-l2/p06-typed-proof-owner-handoff/p06-typed-proof-owner-handoff.lean
```

#### 実行結果全文

```text
samples/lean/current-l2/p06-typed-proof-owner-handoff/p06-typed-proof-owner-handoff.lean:12:8: warning: declaration uses `sorry`
```

#### この出力をどう読むか

- `warning`
  - file 全体は Lean に受理された。
- `declaration uses \`sorry\``
  - theorem 本体は未完成で、placeholder に依存している。
- したがって現在ここで分かるのは
  - theorem 名や module 構造が正しく出ていること
  - sample と obligation の対応が artifact として壊れていないこと
  - しかし mathematical discharge は終わっていないこと

これを一言でまとめると、`p06` は **bridge artifact の成功例**であって、**完成証明の成功例ではない**。

### 7-2. 補足例 `e5-underdeclared-lineage`

`e5` は `CurrentL2ProofSkeleton.lean` ともつながる代表例で、1 file 内に 2 本の stub が入っている。

#### コード全文

```lean
/- current-l2 Lean-first theorem stub (non-production)
subject_kind = fixture_static_cluster
subject_ref = e5-underdeclared-lineage
obligation_kind = canonical_normalization_law
module_name = CurrentL2.E5UnderdeclaredLineage
-/
namespace CurrentL2

-- goal: Review that canonical normalization preserves the rejected static shape for `e5-underdeclared-lineage`.
-- evidence fixture:e5-underdeclared-lineage
-- evidence static_gate_artifact:e5-underdeclared-lineage
theorem e5_underdeclared_lineage__canonical_normalization_law : True := by
sorry

end CurrentL2

/- current-l2 Lean-first theorem stub (non-production)
subject_kind = fixture_static_cluster
subject_ref = e5-underdeclared-lineage
obligation_kind = no_re_promotion
module_name = CurrentL2.E5UnderdeclaredLineage
-/
namespace CurrentL2

-- goal: Review that `e5-underdeclared-lineage` remains rejected and is not re-promoted by later tooling.
-- evidence fixture:e5-underdeclared-lineage
theorem e5_underdeclared_lineage__no_re_promotion : True := by
sorry

end CurrentL2
```

#### 行ごとの解説

- 行 1-6
  - 1 本目の stub のメタ情報である。
  - obligation は `canonical_normalization_law` である。
- 行 7-13
  - 1 本目の theorem 宣言である。
  - subject は `e5-underdeclared-lineage`、theorem body は `sorry` で未完成である。
- 行 15
  - 1 本目の namespace を閉じる。
- 行 17-22
  - 2 本目の stub のメタ情報である。
  - obligation は `no_re_promotion` に切り替わっている。
- 行 23-28
  - 2 本目の theorem 宣言である。
  - こちらも body は `sorry` で、artifact としての整列だけを示している。
- 行 30
  - 2 本目の namespace を閉じる。

#### 実行コマンド

```bash
source "$HOME/.elan/env" && lean samples/lean/current-l2/e5-underdeclared-lineage/e5-underdeclared-lineage.lean
```

#### 実行結果全文

```text
samples/lean/current-l2/e5-underdeclared-lineage/e5-underdeclared-lineage.lean:12:8: warning: declaration uses `sorry`
samples/lean/current-l2/e5-underdeclared-lineage/e5-underdeclared-lineage.lean:27:8: warning: declaration uses `sorry`
```

#### この出力をどう読むか

warning が 2 行出るのは、**file 内に `sorry` を使う theorem 宣言が 2 本ある**からである。

ここで確認しているのは次である。

- `e5` には 2 本の obligation がぶら下がる
- それぞれが Lean theorem として text 化されている
- ただし両方とも未完成 proof の placeholder 付きである

`CurrentL2ProofSkeleton.lean` では「その 2 本が正しい subject / obligation に対応する」と示し、ここでは「実際にそのような stub text が生成されている」と示している。両者を合わせて読むのが重要である。

## 8. corpus 全体を同期して確認する

foundation と generated stub をまとめて確認したいときは次を使う。

```bash
source "$HOME/.elan/env" && python3 scripts/current_l2_lean_sample_sync.py
```

このコマンドで確認しているのは次の点である。

- foundation 4 本が全部 Lean で通るか
- representative current-l2 stub 群が生成済みで、Lean に受理されるか
- explanation file や theorem 名の対応が崩れていないか

2026-04-21 の再実行では、要点は次のとおりだった。

- `foundations`
  - `CurrentL2LabelModel.lean`
  - `CurrentL2IfcSecretExamples.lean`
  - `CurrentL2FiniteIndexFirstLayer.lean`
  - `CurrentL2ProofSkeleton.lean`
  の 4 本がすべて `success: true`、`stdout: ""`、`returncode: 0`
- `current_l2`
  - `e5-underdeclared-lineage`
  - `p06-typed-proof-owner-handoff`
  - `p10`
  - `p11`
  - `p12`
  - `p15`
  - `p16`
  - `p07`
  - `p08`
  - `p09`
  - `p13`
  - `p14`
  が `success: true` だが、`stdout` に `warning: declaration uses \`sorry\`` を含む

初心者向けには、次の 2 行だけ覚えておくとよい。

- foundation `success: true`
  - 実際の小さな証明が通っている
- generated stub `success: true` + `warning`
  - artifact は受理されたが、完全証明ではない

## 9. test で何を固定しているのか

Lean source と説明文の対応が drift していないかは、Python unit test でも確認している。

```bash
python3 -m unittest scripts/tests/test_current_l2_lean_sample_sync.py
```

2026-04-21 の再実行結果:

```text
.........
----------------------------------------------------------------------
Ran 9 tests in 0.004s

OK
```

この test で見ている代表的な点は次のとおりである。

- foundation file 一覧が欠けていないか
- representative sample set が欠けていないか
- generated stub explanation が `sorry` を含む current scope を正しく説明しているか
- theorem 名が generator source と整合しているか

つまり、この test は Lean そのものの定理証明だけでなく、**Lean corpus とその説明文の同期**も守っている。

## 10. Lean 初心者向けの入出力の読み方

この repo では特に次を覚えておくとよい。

- 出力なし
  - まず成功である。
  - 典型例は foundation 4 本である。
- `warning: declaration uses \`sorry\``
  - file 自体は受理されたが、定理の中身は placeholder 付きである。
  - generated stub 側の基本的な読み方である。
- `error: unsolved goals`
  - 証明が閉じていない。
  - 何かが「あと 1 歩足りない」か、「そもそも今の仮定では無理」である。
- `⊢ False`
  - 現在の goal が `False` だという意味である。
  - 仮定から矛盾が出なければ閉じないので、今回の最小 error 例では「その主張は無理」と読めばよい。

読む順番は次のとおりにすると混乱しにくい。

1. `error` があるか
2. `warning` があるか
3. 何も出ないか

## 11. 何が本質で、何が目的か

この repo の Lean の本質は、「最終理論を全部 mechanize した」と主張することではない。現在の本質は次の 2 つを意図的に分けることにある。

1. foundation 側で、今必要な不変条件を小さな proof fragment として本当に通す
2. generated stub 側で、theorem bridge が subject / obligation を壊さず Lean artifact を出していることを確認する

この切り分けが重要な理由は、`samples/lean/current-l2/` の warning を見たときに、

- 「Lean が読めた」
- 「だから定理が完全証明された」

と誤読しないためである。

現在の正確な読みは次のとおりである。

- foundation 側
  - mechanization-ready core
- generated stub 側
  - artifact well-formedness and bridge alignment

## 12. どこで何を宣言しているか

最後に、今回出てきた「型宣言」「関数定義」「定理宣言」「theorem bridge の stub 宣言」が、コード中のどこにあるかを短く整理する。

### 型の宣言

- `CurrentL2LabelModel.lean`
  - `inductive SecurityLabel where`
  - label の型そのものを宣言している。
- `CurrentL2IfcSecretExamples.lean`
  - `structure Labeled (label : SecurityLabel) (α : Type) where`
  - label 付き値のレコード型を宣言している。
- `CurrentL2FiniteIndexFirstLayer.lean`
  - `inductive Lifetime where`
  - `inductive Capability where`
  - lifetime と capability の列挙型を宣言している。
- `CurrentL2ProofSkeleton.lean`
  - `inductive ObligationKind where`
  - `structure ReviewUnit where`
  - `structure LeanStub where`
  - theorem bridge の入力 / 出力に使う型を宣言している。

### 関数定義

- `CurrentL2LabelModel.lean`
  - `def flowsTo : SecurityLabel → SecurityLabel → Prop`
  - `def join : SecurityLabel → SecurityLabel → SecurityLabel`
  - `def CanDeclassify ... : Prop`
  - label flow と declassification 条件を定義している。
- `CurrentL2IfcSecretExamples.lean`
  - `def fingerprint (key : SecretKey) : SecretFingerprint :=`
  - `def declassify ... :=`
  - secret 由来 fingerprint の作成と declassification を定義している。
- `CurrentL2FiniteIndexFirstLayer.lean`
  - `def outlives : Lifetime → Lifetime → Prop`
  - `def captureSubset (lhs rhs : CaptureSet) : Prop :=`
  - `def remoteCallAllowed (remainingCalls : Nat) : Prop :=`
  - `def spendRemoteCall : Nat → Nat`
  - lifetime / capture / budget の最小モデルを定義している。
- `CurrentL2ProofSkeleton.lean`
  - `def obligationName : ObligationKind → String`
  - `def mkLeanStub (unit : ReviewUnit) : LeanStub :=`
  - `def emitStubs (units : List ReviewUnit) : List LeanStub :=`
  - review unit から theorem 名付き stub を組み立てる関数を定義している。

### 定理宣言

- `CurrentL2LabelModel.lean`
  - `theorem flowsTo_trans`
  - `theorem no_declassify_without_authority`
  - `theorem authority_enables_secret_to_public`
  - label 法則と authority 条件の中心定理である。
- `CurrentL2IfcSecretExamples.lean`
  - `theorem declassify_preserves_value`
  - `theorem authorized_live_fingerprint_release_has_witness`
  - `theorem unauthorized_live_fingerprint_release_is_impossible`
  - valid / invalid release を具体例で示す中心定理である。
- `CurrentL2FiniteIndexFirstLayer.lean`
  - `theorem outlives_trans`
  - `theorem ephemeral_only_not_subset_of_empty`
  - `theorem single_budget_is_exhausted_after_one_call`
  - lifetime / capture / budget の first-layer fact を宣言している。
- `CurrentL2ProofSkeleton.lean`
  - `theorem mkLeanStub_preserves_subject`
  - `theorem mkLeanStub_preserves_obligation`
  - `theorem e5_second_stub_obligation`
  - theorem bridge の構造整合を確認する定理である。

### theorem bridge の stub 宣言

- `p06-typed-proof-owner-handoff.lean`
  - `theorem p06_typed_proof_owner_handoff__rollback_cut_non_interference : True := by`
  - runtime try / cut cluster に対する generated theorem stub 宣言である。
  - 直後の `sorry` により、これは未完成 proof 付き artifact だと分かる。
- `e5-underdeclared-lineage.lean`
  - `theorem e5_underdeclared_lineage__canonical_normalization_law : True := by`
  - `theorem e5_underdeclared_lineage__no_re_promotion : True := by`
  - static underdeclared sample に対して 2 本の stub が出ていることを示す。

### どの宣言が何のために必要か

- 型宣言がないと
  - 「何に対して証明しているか」の対象そのものが曖昧になる。
- 関数定義がないと
  - `flowsTo` や `declassify` や `mkLeanStub` の意味が定まらない。
- 定理宣言がないと
  - 「何を正しいと主張しているか」が名前付きで固定されない。
- theorem bridge の stub 宣言がないと
  - sample ごとに、どの obligation がどの Lean artifact 名に落ちるかを inspectable に確認できない。

以上をまとめると、この repo の Lean は

- foundation 側で最小不変条件を実際に証明し
- generated stub 側で theorem bridge の出力面を固定する

という二層構造で読めば、現在地を誤解しにくい。
