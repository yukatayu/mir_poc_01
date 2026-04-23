# hands-on: typing

この文書は、Mir current-L2 の clean near-end sample suite のうち、`typing` family を初めて読む人向けに説明するハンズオンです。

ここでいう `typing` は、単に「値が整数か文字列か」を見る型検査ではありません。
「秘密情報を公開してよいか」「誰に権限があるか」「一時的な能力を外へ逃がしていないか」「予算を超える処理を呼んでいないか」を、実行前に小さな有限の規則として検査する層です。

## 最初に結論

現在 repo で実行できることは次です。

- `SecurityLabel` や `FingerprintAuthority` は言語の組み込みではなく、sample 側で宣言した user-defined theory です。
- `declassify_authority(...)` のような古い専用 predicate は active sample では使いません。
- 成功例は `valid` になります。
- 権限不足、label flow 違反、capture escape、cost bound 超過は `malformed` になります。
- これは「小さな有限表で全部調べられる検査」をまず通す層であり、全てを任意に強い数学的型で表す full dependent type theory ではありません。

## 使うコマンド

repository root で実行します。

```bash
python3 scripts/clean_near_end_samples.py list
python3 scripts/clean_near_end_samples.py run typing --format json
```

個別の `.mir` ファイルを直接指定する current public command は、まだ final API として固定されていません。
現在の入口は family 単位の runner です。
個別に読みたい場合は、次のように source file を開いてから family runner の結果と対応させます。

```bash
sed -n '1,220p' samples/clean-near-end/typing/01_authorized_declassification.mir
python3 scripts/clean_near_end_samples.py run typing --format json
```

## 言葉の準備

この節では、以後に出てくる語を先に定義します。

`sample` は、repo 内に置かれた実行可能な例です。
ここでは `samples/clean-near-end/typing/*.mir` が sample です。

`static check` は、実行前の検査です。
たとえば「Bob には秘密を公開する権限がない」と分かるなら、実際に公開処理を始める前に止めます。

`verdict` は、検査結果です。
この suite では主に `valid` と `malformed` が出ます。
`valid` は current checker がその sample を受け入れたという意味です。
`malformed` は current checker が実行前に拒否したという意味です。

`finite` は、要素数が有限であるという意味です。
たとえば権限が `Observer`, `Holder`, `Releaser`, `Admin` の 4 つだけなら、比較表を全部作れます。
current layer はこの「全部調べられる」範囲を first layer としています。

`index theory` は、sample が自分で宣言する小さな名前の世界です。
たとえば security label の世界、authority の世界、capture capability の世界を作ります。

`preorder` は、「A は B 以下である」という順序関係です。
同じもの同士は必ず比較でき、A <= B かつ B <= C なら A <= C と読める関係です。
ここでは `Observer <= Holder <= Releaser <= Admin` のように使います。

`lattice` は、label の上下関係に加えて、2 つの label を合わせたときの上限と下限を持つ構造です。
この handson では、まず「`Public` は低く、`KeyMaterial` は高い」という直感で読めば十分です。

`capture` は、関数や closure が外側から持ち込んでいる能力の集合です。
「公開してよい closure は `RoomHistory` だけを持ってよい」と決めたのに、短命な `EphemeralToken` を持ち込むと拒否されます。

`cost` は、処理に使う予算です。
ここでは `remote_calls` や `cpu_steps` のような counter を、自然数の上限として扱います。

## 何が組み込みで、何が組み込みではないか

この handson で読む current sample vocabulary は主に次です。
ここでの built-in は、current helper が認識している語彙という意味です。
final public parser grammar や reserved keyword set を固定するものではありません。

- `module`: sample の名前空間を作る語です。
- `use`: 別の sample file で宣言された前提を使う語です。
- `index theory`: user-defined finite theory を宣言する語です。
- `element`: theory の中の要素を宣言する語です。
- `order`: 要素同士の上下関係を宣言する語です。
- `law`: その theory が満たす形を宣言する語です。
- `policy`: 許可規則を宣言する語です。
- `principal`: actor や user のような権限主体を宣言する語です。
- `resource`: label、capture、lifetime を持つ対象を宣言する語です。
- `transition`: 段階を持つ処理を宣言する語です。
- `stage`: transition の中の一段階を宣言する語です。
- `perform`: effectful operation を呼ぶ語です。
- `ensure`: 結果が満たすべき条件を記録する語です。
- `requires`: 必要な条件を記録する語です。
- `declassify`: 高い秘密 label の値を低い公開 label へ落とす操作です。

