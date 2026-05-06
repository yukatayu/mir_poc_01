# operational-product-sample-001 — index

この handoff は、現行 product alpha-1 release-candidate workflow の次に作るべき **実運用プロセス再現 sample suite** のための情報を集約する。

## 基本判断

- 現 repo は product alpha release-candidate workflow-ready だが、final public product ではない。
- 現 product alpha entrypoint は `package.mir.json` + Rust CLI `mirrorea-alpha`。
- direct textual `.mir` grammar と direct Mir-to-machine-code backend は今回の scope ではない。
- 次に必要なのは、world core / membership-chat / sugoroku / hot-plug / devtools / save-load / bundle / portal future を一続きで示す operational sample suite。
- これは product alpha demo を置き換えるものではなく、より実運用寄りの canonical sample line として追加する。

## ファイル一覧

- `01-current-state.md`
  - 現在何ができていて、何がまだ未完か。
- `02-decisions.md`
  - 今回固定する意思決定。
- `03-sample-suite-architecture.md`
  - WorldCore -> MembershipChat -> Sugoroku -> HotPlug -> Devtools の構造。
- `04-source-import-package-model.md`
  - `.mir` 代表 source と `package.mir.json` executable alpha surface の扱い。
- `05-runtime-host-projection-native.md`
  - server/client split、runtime host、native host bundle、future LLVM boundary。
- `06-observability-diagrams.md`
  - どの図・panel・export を出すべきか。
- `07-portal-spatial-future.md`
  - portal / hyperlink world-link / shard / gradient observation / federation の境界。
- `08-theory-verification.md`
  - 型、契約、cut、save/load、auth、replication profile の理論境界。
- `09-sample-matrix.md`
  - OPS-01..OPS-08 sample matrix。
- `10-repo-change-plan.md`
  - 追加する specs / plan / samples / scripts / docs。
- `11-validation-plan.md`
  - 通すべき command と skip 条件。
- `12-subagent-plan.md`
  - sub-agent review 計画。
- `13-operational-rules.md`
  - Codex の作業規則 / stop line / commit push。
- `14-risk-register.md`
  - 危険な誤読と mitigation。
- `15-next-packages.md`
  - P-OPS-01 の後の候補 package。

## sample-blueprints

- `world-core-blueprint.md`
- `membership-chat-blueprint.md`
- `sugoroku-world-blueprint.md`
- `deployment-and-projection-blueprint.md`
- `portal-and-spatial-blueprint.md`

これらはそのまま repo にコピペするものではなく、Codex が `samples/product-alpha1/operational/` の実ファイルを作る時の source of truth として使う。

## 優先順位

矛盾時の優先順位:

```text
specs/ > plan/ > progress.md / tasks.md / samples_progress.md > docs/reports/ > Documentation.md / README.md > tmp_faq/ > this handoff
```

ただし、この handoff は user の最新意思決定を反映する。古い docs と conflict する場合は、規範文書へ mirror して解消する。
