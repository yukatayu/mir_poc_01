import OwnerStatementAcknowledgment
namespace MirroreaProofFirst.OwnerStatementOriginalEntry
universe u v
variable {α : Type u} {κ : Type v}

-- The payload is the WHOLE checked owner plan (signature/Core/site/owner),
-- not a mutable ordinal or an operation-name label. Original is independently
-- retained admitted data. Its constructor/patch/restore frame is a separate
-- obligation; this checker neither establishes that provenance nor authority.
def Admissible (ordinal : α → Option Nat) (original : List α)
 (bound : Bool) (candidate : α) : Prop :=
 bound = false ∧ candidate ∈ original ∧ ordinal candidate = none

def check [DecidableEq α] (ordinal : α → Option Nat) (original : List α)
 (bound : Bool) (candidate : α) : Bool :=
 !bound && original.any (fun prior => decide (prior = candidate ∧ ordinal prior = none))

variable {ordinal : α → Option Nat} {original next : List α} {bound : Bool}
variable {candidate prior : α} {operation : α → κ}

theorem check_exact [DecidableEq α] :
 check ordinal original bound candidate = true ↔ Admissible ordinal original bound candidate := by
 simp [check,Admissible,List.any_eq_true]
 intro _
 constructor
 · rintro ⟨prior,present,rfl,plain⟩
   exact ⟨present,plain⟩
 · rintro ⟨present,plain⟩
   exact ⟨candidate,present,rfl,plain⟩

def Unambiguous (operation : α → κ) (original : List α) : Prop :=
 ∀ first ∈ original, ∀ second ∈ original, operation first = operation second → first = second

-- This applies to every current candidate with the protected operation's
-- identity, even if its mutable ordinal has been erased or its Core replaced.
theorem protected_rejected [DecidableEq α]
 (unique : Unambiguous operation original) (stored : prior ∈ original)
 (guarded : ordinal prior ≠ none) (identity : operation candidate = operation prior) :
 check ordinal original bound candidate = false := by
 apply Bool.eq_false_iff.mpr
 intro accepted
 obtain ⟨_,present,plain⟩ := check_exact.mp accepted
 have same := unique candidate present prior stored identity
 subst candidate
 exact guarded plain

theorem bound_rejected [DecidableEq α] :
 check ordinal original true candidate = false := by simp [check]

theorem unknown_rejected [DecidableEq α] (absent : candidate ∉ original) :
 check ordinal original bound candidate = false := by
 apply Bool.eq_false_iff.mpr
 intro accepted
 exact absent (check_exact.mp accepted).2.1

-- Positive relative completeness is retained beside protected siblings.
-- This is not an all-refusing source-entry policy.
theorem ordinary_accepted [DecidableEq α]
 (stored : candidate ∈ original) (plain : ordinal candidate = none) :
 check ordinal original false candidate = true :=
 check_exact.mpr ⟨rfl,stored,plain⟩

theorem preserved_original_addition [DecidableEq α]
 (accepted : check ordinal original bound candidate = true)
 (retained : ∀ plan ∈ original, plan ∈ next) :
 check ordinal next bound candidate = true := by
 obtain ⟨free,stored,plain⟩ := check_exact.mp accepted
 exact check_exact.mpr ⟨free,retained candidate stored,plain⟩

-- Admission monotonicity above grants no permission to add an arbitrary plan:
-- authenticated checked addition and no-shadow uniqueness remain independent.
#print axioms check_exact
#print axioms protected_rejected
#print axioms bound_rejected
#print axioms unknown_rejected
#print axioms ordinary_accepted
#print axioms preserved_original_addition
end MirroreaProofFirst.OwnerStatementOriginalEntry

namespace MirroreaProofFirst.OwnerStatementOriginalEntry
variable {α : Type u} {κ : Type v}

def selectSource [DecidableEq κ] (handler : α → κ) (original : List α) (name : κ) : List α :=
 original.filter (fun plan => decide (handler plan = name))

