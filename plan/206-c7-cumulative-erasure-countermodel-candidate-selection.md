# Plan 206 - C7 cumulative-erasure countermodel candidate selection

## 役割と権限

これは C7 source-ergonomics に関する **LAB candidate-selection record** である。
Canon は唯一の規範正本であり、本書は Surface grammar、source omission、elaboration rule、Core、
observation、grounds/provenance、authority、failure、identity、history、contract、SCN、OBL、
Gate/Phase、runtime、API を選択又は変更しない。

WRK-0035 は一つの `erase` と一つの `observe` の間の range-only factorization boundary を
L3 evidence として retained した。しかし、その一つの検査を複数の omission candidate に
個別適用してよいこと、又は個別に安全な representation を共通の coarse representation へ
同時に落としてよいことは示していない。Plan 199 が将来の C7 inference/desugaring
equivalence matrix を明示しているため、この gap は concrete source rule を選ばずに検査できる
最小の negative boundary になり得る。

## Authority cut、入力、重複検査

Selection authority cut は `91912c18a8065310e427c1bcf3200fafbc0b7b75` である。

| Input | SHA-256 | Selection への影響 |
| --- | --- | --- |
| ADR-0014 | `b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323` | L3 countermodel は existing LAB lane、pre-registration、negative-list boundary を満たす必要がある |
| theory/03 | `2d703895da4f75bf57848275db6ae03e0abe7d56f62a11ef364af8fe22677641` | BND-001 の actual elaboration contract を local functions に同一視しない |
| P012 | `09ea4d6957c320b4d0647806714a1643101c2022b2893ac76ec7de3bf1db73d5` | V1/R1/SW1/A2 の carrier/occurrence 実現は ordinary design boundary のまま |
| Plan 199 | `aee29c1629757cda4067add79e23b4c947b35aaeb599dc18c3b07de9f478cc0a` | C7 matrix と source-to-elaborated evidence equivalence を future acceptance condition とし、個別 omission の安易な合成を許可しない |
| Plan 200 | `b75d86e6849ca8b12606117d106dc7be46505edd927c7e124836acb9ddbf5dbd` | D12 と C7 stop line は full observation の差を reject するが、multi-erasure はまだ固定していない |
| Plan 204 | `6943f6b1607297f5b5ef176bfe897ac7b031238a9825fc5dd202d2dfa27c11c7` | C3/C4/C5 proper は owner/Canon boundary、別 frontier は fresh preflight が必要 |
| Plan 205 / WRK-0035 | `c85cb43c162d1509ee9de183b4b27a0b2ee83d7188a3acdb0f84861269a52bdf` / `8e27a94f876b9db33d6d30cc56b4569f83094b0cc4d17261bd680497327309a3` | single-erasure factorization、collision、full-codomain non-uniquenessは既存 result。候補はそれらの再証明ではない |
| Report 2493 | `36a2c6d49ba9745b5acedef4a376a986f4821a5e94c0d1149fb0b824539fce51` | current snapshots は WRK-0035 を L3 evidence only と分類する |

Current-cut exact-term search for `eraseA`, `eraseB`, `eraseAB`, two/multiple/cumulative
erasure, common coarsening, mutual/joint/combined/simultaneous omission, coarsening, and
factorization composition found no retained multi-erasure statement outside the already-known
single-erasure C7 records. A Japanese broad wording search found only an unrelated historical
punctuation/open-notation discussion. The single-erasure search hits are expected evidence
references, not a common-coarsening countermodel.

An advisory temporary Oracle review, SHA-256
`d48a8c48c0ee1bdbdd1722be2e21d04041b374dc6b56eaa896e7815590ffa14e`, independently
identified this candidate and its strongest objection. It is not an authority source: the
selection below is grounded in the pinned Canon/LAB inputs and the local duplicate search.

## Candidate comparison

