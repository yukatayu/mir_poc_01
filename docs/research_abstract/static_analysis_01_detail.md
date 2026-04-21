# static_analysis_01_detail

## この文書の目的

この文書は、[static_analysis_01.md](/home/yukatayu/dev/mir_poc_01/docs/research_abstract/static_analysis_01.md) で紹介している **Problem 1**

- typed / information-flow control（情報フロー制御, IFC）
- theorem-first の Lean bridge
- model-check second line reserve

を、**何も知らない読者でも順に追えるように**、コード全文つきで詳しく読み下すための詳細版である。

ここでは特に、次の疑問を潰すことを目的にする。

- このコードはどこで何を宣言しているのか
- どの行が authority / label / capture / budget の制約を書いているのか
- どこで theorem / model-check 側の確認につながっているのか
- なぜ `Reject` になるのか
- 「一見よさそうだが実はダメ」が、どこで弾かれているのか

この文書だけを読んでも理解できるように、元のコード・コマンド・出力例をそのまま載せ、そのあとで行ごとの説明を付ける。

## 先に全体像

Problem 1 の current line は、次の 3 本立てで読むと理解しやすい。

1. typed / IFC の first line
   権限や label-flow を sample 実行で弾く層
2. theorem-first bridge
   sample から Lean artifact を吐けることを確認する層
3. model-check second line reserve
   「どの bad pattern を今の current line で禁止しているか」をまとめ直す層

この 3 本は役割が違う。

- typed / IFC は「その sample を今許すか止めるか」を見る
- theorem-first bridge は「その sample を theorem 側にどう橋渡しするか」を見る
- model-check second line reserve は「その sample を property / bad pattern の観点でどう束ねるか」を見る

## 実行前の準備

repo root で、次をそのままコピペすればよい。

```bash
python3 --version
cargo --version
source "$HOME/.elan/env"
lean --version
```

Lean を使う command は、以後も `source "$HOME/.elan/env"` を前に付けておくとよい。

## 最短確認

まず bundle 全体が壊れていないかを見る。

```bash
python3 scripts/current_l2_guided_samples.py smoke-all --format json
```

2026-04-21 の再実行結果:

```json
[
  {
    "problem_id": "problem1",
    "title": "Problem 1 theorem-first pilot bundle",
    "status": "passed",
    "step_count": 5,
    "successful_steps": 5,
    "failed_step": null,
    "smoke_command": "python3 scripts/current_l2_guided_samples.py smoke problem1",
    "sample_bundle_doc": "samples/problem-bundles/problem1-typed-theorem-model-check.md",
    "primary_samples": [
      "p06-typed-proof-owner-handoff"
    ]
  },
  {
    "problem_id": "problem2",
    "title": "Problem 2 authoritative-room scenario bundle",
    "status": "passed",
    "step_count": 7,
    "successful_steps": 7,
    "failed_step": null,
    "smoke_command": "python3 scripts/current_l2_guided_samples.py smoke problem2",
    "sample_bundle_doc": "samples/problem-bundles/problem2-order-handoff-shared-space.md",
    "primary_samples": [
      "p07-dice-late-join-visible-history",
      "p08-dice-stale-reconnect-refresh"
    ]
  }
]
```

この JSON の `status: "passed"` は、「この bundle の代表的な確認コマンド群は通っている」という意味である。

## 1. `p10` authorized declassification success

対象ファイル:
[p10-typed-authorized-fingerprint-declassification.txt](/home/yukatayu/dev/mir_poc_01/samples/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.txt)

### コード全文

```text
# declassify authority がある場合だけ fingerprint を公開できる current L2 typed/IFC prototype。
place root {
  place room {
    place verifier_boundary {
      option release_authority on fingerprint_state capability write lease live admit declassify_authority(player_a)
      option observer_only on fingerprint_state capability write lease live admit observer_role(player_a)

      chain fingerprint_release_ref = release_authority
      fallback observer_only @ lineage(release_authority -> observer_only)

      perform derive_secret_fingerprint via fingerprint_release_ref
        require write
        ensure fingerprint_bound(secret_key)

      atomic_cut

      perform publish_declassified_fingerprint via fingerprint_release_ref
        require write
        ensure fingerprint_visible(room_members)
    }
  }
}
```

### 行ごとの解説

