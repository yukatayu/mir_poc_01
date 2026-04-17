# 449 — current L2 public operational CLI installed-binary and packaging success-criteria mixed-gate boundary note

## 目的

`specs/examples/394-current-l2-public-operational-cli-second-later-gate-threshold.md`、
`396-current-l2-final-public-parser-checker-runtime-thin-facade-later-support-threshold.md`、
`403-current-l2-public-operational-cli-concrete-shell-actualization-comparison.md`
で fixed した

- runtime-led thin facade
- current-L2 scoped Rust shell actualization
- packaging reserve

を前提に、
**installed-binary promotion と packaging success criteria をどこで mixed gate と読むか**
を docs-first に整理する。

ここで fixed するのは mixed-gate boundary であり、

- installed binary adoption
- final top-level `mir` hierarchy
- final public parser / checker / runtime packaging success criteria

は still later に残す。

## source-backed floor

- current public shell concern は `mir-current-l2 run-source-sample <sample> --host-plan <path> --format pretty|json` である。
- この shell concern は current-L2 scoped Rust concrete shell concern として actualize 済みである。
- principal library-side entry は `run_current_l2_source_sample` / report family に残っている。
- `resolve_current_l2_source_sample_path` や repo-local helper は public shell concern ではない。

## mixed-gate comparison

| candidate | reading | strengths | current risk |
|---|---|---|---|
| continue reserve only | packaging を reserve note のまま維持する | safest | installed-binary seam が曖昧なまま残る |
| mixed-gate boundary | current shell actualization と installed-binary / success criteria を明確に分ける | current repo memory と最も整合する | install 済みと誤読されやすい |
| install-ready adoption | current shell をそのまま installed-binary success criteria に昇格する | operational story は分かりやすい | backend / tooling success criteria を premature に固定する |

## current judgment

current L2 で最も自然なのは、
**mixed-gate boundary**
である。

current shell actualization は source-backed だが、
installed-binary promotion と packaging success criteria は
backend / tooling success criteria に触れるため current package の外へ送る。

## boundary floor

```text
public_operational_cli_mixed_gate = {
  runtime_led_thin_facade_refs[],
  shell_actualization_refs[],
  installed_binary_boundary_refs[],
  packaging_success_criteria_refs[],
  guard_refs[],
  kept_later_refs[]
}
```

## keep

- current-L2 scoped Rust shell actualization を current floor に維持する
- principal library-side entry を維持する
- installed-binary promotion と packaging success criteria は mixed gate に送る
- sample docs では repo-local shell concern と future installed-binary concern を分けて書く

## drop from current package

- installed binary adoption
- final top-level CLI hierarchy
- final public packaging success criteria
- backend / tooling success criteria の確定

## next promoted line

next promoted line は、
**Macro 5 boundary / pilot / framing closeout threshold**
に置く。

## what is not decided here

- installed binary promotion
- final CLI hierarchy
- backend / tooling success criteria
- final public parser / checker / runtime packaging success criteria