一方で、次は組み込みではありません。

- `SecurityLabel`
- `FingerprintAuthority`
- `CaptureScope`
- `Region`
- `CostBudget`
- `FingerprintReleasePolicy`
- `Public`
- `UserSecret`
- `KeyMaterial`
- `Observer`
- `Holder`
- `Releaser`
- `Admin`
- `RoomHistory`
- `EphemeralToken`

これらは sample が自分で宣言した名前です。
言語が「指紋公開専用の魔法の述語」を持っているわけではありません。

## Step 1: 共有前提を読む

typing sample は、まず `samples/clean-near-end/00_index_theories.mir` の前提を使います。

```mir
module CleanNearEnd.IndexTheories

index theory SecurityLabel {
  element Public
  element UserSecret
  element KeyMaterial

  order Public <= UserSecret
  order UserSecret <= KeyMaterial

  join
  meet
  law finite_lattice
}

index theory FingerprintAuthority {
  element Observer
  element Holder
  element Releaser
  element Admin

  order Observer <= Holder
  order Holder <= Releaser
  order Releaser <= Admin

  law finite_preorder
}

index theory CaptureScope {
  element RoomHistory
  element EphemeralToken
  element SecretKeyStore

  law finite_powerset
}

index theory Region {
  element Step
  element Turn
  element Session

  order Step <= Turn
  order Turn <= Session

  law finite_preorder
}

index theory CostBudget {
  counter cpu_steps
  counter remote_calls
  counter writes

  law pointwise_natural_bound
}

policy FingerprintReleasePolicy {
  permit declassify SecurityLabel.KeyMaterial -> SecurityLabel.Public
    requires authority >= FingerprintAuthority.Releaser
}
```

行ごとの読み方です。

- `module CleanNearEnd.IndexTheories` は、この file の宣言を `CleanNearEnd.IndexTheories` という名前でまとめます。
- `index theory SecurityLabel` は、秘密度 label の世界を作ります。
- `element Public` は、公開情報を表す label を作ります。
- `element UserSecret` は、user の秘密情報を表す label を作ります。
- `element KeyMaterial` は、鍵素材のようなより高い秘密情報を表す label を作ります。
- `order Public <= UserSecret` は、`Public` が `UserSecret` 以下、つまり `Public` の方が低い秘密度だと宣言します。
- `order UserSecret <= KeyMaterial` は、`KeyMaterial` がさらに高い秘密度だと宣言します。
- `join` と `meet` は、2 つの label を合わせたときの上限と下限を使う準備です。
- `law finite_lattice` は、この label theory が有限 lattice として扱えることを宣言します。
- `index theory FingerprintAuthority` は、指紋公開に関わる権限の世界を作ります。
- `Observer`, `Holder`, `Releaser`, `Admin` は、低い権限から高い権限へ並ぶ user-defined authority です。
- `law finite_preorder` は、権限比較が有限 preorder として決定可能だと宣言します。
- `CaptureScope` は、closure が捕まえてよい能力の名前を置きます。
- `law finite_powerset` は、capture を「有限集合」として比較することを意味します。
- `Region` は、寿命の長さを `Step <= Turn <= Session` として宣言します。
- `CostBudget` は、費用の counter 名を宣言します。
- `policy FingerprintReleasePolicy` は、秘密 label を公開 label に落とすための許可規則です。
- `permit declassify SecurityLabel.KeyMaterial -> SecurityLabel.Public` は、`KeyMaterial` から `Public` への declassify を対象にします。
- `requires authority >= FingerprintAuthority.Releaser` は、少なくとも `Releaser` 以上の権限が必要だと宣言します。

