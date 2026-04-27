# avatar_fairy_follow_plan_01

この文書は、phase 8 `avatar fairy follow / fallback anchor` の
planned sample family を reader-facing に説明する summary です。

規範判断の正本は `specs/` であり、ここは current plan と sample 境界の説明だけを行います。

## 1. current status

- active runnable representative sample はまだ **ありません**
- historical prototype anchor は
  `samples/prototype/current-l2-dynamic-attach-detach/p03-avatar-controller-attach-detach.txt`
  にあります
- phase 8 planned skeleton family は
  `samples/not_implemented/avatar-fairy-follow/`
  に切りました

この 2 つを混同しないことが重要です。

## 2. なぜ avatar fairy follow を次の representative slice 候補にするのか

Sugoroku vertical slice は membership / publish / witness / handoff を確認するには十分でした。
次に見たいのは、Place をまたぐ観測、visibility、fallback lineage、stale anchor rejection を
もっと直接に表す sample family です。

avatar fairy follow は、そのための小さく practical な題材です。

## 3. planned sample family

| ID | File | Goal | Expected debug focus |
|---|---|---|---|
| `FAIRY-01` | `01_follow_remote_head_with_local_fallback.mir` | remote head が見えている間は follow する | anchor graph / follow summary |
| `FAIRY-02` | `02_remote_head_not_visible_falls_back_to_local.mir` | visibility が落ちたら local anchor へ fallback する | fallback reason / visibility |
| `FAIRY-03` | `03_remote_avatar_leaves_falls_back_to_local.mir` | remote avatar leave 後に stale anchor を reject する | membership + fallback reason |
| `FAIRY-04` | `04_invalid_cross_anchor_chain_rejected.mir` | invalid cross-anchor chain を reject する | static rejection / lineage mismatch |
| `FAIRY-05` | `05_follow_target_reacquired_after_return.mir` | remote target 復帰後に reacquire する | state timeline / anchor switch |
| `FAIRY-06` | `06_model_check_no_detached_anchor_observed.mir` | detached anchor を観測しない safety line を置く | verification / model-check |

## 4. sample family で見たい性質

- `FollowAnchor` 自体は builtin magical primitive にしない
- visibility guard を explicit に置く
- remote -> local fallback lineage を explicit に書く
- stale anchor rejection を hidden repair にしない
- helper-local debug output を final public visualization と混同しない

## 5. まだ未決のこと

- exact helper command surface
- exact debug mode names
- active runner を Sugoroku helper に寄せるか、専用 helper を切るか
- membership / anchor / witness / visualization をどこまで同じ carrier に乗せるか

このため、current `%` は `10%` に留めます。
sample skeleton と説明はあるが、active helper / parser / loader / static carrier はまだない、という意味です。

## 6. historical prototype との関係

historical prototype は attach / detach の荒い発想を残すために有用です。
ただし、そのまま current active sample surface ではありません。

current repo の active line は `samples/clean-near-end/` であり、
avatar fairy follow はその次の docs-first / sample-first promotion candidate です。
