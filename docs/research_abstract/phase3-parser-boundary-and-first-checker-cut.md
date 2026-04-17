# Phase 3 要約 — parser boundary と first checker cut

## この phase の役割

Phase 3 は、**final grammar を決めずに parser boundary と first checker cut を staged spike として切る phase** である。

## 固まった current reading

- stage 1:
  option / chain / lineage / declaration-side guard slot の structural floor
- stage 2:
  `try { ... } fallback { ... }` と `atomic_cut` の structural floor
- stage 3:
  minimal `admit / require / ensure / predicate fragment` floor
- first checker cut には local / structural / decidable なものだけを入れる。
- runtime / theorem / protocol / policy を checker cut に押し込まない。

## source-backed evidence

- `mir-ast` current L2 staged spike
- stage 1 / stage 2 / stage 3 test suites
- parser-to-checker reconnect freeze line

## まだここで決めていないこと

- full request parser
- final lexical / precedence choice
- rich diagnostics / spans
- public parser / public checker API

## 次へ渡したもの

compile-ready code path は、この narrow parser carrier と checker floor を private staged evidence として参照できる。
