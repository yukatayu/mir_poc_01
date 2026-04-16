# 442 — current L2 order/handoff source-surface wording reserve note

## 目的

`specs/examples/436-current-l2-order-handoff-emitted-artifact-schema-reserve-note.md`
までで fixed した

- cut family decomposition
- order relation decomposition
- adequacy corpus
- property-language bridge
- refs-only emitted-artifact reserve schema

を前提に、
**final syntax を固定せずに、order / handoff line をどう説明するか**
の docs-first wording reserve を整理する。

ここで fixed するのは説明語彙の cut であり、

- final source-surface handoff syntax
- low-level fence-like public tokens
- concrete theorem / protocol tool schema

は still later に残す。

## source-backed floor

- current working relation names は
  `program_order / dependency_order / publication_order / observation_order / witness_order / finalization_order / scoped_happens_before`
  である。
- `atomic_cut` は local finalization nucleus であり、`barrier` / snapshot-only observation / `durable_cut` と同一視しない。
- `authority_serial_transition_family` が first kept candidate、`witness_aware_commit_family` が second candidate である。
- dice-owner handoff family は adequacy corpus の motivating scenario に already ある。

## wording comparison

| candidate | reading | strengths | current risk |
|---|---|---|---|
| relation-name-first wording | relation family 名をそのまま前面に出す | theory / verifier mapping に強い | user-facing prose では dry に見える |
| plain-language stage wording | `公開する / 観測する / 受け渡し証拠を持つ / 局所確定する` のような段階語で説明する | example explanation に強い | relation family との対応が曖昧になる |
| low-level fence wording | `fence<seq_cst>` のような backend-like token で説明する | hardware / compiler reference には近い | source surface を premature に固定する |
| room-macro wording | `update_global_state` のような high-level macro で説明する | room profile macro の直観は強い | publication / witness / finalization の分解が消える |

## current judgment

current L2 で最も自然なのは、
**snake_case relation family 名を principal wording に保ちつつ、
plain-language stage wording を explanation layer として重ねる**
ことである。

## explanation floor

current package が持つ minimum は次である。

| docs-first wording | current guarantee | current non-guarantee |
|---|---|---|
| `publication_order` / 公開する | handoff 前に公開側へ結果を出したことを表す | 受け手が見たことや durability を意味しない |
| `observation_order` / 観測する | 受け手が公開済みの結果を見たことを表す | witness や replay safety を意味しない |
| `witness_order` / 受け渡し証拠を持つ | receipt / proof / handoff evidence を表す | fairness や total order を意味しない |
| `finalization_order` / 局所確定する | local rollback frontier を越えられなくする | global snapshot や durable commit を意味しない |
| `authority_serial_transition_family` / 権限遷移を直列化する | room-local authority handoff を過剰な overlap なしに読む | fairness / provider proof / global consensus を意味しない |

## keep

- snake_case relation family 名を working docs vocabulary に維持する
- dice-owner handoff のような motivating scenario では plain Japanese verbs を補助に使う
- low-level `memory_order` / `kill_dependency` line は reference family に留める
- rough syntax stimuli A-D は comparison material に留める

## drop from current package

- final source syntax token
- `fence<seq_cst>` 系 public wording
- room macro / transaction-like sugar の採用
- consumer-shaped tool schema

## guard

- `atomic_cut` を snapshot / durable cut / seq_cst fence と短絡しない
- snake_case relation names を final public keyword と書かない
- rough dice syntax を adopted surface と書かない
- thread と node を完全同一の runtime artifact と書かない

## next promoted line

next promoted line は、
**modality internalization trigger note**
に置く。

## what is not decided here

- final source-surface handoff syntax
- final emitted-artifact schema
- final property language
- low-level `memory_order` final stance
- concrete theorem / protocol tool binding