| Candidate | Disposition | Reason |
| --- | --- | --- |
| C7 cumulative-erasure countermodel | selected for pre-registration only | a fixed finite countermodel can show that two individually fiber-constant erasures do not justify their common coarsening; it has Plan 199's future C7 matrix as an explicit consumer |
| no-candidate | mandatory fallback | use if the multi-erasure statement is duplicate, has no actual matrix consumer, or reduces to the existing single collision without an independently checkable coarsening relation |
| full-codomain reconstructor nonexistence | not selected | an empty-range model is mathematically possible but adds only a vacuity variation to WRK-0035's already retained global-reconstructor warning |
| grounds/artifact uniqueness countermodel | not selected | replacing `observe` by `grounds`, or restating non-injectivity, is a direct instantiation of WRK-0035 and has no new validation consequence |
| C0-D, C1, C2-B, C3/C4/C5 proper, C6 | not selected | their first substantive checks require exact domain, snapshot, identity, pending/result, occurrence/facet, or scalar/terminal semantics reserved to ordinary Canon design |

## Proposed `C7-CUM-PRE` pre-registration

### Narrow question

Can one fixed, artifact-local finite model exhibit two local erasures and a common coarsening
such that each local erasure preserves its own observation but the common coarsening fails to
preserve the paired observation?

The planned model has no Mir interpretation:

```text
E    = {left, right}
S_A  = {keepA_left, keepA_right}
S_B  = {keepB_left, keepB_right}
S_AB = Unit

erase_A(left/right)  = keepA_left/keepA_right
erase_B(left/right)  = keepB_left/keepB_right
erase_AB(left/right) = ()
coarsen_A(_) = ()
coarsen_B(_) = ()

observe_A(left/right) = false/true
observe_B(left/right) = false/true
joint(e) = (observe_A(e), observe_B(e))
```

The sole retained outcomes may be: the two common-coarsening equations;
`FiberConstant erase_A observe_A`; `FiberConstant erase_B observe_B`; and an explicit
`Collision erase_AB joint`, hence failure of the cumulative fiber-constancy predicate. The
conclusion is only that individually checked erasures do not in general justify simultaneous
omission; the final cumulative representation must be checked directly.

The result must not assert that `erase_A`, `erase_B`, or `erase_AB` is a source transformation,
that `observe_A`/`observe_B` is a source fact or a ground, or that any actual pair of Mir facts is
omittable. It must not state a general composition theorem, a lattice/order law, a quotient,
a reconstruction function, or an acceptance algorithm.

### Falsifier and stop line

Use `no-candidate` rather than pre-registering if a current-cut search finds a
statement-equivalent multi-erasure/common-coarsening countermodel; if Plan 199's matrix cannot
serve as a distinct consumer; or if the intended conclusion is merely WRK-0035's single
collision with names changed. Freeze a later working record if the finite model needs a concrete
source/elaboration artifact, a grounds/provenance relation, a semantic dependency graph,
choice/quotient/decidable/finite interface beyond its fixed local constructors, an actual
omission rule, or any new repository surface.

### Scope and non-effects

The candidate may use only the existing `plan/` and `docs/reports/` LAB lanes. Its evidence may
be one fenced, artifact-local Lean block in
`plan/wrk-0036-c7-cumulative-erasure-countermodel.md`, extracted to a disposable temporary file
and run with `lean --trust=0`. It may define no reusable module, helper family, schema, CI/Make
surface, parser/checker/runtime behavior, sample workflow, or public interface.

It changes no Canon source, BND-001 clause, THM-001, P012 disposition, source grammar,
elaboration contract, Core/judgment, equality/identity, authority/failure/history relation,
SCN, OBL/theory status, Gate/Phase, implementation, conformance, or public behavior.

## Execution order

1. Commit this selection and its LAB snapshots. Do not create or run Lean source in this package.
2. Create and push `working/WRK-0036-c7-cumulative-erasure-countermodel.md` with exact digests,
   consumer, alternative, falsifier, non-effects, rollback, and registration commands.
3. Only after the registration push, add the one fenced evidence block, run the registered
   fixed-model commands, and retain only the stated countermodel or the first falsifier.
4. Link the evidence in result-only working-record metadata, then synchronize LAB snapshots.

## Selection outcome

`C7-CUM-PRE` is selected for L3 pre-registration only. The strongest objection is that a future
C7 process might already require direct checking of every cumulative representation; if so this
countermodel changes no validation decision and is theorem churn. The explicit matrix consumer,
common-coarsening equations, and fixed two-local-erasure shape are therefore mandatory at
registration and evidence time. No `WRK-0036`, Lean source, or outcome command exists at this
selection cut.
