# Phase 2 要約 — parser-free PoC と detached validation loop

## 何をした phase か

Phase 2 は、current L2 semantics を
**parser をまだ固定せずに、fixture と helper で継続検証できる形へ落とす phase**
である。

主に整えたのは次である。

- AST fixture schema
- evaluation state / step semantics
- host stub harness
- bundle / batch / selection / profile helper
- detached artifact の emit / save / compare

## 実際の loop

現在の最小 loop は、おおむね次のように読める。

```text
fixture JSON
  -> run_bundle
  -> detached bundle artifact
  -> optional aggregate artifact
  -> compare helper
```

artifact 側は、少なくとも次を分ける。

- `payload_core`
- `bundle_context`
- `detached_noncore`
- explanation

ここで compare の中心にするのは `payload_core` だけである。

## 具体例

たとえば 2 つの fixture を比べるとき、current narrow compare は次のような core に絞る。

```json
{
  "static_verdict": "runtime_ready",
  "entered_evaluation": true,
  "terminal_outcome": "success",
  "event_kinds": ["perform", "commit"],
  "non_admissible_metadata": [],
  "narrative_explanations": ["writer selected"]
}
```

`must_explain` や長文 audit は compare core に上げない。

## なぜ parser-free を続けたか

もし final parser grammar を先に決めると、

- syntax bug
- semantics regression
- fixture authoring helper の推論

が混ざりやすい。

Phase 2 ではそれを避けるため、
**hand-written fixture を正本にして、helper は non-production の補助に留める**
方針を取った。

## この phase で得たもの

- current L2 を「1 本ずつ狭く回す」だけでなく、
  「実行し、artifact を保存し、比較し、fixture を足してまた回す」入口
- detached exporter と aggregate summary の最小 cut
- fixture authoring template と scaffold helper

## まだ未決のもの

- final exporter API
- final storage/path policy
- richer host interface
- typed coverage carrier の本格 actualization

これらは後段に残している。

## 次 phase へ渡したもの

Phase 3 は、Phase 2 の loop を壊さずに、
parser boundary と first checker cut を private staged spike として narrow に切る。
