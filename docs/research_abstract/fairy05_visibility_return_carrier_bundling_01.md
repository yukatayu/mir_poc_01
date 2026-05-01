# FAIRY-05 visibility-return carrier bundling 01

## 何を整理したか

`R3` では、`FAIRY-05` を active 化せずに、
visibility-return witness をどの carrier shape で読むのが repo-local boundary reading として
一番 honest かを docs-first に narrow にした。

## current fixed facts

- `FAIRY-05` は planned family のまま
- helper closeout `fairy05_reopen_gate` は implementation inventory として
  `carrier_choice = UNRESOLVED` のまま
- `state_timeline` / `anchor_switch` は planning-only candidate labels
- planned sample は、visibility-return witness が timeline witness refs に現れてから
  switch が起きるべきだと読める

## candidate matrix

- timeline event
  - ordering は見やすい
  - switch boundary が弱い
- anchor-switch event
  - switch boundary は見やすい
  - ordering / witness lineage が弱い
- witness event
  - witness 自体は explicit
  - switch frontier と ordering が弱い
- typed bundle
  - timeline + switch + witness ref を一緒に持てる
  - final public API と誤読させない stop line が必要

## provisional recommendation

`R3` close-time provisional recommendation は、
`FAIRY-05` を **typed bundle over `state_timeline` + `anchor_switch`** として読むことです。

ただし、これは helper closeout schema の変更ではありません。
implementation inventory は `UNRESOLVED` のまま残し、
repo memory 側だけが provisional recommendation を持ちます。

visibility-return witness は standalone top-level carrier ではなく、
timeline witness refs の内側に入る reading を repository memory に置きます。

## 何を言っていないか

- `FAIRY-05` が active runnable sample になった
- `state_timeline` / `anchor_switch` が current debug mode になった
- final public avatar / visualization API が決まった

## 関連

- `../../plan/31-fairy05-visibility-return-carrier-bundling.md`
- `avatar_fairy_follow_plan_01.md`
- `../hands_on/fairy05_visibility_return_carrier_bundling_01.md`