| 行 | 説明 |
|---|---|
| 1 | コメント行で、この sample の狙いを先に宣言している。`declassify authority` がある場合だけ公開できる、というのが中心主張である。 |
| 2 | `place root {` は最上位の場所を開く。`place` は current L2 sample で「処理や状態の置き場所」を表す語である。 |
| 3 | `place room {` は root の内側に room という局所空間を作る。今回の sample は room 単位で読む。 |
| 4 | `place verifier_boundary {` は、この sample が verifier に近い境界での確認であることを示す。authority や公開条件を境界で読む、という意図がある。 |
| 5 | `option release_authority ... declassify_authority(player_a)` は一番重要な行の一つである。`release_authority` という option を `fingerprint_state` 上に置き、`capability write`、`lease live` を持ち、さらに `declassify_authority(player_a)` という admit 条件を付けている。つまり「player_a が declassify authority を持つときだけ使える公開権限」の宣言である。 |
| 6 | `observer_only` は対照用の option で、`observer_role(player_a)` しか持たない。観察者であることと公開権限があることを分けている。ここが `p11` との比較で効く。 |
| 7 | 空行。option 宣言群と、その後の参照・実行部を読みやすく分けている。 |
| 8 | `chain fingerprint_release_ref = release_authority` は、以後の操作が `release_authority` を主参照としてたどることを宣言する。`chain` は「この名前でこの option を参照する」という束縛に近い。 |
| 9 | `fallback observer_only @ lineage(release_authority -> observer_only)` は、主参照が使えないときの fallback 候補を置いている。ただし lineage を明示している点が大事で、「どの option からどの option への関係か」を隠さない。 |
| 10 | 空行。宣言部と perform 群を分けている。 |
| 11 | `perform derive_secret_fingerprint via fingerprint_release_ref` は、秘密鍵由来の fingerprint を導出する操作である。`via fingerprint_release_ref` なので、8 行目で束縛した chain を使っている。 |
| 12 | `require write` は、この操作に write capability が必要であることを示す。読み取りだけでは足りない。 |
| 13 | `ensure fingerprint_bound(secret_key)` は、この操作が終わったあとに「この fingerprint は secret_key に結びついている」という事後条件を宣言する。つまり fingerprint が secret 由来であることを自分で明示している。 |
| 14 | 空行。 |
| 15 | `atomic_cut` は current L2 で重要な区切りである。ここまでの効果・証拠を 1 つの区切りとして切り出す読みを与える。theorem / model-check 側の preview とも接続しやすい。 |
| 16 | 空行。 |
| 17 | `perform publish_declassified_fingerprint via fingerprint_release_ref` は公開操作本体である。名前に `declassified` が入っていることが重要で、単なる publish ではなく「declassify された fingerprint を publish する」という意図を表す。 |
| 18 | ここでも `require write` を要求している。公開操作も state 更新なので write capability が必要である。 |
| 19 | `ensure fingerprint_visible(room_members)` は、結果として room のメンバーに見えることを事後条件として宣言する。公開対象が `room_members` であることを明示している。 |
| 20-22 | それぞれ `verifier_boundary`、`room`、`root` を閉じる。構造を閉じ忘れていないことを示す。 |

### 実行コマンド

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- \
  check-source-sample \
  samples/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.txt \
  --format pretty
