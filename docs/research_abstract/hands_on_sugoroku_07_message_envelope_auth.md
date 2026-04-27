# hands-on: Sugoroku MessageEnvelope / Auth seam

## 目的

この文書は、Sugoroku helper に追加した helper-local `MessageEnvelope` / `AuthEvidence` / `PrincipalClaim`
first cut を読むための短い hands-on です。

ここで見るのは **repo-local current layer の evidence carrier** です。
final public auth protocol、session management、real socket transport を意味しません。

## 何を actualize したか

current Sugoroku helper は、`message_envelopes` を sample result に含めます。

各 envelope では少なくとも次を分けて見ます。

- transport
- principal claim
- auth evidence
- membership epoch / member incarnation
- capability requirements
- authorization checks
- witness refs
- dispatch outcome

current first cut の重要な前提は次です。

- transport は `local_queue`
- auth evidence は `none`
- membership freshness は `membership_epoch` / `member_incarnation`
- witness は auth ではなく `witness_refs`

つまり、membership freshness や witness を authentication に潰していません。

## 実行コマンド

```bash
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes
python3 scripts/sugoroku_world_samples.py run 04_non_owner_roll_rejected --format json
python3 scripts/sugoroku_world_samples.py closeout --format json
```

## 読み方

`03_roll_publish_handoff` では、少なくとも 2 本の envelope が見えます。

- `roll_request#1`
  - `ParticipantPlace[Alice] -> SugorokuGamePlace#1`
  - auth は `none`
  - capability requirement は `DiceOwner(Alice)` と `ActiveParticipant(Alice)`
  - witness はまだ不要
- `handoff_notice#1`
  - `SugorokuGamePlace#1 -> ParticipantPlace[Bob]`
  - auth は `none`
  - witness は `draw_pub#1`
  - capability / witness / transport が別 lane のまま見える

`04_non_owner_roll_rejected` では、rejection も同じ carrier で見えます。

- `bad_roll_request#1`
  - principal claim は `Carol`
  - capability requirement は `DiceOwner(Carol)`
  - `authorization_checks` に `dice_owner(SugorokuGame#1) = Carol`
  - `dispatch_outcome = rejected`

## stop line

この package で fixed にしないもの:

- final public `MessageEnvelope` schema
- final public `AuthEvidence` kind
- session token / signature protocol
- real network transport
- federation / multi-server identity
- final public audit retention policy

## 関連

- `samples/clean-near-end/sugoroku-world/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/reports/0921-message-envelope-auth-seam-first-cut.md`
