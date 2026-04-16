# 427 — current L2 sample-visible theorem model-check property summary wording

## 目的

sample-facing docs / README / bless-review flow に出す
theorem/model-check summary wording を、

- row-local machine-facing carrier
- small-cluster projection reserve
- sample-facing summary

の 3 層を崩さずに整理する。

ここで fixed するのは
**sample-facing summary が何を言ってよく、何をまだ言わないか**
であり、

- final property language
- concrete model-check tool binding
- production checker/runtime-policy contract

は still later に残す。

## source-backed floor

- current line は
  `formal_hook -> row-local machine-facing carrier -> emitted artifact wiring -> sample-facing summary`
  までである。
- row-local carrier は machine-facing floor に留める。
- small-cluster semantic projection は reserve に留める。
- room protocol / fairness / replay / provider receipt family は order/handoff reserve に残す。

## current judgment

1. sample-facing summary は
   **carrier floor を row-local machine-facing に保ったまま、
   theorem/model-check evidence の presence / guard / reached status を要約する wording**
   に留める。
2. theorem-side summary と model-check-side summary は並置してよいが、
   これを settled property language と書かない。
3. `inventory` / `regression` success と reviewed repo-local sync は current bless flow の floor に留め、
   retained external artifact や production receipt を前提にしない。
4. room protocol / fairness / replay / provider receipt family は sample-facing summary 側へ押し込まない。

## guard

- row-local carrier を settled property language と書かない。
- room protocol / fairness / replay family を sample summary に混ぜない。
- concrete model-check brand を fixed しない。
- production checker/runtime-policy contract を fixed しない。

## next promoted line

next promoted line は、
**order / handoff property-language bridge note**
に置く。

## what is not decided here

- first settled property language
- concrete model-check tool binding
- production checker/runtime-policy contract