```

### 出力例

```text
static_gate_verdict: valid
terminal_outcome: success
typed_checker_hint_status: reached
```

### 出力の読み方

| 出力行 | 読み方 |
|---|---|
| `static_gate_verdict: valid` | 構文と基本 shape は通っている。つまり「読めるコード」ではある。 |
| `terminal_outcome: success` | そのうえで意味づけの line でも許可された。authority を持つ declassification は current line では通る。 |
| `typed_checker_hint_status: reached` | typed checker-adjacent line まで到達している。単なる parser 成功ではなく、typed / IFC の current line で確認されている。 |

### どこで何を宣言しているか

- declassify 権限の宣言:
  `option release_authority ... admit declassify_authority(player_a)`
- 公開先の宣言:
  `ensure fingerprint_visible(room_members)`
- 秘密由来であることの宣言:
  `ensure fingerprint_bound(secret_key)`

### この性質がないと何に困るか

declassify authority の明示がないと、「秘密鍵由来 fingerprint を誰が public に出してよいか」が曖昧になる。
具体例を一言で言うと、**権限のない観察者が secret 由来 fingerprint を room 全体へ貼れてしまう**。

## 2. `p11` unauthorized release rejection

対象ファイル:
[p11-typed-unauthorized-fingerprint-release.txt](/home/yukatayu/dev/mir_poc_01/samples/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.txt)

### コード全文

```text
# release authority を持たない holder が fingerprint を publish しようとして止まる current L2 typed/IFC prototype。
place root {
  place room {
    place verifier_boundary {
      option fingerprint_holder on fingerprint_state capability write lease live admit fingerprint_holder(player_a)
      option delegated_release on fingerprint_state capability write lease live admit delegate_granted(player_a)

      chain fingerprint_release_ref = fingerprint_holder
      fallback delegated_release @ lineage(fingerprint_holder -> delegated_release)

      perform derive_secret_fingerprint via fingerprint_release_ref
        require write
        ensure fingerprint_bound(secret_key)

      atomic_cut

      perform publish_without_authority via fingerprint_release_ref
        require write
        ensure fingerprint_visible(room_members)
    }
  }
}
```

### 行ごとの解説

| 行 | 説明 |
|---|---|
| 1 | この sample の狙いを明示するコメントである。`holder` であることと `release authority` を持つことは違う、と先に教えている。 |
| 2-4 | `root` → `room` → `verifier_boundary` という構造は `p10` と同じで、比較しやすくしている。 |
| 5 | `fingerprint_holder(player_a)` は「fingerprint を保持している」だけであって、「公開してよい」ことまでは言っていない。ここがもっとも重要な違いである。 |
| 6 | `delegate_granted(player_a)` は委譲候補だが、ここでも `declassify_authority` とは書いていない。つまり release authority の根拠としては弱い。 |
| 8 | 主参照は holder 側を向いている。つまり publish 時にも holder 権限を前提に進もうとしている。 |
| 9 | fallback もあるが、lineage を明示しただけで、「公開権限が得られた」とは書いていない。 |
| 11-13 | ここまでは `p10` と似ている。秘密由来 fingerprint を作ること自体はできてしまうので、一見もっともらしく見える。 |
| 15 | `atomic_cut` によって、導出と公開を区切っている。 |
| 17 | `publish_without_authority` という操作名が、この sample の失敗原因をそのまま示している。名前だけでなく意味でも authority 不足で止まる。 |
| 18-19 | write を要求し、可視化先は room_members である。つまり「本気で公開しようとしている」コードである。 |

### 実行コマンド

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- \
  check-source-sample \
  samples/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.txt \
  --format pretty
```

### 出力例

```text
static_gate_verdict: valid
terminal_outcome: Reject
typed_checker_hint_status: reached
```

### 出力の読み方

| 出力行 | 読み方 |
|---|---|
| `static_gate_verdict: valid` | まず「読めるコード」ではある。構文エラーではない。 |
| `terminal_outcome: Reject` | しかし意味の line では拒否される。つまり parser 成功後に、authority 条件違反として止められている。 |
| `typed_checker_hint_status: reached` | 拒否は typed / IFC line まで到達したうえで行われている。ここが「一見大丈夫そうだが実はダメ」を弾いているポイントである。 |

### どこで何を宣言しているか

- holder であることの宣言:
  `option fingerprint_holder ... admit fingerprint_holder(player_a)`
- authority がないまま publish している箇所:
  `perform publish_without_authority via fingerprint_release_ref`
- 公開先の宣言:
  `ensure fingerprint_visible(room_members)`

### この性質がないと何に困るか

holder と releaser を分けないと、「持っている人は誰でも出してよい」ことになってしまう。
具体例は、**診断用 fingerprint を見られるだけの担当者が、そのまま公開もできてしまう**である。

## 3. `p12` label-flow mismatch rejection

対象ファイル:
[p12-typed-classified-fingerprint-publication-block.txt](/home/yukatayu/dev/mir_poc_01/samples/prototype/current-l2-typed-proof-model-check/p12-typed-classified-fingerprint-publication-block.txt)

### コード全文

```text
# classified な fingerprint を declassify せず public board へ publish しようとして止まる current L2 typed/IFC prototype。
place root {
  place room {
    place verifier_boundary {
      option classified_holder on fingerprint_state capability write lease live admit classified_holder(player_a)
      option audit_observer on fingerprint_state capability write lease live admit observer_role(player_b)

      chain fingerprint_release_ref = classified_holder
      fallback audit_observer @ lineage(classified_holder -> audit_observer)

      perform derive_secret_fingerprint via fingerprint_release_ref
        require write
        ensure fingerprint_bound(secret_key)

      atomic_cut

      perform publish_classified_fingerprint_to_public_board via fingerprint_release_ref
        require write
        ensure fingerprint_visible(public_board)
    }
  }
}
```

### 行ごとの解説

