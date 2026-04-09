# Report 0379 — phase5 small decidable core and proof boundary inventory

- Date: 2026-04-09 18:33 JST
- Author / agent: Codex
- Scope: Phase 5 task package 1 として、small decidable core / theorem prover / protocol verifier / runtime policy の boundary inventory を docs-first に集約し、current package を checkpoint close できるか確認する
- Decision levels touched: L2 examples / non-normative roadmap and repository memory wording

## 1. Objective

`tasks.md` で最優先 active package だった

- `static analysis / type / theorem prover / async-control boundary inventory`

を current repo で自走可能な範囲まで進め、次を 1 本の正本に畳む。

- core checker に残す local / structural / decidable floor
- theorem prover に送る global invariant
- model checker / protocol verifier に送る protocol property
- runtime policy / operational layer に残すもの

そのうえで、Phase 5 current package を first inventory package close と読んでよいかを mirror に反映する。

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/reports/0358-async-control-memory-boundary-inventory.md`
- `specs/examples/121-shared-space-authoritative-room-baseline.md`
- `specs/examples/122-shared-space-catalog-working-subset-comparison.md`
- `specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md`
- `specs/examples/124-shared-space-authoritative-room-delegated-rng-provider-placement.md`
- `specs/examples/125-shared-space-control-plane-carrier-threshold.md`

## 3. Actions taken

1. Phase 5 inventory を `specs/examples/30...` と `docs/reports/0358...` の 2 本に分かれた状態から見直し、current repo で必要な split を
   - `core_static_checker`
   - `theorem_prover_boundary`
   - `protocol_verifier_boundary`
   - `runtime_policy_boundary`
   の 4 つに整理した。
2. 新しい正本として `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md` を追加し、current L2 fallback chain、`try` / `atomic_cut`、authoritative room witnessed draw、activation visibility の 4 例で placement を説明した。
3. `Documentation.md` と `specs/00-document-map.md` に 126 の導線を追加した。
4. `plan/11`、`plan/12`、`plan/13`、`plan/17` を更新し、Phase 5 の current package を checkpoint close と読み直した。
5. `progress.md` と `tasks.md` を更新し、mainline を active inventory line から checkpoint maintenance / later reopen candidate へ切り替えた。
6. `plan/90-source-traceability.md` に addendum を追加した。
7. local validation を通した後、reviewer 起動を 1 回試みたが、この session では completion を待機できる handle を取得できなかったため、`AGENTS.md` の fallback rule に従って local diff inspection と source cross-check へ切り替えた。

## 4. Files changed

- `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/reports/0379-phase5-small-decidable-core-and-proof-boundary-inventory.md`
- `docs/reports/0380-review-phase5-small-decidable-core-and-proof-boundary-inventory.md`

## 5. Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M %Z' && df -h . && free -h
2026-04-09 18:22 JST
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        99G   90G  4.5G  96% /
               total        used        free      shared  buff/cache   available
Mem:           960Mi       721Mi        69Mi       6.0Mi       169Mi       238Mi
Swap:             0B          0B          0B

$ python3 scripts/new_report.py --slug phase5-small-decidable-core-and-proof-boundary-inventory
/home/yukatayu/dev/mir_poc_01/docs/reports/0379-phase5-small-decidable-core-and-proof-boundary-inventory.md

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 379 numbered report(s).

$ git diff --check
[no output]

$ date '+%Y-%m-%d %H:%M JST'
2026-04-09 18:33 JST
```

reviewer completion は取得できなかったため、closeout は `docs/reports/0380-review-phase5-small-decidable-core-and-proof-boundary-inventory.md` の local evidence fallback に従う。

## 6. Evidence / findings

- current repo の source-backed line では、core checker に残してよいのは local / structural / decidable floor に限られる。
- current repo の source-backed line では、`atomic_cut` は place-local finalizing cut の最小核であり、higher-level ordering / fairness / witnessed draw protocol を同じ layer に押し込まない方が自然である。
- `specs/examples/126...` の 4-way split により、
  - same-lineage floor
  - malformed / underdeclared rejection
  - minimal capability strengthening prohibition
  - request-local / option-local clause attachment
  - minimal predicate fragment well-formedness
  - `try` / rollback locality structural floor
  を core checker 側に残しつつ、
  - canonical normalization
  - no re-promotion
  - rollback / cut non-interference
  - membership / authority / fairness protocol
  - activation / auth / retention / retry
  を外に残す line が 1 本で読めるようになった。
- これにより、Phase 5 は「強い型システムの即時 actualization」ではなく「small decidable core と proof / protocol / runtime obligation の inventory を切る段階」として checkpoint close に入れてよい。
- `plan/ 更新済み`
- `progress.md 更新済み`
- `tasks.md 更新済み`

## 7. Changes in understanding

- Phase 5 current package は、Phase 4 package と同様に docs-first の **first package close** まで到達したと読んでよい。
- Phase 5 の next pressure は low-level memory-order immediate introduction ではなく、proof-obligation matrix と external handoff artifact をどこまで narrow に切るかである。
- したがって current mainline は、deep inventory line そのものより checkpoint maintenance を先に置く方が整合する。

## 8. Open questions

- proof-obligation matrix を artifact / relation / report のどこに置くのが最小か。
- external handoff artifact を theorem prover / protocol verifier / runtime policy のどの組み合わせで分けるのが自然か。
- low-level memory-order family を将来 external verifier vocabulary としてだけ残すか、それとも current repo では完全に外に置き続けるか。
- public checker API を narrow に actualize する pressure が出たとき、Phase 3 reserve path と Phase 5 later package をどう接続するか。

## 9. Suggested next prompt

Phase 5 current package close の次段として、`core_static_checker` / `theorem_prover_boundary` / `protocol_verifier_boundary` / `runtime_policy_boundary` の 4-way split を保ったまま、proof-obligation matrix と external handoff artifact の最小 shape を docs-first に比較してください。
