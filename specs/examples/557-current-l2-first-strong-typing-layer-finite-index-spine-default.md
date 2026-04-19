# 557 — current L2 first strong typing layer finite-index spine default

## 目的

current L2 の stronger typed surface を final public core へ premature に上げずに、
first strong typing layer の principal target を
**有限で決定可能な index / mode / order fragment**
へ narrow に固定する。

ここで固定するのは
**current L2 first strong typing layer finite-index spine default**
であり、

- full dependent type
- arbitrary type-level computation
- full dependent pattern matching
- compiler 内 general theorem proving
- final public type syntax
- final public verifier contract

はまだ固定しない。

## current default

1. full dependent type は first public core に要求しない。
2. first strong typing layer の principal target は、
   type-level partial order / finite lattice / capture set / lifetime preorder / simple cost bound
   を含む finite decidable fragment に置く。
3. principal typing spine は
   `Ψ ; Γ ; Δ ⊢ e : A @ m ! ε ▷ C`
   を current conceptual floor に置く。
4. ここで
   - `Ψ` は index / mode / order theory
   - `Γ` は ordinary context
   - `Δ` は linear / capability context
   - `ε` は effect row
   - `C` は constraints / residual obligations
   を表す。
5. user-defined index theory は allowed だが、first cut では次の decidable family に制限する。
   - finite poset
   - finite lattice
   - powerset lattice
   - region / lifetime preorder
   - finite capture set inclusion
   - simple numeric resource bound / ordered monoid
6. security label system は first-class target に置く。
   - `Labeled ℓ A`
   - `flows_to`
   - pc label
   - explicit declassification with authority
7. lifetime / capture checking も first-class target に置く。
   - outlives constraints
   - capture set constraints
   - capability escape check
8. taint は IFC label または powerset lattice specialization として扱う。
9. cost bounds は simple resource index に留め、complex recurrence / probabilistic / amortized bound は theorem side に送る。
10. inference policy は
    - local inference aggressive
    - public/module boundary annotation required
    - declassification / handoff / external effect annotation required
    を current default にする。
11. prove/check 側は
    - finite index constraints に対する checker soundness
    - decidable fragment に対する limited completeness
    - first explicit-flow fragment に対する noninterference
    - selected resource model に対する cost soundness
    までを current first line に置く。

## practical reading

- stronger typed surface promotion は still mixed gate に残す。
- ただし checker-adjacent principal を vague なままにせず、
  finite decidable fragment を first strong typing layer の current target として扱う。
- IFC / taint / lifetime / capture / simple cost は
  separate future wish-list ではなく、同じ principal typing spine 上の first-class package として扱う。
- theorem side は、これを越える arbitrary index reasoning / richer cost proof / stronger noninterference を引き取る reserve line に置く。

## representative examples to carry

- secret key cannot flow to `Public`
- sanitize removes taint only with proof/capability
- remote call count `<= 0` check
- lifetime / capture escape rejected

## stop line

- full dependent type in first public core
- arbitrary type-level computation
- general theorem proving in compiler
- final public type syntax
- stronger typed surface principal promotion
- final public verifier contract

## next package effect

- `plan/18` と `progress.md` / `tasks.md` の Problem 1 current first line は、
  finite index / IFC / capture / lifetime / simple cost principal target を明示して読む。
- later mixed gate は、
  stronger typed surface promotion と final public theorem/model-check/verifier seams に narrowed してよい。