| 行 | 説明 |
|---|---|
| 1 | `classified` な fingerprint を `public_board` に出そうとして止まる sample である。authority の有無ではなく、label-flow mismatch を見たい。 |
| 5 | `classified_holder` は classified 情報を持つ側である。 |
| 6 | `audit_observer` は監査側の対照 option である。公開権限とは別物である。 |
| 8-9 | chain と fallback を置いているが、ここでも declassification そのものは宣言していない。 |
| 11-13 | 秘密由来 fingerprint を作るところは前と同じである。 |
| 15 | `atomic_cut`。 |
| 17 | `publish_classified_fingerprint_to_public_board` は操作名の時点で危険信号である。classified なものを public board に出すと宣言している。 |
| 19 | `ensure fingerprint_visible(public_board)` が決定的である。公開先が `room_members` ですらなく、さらに広い `public_board` になっている。 |

### 実行コマンド

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- \
  check-source-sample \
  samples/prototype/current-l2-typed-proof-model-check/p12-typed-classified-fingerprint-publication-block.txt \
  --format pretty
```

### 出力例

```text
static_gate_verdict: valid
terminal_outcome: Reject
typed_checker_hint_status: reached
```

### 出力の読み方

ここで重要なのは、「authority がないから」ではなく、「classified なものを public に流そうとしているから」Reject になる点である。

### どこで何を宣言しているか

- classified 側の主体:
  `option classified_holder ...`
- 公開先が public である宣言:
  `ensure fingerprint_visible(public_board)`
- 危険な操作意図の宣言:
  `perform publish_classified_fingerprint_to_public_board ...`

### この性質がないと何に困るか

label-flow 制約がないと、**classified 扱いの結果を public board にそのまま書き出せてしまう**。
authority があるかどうかとは別に、流してよい label かどうかを見ないと危険である。

## 4. `p15` capture / lifetime escape rejection

対象ファイル:
[p15-typed-capture-escape-rejected.txt](/home/yukatayu/dev/mir_poc_01/samples/prototype/current-l2-typed-proof-model-check/p15-typed-capture-escape-rejected.txt)

### コード全文

```text
# ephemeral token を capture scope の外へ publish しようとして止まる current L2 typed/capture-lifetime prototype。
place root {
  place room {
    place verifier_boundary {
      option session_owner on session_state capability write lease live admit session_owner(player_a)
      option audit_observer on session_state capability write lease live admit observer_role(player_b)

      chain session_ref = session_owner

      perform derive_ephemeral_session_token via session_ref
        require write
        ensure session_token_bound(ephemeral_scope)

      atomic_cut

      perform publish_captured_session_token via session_ref
        require write
        ensure session_token_visible(room_members)
    }
  }
}
```

### 行ごとの解説

| 行 | 説明 |
|---|---|
| 1 | ここでは IFC というより capture / lifetime 側の失敗を見る。`ephemeral token` は一時的スコープに閉じ込めたい値である。 |
| 5 | `session_owner` が session 状態を持つ主体である。 |
| 8 | `chain session_ref = session_owner` で session 所有者を主参照にする。 |
| 10-12 | `derive_ephemeral_session_token` によって token を作り、その token が `ephemeral_scope` に束縛されることを `ensure session_token_bound(ephemeral_scope)` で明示している。これが「外に出してはいけない」という根拠になる。 |
| 14 | `atomic_cut`。 |
| 16 | `publish_captured_session_token` は危険な操作である。capture された token を publish しようとしている。 |
| 18 | `ensure session_token_visible(room_members)` で room 全体への可視化を要求している。ephemeral scope と衝突する。 |

### 実行コマンド

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- \
  check-source-sample \
  samples/prototype/current-l2-typed-proof-model-check/p15-typed-capture-escape-rejected.txt \
  --format pretty
```

### 出力例

```text
static_gate_verdict: valid
terminal_outcome: Reject
typed_checker_hint_status: reached
```

### どこで何を宣言しているか

- token が一時スコープに束縛される宣言:
  `ensure session_token_bound(ephemeral_scope)`
- 外へ漏らそうとしている宣言:
  `ensure session_token_visible(room_members)`

### この性質がないと何に困るか

capture / lifetime 制約が弱いと、**一時的セッショントークンが room 全体に漏れる**。
ログイン後だけ有効なトークンを全員に貼り出してしまう、という典型的な事故になる。

## 5. `p16` remote-call budget rejection

対象ファイル:
[p16-typed-remote-call-budget-exceeded.txt](/home/yukatayu/dev/mir_poc_01/samples/prototype/current-l2-typed-proof-model-check/p16-typed-remote-call-budget-exceeded.txt)

### コード全文