大事なのは、これらが全て source 側の宣言だという点です。
checker はこの宣言を読んで有限比較を行います。

## Step 2: 成功する declassification を読む

成功例は `samples/clean-near-end/typing/01_authorized_declassification.mir` です。

```mir
module CleanNearEnd.AuthorizedDeclassification

use CleanNearEnd.IndexTheories

principal Alice : FingerprintAuthority.Releaser

resource secret_key : PrivateKey
  label SecurityLabel.KeyMaterial
  capture SecretKeyStore
  lifetime Session

resource public_board : Board
  label SecurityLabel.Public
  lifetime Session

transition release_fingerprint(actor = Alice) {
  stage derive:
    fp_secret <- perform derive_fingerprint(secret_key)
      ensure label(fp_secret) = SecurityLabel.KeyMaterial

  stage declassify:
    fp_public <- declassify fp_secret
      from SecurityLabel.KeyMaterial
      to SecurityLabel.Public
      using FingerprintReleasePolicy
      requires authority(Alice) >= FingerprintAuthority.Releaser

  stage publish:
    receipt <- perform publish_fingerprint(fp_public)
      to public_board
      after declassify(fp_public)
      requires witness(fp_public)
      ensure label(fp_public) = SecurityLabel.Public
}
```

行ごとの読み方です。

- `module CleanNearEnd.AuthorizedDeclassification` は、この sample の名前です。
- `use CleanNearEnd.IndexTheories` は、Step 1 の label、authority、policy を使うという意味です。
- `principal Alice : FingerprintAuthority.Releaser` は、Alice が `Releaser` 権限を持つ主体だと宣言します。
- `resource secret_key : PrivateKey` は、`secret_key` という resource を宣言します。
- `label SecurityLabel.KeyMaterial` は、`secret_key` が高い秘密 label を持つことを示します。
- `capture SecretKeyStore` は、この resource が `SecretKeyStore` capability と関係することを示します。
- `lifetime Session` は、この resource が session の間有効だと示します。
- `resource public_board : Board` は、公開先の board を宣言します。
- `label SecurityLabel.Public` は、その board が公開 label の場所だと示します。
- `transition release_fingerprint(actor = Alice)` は、Alice が actor として行う処理を宣言します。
- `stage derive:` は、秘密から指紋を導く段階です。
- `fp_secret <- perform derive_fingerprint(secret_key)` は、`derive_fingerprint` という effectful operation を呼び、その結果を `fp_secret` に入れます。
- `ensure label(fp_secret) = SecurityLabel.KeyMaterial` は、結果 `fp_secret` も高い秘密 label だと記録します。
- `stage declassify:` は、秘密 label を公開 label へ落とす段階です。
- `fp_public <- declassify fp_secret` は、`fp_secret` を公開可能な値へ変換する操作です。
- `from SecurityLabel.KeyMaterial` は、変換前 label が `KeyMaterial` であることを明示します。
- `to SecurityLabel.Public` は、変換後 label が `Public` であることを明示します。
- `using FingerprintReleasePolicy` は、Step 1 の policy に従って許可判定することを示します。
- `requires authority(Alice) >= FingerprintAuthority.Releaser` は、Alice の権限が `Releaser` 以上であることを要求します。
- `stage publish:` は、公開 board へ書く段階です。
- `receipt <- perform publish_fingerprint(fp_public)` は、公開 effect を呼び、receipt を受け取ります。
- `to public_board` は、公開先を示します。
- `after declassify(fp_public)` は、公開は declassify の後でなければならないことを示します。
- `requires witness(fp_public)` は、公開してよい値である証拠を要求します。
- `ensure label(fp_public) = SecurityLabel.Public` は、公開される値が `Public` label であることを確認します。

この sample で checker が解く主な constraint は次です。

- `Alice` の authority は `Releaser` 以上か。
- `FingerprintReleasePolicy` は `KeyMaterial -> Public` の declassify を許しているか。
- `fp_public` の label は `Public` として扱えるか。

## Step 3: 成功例を実行する

family 全体を実行します。

```bash
python3 scripts/clean_near_end_samples.py run typing --format json
```

