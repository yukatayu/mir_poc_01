# Report 0375 — shared space authoritative room delegated rng provider placement

- Date: 2026-04-09T07:47:15.068668Z
- Author / agent: Codex
- Scope: Phase 4 shared-space / membership side line の current working subset から、authoritative room 側の `delegated_rng_service` provider-placement practical cut を docs-first に整理する
- Decision levels touched: L2 / L3

## 1. Objective

`specs/examples/121-shared-space-authoritative-room-baseline.md` の authoritative room baseline と、
`specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md` の minimal witness core を前提に、

- `delegated_rng_service`

を authoritative room 側でも provider-placement candidate としてどこまで practical に読めるかを比較し、

- authority placement
- consistency mode
- witness core
- provider receipt

の cut を崩さない current first choice を整理する。

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `tasks.md`
- `specs/examples/121-shared-space-authoritative-room-baseline.md`
- `specs/examples/122-shared-space-catalog-working-subset-comparison.md`
- `specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md`
- `docs/reports/0369-authoritative-room-baseline-docs-first-refinement.md`
- `docs/reports/0371-shared-space-catalog-working-subset-comparison.md`
- `docs/reports/0373-shared-space-auditable-authority-witness-minimal-shape.md`

## 3. Actions taken

1. `authority_rng` facade 内に delegation を隠す案、provider placement だけを room profile に上げる案、provider placement と delegated attestation を同時に上げる案を比較した。
2. `specs/examples/124-shared-space-authoritative-room-delegated-rng-provider-placement.md` を追加し、authoritative room 側の current first choice を整理した。
3. `Documentation.md` と `specs/00-document-map.md` に導線を追加した。
4. `plan/11`、`plan/12`、`plan/16`、`plan/17`、`progress.md`、`tasks.md`、`plan/90` を更新し、current next narrow step を control-plane separated causal carrier threshold comparison に移した。

## 4. Files changed

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md`
- `specs/examples/124-shared-space-authoritative-room-delegated-rng-provider-placement.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/reports/0375-shared-space-authoritative-room-delegated-rng-provider-placement.md`

## 5. Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M %Z'; df -h .; free -h
2026-04-09 16:46 JST
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        99G   90G  4.5G  96% /
               total        used        free      shared  buff/cache   available
Mem:           960Mi       818Mi        70Mi       1.1Mi       225Mi       141Mi
Swap:           19Gi       1.3Gi        18Gi

$ python3 scripts/new_report.py --slug shared-space-authoritative-room-delegated-rng-provider-placement
/home/yukatayu/dev/mir_poc_01/docs/reports/0375-shared-space-authoritative-room-delegated-rng-provider-placement.md

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 376 numbered report(s).

$ git diff --check
[no output]
```

## 6. Evidence / findings

- authoritative room 側で `delegated_rng_service` を practical candidate にしたい理由は、authority placement を変えることではなく RNG source を replaceable layer に出すことにある。
- current first choice では、authority は request / lock / commit / publish の owner のままであり、provider は draw result を返すが room state を commit しない。
- `auditable_authority_witness` の minimal witness core
  - `witness_kind`
  - `action_ref`
  - `draw_slot`
  - `draw_result`
  はそのまま維持できる。
- `provider_draw_ref` / `provider_receipt_ref` を current witness core に入れると、provider placement と fairness claim strengthening が同じ軸に潰れやすい。
- したがって current first choice は、room profile では `fairness_source = delegated_rng_service` だけを上げ、provider receipt は audit / receipt side optional attachment に留める cut である。
- reviewer finding は mirror / provenance drift に限られ、`progress.md`、`plan/17`、`plan/90` の補正で解消した。

## 7. Changes in understanding

- authoritative room baseline の次の practical widening は、authority model や consistency mode を変えることではなく、RNG provider placement を明示的に差し替えることとして切れる。
- provider placement と witness requirement を別軸に保つことで、`delegated_provider_attestation` を still later candidate に残したまま、debug provider / HW entropy / external RNG service の差し替え余地を docs-first に扱える。
- Phase 4 current next narrow step は、provider placement の comparison 自体ではなく、control-plane separated causal carrier を reopen する threshold へ移してよい。

## 8. Open questions

- provider receipt / draw ref をいつ typed required attachment に上げるか。
- `delegated_provider_attestation` を room profile row に昇格させるか、それとも optional audit capability に留めるか。
- provider failure policy を room profile / external policy / audit attachment のどこで固定するか。
- control-plane separated causal carrier をどの stop line から reopen するか。

## 9. Suggested next prompt

`shared-space / membership line について、control-plane separated causal carrier を authoritative room side line に reopen する threshold を docs-first に比較し、membership_epoch / member_incarnation / activation frontier を data-plane witness / provider attachment とどこまで分けるかを source-backed に整理してください。`