```text
# remote call budget が 0 のまま追加 call を試みて止まる current L2 typed/simple-cost prototype。
place root {
  place room {
    place verifier_boundary {
      option quota_owner on quota_state capability write lease live admit quota_owner(player_a)
      option audit_observer on quota_state capability write lease live admit observer_role(player_b)

      chain remote_budget_ref = quota_owner

      perform reserve_remote_call_budget via remote_budget_ref
        require write
        ensure remote_call_budget_remaining(zero_budget)

      atomic_cut

      perform invoke_remote_call_without_budget via remote_budget_ref
        require write
        ensure remote_api_visible(remote_api_gateway)
    }
  }
}
```

### 行ごとの解説

| 行 | 説明 |
|---|---|
| 1 | ここでは simple cost, つまり「残り予算」で止めるパターンを見る。 |
| 5 | `quota_owner` は予算を持つ側である。 |
| 8 | `remote_budget_ref` という chain 名から、以後は予算参照として使うことが分かる。 |
| 10-12 | `reserve_remote_call_budget` の結果として、残予算が `zero_budget` であると明示している。つまり「もう残っていない」と自分で宣言している。 |
| 14 | `atomic_cut`。 |
| 16 | `invoke_remote_call_without_budget` は、その名の通り予算なし呼び出しである。 |
| 18 | `ensure remote_api_visible(remote_api_gateway)` は外部 API に効果が及ぶことを宣言している。外部効果なので budget 管理が重要になる。 |

### 実行コマンド

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- \
  check-source-sample \
  samples/prototype/current-l2-typed-proof-model-check/p16-typed-remote-call-budget-exceeded.txt \
  --format pretty
```

### 出力例

```text
static_gate_verdict: valid
terminal_outcome: Reject
typed_checker_hint_status: reached
```

### どこで何を宣言しているか

- 残予算が 0 である宣言:
  `ensure remote_call_budget_remaining(zero_budget)`
- 外部 API を叩こうとしている宣言:
  `perform invoke_remote_call_without_budget ...`

### この性質がないと何に困るか

budget 制約がないと、**quota 1 の API を何度でも呼べてしまう**。
課金 API や rate-limit 付き API ではそのまま障害や過剰請求につながる。

## 6. theorem-first bridge の代表例 `p06`

対象ファイル:
[p06-typed-proof-owner-handoff.txt](/home/yukatayu/dev/mir_poc_01/samples/prototype/current-l2-typed-proof-model-check/p06-typed-proof-owner-handoff.txt)

### コード全文

```text
# option admit と request contract を合わせ、typed marker つき handoff が runtime cluster / verifier preview に届くことを見る current L2 prototype。
place root {
  place room {
    place proof_authority {
      option current_owner on proof_state capability write lease live admit owner_is(player_a)
      option delegated_owner on proof_state capability write lease live admit delegate_granted(player_a)

      chain proof_owner_ref = current_owner
      fallback delegated_owner @ lineage(current_owner -> delegated_owner)

      perform publish_roll_result via proof_owner_ref
        require write
        ensure result_is_visible(room_members)

      atomic_cut

      perform handoff_proof_owner via proof_owner_ref
        require write
        ensure owner_is(player_b)
    }
  }
}
```

### 行ごとの解説

| 行 | 説明 |
|---|---|
| 1 | コメントがこの sample の用途を明示している。typed marker つき handoff が runtime cluster / verifier preview に届くこと、つまり theorem-first bridge の代表例として使うことが狙いである。 |
| 4 | `place proof_authority` という名前が、これが proof 関連の authority handoff の sample であることを表している。 |
| 5 | `current_owner` は現在の所有者 option で、`owner_is(player_a)` を admit 条件にする。 |
| 6 | `delegated_owner` は委譲された所有権候補である。 |
| 8-9 | chain と fallback により、「現在の owner を主に見つつ、必要なら委譲先へたどる」構造を作る。 |
| 11-13 | `publish_roll_result` で結果を可視化する。あとで `handoff_proof_owner` を行うので、publish と handoff の両方が 1 本の sample に入っている。 |
| 15 | `atomic_cut`。theorem preview では rollback / cut 非干渉の subject に使われる。 |
| 17-19 | `handoff_proof_owner` により所有者が `player_b` に変わることを宣言する。これが theorem stub に映る。 |

### `emit-theorem` コマンド

```bash
python3 scripts/current_l2_guided_samples.py emit-theorem problem1
```

### 出力例

```text
Problem 1 theorem-first emitted artifact loop
output dir: target/current-l2-guided/problem1-theorem-pilot
pilot summary markdown: target/current-l2-guided/problem1-theorem-pilot/pilot-summary.md

