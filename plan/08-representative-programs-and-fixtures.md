# plan/08 — representative programs と fixtures

## representative stack

current repo では sample 表現を少なくとも次の 4 層に分ける。

1. `samples/current-l2/`
   - base fixed-subset source corpus
2. `samples/clean-near-end/`
   - active clean sample suite
3. fixture corpus
   - machine-readable detached/static/interpreter validation
4. archive
   - `samples/old/2026-04-22-pre-clean-near-end/`
   - `samples/lean/old/2026-04-22-pre-clean-near-end/`

## active sample policy

- active canonical sample root は `samples/clean-near-end/`
- family は `typing` / `order-handoff` / `model-check` / `modal`
- current suite sizeは 16 本
- generated Lean stub は `samples/lean/clean-near-end/`

## base source corpus policy

- `samples/current-l2/` は base fixed-subset corpus
- extension `.txt` は final grammar を意味しない
- authored sixteen base corpus は current runner / lowerer / ladder evidence として維持

## archive policy

- old prototype / not_implemented / problem-bundle / old Lean corpus は archive に移す
- old `p..` sample は active language sample ではない
- historical comparison が必要なときだけ archive を参照する

## active suite families

### typing

- authorized declassification success
- unauthorized declassification rejection
- label-flow rejection
- capture escape rejection
- cost-bound rejection

### order / handoff

- authorized publish/handoff success
- missing witness rejection
- handoff-before-publication rejection
- stage-block authorized handoff
- delegated RNG practical route
- auditable authority witness

### model-check

- Peterson SC pass
- Peterson relaxed counterexample
- broken mutex counterexample

### modal

- stable/later minimal
- published/witnessed bridge

## current judgment

- active current layer は clean suite を first に読む
- base current-l2 corpus は lowerer / fixed-subset floor を支える
- archive は historical comparison に限定する