出力の重要部分は次の形になります。

```json
{
  "sample": "01_authorized_declassification",
  "static_verdict": "valid",
  "terminal_outcome": "success",
  "constraints_solved": [
    "authority(Alice) >= FingerprintAuthority.Releaser",
    "declassify SecurityLabel.KeyMaterial -> SecurityLabel.Public permitted by FingerprintReleasePolicy",
    "label(fp_public) = SecurityLabel.Public"
  ]
}
```

読み方です。

- `sample` は、どの sample の結果かを示します。
- `static_verdict: valid` は、実行前検査が通ったことを示します。
- `terminal_outcome: success` は、sample の期待される終端結果が成功であることを示します。
- `constraints_solved` は、checker が有限 theory 上で解けた条件です。

## Step 4: 権限不足で落ちる例を読む

失敗例は `samples/clean-near-end/typing/02_unauthorized_declassification_rejected.mir` です。

```mir
module CleanNearEnd.UnauthorizedDeclassificationRejected

use CleanNearEnd.IndexTheories

principal Bob : FingerprintAuthority.Observer

resource secret_key : PrivateKey
  label SecurityLabel.KeyMaterial
  capture SecretKeyStore
  lifetime Session

transition bad_release(actor = Bob) {
  stage derive:
    fp_secret <- perform derive_fingerprint(secret_key)

  stage declassify:
    fp_public <- declassify fp_secret
      from SecurityLabel.KeyMaterial
      to SecurityLabel.Public
      using FingerprintReleasePolicy
      requires authority(Bob) >= FingerprintAuthority.Releaser
}
```

成功例との違いは小さいですが、意味は決定的に違います。

- `principal Bob : FingerprintAuthority.Observer` は、Bob が `Observer` 権限しか持たないことを宣言します。
- policy は `Releaser` 以上を要求します。
- authority order は `Observer <= Holder <= Releaser <= Admin` です。
- したがって `Observer >= Releaser` は false です。
- checker はここで止めます。

出力の重要部分は次の形です。

```json
{
  "sample": "02_unauthorized_declassification_rejected",
  "static_verdict": "malformed",
  "reason_family": "authority_preorder_constraint_failed",
  "constraints_failed": [
    "FingerprintAuthority.Observer >= FingerprintAuthority.Releaser"
  ],
  "entered_evaluation": false
}
```

`entered_evaluation: false` が重要です。
これは「危険な公開処理を少し実行してから失敗した」のではなく、「実行に入る前に sample を拒否した」という意味です。

## Step 5: label flow 違反を見る

`samples/clean-near-end/typing/03_label_flow_rejected.mir` は、declassify せずに秘密を公開しようとします。

重要部分は次です。

```mir
transition bad_publish_without_declassify(actor = Alice) {
  stage derive:
    fp_secret <- perform derive_fingerprint(secret_key)
      ensure label(fp_secret) = SecurityLabel.KeyMaterial

  stage publish:
    receipt <- perform publish_fingerprint(fp_secret)
      to public_board
}
```

ここで `fp_secret` は `KeyMaterial` label のままです。
`public_board` は `Public` label です。
高い秘密 label の値を低い公開場所へ直接流すことはできません。

出力の重要部分です。

```json
{
  "sample": "03_label_flow_rejected",
  "static_verdict": "malformed",
  "reason_family": "label_flow_constraint_failed",
  "constraints_failed": [
    "SecurityLabel.KeyMaterial <= SecurityLabel.Public"
  ],
  "entered_evaluation": false
}
```

`SecurityLabel.KeyMaterial <= SecurityLabel.Public` は false です。
したがって checker は拒否します。

## Step 6: capture escape を見る

`samples/clean-near-end/typing/04_capture_escape_rejected.mir` は、短命な token を公開 closure の中へ逃がす例です。

重要部分は次です。

```mir
capture theory RoomCapture {
  capability RoomHistory
  capability EphemeralToken

  allow public_closure captures { RoomHistory }
}

resource token : SessionToken
  capture EphemeralToken
  lifetime Step

fn make_public_closure(token : SessionToken @ capture(EphemeralToken))
  -> Fn[Unit -> Unit] @ capture(RoomHistory) {
  return fn () {
    use token
  }
}
```

