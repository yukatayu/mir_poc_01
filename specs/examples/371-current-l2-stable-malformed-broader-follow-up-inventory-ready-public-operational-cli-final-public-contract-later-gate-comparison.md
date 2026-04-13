# 371 — current L2 stable-malformed-broader-follow-up-inventory-ready public-operational-cli-final-public-contract-later-gate comparison

## 目的

`specs/examples/370-current-l2-stable-malformed-broader-follow-up-inventory-ready-minimal-stable-malformed-broader-follow-up-inventory-threshold.md`
で stable malformed broader follow-up inventory の minimum を fixed した次段として、

- `run_current_l2_source_sample` first docs-only candidate を保ったまま、final public parser / checker / runtime API と public operational CLI をどの順で reopen するか
- library-side final public contract と repo-local operational CLI をどこまで separate gate に保つか
- repo layout / accepted-set / explicit host-plan coupling / repo-local helper surface をどこまで current contract の外へ残すか

を比較する。

ここで固定するのは
**current L2 stable-malformed-broader-follow-up-inventory-ready public-operational-cli-final-public-contract-later-gate comparison**
であり、

- final public parser / checker / runtime API の具体形
- public operational CLI の具体形
- host-plan decoupling の具体形

はまだ固定しない。

## scope

- current package は docs-only later-gate close に留める。
- `run_current_l2_source_sample` current gate は entry criteria として維持し、new public symbol promotion は行わない。
- repo-local scripts / examples / support helpers は public contract 候補へ上げない。

## current 前提

current repo では次が成立している。

1. `specs/examples/355...356` により、public-looking surface は already-public parser-free stack / crate-public but non-production tranche / repo-local helper surface の 3 bucket split に整理済みである。
2. `specs/examples/363...364` により、later public pressure の first docs-only candidate は `mir_runtime::current_l2::run_current_l2_source_sample` に narrow に置いている。
3. `plan/09-helper-stack-and-responsibility-map.md` の current reading では、final public parser / checker / runtime API、public runner / exporter CLI、public theorem / model-check / checker migration は still later に残している。
4. current package の entry criteria は library-side current candidate を保ったまま、その先の later ordering だけを narrow に決めることである。

したがって current 問いは、
**`run_current_l2_source_sample` current gate を保ったまま、final public parser / checker / runtime API を first later gate、public operational CLI を second later gate に分けて残すのが最小か**
である。

## 比較観点

1. first docs-only candidate と final public contract を混同せずに済むか
2. library-side final public contract と CLI を別 gate に保てるか
3. repo layout / accepted-set / explicit host-plan coupling を current final contract に混ぜずに済むか
4. repo-level next line を shared-space admission / compile-time visibility reopen へ clean に handoff できるか

## 比較対象

### 案 1. final public parser / checker / runtime API を first later gate、public operational CLI を second later gate に置く

#### shape

```text
public_operational_later_gate = {
  current_candidate = run_current_l2_source_sample,
  first_later_gate = final_public_parser_checker_runtime_api,
  second_later_gate = public_operational_cli,
  excluded = [
    resolve_current_l2_source_sample_path,
    accepted_set_hard_coding,
    repo_local_examples_and_python_helpers
  ]
}
```

#### 利点

- current docs-only candidate が library entry であることと整合する。
- CLI を library contract の上に載る later operational surface として separate gate に保てる。
- repo layout / accepted-set / host-plan coupling を final public contract へ早く混ぜずに済む。

#### 欠点

- final public library contract actualization は別 package に残る。

### 案 2. public operational CLI を first later gate にする

#### 利点

- operational usage entry は見えやすい。

#### 欠点

- current candidate が library entry なのに CLI を先に切ることになり、順序が不自然である。
- repo-local script / wrapper surface を final public contract と混ぜやすい。

### 案 3. final public contract と public CLI を同じ later gate にする

#### 利点

- future line は単純に見える。

#### 欠点

- library-side contract と operational CLI の責務が混ざる。
- host-plan / repo layout / helper boundary を separate に残しにくい。

## current judgment

current L2 で最も自然なのは、
**案 1. final public parser / checker / runtime API を first later gate、public operational CLI を second later gate に置く**
である。

理由は次の通り。

1. current docs-only candidate は `run_current_l2_source_sample` という library-side entry なので、その先の first later gate も library-side final contract と読む方が自然である。
2. CLI は repo-level operational wrapper であり、library-side final contract と separate gate にした方が coupling を抑えられる。
3. `resolve_current_l2_source_sample_path`、accepted-set hard-coding、repo-local Python / example helpers は still non-public bucket に残す必要があり、CLI first にすると boundary が崩れやすい。

## current first choice details

- current gate entry は `run_current_l2_source_sample` に据え置く。
- first later gate は final public parser / checker / runtime API に置く。
- second later gate は public operational CLI に置く。
- `run_current_l2_runtime_skeleton` / `lower_current_l2_fixed_source_text` / parser carrier は first later gate の support pressure 候補へ残し、current contract には still 混ぜない。
- `resolve_current_l2_source_sample_path`、accepted-set hard-coding、repo-local examples / support modules / Python orchestration helper は excluded bucket に残す。
- theorem / model-check / public-checker migration line はこの package に混ぜず、separate later line に残す。

## next promoted line

next promoted line は、
**public-operational-cli-final-public-contract-later-gate-ready shared-space-admission-compile-time-visibility-reopen comparison**
に置く。

## open questions

- final public library contract 側で `FixtureHostPlan` coupling をどう薄くするか
- `run_current_l2_runtime_skeleton` を later public-facing support として扱う必要があるか
- public operational CLI second gate を final public library contract actualization の直後に開くか
- public theorem / model-check / checker migration pressure と public contract line をいつ接続するか
