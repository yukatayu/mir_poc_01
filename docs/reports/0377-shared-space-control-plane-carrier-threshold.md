# Report 0377 — shared space control plane carrier threshold

- Date: 2026-04-09T08:40:53.499518Z
- Author / agent: Codex
- Scope: shared-space / membership line における control-plane separated causal carrier の reopen threshold comparison
- Decision levels touched: L2 / L3

## 1. Objective

authoritative room baseline、working subset、minimal witness core、delegated-provider practical cut を前提に、

- `membership_epoch + member_incarnation`

を current default として維持できる条件と、

- control-plane separated causal carrier

を reopen すべき条件を docs-first に整理する。

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
- `specs/examples/121-shared-space-authoritative-room-baseline.md`
- `specs/examples/122-shared-space-catalog-working-subset-comparison.md`
- `specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md`
- `specs/examples/124-shared-space-authoritative-room-delegated-rng-provider-placement.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `tasks.md`
- `docs/reports/0280-shared-space-reconnect-and-inflight-policy-cut.md`
- `docs/reports/0282-shared-space-causal-metadata-and-membership-epoch-comparison.md`
- `docs/reports/0371-shared-space-catalog-working-subset-comparison.md`
- `docs/reports/0373-shared-space-auditable-authority-witness-minimal-shape.md`
- `docs/reports/0375-shared-space-authoritative-room-delegated-rng-provider-placement.md`

## 3. Actions taken

1. `specs/examples/125-shared-space-control-plane-carrier-threshold.md` を追加し、control-plane separated causal carrier の threshold comparison を 3 案で整理した。
2. authoritative room の stale mismatch 例を追加し、membership-only carrier では区別しにくい control-plane change を具体化した。
3. current first choice を「`membership_epoch + member_incarnation` を維持し、reopen するなら `control_epoch` 相当の lightweight split を first cut にする」として明文化した。
4. `Documentation.md`、`specs/00-document-map.md`、`specs/examples/122...124...`、`plan/11`、`plan/12`、`plan/16`、`plan/17`、`plan/90`、`progress.md`、`tasks.md` を更新し、Phase 4 current package を checkpoint close に寄せた。

## 4. Files changed

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/122-shared-space-catalog-working-subset-comparison.md`
- `specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md`
- `specs/examples/124-shared-space-authoritative-room-delegated-rng-provider-placement.md`
- `specs/examples/125-shared-space-control-plane-carrier-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/reports/0377-shared-space-control-plane-carrier-threshold.md`

## 5. Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-09 17:50 JST

$ python3 scripts/new_report.py --slug shared-space-control-plane-carrier-threshold
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0377-shared-space-control-plane-carrier-threshold.md

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 377 numbered report(s).

$ git diff --check
[no output]

$ git status --short --branch
## main...origin/main
 M Documentation.md
 M plan/11-roadmap-near-term.md
 M plan/12-open-problems-and-risks.md
 M plan/16-shared-space-membership-and-example-boundary.md
 M plan/17-research-phases-and-autonomy-gates.md
 M plan/90-source-traceability.md
 M progress.md
 M specs/00-document-map.md
 M specs/examples/122-shared-space-catalog-working-subset-comparison.md
 M specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md
 M specs/examples/124-shared-space-authoritative-room-delegated-rng-provider-placement.md
 M tasks.md
?? docs/reports/0377-shared-space-control-plane-carrier-threshold.md
?? specs/examples/125-shared-space-control-plane-carrier-threshold.md
```

## 6. Evidence / findings

- current package の強い基準は `specs/examples/121...` から `124...` に残り、`125...` はそれを壊さずに stop line を 1 段明確化する comparison になった。
- stale mismatch の核心は「membership は変わっていないが control-plane だけが変わる」ケースであり、authoritative room の authority handoff / provider rotation / activation frontier compare を current line に入れない限り、`membership_epoch + member_incarnation` は current default として維持できる。
- reopen する場合の first cut を `control_epoch` 相当に留めることで、full control-plane log、provider attestation、auth layering、export schema finalization を同時に押し広げずに済む。
- reviewer completion は wait window 内で取得できなかったため、review record は `docs/reports/0378-review-shared-space-control-plane-carrier-threshold.md` に local evidence fallback として残した。

## 7. Changes in understanding

- shared-space / membership line の current package は、control-plane threshold comparison までで checkpoint close とみなしてよい。
- Phase 4 current package をさらに押し広げるより、次の promoted line は Phase 5 inventory に移す方が自然である。
- control-plane separated carrier は current default ではなく、authority handoff / provider binding / activation frontier が room rule 側へ上がった時だけ reopen すべき stronger candidate だと明文化できた。

## 8. Open questions

- `control_epoch` 相当の first reopen cut を actual artifact / compare surface にどこまで mirror するか。
- authority handoff と provider binding rotation のどちらを threshold trigger の first practical anchor にするか。
- append-friendly room 側でも control-plane split を同じ threshold で読むか、それとも別 threshold を持つか。
- reviewer completion が返った場合の指摘反映。

## 9. Suggested next prompt

`small decidable core と external verifier 境界を揃えるため、static analysis / type / theorem prover / async-control boundary inventory を docs-first に進め、authoritative room baseline / control-plane threshold と衝突しない first cluster を source-backed に整理してください。`