- p06-typed-proof-owner-handoff: representative theorem-first sample
  pilot_status: reached
  lean_stub_artifact_count: 1
```

### 出力の読み方

| 出力行 | 読み方 |
|---|---|
| `representative theorem-first sample` | `p06` が theorem-first bridge の代表例として選ばれている。 |
| `pilot_status: reached` | theorem-first pilot がこの sample まで到達している。 |
| `lean_stub_artifact_count: 1` | 少なくとも 1 本の Lean stub artifact が生成されている。 |

### どこで何を宣言しているか

- handoff の宣言:
  `perform handoff_proof_owner via proof_owner_ref`
- 所有者変更の宣言:
  `ensure owner_is(player_b)`
- theorem-first bridge の入口になる sample だと分かる箇所:
  コメント行の `runtime cluster / verifier preview`

## 7. model-check second line reserve のまとめ出力

### 実行コマンド

```bash
python3 scripts/current_l2_guided_samples.py emit-reserve model-check-second-line
```

### 出力例

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

### 出力の読み方

| 行 | 意味 |
|---|---|
| `authority release positive carrier` | `p10` は「通るべき正例」の carrier である。 |
| `authority miss rejection` | `p11` は authority 不足として reject される。 |
| `label-flow rejection` | `p12` は label-flow 違反として reject される。 |
| `capture/lifetime rejection` | `p15` は scope / lifetime 逸脱で reject される。 |
| `simple cost rejection` | `p16` は予算不足で reject される。 |

この出力のよいところは、「Reject の理由が全部同じではない」ことを 1 つの package で見比べられる点である。

## 8. Lean foundation 1: `CurrentL2IfcSecretExamples.lean`

対象ファイル:
[CurrentL2IfcSecretExamples.lean](/home/yukatayu/dev/mir_poc_01/samples/lean/foundations/CurrentL2IfcSecretExamples.lean)

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

| 行 | 説明 |
|---|---|
| 1-7 | `/-! ... -/` は Lean のドキュメントコメントである。この file の役割を先に書いている。final public typed surface ではなく、proof fragment だと明言している。 |
| 9 | `namespace CurrentL2IfcSecretExamples` は名前空間を開く。以後の定義名が他の file と衝突しにくくなる。 |
| 11-14 | `inductive SecurityLabel` は label 型の定義である。`low` と `high` の 2 点ラティスに近い最小モデルを置いている。`deriving DecidableEq, Repr` は、等価判定と表示用表現を自動生成する。 |
| 16 | `open SecurityLabel` で `SecurityLabel.low` を毎回書かず `low` と短く書けるようにする。 |
| 18-21 | `def flowsTo` は label 間の流れを表す関係である。`low` はどこへでも流せる、`high -> high` は許す、`high -> low` は許さない、というルールをここで定義している。 |
| 23-24 | `def CanDeclassify ... := hasAuthority = true ∨ flowsTo ...` が核心である。authority があるか、もしくはそもそも flow 上許されているときだけ declassify 可と定める。 |
| 26-27 | `structure Labeled` は「label を型にぶら下げた値」を表す。ここが依存型的な読みの最小例である。`label` という型引数を値の型に持ち込んでいる。 |
| 29-31 | `abbrev` は別名定義である。`SecretKey`、`SecretFingerprint`、`PublicFingerprint` を型レベルで区別している。 |
| 33-35 | `fingerprint` は secret key から secret fingerprint を作る関数である。`"fp:" ++ key.value` で payload を作る。 |
| 36-42 | `declassify` は authority 証明 `_proof` を要求する関数である。ここが重要で、ただの Bool ではなく `CanDeclassify ...` という論理条件の証明を引数に取る。つまり条件が証明できなければ呼べない。 |
| 44-50 | `theorem declassify_preserves_value` は declassify しても payload 自体は変わらないことを示す。`rfl` は「定義通りに見れば自明」という意味の Lean の証明キーワードである。 |
| 52-54 | `no_secret_release_without_authority` は authority なし high -> low が不可能であることを示す。`simp` は定義を展開して単純化する。 |
| 56-58 | `authorized_secret_release_is_available` は authority があれば high -> low が可能になることを示す。 |
| 60-62 | `low_to_low_release_without_authority_is_available` は low -> low なら authority が不要であることを示す。public 情報の素通しまで禁止していない。 |
| 64-68 | `publicAuditNote` と `unchangedPublicAuditNote` は low 情報の単純な通過例である。valid な正例を先に置いている。 |
| 70-76 | `unchanged_public_audit_note_keeps_payload` は low 情報を low のまま通すだけなら payload が変わらないことを示す。`simpa` は `simp` で証明できる形へ整える。 |
| 78-79 | `liveSecretKey` は具体例の秘密鍵である。 |
| 81-83 | `fingerprint_keeps_secret_payload` は fingerprint 関数の出力が期待通り `"fp:sk_live"` になることを示す。 |
| 85-86 | `authorizedPublicFingerprint` は authority あり declassify の具体例である。ここで `authorized_secret_release_is_available` という証明を渡している。 |
| 88-94 | `authorized_public_fingerprint_keeps_payload` は authorized declassification 後も payload がそのまま残ることを示す。 |
| 96-100 | `invalid_release_has_no_authority_proof` は、authority なし high -> low の証明オブジェクトが存在しないことを示す。`∃` は存在量化、`rcases` は存在証明を分解して中身を取り出す。 |
| 102-104 | `valid_release_has_authority_proof` は authority ありなら証明オブジェクトを 1 つ作れることを示す。 |
| 106-114 | `authorized_live_fingerprint_release_has_witness` は authorized な live fingerprint release には実際に witness があることを示す。`refine ⟨..., ?_⟩` は「存在する witness の最初の成分を先に与え、残りの証明をあとで埋める」書き方である。 |
| 116-121 | `unauthorized_live_fingerprint_release_is_impossible` は unauthorized な witness が存在しないことを示す。つまり `p11` 型の失敗を Lean 側でも支える。 |
| 123 | 名前空間を閉じる。 |

### どこで何を宣言しているか

- label 型の宣言:
  `inductive SecurityLabel where | low | high`
- flow 関係の宣言:
  `def flowsTo : SecurityLabel → SecurityLabel → Prop`
- declassify 条件の宣言:
  `def CanDeclassify ... := hasAuthority = true ∨ flowsTo ...`
- label 付き値の型宣言:
  `structure Labeled (label : SecurityLabel) (α : Type) where`
- 「authority がない release は不可能」の定理:
  `theorem unauthorized_live_fingerprint_release_is_impossible ...`

### 実行コマンド

```bash
source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2IfcSecretExamples.lean
```

2026-04-21 の再実行では出力なしで終了した。Lean では、証明が全部通ると通常は何も表示されない。

## 9. Lean foundation 2: `CurrentL2FiniteIndexFirstLayer.lean`

対象ファイル:
[CurrentL2FiniteIndexFirstLayer.lean](/home/yukatayu/dev/mir_poc_01/samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean)

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

| 行 | 説明 |
|---|---|
| 1-7 | file コメント。capture set、lifetime preorder、simple budget という 3 つを扱うと宣言している。 |
| 9-14 | `Lifetime` 型を `step` と `session` の 2 値で定義する。step は短い寿命、session は長い寿命と読むと分かりやすい。 |
| 18-21 | `outlives` は lifetime の順序である。`session` は何でも outlive し、`step` は `session` を outlive できない。 |
| 23-37 | `outlives_refl`、`session_outlives_step`、`step_does_not_outlive_session`、`outlives_trans` で寿命順序の基本法則を証明する。`cases` は場合分けである。 |
| 39-42 | `Capability` 型は `roomHistory` と `ephemeralToken` の 2 種を置く。 |
| 46 | `CaptureSet := Capability → Bool` は、「各 capability を含むかどうか」を返す関数として capture set を表す。 |
| 48-49 | `captureSubset` は capture set の包含である。lhs に入っている capability は rhs にも入っていなければならない。 |
| 51-61 | `emptyCapture`、`fullCapture`、`ephemeralOnly`、`roomHistoryOnly` は代表的な capture set である。 |
| 63-78 | `capture_subset_refl`、`empty_capture_subset`、`capture_subset_trans` で capture 包含の基本法則を証明する。 |
| 80-95 | `ephemeral_only_subset_of_full_capture`、`ephemeral_only_not_subset_of_empty`、`room_history_only_not_subset_of_ephemeral_only` で、通る例と通らない例を具体的に示す。`p15` の背景説明になる。 |
| 97-102 | `remoteCallAllowed` と `spendRemoteCall` は simple budget の最小モデルである。残り回数が正なら許可、1 回使うと 1 減る。 |
| 104-122 | 予算 0 は拒否、1 は許可、1 回使うと 1 は尽きる、2 は 1 回使ってもまだ許可、という基本法則を証明している。`p16` の背景説明になる。 |
| 124 | 名前空間を閉じる。 |

### どこで何を宣言しているか

- lifetime 型の宣言:
  `inductive Lifetime where | step | session`
- outlives 関係の宣言:
  `def outlives : Lifetime → Lifetime → Prop`
- capture set の表現:
  `abbrev CaptureSet := Capability → Bool`
- capture 包含の宣言:
  `def captureSubset (lhs rhs : CaptureSet) : Prop := ...`
- remote-call budget の宣言:
  `def remoteCallAllowed ...`
  `def spendRemoteCall ...`

### 実行コマンド

```bash
source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean
```

2026-04-21 の再実行では出力なしで成功した。

## 10. 依存型っぽい最小 success / failure 例

これは [static_analysis_01.md](/home/yukatayu/dev/mir_poc_01/docs/research_abstract/static_analysis_01.md) で紹介している、その場で型と relation を定義する最小例である。

### 通る最小例

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
source "$HOME/.elan/env" && lean /tmp/current_l2_ifc_valid.lean'
```