-- Declarative positional agreement, including the absence of every extra
-- position. The independently admitted original inventory supplies order.
-- Neither an authentic subset nor a prefix establishes source completeness.
def ManifestAdmissible [DecidableEq κ] (handler : α → κ)
 (original manifest : List α) : Prop :=
 manifest ≠ [] ∧ ∀ first, manifest.head? = some first →
   ∀ i : Nat, manifest[i]? = (selectSource handler original (handler first))[i]?

def manifestCheck [DecidableEq α] [DecidableEq κ] (handler : α → κ)
 (original manifest : List α) : Bool :=
 match manifest with
 | [] => false
 | first :: rest => decide (first :: rest = selectSource handler original (handler first))

theorem manifestCheck_exact [DecidableEq α] [DecidableEq κ]
 (handler : α → κ) (original manifest : List α) :
 manifestCheck handler original manifest = true ↔ ManifestAdmissible handler original manifest := by
 cases manifest with
 | nil => simp [manifestCheck,ManifestAdmissible]
 | cons first rest =>
   simp only [manifestCheck,decide_eq_true_eq]
   constructor
   · intro same
     refine ⟨by simp, ?_⟩
     intro head firstEq i
     have headEq : first = head := by simpa using firstEq
     subst head
     rw [same]
   · intro ready
     apply List.ext_getElem?
     intro i
     exact ready.2 first rfl i

theorem manifest_ready_index [DecidableEq α] [DecidableEq κ]
 (handler : α → κ) (original manifest : List α) (first : α)
 (accepted : manifestCheck handler original manifest = true)
 (head : manifest.head? = some first) (i : Nat) :
 manifest[i]? = (selectSource handler original (handler first))[i]? :=
 (manifestCheck_exact handler original manifest).mp accepted |>.2 first head i

theorem manifest_rejects_different_list [DecidableEq α] [DecidableEq κ]
 (handler : α → κ) (original : List α) (first : α) (rest : List α)
 (different : first :: rest ≠ selectSource handler original (handler first)) :
 manifestCheck handler original (first :: rest) = false := by
 simp [manifestCheck,different]

theorem manifest_rejects_length_loss [DecidableEq α] [DecidableEq κ]
 (handler : α → κ) (original : List α) (first : α) (rest : List α)
 (different : (first :: rest).length ≠ (selectSource handler original (handler first)).length) :
 manifestCheck handler original (first :: rest) = false := by
 apply manifest_rejects_different_list
 intro same
 exact different (congrArg List.length same)

theorem manifest_original_accepted [DecidableEq α] [DecidableEq κ]
 (handler : α → κ) (original : List α) (name : κ)
 (nonempty : selectSource handler original name ≠ []) :
 manifestCheck handler original (selectSource handler original name) = true := by
 cases selected : selectSource handler original name with
 | nil => exact False.elim (nonempty selected)
 | cons first rest =>
   have member : first ∈ selectSource handler original name := by rw [selected]; simp
   have same : handler first = name := by
     simpa [selectSource] using (List.mem_filter.mp member).2
   simp [manifestCheck,same,selected]

-- Positional exactness is not origin authentication: decoding and trusted
-- compiler/admission promotion, no-shadow, code/currentness and patch frames
-- remain distinct obligations. The checker grants no authority.
#print axioms manifestCheck_exact
#print axioms manifest_ready_index
#print axioms manifest_rejects_different_list
#print axioms manifest_rejects_length_loss
#print axioms manifest_original_accepted
end MirroreaProofFirst.OwnerStatementOriginalEntry

namespace MirroreaProofFirst.OwnerStatementOriginalEntry
variable {α : Type u} {κ : Type v}
-- A different handler may be added without changing any position of the
-- retained handler. This establishes source-list preservation independently
-- of FIFO/phase; admission, ownership and the physical write frame are separate.
theorem selectSource_append_unrelated [DecidableEq κ]
 (handler : α → κ) (original extra : List α) (name : κ)
 (disjoint : ∀ p ∈ extra, handler p ≠ name) :
 selectSource handler (original ++ extra) name = selectSource handler original name := by
 simp only [selectSource,List.filter_append]
 have empty : extra.filter (fun p => decide (handler p = name)) = [] := by
   apply List.filter_eq_nil_iff.mpr
   intro p present
   simp [disjoint p present]
 rw [empty,List.append_nil]