読み方です。

- `capture theory RoomCapture` は、closure が何を捕まえてよいかを宣言します。
- `capability RoomHistory` は、部屋の履歴を見る能力です。
- `capability EphemeralToken` は、短時間だけ有効な token です。
- `allow public_closure captures { RoomHistory }` は、公開 closure が捕まえてよいのは `RoomHistory` だけだと宣言します。
- `resource token` は、`EphemeralToken` を持つ短命 resource です。
- `lifetime Step` は、非常に短い寿命です。
- `make_public_closure` は、`EphemeralToken` を持つ `token` を受け取ります。
- 戻り値は `capture(RoomHistory)` の closure だと宣言されています。
- しかし closure 本体で `use token` しているため、実際には `EphemeralToken` も捕まえています。

出力の重要部分です。

```json
{
  "sample": "04_capture_escape_rejected",
  "static_verdict": "malformed",
  "reason_family": "capture_escape",
  "constraints_failed": [
    "{EphemeralToken} <= {RoomHistory}",
    "Region.Session <= Region.Step"
  ],
  "entered_evaluation": false
}
```

`{EphemeralToken} <= {RoomHistory}` は false です。
集合として見たとき、`EphemeralToken` は `RoomHistory` の中に含まれていません。
`Region.Session <= Region.Step` も false です。
長い session の値を短い step の内側へ安全に閉じ込めたことにはできない、という lifetime 側の制約も同時に失敗しています。

## Step 7: cost bound 超過を見る

`samples/clean-near-end/typing/05_cost_bound_rejected.mir` は、remote call 予算が 0 の関数から remote call を呼んでしまう例です。

重要部分は次です。

```mir
effect fetch_remote_bonus
  output bonus : Bonus
  effect remote_call
  cost <= { remote_calls: 1, cpu_steps: 20, writes: 0 }

fn move_without_remote_budget(player : Player)
  -> Board
  cost <= { remote_calls: 0, cpu_steps: 100, writes: 0 } {
  bonus <- perform fetch_remote_bonus()
  require remote_calls: 0
  return apply_bonus(player, bonus)
}
```

読み方です。

- `effect fetch_remote_bonus` は、外部から bonus を取る effect を宣言します。
- `effect remote_call` は、この effect が remote call を含むことを示します。
- `cost <= { remote_calls: 1, cpu_steps: 20, writes: 0 }` は、この effect の費用が remote call 1 回、CPU step 20 回、write 0 回の範囲に収まるという上限を示します。
- `fn move_without_remote_budget` は、remote call 予算 0 の関数です。
- 関数本体で `perform fetch_remote_bonus()` を呼ぶため、remote call が 1 回必要になります。
- `require remote_calls: 0` は、この場所では remote call を使わないことを要求している、という sample 側の明示です。
- しかし宣言された予算は `remote_calls: 0` です。

出力の重要部分です。

```json
{
  "sample": "05_cost_bound_rejected",
  "static_verdict": "malformed",
  "reason_family": "cost_bound_exceeded",
  "constraints_failed": [
    "remote_calls 1 <= 0"
  ],
  "entered_evaluation": false
}
```

`remote_calls 1 <= 0` は false です。
したがって checker は実行前に拒否します。

## この handson で確認できたこと

`typing` family は、以下を現在の repo で確認できます。

- user-defined authority preorder による declassification 許可。
- user-defined security label lattice による label flow 拒否。
- finite capture set による capture escape 拒否。
- finite region / lifetime 方向の基礎。
- simple cost bound による remote call 予算超過拒否。

## まだ final product ではないこと

次はまだ deferred です。

- final public parser grammar。
- final public checker API。
- full dependent type theory。
- すべての residual obligation を Lean で完全 discharge する proof pipeline。
- production runtime policy engine。

この handson の到達点は、「repo-local alpha-ready current layer の typing sample は実行可能で、実行前に通る例と落ちる例を観察できる」です。
