# 385 — current L2 sample-facing-theorem-model-check-evidence-summary-and-bless-review-flow-ready docs-first-io-host-facing-port-boundary comparison

## 目的

`specs/examples/384-current-l2-sample-facing-theorem-model-check-evidence-summary-and-bless-review-flow-ready-minimal-sample-facing-theorem-model-check-evidence-summary-and-bless-review-flow-threshold.md`
で sample-visible theorem/model-check milestone を閉じた次段として、

- `stdin/stdout` を language core の privileged primitive として扱うか、capability-scoped な I/O port / adapter boundary として切るか
- visualizer / host substrate / host runtime 側を current docs-first boundary に入れつつ、FFI / game engine adapter をどこまで still later に残すか
- current helper / harness を production host interface に誤昇格させず、working label のまま boundary を残せるか

を比較する。

ここで固定するのは
**current L2 sample-facing-theorem-model-check-evidence-summary-and-bless-review-flow-ready docs-first-io-host-facing-port-boundary comparison**
であり、

- actual host interface contract
- FFI / game engine adapter actualization
- final terminology / subsystem affiliation

はまだ固定しない。

## scope

- current package は docs-only comparison に留める。
- current sample-visible milestone と public operational later ordering を entry criteria とし、new code / host helper / public API は増やさない。
- current helper-local host harness、repo-local scripts、public CLI later gate は actual host-facing contract に昇格させない。

## current 前提

current repo では次が成立している。

1. `specs/examples/383...384` により、sample-facing theorem/model-check evidence summary と repo-local bless/review flow は docs-first package として固定済みである。
2. `specs/examples/363...364` と `371...372` により、public operational surface は `run_current_l2_source_sample` current gate、final public parser / checker / runtime API first later gate、public operational CLI second later gate の順に整理済みである。
3. `specs/07-typed-effects-wiring-platform.md` と `specs/10-open-questions.md` には、Typed-Effect Wiring Platform、richer host interface、visualizer / adapter / external substrate の high-level anchor がある。
4. `faq_003.md` と `plan/10-roadmap-overall.md` の current reading では、`stdin/stdout` privileged 化より先に docs-first の capability-scoped I/O / adapter boundary を切る方が自然だと整理済みである。

したがって current 問いは、
**language core を privileged I/O primitive で早く凍らせず、capability-scoped port / adapter boundary を docs-first に切り、その consumer/provider 側と later adapter line をどう押し分けるのが最小か**
である。

## 比較観点

1. privileged `stdin/stdout` 読みを避けつつ、I/O entry を current docs-only boundary に置けるか
2. visualizer / host substrate / host runtime 側を first cut に入れつつ、FFI / engine adapter を separate gate に残せるか
3. current host harness と production host interface を混ぜずに済むか
4. `host-facing port` を working label に留め、final terminology / subsystem affiliation を still open に保てるか

## 比較対象

### 案 1. capability-scoped I/O port / adapter boundary を docs-first first cut にし、visualizer / host substrate / host runtime を consumer/provider 側、FFI / engine adapter を later gate に置く

#### shape

```text
docs_first_io_host_facing_boundary = {
  boundary_kind = current_l2_docs_first_host_facing_io_adapter_boundary,
  current_boundary_refs = [
    capability_scoped_input_output_port_boundary,
    docs_only_host_adapter_boundary
  ],
  consumer_provider_refs = [
    visualizer_consumer_side,
    host_substrate_provider_side,
    host_runtime_provider_side
  ],
  guard_refs = [
    avoid_privileged_stdin_stdout_primitive,
    keep_current_host_harness_non_production,
    keep_working_label_non_final,
    keep_typed_effect_and_mirrorea_affiliation_open
  ],
  kept_later_refs = [
    actual_host_interface_contract,
    ffi_adapter_line,
    game_engine_adapter_line,
    final_term_selection
  ]
}
```

#### 利点

- language core に privileged I/O primitive を持ち込まずに済む。
- visualizer / host substrate / host runtime を consumer/provider 側として先に整理できる。
- FFI / game engine adapter を actual binding line として separate gate に保てる。

#### 欠点

- actual host interface contract や final naming は次段へ残る。

### 案 2. `stdin/stdout` を privileged primitive として先に固定する

#### 利点

- surface は一見分かりやすい。

#### 欠点

- language core に host-specific intuition を早く固定しやすい。
- Typed-Effect / adapter boundary と current docs reading に逆流しやすい。

### 案 3. visualizer / host substrate / FFI / engine adapter を同じ package で actual target inventory にする

#### 利点

- future integration candidates は一度に見える。

#### 欠点

- docs-first boundary と heavy actualization line が混ざる。
- first target profile を user decision 前に早く固定しやすい。

## current judgment

current L2 で最も自然なのは、
**案 1. capability-scoped I/O port / adapter boundary を docs-first first cut にし、visualizer / host substrate / host runtime を consumer/provider 側、FFI / engine adapter を later gate に置く**
である。

理由は次の通り。

1. current repo は fixed-subset runnable path を持つが、host-facing integration は still docs-first boundary の段階であり、privileged primitive や production host interface を早く固定しない方がよい。
2. visualizer / host substrate / host runtime と FFI / engine adapter は integration cost と user decision 量が違うため、first cut では押し分ける方が drift が少ない。
3. `host-facing port` を working label に留め、Typed-Effect Wiring Platform が Mirrorea の subsystem か sibling かも OPEN に残す方が、later actualization の自由度を保てる。

## current first choice details

- language core では `stdin/stdout` privileged primitive を採らない。
- first docs-only boundary は capability-scoped input/output port / adapter boundary に置く。
- visualizer / host substrate / host runtime は consumer/provider 側の first inventory に置く。
- current host harness / repo-local helper は production host interface に昇格させない。
- FFI / game engine adapter は later gate に残し、first target profile は user decision 前に固定しない。
- `host-facing port` は settled term にせず、working label に留める。
- Typed-Effect Wiring Platform と Mirrorea/shared-space の affiliation は OPEN に残す。

## next promoted line

next promoted line は、
**docs-first-io-host-facing-port-boundary-ready stable-malformed-missing-option-first-reopen-actualization comparison**
に置く。

## open questions

- capability-scoped port / adapter boundary の final term をどこで narrow に選ぶか
- visualizer / host substrate / host runtime の first target profile をどの順で actualize するか
- FFI / game engine adapter をどの adapter contract から reopen するか
- Typed-Effect Wiring Platform と Mirrorea/shared-space の subsystem affiliation をどこで fixed するか
