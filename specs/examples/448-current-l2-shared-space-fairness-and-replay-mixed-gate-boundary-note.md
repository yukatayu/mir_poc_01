# 448 — current L2 shared-space fairness and replay mixed-gate boundary note

## 目的

`specs/examples/432-current-l2-shared-space-fairness-and-replay-strengthening-reserve-note.md`
と
`plan/16-shared-space-membership-and-example-boundary.md`
で fixed した

- authoritative room minimal scenario
- fairness / replay strengthening reserve
- provider placement / witness requirement / fairness source / replay attachment の軸分解

を前提に、
**shared-space fairness / replay line をどこで mixed gate と読むか**
を docs-first に整理する。

ここで fixed するのは mixed-gate boundary であり、

- final operational catalog
- fairness operational profile
- concrete replay protocol
- application-specific room policy

は still later に残す。

## source-backed floor

- shared-space line は identity / admission / authority / room-profile までは docs-first に進んでいる。
- fairness / replay strengthening reserve は already fixed 済みである。
- provider placement、witness requirement、fairness source / trust model、replay attachment、payload / observation visibility は separate axes として保たれている。
- replay / fairness family は主に `protocol_verifier_boundary` と `runtime_policy_boundary` に残る。

## mixed-gate comparison

| candidate | reading | strengths | current risk |
|---|---|---|---|
| continue self-driven reserve | fairness / replay line をもう少し docs-first で進める | comparison は増やせる | final catalog を hidden promotion しやすい |
| mixed-gate boundary | axis separation と stop line を固定し、operational profile は mixed gate に送る | current repo memory と最も整合する | self-driven closeout と誤読されやすい |
| user-spec-required now | ただちに user decision 領域へ送る | overclaim は防げる | docs-first boundary が薄く終わる |

## current judgment

current L2 で最も自然なのは、
**mixed-gate boundary**
である。

current package が固定するのは、
fairness / replay line が

- still useful docs-first comparison を持つ一方で、
- final operational profile / final room policy / final catalog に踏み込む時点で mixed gate へ入る

という boundary である。

## boundary floor

```text
shared_space_fairness_replay_mixed_gate = {
  authority_placement_refs[],
  provider_placement_refs[],
  witness_requirement_refs[],
  fairness_source_refs[],
  replay_attachment_refs[],
  payload_visibility_refs[],
  mixed_gate_refs[],
  kept_later_refs[]
}
```

## keep

- provider placement / witness requirement / fairness source / replay attachment を別軸に保つ
- authoritative room motivating scenario を comparison material に保つ
- fairness / replay family を checker floor に collapse しない
- final operational catalog / profile は mixed gate に送る

## drop from current package

- final fairness policy
- final replay invalidation protocol
- final room-profile catalog
- distributed fairness protocol / concrete authority binding の adoption

## next promoted line

next promoted line は、
**public operational CLI installed-binary / packaging success-criteria mixed-gate boundary note**
に置く。

## what is not decided here

- final operational catalog
- fairness operational profile
- replay invalidation protocol
- application-specific room policy
- concrete authority binding / provider protocol