theorem manifest_disjoint_addition_preserved [DecidableEq α] [DecidableEq κ]
 (handler : α → κ) (original extra manifest : List α) (first : α)
 (head : manifest.head? = some first)
 (accepted : manifestCheck handler original manifest = true)
 (disjoint : ∀ p ∈ extra, handler p ≠ handler first) :
 manifestCheck handler (original ++ extra) manifest = true := by
 apply (manifestCheck_exact handler (original ++ extra) manifest).mpr
 have ready := (manifestCheck_exact handler original manifest).mp accepted
 refine ⟨ready.1, ?_⟩
 intro p hp i
 have same : p = first := Option.some.inj (hp.symm.trans head)
 subst p
 rw [selectSource_append_unrelated handler original extra (handler first) disjoint]
 exact ready.2 first head i

-- Equality of the filtered source is a semantic condition with a separately
-- executable checker; equality of arbitrary source-control pointers is absent.
def ReplacementAdmissible [DecidableEq κ] (handler : α → κ)
 (original next : List α) (name : κ) : Prop :=
 ∀ i : Nat, (selectSource handler next name)[i]? = (selectSource handler original name)[i]?
def replacementCheck [DecidableEq α] [DecidableEq κ] (handler : α → κ)
 (original next : List α) (name : κ) : Bool :=
 decide (selectSource handler next name = selectSource handler original name)
theorem replacementCheck_exact [DecidableEq α] [DecidableEq κ]
 (handler : α → κ) (original next : List α) (name : κ) :
 replacementCheck handler original next name = true ↔ ReplacementAdmissible handler original next name := by
 simp only [replacementCheck,decide_eq_true_eq,ReplacementAdmissible]
 constructor
 · intro same i; rw [same]
 · exact List.ext_getElem?

theorem replacement_manifest_exact [DecidableEq α] [DecidableEq κ]
 (handler : α → κ) (original next manifest : List α) (first : α)
 (head : manifest.head? = some first)
 (accepted : manifestCheck handler original manifest = true) :
 replacementCheck handler original next (handler first) = true ↔
 manifestCheck handler next manifest = true := by
 have eqOriginal : manifest = selectSource handler original (handler first) := by
   apply List.ext_getElem?
   intro i
   exact manifest_ready_index handler original manifest first accepted head i
 cases manifest with
 | nil => simp at head
 | cons p rest =>
   have same : p = first := by simpa using head
   subst p
   simp only [replacementCheck,manifestCheck,decide_eq_true_eq]
   rw [← eqOriginal]
   exact eq_comm

#print axioms selectSource_append_unrelated
#print axioms replacement_manifest_exact
#print axioms manifest_disjoint_addition_preserved
#print axioms replacementCheck_exact
end MirroreaProofFirst.OwnerStatementOriginalEntry

namespace MirroreaProofFirst.OwnerStatementOriginalEntry.Publication
universe u v
variable {σ : Type u} {γ : Type v}
-- A mathematical pair of exclusive slots. Source is the full protected
-- logical frame, not a pointer/serialized label. Candidate carries a copy of
-- data and has no executable slot. Physical exclusive ownership, complete
-- frame extraction, authenticated origin and exclusion during check/move are
-- independent refinement obligations; copying this model grants no authority.
structure Owned (σ : Type u) (γ : Type v) where
 source : σ
 configuration : γ
 deriving DecidableEq
structure Candidate (σ : Type u) (γ : Type v) where
 basis : σ
 preview : σ
 configuration : γ
 deriving DecidableEq
abbrev Slots (σ : Type u) (γ : Type v) := Option (Owned σ γ) × Option (Owned σ γ)

