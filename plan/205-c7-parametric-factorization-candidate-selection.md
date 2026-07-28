# Plan 205 - C7 parametric factorization candidate selection

## 役割と権限

これは C7 source-ergonomics に関する **LAB candidate-selection record** である。
Canon は唯一の規範正本であり、本書は source grammar、elaboration rule、Core、observation、
authority、failure、identity、history、contract、SCN、OBL、Gate/Phase、runtime、API を選択又は変更しない。

Plan 199 の C7 規律は「省略する fact が一意に決まり、elaborated artifact から fact と根拠を
検査・復元できること」を設計制約として記録する。本選別は、その具体的な source rule を
決めず、任意の数学的 function に限った構成的 conditional lemma が ADR-0014 の existing-lane
L3 route に載るかを調べる。

## 入力と重複検査

| 入力 | 読み方 | 選別への影響 |
| --- | --- | --- |
| ADR-0014 と `working/README.md` | existing LAB lane 内の conditional lemma は、reserved boundary と新 helper/schema/CI/API を避ける限り L3 pre-registration できる | result は parametric local theorem に限る |
| theory/03 と P012 | actual source/elaboration/receipt/identity semantics は未選択又は reserved | local parameter を Mir source 又は observation primitive に同一視しない |
| Plan 199 C7 | unique determination と reconstructible elaborated basis は non-normative design constraint | theorem は extensional unique observation だけを扱い、grounds/provenance を完了扱いしない |
| Plan 204 | WRK-0034 fixed-presentation line は no-candidate、C7 factorization は fresh preflight | fixed finite reply model を変更又は再実行しない |
| WRK-0005 | fixed actual-outcome fiber の all-pairs `SameOutcome` relation。`ExistsUnique` を証明しないと明記する | arbitrary `E`/`S`/`O`、image、fiber constancy の theorem とは statement と consumer が異なり、duplicate ではない |
| WRK-0017 | generic `by_cases` が hidden classical dependency を持った frozen falsifier | primary theorem は `Classical.choice`、quotient、decidable equality を使わないことを outcome condition にする |

At selection cut `954f9e73498a2a0043cba45398815b61d0ee22bf`, exact-symbol and phrase
searches over `plan/`, `docs/reports/`, `mirrorea_canon/working/`, `theory/`,
`spec/`, and `meta/` found no retained `FiberConstant`, `UniqueObsOnRange`,
or statement-equivalent range-observation theorem. The only relevant older hits
were WRK-0005's explicitly non-`ExistsUnique` outcome relation and WRK-0017's
classical-dependency falsifier. The documented fenced Lean LAB routes are
`plan/174-local-predicate-proposition-decidability-selection.md`,
`plan/wrk-0033-v1r1-presentation-refinement.md`, and
`plan/wrk-0034-v1-r1-finite-sequence-refinement.md`; retaining one fenced
block in a new ordinary `plan/` evidence artifact therefore stays in an
existing documented LAB lane.

## Candidate comparison

| Candidate | Disposition | Reason |
| --- | --- | --- |
| Concrete Mir source omission or desugaring rule | not selected | identifies source/elaboration artifacts and authorizes an external contract; reserved |
| C7 parametric pointwise factorization | selected for pre-registration only | arbitrary types and uninterpreted functions can state a constructive condition without selecting a Mir object or authorizing omission |
| Global reconstruction function on all `S` | not selected | false outside `range erase`; existence/uniqueness needs additional assumptions and may need choice |
| Function package over `Set.range erase` | not selected | constructing a representative-based function needs a choice boundary; not needed for the pointwise proposition |
| Quotient factorization | not selected | changes the local mathematical presentation and invokes quotient machinery; no need to retain it in the first candidate |
| Finite counterexample search | not selected | would add decidability/finite interface assumptions that the pointwise theorem does not need |

## Proposed `C7-FAC-PRE` pre-registration

### Narrow question

For arbitrary universe-polymorphic types `E`, `S`, and `O`, and local uninterpreted
functions `erase : E -> S` and `observe : E -> O`, is the following fiber-constancy
predicate constructively equivalent to pointwise unique realized observation at every
member of `range erase`?

```text
FiberConstant := forall x y, erase x = erase y -> observe x = observe y
UniqueObservedOnImage := forall s, (exists e, erase e = s) ->
  exists! o, exists e, erase e = s and observe e = o
```

The candidate may additionally prove only that an explicit collision
`exists x y, erase x = erase y and observe x != observe y` refutes both predicates,
and that the full-codomain uniqueness claim is false in a fixed `Unit`/`Bool`/`Bool`
countermodel. It must not prove an executable reconstructor, a global function on all
`S`, the converse extraction of a collision from negated fiber constancy, or a quotient
presentation.

### Standing scope and non-effects

The candidate uses `plan/` and `docs/reports/` only. It may add one fenced Lean block
to `plan/wrk-0035-c7-parametric-factorization.md`, materialized solely into a disposable
temporary file for `lean --trust=0`. Names inside that block are artifact-local and not a
module, helper family, schema, API, or public representation.

It does not identify `E`, `S`, `O`, `erase`, or `observe` with Mir source, Surface,
Core, elaborated artifact, request, result, receipt, occurrence, Diagnostic, authority,
failure, history, persistence, transport, or an observation primitive. It does not establish
fiber constancy for a concrete artifact; authorize source omission, desugaring, normalization,
or reconstruction; select equality or a carrier; establish computability, decidability,
complexity, provenance, inspectable grounds, or an extraction algorithm; change a Canon
file outside a new L3 working record and exact metadata; or change OBL, SCN, Gate/Phase,
implementation, conformance, or public status.

### Falsifier and stop line

Use `no-candidate` instead of pre-registration if source search finds an alpha- or
statement-equivalent theorem at the pinned cut. Freeze the later working record if the
primary pointwise theorem requires `Classical.choice`, `Classical`, `Quotient`,
`Quot.sound`, decidable equality, a finite interface, a Mir-specific assumption, or an
interpretation of a local parameter as a source/semantic contract. Escalate rather than
repair if the work needs a concrete source rule, an observational equivalence/setoid,
an identity/authority/failure/history relation, a global reconstruction function, a quotient
carrier, or a new helper/schema/CI/Make/API/evidence lane.

## Evidence route and execution order

1. Commit this selection and synchronize LAB snapshots. Do not write or run the Lean source.
2. Create and push `working/WRK-0035-c7-parametric-factorization.md` with exact authority
   digests, alternative, expected falsifier, rollback, non-effects, and registration commands.
3. Only after that registration is committed and pushed, add the one fenced source block to
   `plan/wrk-0035-c7-parametric-factorization.md` and run its registered commands.
4. Retain only the stated constructive results or the first registered falsifier. A function
   package, quotient formulation, concrete instantiation, or source authorization needs a
   forward successor or ordinary Canon design.

## Execution outcome

Selection only. No `WRK-0035` record, Lean source, generated artifact, or outcome command
exists at this selection cut.
