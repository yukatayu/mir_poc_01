# public API / parser grammar gate plan 01

## この文書の役割

この文書は、`P18` public API / parser grammar gate の
**reader-facing summary** です。

- 規範判断の正本は `specs/`
- 長期の repository memory は `plan/`
- ここでは repo-side first cut で何を fixed し、
  何を still later に残したかを短く読む

## current reading

`P18` は final public freeze そのものではありません。
current repo で close したのは、次の docs-first inventory です。

- final parser grammar と current companion notation を混同しない
- current parser-free / helper / crate-local cut を final public API と混同しない
- viewer / adapter / projection / hot-plug / transport / auth の current preview に qualifier を残す
- mixed gate と true user-spec hold line を分ける

## repo-side で fixed したこと

- current `.mir` surface は companion notation であり、final grammar ではない
- `mir_hilight.html` は readable viewer であり、parser / checker / verifier ではない
- `VerificationLayer` current reading は typed explanation / evidence carrier に留め、emitted row / preview / downstream consumer / later handoff-contract の split を collapse しない
- `MessageEnvelope` / `AuthEvidence` seam は current lane inventory として読む
- visualization / telemetry は `label` / `authority` / `redaction` /
  `retention_scope` / `source_refs` を持つ typed effect として読む
- typed external `host_boundary`、projection validity report、hot-plug package inventory、
  network `process_boundary`、storage guardrail は public-boundary inventory として読む

## still later

次は still later です。

- final parser grammar
- final public parser / checker / runtime / verifier API
- final public auth / adapter / viewer / projection / hot-plug / transport ABI
- final public visualization / telemetry schema
- exact host schema
- actual LLVM build / backend choice

## post-`P18` true user-spec hold line

repo-side framingの後でも、次は user choice が要ります。

- installed binary / packaging adoption target
- FFI / engine adapter / host integration target
- first shipped public surface scope
- broader application target
- exhaustive shared-space operational catalog

## 関連文書

- `../hands_on/public_api_parser_gate_01.md`
- `mirrorea_future_axis_01.md`
- `../../plan/27-public-api-parser-gate-roadmap.md`
- `../../progress.md`
- `../../tasks.md`
