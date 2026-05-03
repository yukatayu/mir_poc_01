# 13 — Autonomous Package Sequence

## P-A1-00 practical-alpha-rebaseline

Actions:

- add `specs/18-practical-alpha1-scope.md`
- add `plan/44-practical-alpha1-roadmap.md`
- update progress/tasks/samples_progress to separate evidence closeout and practical readiness
- update README/Documentation if needed
- report + validation + commit/push

## P-A1-01 alpha-source front-door design

Actions:

- define limited alpha source/package format
- add initial source fixtures under `samples/practical-alpha1/`
- implement parser/loader or JSON loader if parser is too early, but document as front-door
- add parse positive/negative tests

## P-A1-02 typed IR/checker integration

Actions:

- define IR structures or crate/module
- connect existing LIF/VAR/CUT obligations to IR checker
- add CLI/script `check`
- ensure positive proof is explicit, not absence of reject

## P-A1-03 local runtime from runtime plan

Actions:

- make runtime consume checked plan, not sample ID only
- implement minimal reusable world runtime
- run local world from practical source package
- export event DAG

## P-A1-04 package/hot-plug practical API

Actions:

- package manifest schema
- package admission checker
- attach debug/rate-limit/auth layer from manifest
- object package attach
- incompatible/missing capability negatives

## P-A1-05 transport practical E2E

Actions:

- run practical package in Docker/local TCP
- use same package input as local run
- verify lanes separated
- route trace export

## P-A1-06 devtools viewer

Actions:

- create JSON schemas and viewer command
- HTML/devtools viewer if feasible
- event DAG / route / membership / hot-plug / fallback

## P-A1-07 local save/load command

Actions:

- save/load through practical CLI/runtime
- local roundtrip
- stale membership non-resurrection
- invalid distributed cut reject

## P-A1-08 practical alpha product prototype

Actions:

- full run of one product-like world
- local + Docker
- package/layer attach
- avatar fallback
- local save/load
- devtools export
- hands-on docs

## Rule for continuation

After each package, if next package is narrow and no user decision is needed, continue. If next step requires U1 or a product choice, stop with a precise blocker.
