# avatar_fairy_follow_current_slice_01

この文書は、phase 8 `avatar fairy follow / fallback anchor` の
current representative slice と residual planned family を reader-facing に説明する summary です。

規範判断の正本は `specs/` であり、ここは current plan と sample 境界の説明だけを行います。

## 1. current status

- active runnable representative slice は `samples/clean-near-end/avatar-follow/` と
  `scripts/avatar_follow_samples.py` にあります
- historical prototype anchor は
  `samples/prototype/current-l2-dynamic-attach-detach/p03-avatar-controller-attach-detach.txt`
  にあります
- residual planned family は
  `samples/not_implemented/avatar-fairy-follow/`
  に残しています

この 2 つを混同しないことが重要です。

## 2. なぜ avatar fairy follow を representative slice にしたのか

Sugoroku vertical slice は membership / publish / witness / handoff を確認するには十分でした。
次に見たいのは、Place をまたぐ観測、visibility、fallback lineage、stale anchor rejection を
もっと直接に表す sample family です。

avatar fairy follow は、そのための小さく practical な題材です。

## 3. current active slice と residual planned family

| ID | Path | Role | Expected debug focus |
|---|---|---|---|
| `FAIRY-01` | `samples/clean-near-end/avatar-follow/01_...` | active positive canary | anchor graph / follow summary |
| `FAIRY-02` | `samples/clean-near-end/avatar-follow/02_...` | active visibility-loss fallback canary | fallback reason / visibility |
| `FAIRY-03` | `samples/clean-near-end/avatar-follow/03_...` | active fallback-after-reject canary | membership + fallback reason |
| `FAIRY-04` | `samples/clean-near-end/avatar-follow/04_...` | active rejection canary | static rejection / lineage mismatch |
| `FAIRY-06` | `samples/clean-near-end/avatar-follow/06_...` | active verification canary | verification / model-check |
| `FAIRY-05` | `samples/not_implemented/avatar-fairy-follow/05_...` | residual planned reacquire widening | state timeline / anchor switch |

## 4. sample family で見たい性質

- `FollowAnchor` 自体は builtin magical primitive にしない
- visibility guard を explicit に置く
- remote -> local fallback lineage を explicit に書く
- stale anchor rejection を hidden repair にしない
- helper-local debug output を final public visualization と混同しない

## 5. current helper surface

- command:
  `python3 scripts/avatar_follow_samples.py`
- debug modes:
  `summary`, `anchors`, `membership`, `verification`
- current closeout:
  `python3 scripts/avatar_follow_samples.py closeout --format json`

これは helper-local evidence surface であり、final public runtime / visualization API ではありません。

## 6. まだ未決のこと

- `FAIRY-05` reacquire-after-return を active へ昇格するか
- membership / anchor / witness / visualization をどこまで同じ carrier に乗せるか
- future hot-plug / projection / transport widening とどこで接続するか

このため、phase 8 全体としてはまだ widening 余地があります。
current helper cut は widened representative slice であり、family 全体完成を意味しません。

## 7. historical prototype との関係

historical prototype は attach / detach の荒い発想を残すために有用です。
ただし、そのまま current active sample surface ではありません。

current repo の active line は `samples/clean-near-end/` であり、
avatar fairy follow もその中に widened representative slice を持つようになりました。
ただし `FAIRY-05` residual family は依然として `samples/not_implemented/` に残ります。