def Ready (slots : Slots σ γ) (candidate : Candidate σ γ) : Prop :=
 ∃ current, slots.1 = some current ∧ slots.2 = none ∧
   current.source = candidate.basis ∧ candidate.preview = current.source

def publish [DecidableEq σ] (slots : Slots σ γ) (candidate : Candidate σ γ) : Option (Slots σ γ) :=
 match slots.1,slots.2 with
 | some current,none =>
   if current.source = candidate.basis ∧ candidate.preview = current.source then
     some (none,some ⟨current.source,candidate.configuration⟩)
   else none
 | _,_ => none

def Published (slots next : Slots σ γ) (candidate : Candidate σ γ) : Prop :=
 ∃ current, slots.1 = some current ∧ slots.2 = none ∧
   current.source = candidate.basis ∧ candidate.preview = current.source ∧
   next = (none,some ⟨current.source,candidate.configuration⟩)

theorem publish_exact [DecidableEq σ] (slots next : Slots σ γ) (candidate : Candidate σ γ) :
 publish slots candidate = some next ↔ Published slots next candidate := by
 rcases slots with ⟨old,destination⟩
 cases old with
 | none => simp [publish,Published]
 | some current =>
   cases destination with
   | some occupied => simp [publish,Published]
   | none =>
     simp only [publish,Published,Option.some.injEq,exists_eq_left']
     split <;> simp_all [eq_comm]

theorem publish_complete [DecidableEq σ] (slots : Slots σ γ) (candidate : Candidate σ γ) :
 (∃ next, publish slots candidate = some next) ↔ Ready slots candidate := by
 constructor
 · rintro ⟨next,run⟩
   obtain ⟨current,old,free,basis,preview,_⟩ := (publish_exact slots next candidate).mp run
   exact ⟨current,old,free,basis,preview⟩
 · rintro ⟨current,old,free,basis,preview⟩
   exact ⟨_,(publish_exact slots _ candidate).mpr ⟨current,old,free,basis,preview,rfl⟩⟩

theorem published_single_owner [DecidableEq σ] {slots next : Slots σ γ} {candidate : Candidate σ γ}
 (run : publish slots candidate = some next) :
 next.1 = none ∧ ∃ owner, next.2 = some owner := by
 obtain ⟨current,_,_,_,_,rfl⟩ := (publish_exact slots next candidate).mp run
 exact ⟨rfl,_,rfl⟩

theorem published_preserves_source [DecidableEq σ] {slots next : Slots σ γ} {candidate : Candidate σ γ}
 (run : publish slots candidate = some next) (property : σ → Prop)
 (original : ∀ current, slots.1 = some current → property current.source) :
 ∀ current, next.2 = some current → property current.source := by
 obtain ⟨current,old,_,_,_,rfl⟩ := (publish_exact slots next candidate).mp run
 intro other found
 have same : other = ⟨current.source,candidate.configuration⟩ := (Option.some.inj found).symm
 subst other
 exact original current old

theorem published_no_repeat [DecidableEq σ] {slots next : Slots σ γ} {candidate : Candidate σ γ}
 (run : publish slots candidate = some next) (other : Candidate σ γ) : publish next other = none := by
 obtain ⟨current,_,_,_,_,rfl⟩ := (publish_exact slots next candidate).mp run
 rfl

theorem arbitrary_configuration_accepted [DecidableEq σ] (current : Owned σ γ) (configuration : γ) :
 publish (some current,none) ⟨current.source,current.source,configuration⟩ =
   some (none,some ⟨current.source,configuration⟩) := by simp [publish]

theorem stale_basis_refused [DecidableEq σ] (current : Owned σ γ) (candidate : Candidate σ γ)
 (changed : current.source ≠ candidate.basis) :
 publish (some current,none) candidate = none := by simp [publish,changed]

theorem changed_preview_refused [DecidableEq σ] (current : Owned σ γ) (candidate : Candidate σ γ)
 (changed : candidate.preview ≠ current.source) :
 publish (some current,none) candidate = none := by simp [publish,changed]

#print axioms publish_exact
#print axioms publish_complete
#print axioms published_single_owner
#print axioms published_preserves_source
#print axioms published_no_repeat
#print axioms arbitrary_configuration_accepted
#print axioms stale_basis_refused
#print axioms changed_preview_refused
end MirroreaProofFirst.OwnerStatementOriginalEntry.Publication


namespace MirroreaProofFirst.OwnerStatementOriginalEntry.DependencyFrame
variable {α κ β σ π : Type}
-- Dependencies of a retained source must be included in its protected frame.
-- This finite reader check preserves exactly the selected queries (for example
-- existing causal predecessor lists), independently of unrelated additions.
def queriesCheck [DecidableEq α] (keys : List κ) (before after : κ → α) : Bool :=
 keys.all fun key => decide (before key = after key)
def Agrees (keys : List κ) (before after : κ → α) : Prop :=
 ∀ key ∈ keys, before key = after key

variable {keys : List κ} {before after : κ → α}

theorem queriesCheck_exact [DecidableEq α] :
 queriesCheck keys before after = true ↔ Agrees keys before after := by
 simp [queriesCheck,Agrees,List.all_eq_true]

theorem query_results_equal [DecidableEq α]
 (checked : queriesCheck keys before after = true) : keys.map before = keys.map after := by
 apply List.map_congr_left
 intro key member
 exact queriesCheck_exact.mp checked key member

theorem dependent_continuation_preserved [DecidableEq α]
 (checked : queriesCheck keys before after = true) (continuation : List α → β) :
 continuation (keys.map before) = continuation (keys.map after) := by
 rw [query_results_equal checked]

theorem outside_change_accepted [DecidableEq α] [DecidableEq κ]
 (keys : List κ) (before : κ → α) (changed : κ) (replacement : α)
 (outside : changed ∉ keys) :
 queriesCheck keys before (fun key => if key = changed then replacement else before key) = true := by
 apply queriesCheck_exact.mpr
 intro key member
 have different : key ≠ changed := by intro same; subst key; exact outside member
 simp [different]

def traceCheck [DecidableEq α] : List α → List α → Bool
 | [],_ => true
 | _::_,[] => false
 | first::rest,next::tail => decide (first = next) && traceCheck rest tail

theorem traceCheck_exact [DecidableEq α] (before after : List α) :
 traceCheck before after = true ↔ ∃ extra, after = before ++ extra := by
 induction before generalizing after with
 | nil => simp [traceCheck]
 | cons first rest ih =>
   cases after with
   | nil => simp [traceCheck]
   | cons next tail =>
     constructor
     · intro checked
       have parts : first = next ∧ traceCheck rest tail = true := by
         simpa only [traceCheck,Bool.and_eq_true,decide_eq_true_eq] using checked
       obtain ⟨extra,same⟩ := (ih tail).mp parts.2
       refine ⟨extra, ?_⟩
       simp [parts.1,same]
     · rintro ⟨extra,same⟩
       have parts : next = first ∧ tail = rest ++ extra := by simpa using same
       simp only [traceCheck,Bool.and_eq_true,decide_eq_true_eq]
       exact ⟨parts.1.symm,(ih tail).mpr ⟨extra,parts.2⟩⟩

theorem original_observation_preserved [DecidableEq α] (before after : List α)
 (checked : traceCheck before after = true) (query : α → Bool)
 (found : before.find? query = some row) : after.find? query = some row := by
 obtain ⟨extra,rfl⟩ := (traceCheck_exact before after).mp checked
 simp [List.find?_append,found]

-- The local owner facade is called only after swapping the actual shared
-- snapshot into it, and swapped back after the call. Its parked snapshot is
-- representation state. This theorem covers that call's visible result only;
-- it does not justify changing shared authority, another direct entry, or a
-- snapshot observer. Those require independent current-authority/frame checks.
structure Local (σ π : Type) where
 shared : σ
 parked : σ
 owner : π
 deriving DecidableEq

def ownerStep (operation : σ → π → σ × π × β) (state : Local σ π) : Local σ π × β :=
 let result := operation state.shared state.owner
 (⟨result.1,state.parked,result.2.1⟩,result.2.2)
def visible (result : Local σ π × β) : σ × π × β :=
 (result.1.shared,result.1.owner,result.2)

theorem parked_refresh_keeps_owner_step (operation : σ → π → σ × π × β)
 (state : Local σ π) (parked : σ) :
 visible (ownerStep operation {state with parked := parked}) =
 visible (ownerStep operation state) := rfl

#print axioms queriesCheck_exact
#print axioms query_results_equal
#print axioms dependent_continuation_preserved
#print axioms outside_change_accepted
#print axioms traceCheck_exact
#print axioms original_observation_preserved
#print axioms parked_refresh_keeps_owner_step
end MirroreaProofFirst.OwnerStatementOriginalEntry.DependencyFrame


namespace MirroreaProofFirst.OwnerStatementOriginalEntry.DecodedOrigin
universe u v
variable {α : Type u} {κ : Type v}
-- α is the full typed payload, including program/Core/source/owner/budget and
-- scope. The complete ordered nested inventory includes pre-staged successors.
-- expected is independently held admitted data. Constructing a mathematical
-- value here does not authenticate it: that origin is an explicit refinement
-- obligation of the sole physical issuer/expected-control boundary.
structure Image (α : Type u) where
 primary : α
 nested : List α
 deriving DecidableEq
structure Decoded (α : Type u) where
 image : Image α
 deriving DecidableEq
structure Promoted (α : Type u) where
 image : Image α
 deriving DecidableEq

def Corresponds (expected : Image α) (candidate : Decoded α) : Prop :=
 candidate.image.primary = expected.primary ∧
 ∀ i : Nat, candidate.image.nested[i]? = expected.nested[i]?

def check [DecidableEq α] (expected : Image α) (candidate : Decoded α) : Bool :=
 decide (candidate.image.primary = expected.primary) &&
 decide (candidate.image.nested = expected.nested)

theorem check_exact [DecidableEq α] (expected : Image α) (candidate : Decoded α) :
 check expected candidate = true ↔ Corresponds expected candidate := by
 simp only [check,Bool.and_eq_true,decide_eq_true_eq,Corresponds]
 constructor
 · rintro ⟨primary,nested⟩
   exact ⟨primary,fun i => congrArg (fun xs : List α => xs[i]?) nested⟩
 · rintro ⟨primary,nested⟩
   exact ⟨primary,List.ext_getElem? nested⟩

def promote [DecidableEq α] (expected : Image α) (candidate : Decoded α) : Option (Promoted α) :=
 if check expected candidate then some ⟨candidate.image⟩ else none

def Promotes (expected : Image α) (candidate : Decoded α) (result : Promoted α) : Prop :=
 Corresponds expected candidate ∧ result.image = candidate.image

theorem promote_exact [DecidableEq α] (expected : Image α)
 (candidate : Decoded α) (result : Promoted α) :
 promote expected candidate = some result ↔ Promotes expected candidate result := by
 simp only [promote,Promotes]
 split
 · rename_i valid
   have corr := (check_exact expected candidate).mp valid
   cases result with
   | mk image =>
     simp only [Option.some.injEq,Promoted.mk.injEq]
     constructor
     · intro same; exact ⟨corr,same.symm⟩
     · rintro ⟨_,same⟩; exact same.symm
 · rename_i invalid
   have bad : ¬ Corresponds expected candidate := by
     intro corr; exact invalid ((check_exact expected candidate).mpr corr)
   simp [bad]

theorem promote_complete [DecidableEq α] (expected : Image α) (candidate : Decoded α) :
 (∃ result, promote expected candidate = some result) ↔ Corresponds expected candidate := by
 constructor
 · rintro ⟨result,run⟩; exact ((promote_exact expected candidate result).mp run).1
 · intro corr; exact ⟨⟨candidate.image⟩,(promote_exact expected candidate _).mpr ⟨corr,rfl⟩⟩

theorem promoted_payload_original [DecidableEq α] (expected : Image α)
 (candidate : Decoded α) (result : Promoted α)
 (run : promote expected candidate = some result) : result.image = expected := by
 obtain ⟨⟨primary,nested⟩,out⟩ := (promote_exact expected candidate result).mp run
 rw [out]
 have exactNested := List.ext_getElem? nested
 cases expected
 cases candidate with
 | mk image => cases image; simp_all

theorem original_accepted [DecidableEq α] (expected : Image α) :
 promote expected ⟨expected⟩ = some ⟨expected⟩ := by simp [promote,check]

theorem mismatch_refused [DecidableEq α] (expected : Image α) (candidate : Decoded α)
 (different : candidate.image ≠ expected) : promote expected candidate = none := by
 cases run : promote expected candidate with
 | none => rfl
 | some result =>
   have out := ((promote_exact expected candidate result).mp run).2
   exact False.elim (different (out.symm.trans (promoted_payload_original expected candidate result run)))

-- Link exact payload promotion to the existing ORIGINAL owner-entry checker.
-- No assumption says this entry already rejects the protected operation.
theorem promoted_protected_rejected [DecidableEq α] [DecidableEq κ]
 (expected : Image (List α)) (candidate : Decoded (List α))
 (result : Promoted (List α)) (run : promote expected candidate = some result)
 (operation : α → κ) (ordinal : α → Option Nat) (bound : Bool)
 (prior plan : α) (unique : Unambiguous operation expected.primary)
 (stored : prior ∈ expected.primary) (guarded : ordinal prior ≠ none)
 (sameOperation : operation plan = operation prior) :
 OwnerStatementOriginalEntry.check ordinal result.image.primary bound plan = false := by
 rw [promoted_payload_original expected candidate result run]
 exact protected_rejected unique stored guarded sameOperation

-- The private physical decoder must have no conversion from Decoded to an
-- executable M8 instance except this independently bound promotion. Rust
-- privacy/constructor/caller closure, faithful codec, trusted control origin,
-- collision resistance if exact data equality is replaced by commitments,
-- M9 current authorization, exclusive runtime ownership and resource bounds
-- are not proved by these lemmas and are not additional axioms.
#print axioms check_exact
#print axioms promote_exact
#print axioms promote_complete
#print axioms promoted_payload_original
#print axioms original_accepted
#print axioms mismatch_refused
#print axioms promoted_protected_rejected
end MirroreaProofFirst.OwnerStatementOriginalEntry.DecodedOrigin

namespace MirroreaProofFirst.OwnerStatementOriginalEntry.TraceAllocator
variable {α : Type}
-- Raw owner-local IDs. The physical formatter must be injective and the
-- counter bounded; this model grants no occurrence authenticity or authority.
structure Trace (α : Type) where
 rows : List (Nat × α)
 next : Nat
 deriving DecidableEq

def Valid (t : Trace α) : Prop :=
 (t.rows.map Prod.fst).Nodup ∧ ∀ row ∈ t.rows, row.1 < t.next

def validCheck (t : Trace α) : Bool :=
 decide (t.rows.map Prod.fst).Nodup && t.rows.all (fun row => decide (row.1 < t.next))

theorem validCheck_exact (t : Trace α) : validCheck t = true ↔ Valid t := by
 simp [validCheck,Valid,List.all_eq_true]

def Extension (before after : Trace α) : Prop :=
 (∃ extra, after.rows = before.rows ++ extra) ∧ before.next ≤ after.next ∧ Valid after

def extensionCheck [DecidableEq α] (before after : Trace α) : Bool :=
 DependencyFrame.traceCheck before.rows after.rows && decide (before.next ≤ after.next) && validCheck after

theorem extensionCheck_exact [DecidableEq α] (before after : Trace α) :
 extensionCheck before after = true ↔ Extension before after := by
 simp only [extensionCheck,Bool.and_eq_true,decide_eq_true_eq,validCheck_exact,
   DependencyFrame.traceCheck_exact,Extension]
 exact and_assoc

theorem next_fresh (t : Trace α) (valid : Valid t) : t.next ∉ t.rows.map Prod.fst := by
 intro member
 obtain ⟨row,present,same⟩ := List.mem_map.mp member
 have bound := valid.2 row present
 omega

def append (t : Trace α) (payload : α) : Trace α :=
 ⟨t.rows ++ [(t.next,payload)],t.next+1⟩

theorem append_valid (t : Trace α) (payload : α) (valid : Valid t) : Valid (append t payload) := by
 constructor
 · simp only [append,List.map_append,List.map_cons,List.map_nil]
   exact List.nodup_append.mpr ⟨valid.1,by simp,by
     intro key member other same
     simp only [List.mem_cons,List.not_mem_nil,or_false] at same
     subst other
     exact fun equal => next_fresh t valid (equal ▸ member)⟩
 · intro row member
   rcases List.mem_append.mp member with old | added
   · have bound := valid.2 row old
     change row.1 < t.next+1
     omega
   · simp only [List.mem_cons,List.not_mem_nil,or_false] at added
     subst row
     simp [append]

theorem append_extension (t : Trace α) (payload : α) (valid : Valid t) :
 Extension t (append t payload) := by
 exact ⟨⟨[(t.next,payload)],rfl⟩,Nat.le_add_right _ _,append_valid t payload valid⟩

def push (limit : Nat) (t : Trace α) (payload : α) : Option (Trace α) :=
 if t.next < limit then some (append t payload) else none

def Appended (limit : Nat) (before after : Trace α) (payload : α) : Prop :=
 before.next < limit ∧ after.rows = before.rows ++ [(before.next,payload)] ∧ after.next = before.next+1

theorem push_exact (limit : Nat) (t next : Trace α) (payload : α) :
 push limit t payload = some next ↔ Appended limit t next payload := by
 cases next with
 | mk rows cursor =>
   simp only [push,Appended,append]
   split <;> simp_all [Trace.mk.injEq,eq_comm] <;> omega

theorem push_preserves (limit : Nat) (t next : Trace α) (payload : α)
 (valid : Valid t) (run : push limit t payload = some next) :
 Valid next ∧ next.next ≤ limit := by
 obtain ⟨capacity,rows,cursor⟩ := (push_exact limit t next payload).mp run
 have same : next = append t payload := by cases next; simp_all [append]
 rw [same]
 exact ⟨append_valid t payload valid,by simp only [append];omega⟩

theorem old_lookup_preserved (t : Trace α) (payload : α) (key : Nat)
 (valid : Valid t) (old : key ∈ t.rows.map Prod.fst) :
 (append t payload).rows.find? (fun row => row.1 == key) = t.rows.find? (fun row => row.1 == key) := by
 have different : t.next ≠ key := by intro same;subst key;exact next_fresh t valid old
 simp only [append,List.find?_append]
 cases found : t.rows.find? (fun row => row.1 == key) with
 | some row => simp
 | none => simp [different]

-- A publication step rechecks independently defined validity, rather than
-- assuming every arbitrary appended row is safe. Ordinary append preserves it.
inductive Steps : Trace α → Trace α → Prop where
 | refl : Steps t t
 | append : Steps initial t → (run : push limit t payload = some next) → Steps initial next
 | publish : Steps initial t → Extension t next → Steps initial next

theorem steps_valid (valid : Valid initial) (path : Steps initial current) : Valid current := by
 induction path with
 | refl => exact valid
 | append _ run ih => exact (push_preserves _ _ _ _ ih run).1
 | publish _ extension _ => exact extension.2.2

#print axioms validCheck_exact
#print axioms extensionCheck_exact
#print axioms next_fresh
#print axioms append_valid
#print axioms append_extension
#print axioms push_exact
#print axioms push_preserves
#print axioms old_lookup_preserved
#print axioms steps_valid
end MirroreaProofFirst.OwnerStatementOriginalEntry.TraceAllocator
