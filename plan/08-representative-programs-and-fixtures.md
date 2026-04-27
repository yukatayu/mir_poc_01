# plan/08 — representative programs と fixtures

## representative stack

current repo では sample 表現を少なくとも次の 5 層に分ける。

1. `samples/current-l2/`
   - base fixed-subset source corpus
2. `samples/clean-near-end/`
   - active clean sample suite
3. `samples/not_implemented/`
   - planned skeleton family
4. fixture corpus
   - machine-readable detached/static/interpreter validation
5. archive
   - `samples/old/2026-04-22-pre-clean-near-end/`
   - `samples/lean/old/2026-04-22-pre-clean-near-end/`

## active sample policy

- active canonical sample root は `samples/clean-near-end/`
- active base source corpus は `samples/current-l2/`
- active proof evidence は `samples/lean/`
- active clean suite family は `typing` / `order-handoff` / `model-check` / `modal`
- generated Lean stub は `samples/lean/clean-near-end/`
- `samples/clean-near-end/sugoroku-world/` 10 本が current Mirrorea vertical-slice family である
- `samples/clean-near-end/avatar-follow/` 5 本が current phase 8 representative-slice family である
- `samples/clean-near-end/network-transport/` は current phase 13 helper-local canary landing page であり、primary source は主に Sugoroku family を再利用する

## base source corpus policy

- `samples/current-l2/` は base fixed-subset corpus
- extension `.txt` は final grammar を意味しない
- authored sixteen base corpus は current runner / lowerer / ladder evidence として維持

## archive policy

- old prototype / problem-bundle / old Lean corpus は archive に移す
- `samples/not_implemented/` は archive ではなく planned skeleton family として維持する
- `samples/generated/` は non-Lean generated sample artifact の reserve として空けておく
- old `p..` sample は active language sample ではない
- historical comparison が必要なときだけ archive を参照する

## planned skeleton policy

- `samples/not_implemented/avatar-fairy-follow/` は phase 8 residual planned family であり、現在は `FAIRY-05` を active representative slice の外側に残す
- `samples/not_implemented/typed-external-boundary/` は phase 9 planned family であり、current helper-local evidence は `EXT-03` / `EXT-04` synthetic preview subset に留める
- `samples/not_implemented/network-transport/` は phase 13 future source/backlog family であり、active helper-local canary とは別に残す
- planned skeleton は `samples_progress.md` で `10%` 以下の row として追い、active closeout evidence と混同しない

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
