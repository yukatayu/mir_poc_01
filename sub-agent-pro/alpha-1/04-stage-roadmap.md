# 04 — Practical Alpha-1 Roadmap

## 1. New stage model

従来 Stage A..F は evidence closeout として保持する。Practical alpha-1 は新しい stage model を使う。

## PA1-0 — Rebaseline / honesty repair

Goal:
- progress semantics を practical 100% 前提へ修正。
- specs / plan を追加。
- current-scope evidence closeout を practical readiness と分離。

Deliverables:
- `specs/18-practical-alpha1-scope.md`
- `plan/44-practical-alpha1-roadmap.md`
- updated `progress.md`, `tasks.md`, `samples_progress.md`
- report

## PA1-1 — Alpha source front-door

Goal:
- limited alpha grammar / parser / source package manifest.
- at least one source file parses into IR.

Deliverables:
- alpha source format docs
- parser crate/module or script
- positive/negative parse tests
- source package sample

Non-goal:
- final public parser grammar

## PA1-2 — Typed IR and checker

Goal:
- reusable checker pipeline.
- existing synthetic LIF/VAR/CUT rows become checker tests over IR where possible.

Deliverables:
- IR definitions
- checker diagnostics
- LIF/VAR/CUT checker tests
- CLI `check` command

## PA1-3 — Runtime plan execution

Goal:
- IR/runtime plan can run local world without sample-ID keyed bridge.

Deliverables:
- reusable runtime API
- CLI `run-local`
- event DAG export
- local Sugoroku-style world sample

## PA1-4 — Package and hot-plug API

Goal:
- runtime package / layer / object manifest admission.
- HotPlugRequest / Verdict / activation cut in reusable API.

Deliverables:
- package manifest schema
- package checker
- debug/rate-limit/auth layer samples
- object package attach sample

## PA1-5 — Transport and Docker product E2E

Goal:
- product-like Docker/local TCP E2E from alpha package, not only hand-coded sample-ID bridge.

Deliverables:
- transport config
- CLI `run-docker` or test harness
- stale membership / missing capability / missing witness negatives
- observer-safe route trace

## PA1-6 — Devtools / viewer

Goal:
- developer can inspect event DAG, route trace, membership timeline, hot-plug lifecycle, fallback degradation.

Deliverables:
- JSON schema
- HTML/devtools viewer
- CLI `export-devtools`
- redaction tests

## PA1-7 — Local save/load

Goal:
- local save/load command over runtime package.
- invalid distributed cut rejection remains explicit.

Deliverables:
- CLI `save`, `load` or equivalent
- local roundtrip test
- stale membership non-resurrection
- invalid distributed cut negative sample

## PA1-8 — Practical product prototype

Goal:
- a small product prototype can be built and run.

Deliverables:
- example world package
- local + Docker modes
- package/layer hot-plug
- avatar placeholder/custom package
- local save/load
- devtools viewer
- README hands-on

## 2. Completion policy

A stage is 100% only when:

- implementation exists
- positive and negative samples pass
- debug/devtools output exists if relevant
- docs are updated
- report exists
- validation ran
- commit/push completed
- non-claims are explicit

## 3. No stage may claim practical alpha-1 alone

Practical alpha-1 requires PA1-0..PA1-8. Individual stage completion is not product completion.