2026-04-21 の再実行では出力なしで成功した。

#### コードの読み方

| 行 | 説明 |
|---|---|
| `inductive SecurityLabel ...` | label 型を作る。 |
| `def flowsTo ...` | high -> low が False になる流れ関係を置く。 |
| `def CanDeclassify ...` | authority があるか flow が通るときだけ許す。 |
| `structure Labeled ...` | label 付き値を型で表す。 |
| `example : CanDeclassify true ...` | authority あり high -> low の主張を立てる。 |
| `simp [CanDeclassify, flowsTo]` | 定義展開だけでこの主張が通ることを示す。 |

### 弾かれる最小例

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
source "$HOME/.elan/env" && lean /tmp/current_l2_ifc_invalid.lean'
```

出力例:

```text
/tmp/current_l2_ifc_invalid.lean:16:70: error: unsolved goals
⊢ False
```

#### 出力の読み方

- `unsolved goals`
  まだ証明すべき目標が残っている、という意味である。
- `⊢ False`
  最終的に偽を示すしかない状態になっており、この仮定では goal を閉じられない、という意味である。

つまり「authority なし high -> low」は、定義したルールのもとでは証明できない。

## 11. foundation と generated stub の違い

### 実行コマンド

```bash
source "$HOME/.elan/env" && python3 scripts/current_l2_lean_sample_sync.py
```

### 出力の要点

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

### 読み方

- foundation 側の `success: true`
  実際の小さな証明が通っている。
- generated stub 側の `success: true` + `warning`
  Lean file としては受理されたが、中身にはまだ `sorry` が残っている。

## 12. この文書全体での「どこで何を宣言しているか」まとめ

### 型の宣言

- label 型:
  `inductive SecurityLabel where`
- label 付き値:
  `structure Labeled (label : SecurityLabel) (α : Type) where`
- lifetime 型:
  `inductive Lifetime where`
- capability 型:
  `inductive Capability where`

### 制約の宣言

- authority 制約:
  `def CanDeclassify (hasAuthority : Bool) ...`
- label-flow 制約:
  `def flowsTo : SecurityLabel → SecurityLabel → Prop`
- capture 制約:
  `def captureSubset (lhs rhs : CaptureSet) : Prop := ...`
- budget 制約:
  `def remoteCallAllowed (remainingCalls : Nat) : Prop := ...`

### sample 側の宣言

- option の権限宣言:
  `option ... admit ...`
- 実行したい操作の宣言:
  `perform ...`
- 必要 capability の宣言:
  `require write`
- 事後条件の宣言:
  `ensure ...`
- 区切りの宣言:
  `atomic_cut`

### theorem-first bridge の宣言

- sample 側では `p06` のコメントと `handoff_proof_owner` が bridge representative の意味を持つ。
- helper 側では
  `p06-typed-proof-owner-handoff: representative theorem-first sample`
  が代表例指定である。

### model-check second line reserve の宣言

- helper 出力で
  `authority miss rejection`
  `label-flow rejection`
  `capture/lifetime rejection`
  `simple cost rejection`
  と分類されている部分が、reserve 側の bad pattern 分類である。

## 13. まとめ

Problem 1 の current line では、すでに次が具体的に確認できる。

- authority がある declassification は通る
- authority がない release は止まる
- classified -> public の label mismatch は止まる
- capture / lifetime 逸脱は止まる
- remote-call budget 超過は止まる
- それらを Lean foundation と theorem/model-check preview に結び付けて読める

大事なのは、「全部を final public calculus として閉じた」と主張しているのではない点である。
今の repo で言っているのは、**sample 実行で bad pattern を弾けること、Lean foundation でその根拠の一部を小さく mechanize できていること、そして theorem/model-check 側へ橋渡しする artifact route が見えていること**である。
