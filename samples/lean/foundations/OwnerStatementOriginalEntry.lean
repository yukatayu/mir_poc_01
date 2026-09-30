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

namespace MirroreaProofFirst.OwnerStatementOriginalEntry.TraceAllocator
variable {α : Type}
-- This allocator has no detached reservations: one appended raw row consumes
-- one counter unit. Count balance is necessary, not occurrence authenticity.
def CountedExtension (before after : Trace α) : Prop :=
 Extension before after ∧ after.next + before.rows.length = before.next + after.rows.length

def countedCheck [DecidableEq α] (before after : Trace α) : Bool :=
 extensionCheck before after &&
 decide (after.next + before.rows.length = before.next + after.rows.length)

theorem countedCheck_exact [DecidableEq α] (before after : Trace α) :
 countedCheck before after = true ↔ CountedExtension before after := by
 simp [countedCheck,CountedExtension,extensionCheck_exact]

theorem append_counted (t : Trace α) (payload : α) (valid : Valid t) :
 CountedExtension t (append t payload) := by
 refine ⟨append_extension t payload valid, ?_⟩
 simp only [append,List.length_append,List.length_cons,List.length_nil]
 omega

theorem unchanged_rows_no_advance (before after : Trace α)
 (extension : CountedExtension before after) (rows : after.rows = before.rows) :
 after.next = before.next := by
 have balance := extension.2
 rw [rows] at balance
 omega

theorem counted_transitive (first middle last : Trace α)
 (left : CountedExtension first middle) (right : CountedExtension middle last) :
 CountedExtension first last := by
 rcases left with ⟨⟨⟨a,rowsA⟩,cursorA,validA⟩,balanceA⟩
 rcases right with ⟨⟨⟨b,rowsB⟩,cursorB,validB⟩,balanceB⟩
 refine ⟨⟨⟨a++b, ?_⟩,Nat.le_trans cursorA cursorB,validB⟩, ?_⟩
 · rw [rowsB,rowsA,List.append_assoc]
 · omega

-- Under a known row cost, no fabricated cursor-only publication can consume
-- the original headroom. Actual added rows remain charged and may consume it;
-- reservation of a pending operation is a separate resource obligation.
theorem unchanged_headroom (before after : Trace α) (cost limit : Nat)
 (extension : CountedExtension before after) (rows : after.rows = before.rows) :
 after.next + cost ≤ limit ↔ before.next + cost ≤ limit := by
 rw [unchanged_rows_no_advance before after extension rows]

#print axioms countedCheck_exact
#print axioms append_counted
#print axioms unchanged_rows_no_advance
#print axioms counted_transitive
#print axioms unchanged_headroom
end MirroreaProofFirst.OwnerStatementOriginalEntry.TraceAllocator

namespace MirroreaProofFirst.OwnerStatementOriginalEntry.ResultSlots
variable {α β : Type}
-- Width is derived from the original manifest, not an independently supplied
-- promise. A held actual attempt whose outer result has not been recorded is
-- still owed a slot. Accepting an already recorded result consumes no new slot.
structure State (α β : Type) where
 manifest : List α
 prior : List β
 current : List β
 held : Bool
 accepted : Nat

def Owed (s : State α β) : Nat := (s.manifest.drop s.current.length).length
def Valid (capacity : Nat) (s : State α β) : Prop :=
 s.current.length ≤ s.manifest.length ∧
 (s.prior ++ s.current).length + Owed s ≤ capacity

def Ready (capacity : Nat) (manifest : List α) (retained : List β) : Prop :=
 ∃ room : Nat, retained.length + room = capacity ∧ manifest.length ≤ room

def prepare (capacity : Nat) (manifest : List α) (retained : List β) : Option (State α β) :=
 if retained.length + manifest.length ≤ capacity then
 some ⟨manifest,retained,[],false,0⟩ else none

def Prepared (capacity : Nat) (manifest : List α) (retained : List β) (s : State α β) : Prop :=
 Ready capacity manifest retained ∧ s = ⟨manifest,retained,[],false,0⟩

theorem room_exact (capacity : Nat) (manifest : List α) (retained : List β) :
 retained.length + manifest.length ≤ capacity ↔ Ready capacity manifest retained := by
 constructor
 · intro room
   exact ⟨capacity-retained.length,by omega,by omega⟩
 · rintro ⟨room,balance,enough⟩;omega

theorem prepare_exact (capacity : Nat) (manifest : List α) (retained : List β) (s : State α β) :
 prepare capacity manifest retained = some s ↔ Prepared capacity manifest retained s := by
 unfold prepare Prepared
 split
 · rename_i room
   have ready := (room_exact capacity manifest retained).mp room
   simp [ready,eq_comm]
 · rename_i lack
   have bad : ¬ Ready capacity manifest retained := by
    intro ready;exact lack ((room_exact capacity manifest retained).mpr ready)
   simp [bad]

theorem valid_iff (capacity : Nat) (s : State α β) :
 Valid capacity s ↔ s.current.length ≤ s.manifest.length ∧
 s.prior.length+s.manifest.length ≤ capacity := by
 simp only [Valid,Owed,List.length_append,List.length_drop]
 omega

theorem prepared_valid (capacity : Nat) (manifest : List α) (retained : List β) (s : State α β)
 (run : prepare capacity manifest retained = some s) : Valid capacity s := by
 obtain ⟨ready,rfl⟩ := (prepare_exact capacity manifest retained s).mp run
 rw [valid_iff]
 exact ⟨by simp, (room_exact capacity manifest retained).mpr ready⟩

def begin (s : State α β) : Option (State α β) :=
 if !s.held && s.current.length == s.accepted && s.current.length < s.manifest.length then some {s with held:=true} else none

def publish (s : State α β) (result : β) : Option (State α β) :=
 if s.held && s.current.length == s.accepted && s.current.length < s.manifest.length then
 some {s with current:=s.current++[result]} else none

theorem begin_preserves (capacity : Nat) (s next : State α β)
 (valid : Valid capacity s) (run : begin s = some next) :
 Valid capacity next ∧ Owed next = Owed s := by
 unfold begin at run
 split at run
 · cases Option.some.inj run
   exact ⟨valid,rfl⟩
 · simp at run

theorem publish_preserves (capacity : Nat) (s next : State α β) (result : β)
 (valid : Valid capacity s) (run : publish s result = some next) :
 Valid capacity next ∧ Owed next+1 = Owed s := by
 unfold publish at run
 split at run
 · rename_i ready
   cases Option.some.inj run
   have bound := (valid_iff capacity s).mp valid
   simp only [Bool.and_eq_true,decide_eq_true_eq] at ready
   constructor
   · rw [valid_iff]
     simp only [List.length_append,List.length_cons,List.length_nil]
     exact ⟨by omega,bound.2⟩
   · simp only [Owed,List.length_drop,List.length_append,List.length_cons,List.length_nil]
     omega
 · simp at run

theorem held_result_has_room (capacity : Nat) (s : State α β)
 (valid : Valid capacity s) (pending : s.current.length < s.manifest.length) :
 (s.prior++s.current).length < capacity := by
 have bound := (valid_iff capacity s).mp valid
 simp only [List.length_append]
 omega


-- Recording a result is distinct from accepting its source completion. The
-- same held invocation stays held until that separate acknowledgment succeeds.
def accept (s : State α β) : Option (State α β) :=
 if s.held && s.current.length == s.accepted+1 then
 some {s with held:=false,accepted:=s.accepted+1} else none

theorem accept_preserves (capacity : Nat) (s next : State α β)
 (valid : Valid capacity s) (run : accept s = some next) :
 Valid capacity next ∧ Owed next = Owed s := by
 unfold accept at run
 split at run
 · cases Option.some.inj run
   exact ⟨valid,rfl⟩
 · simp at run

-- Deliberate reactivation uses the actual retained results of the completed
-- invocation and the original manifest; it is not a retry or history reset.
def again (capacity : Nat) (s : State α β) : Option (State α β) :=
 if !s.held && s.accepted == s.manifest.length && s.current.length == s.manifest.length then
 prepare capacity s.manifest (s.prior++s.current) else none

theorem again_preserves (capacity : Nat) (s next : State α β)
 (run : again capacity s = some next) :
 Valid capacity next ∧ next.prior=s.prior++s.current ∧ next.manifest=s.manifest := by
 unfold again at run
 split at run
 · have valid := prepared_valid capacity s.manifest (s.prior++s.current) next run
   obtain ⟨_,same⟩ := (prepare_exact capacity s.manifest (s.prior++s.current) next).mp run
   exact ⟨valid,by rw [same],by rw [same]⟩
 · simp at run

#print axioms room_exact
#print axioms prepare_exact
#print axioms valid_iff
#print axioms prepared_valid
#print axioms begin_preserves
#print axioms publish_preserves
#print axioms held_result_has_room
#print axioms accept_preserves
#print axioms again_preserves
end MirroreaProofFirst.OwnerStatementOriginalEntry.ResultSlots

namespace MirroreaProofFirst.OwnerStatementOriginalEntry.CarrierFrame
variable {α : Type}
-- Outbound movement selects envelopes; inbound service is FIFO. Preserve the
-- entire prefix through the last original-request message, not all unrelated
-- traffic and not only a filtered inbox that could silently skip its head.
def barrier (original : α → Bool) : List α → List α
 | [] => []
 | x::xs => if xs.any original then x::barrier original xs else if original x then [x] else []

theorem barrier_empty (original : α → Bool) (xs : List α) :
 barrier original xs = [] ↔ xs.any original = false := by
 induction xs with
 | nil => simp [barrier]
 | cons x xs ih =>
   cases tail : xs.any original <;> cases head : original x <;>
    simp [barrier,List.any_cons,tail,head]

theorem barrier_head (original : α → Bool) (xs : List α)
 (present : xs.any original = true) :
 (barrier original xs).head? = xs.head? := by
 cases xs with
 | nil => simp at present
 | cons x xs =>
   simp only [barrier,List.head?_cons]
   split
   · rfl
   · rename_i absent
     have head : original x = true := by simpa [absent] using present
     simp [head]

theorem irrelevant_filter (original : α → Bool) (xs : List α)
 (absent : xs.any original = false) : xs.filter original = [] := by
 induction xs with
 | nil => rfl
 | cons x xs ih =>
   simp only [List.any_cons,Bool.or_eq_false_iff] at absent
   simp [absent.1,ih absent.2]

theorem barrier_append_irrelevant (original : α → Bool) (xs extra : List α)
 (absent : extra.any original = false) :
 barrier original (xs++extra) = barrier original xs := by
 induction xs with
 | nil => simpa using (barrier_empty original extra).mpr absent
 | cons x xs ih =>
   simp only [List.cons_append,barrier,List.any_append,absent,Bool.or_false,ih]

structure Queues (α : Type) where
 outgoing : List α
 incoming : List α
 deriving DecidableEq

def Preserves (original : α → Bool) (before after : Queues α) : Prop :=
 (∀ i : Nat, (before.outgoing.filter original)[i]? = (after.outgoing.filter original)[i]?) ∧
 barrier original before.incoming = barrier original after.incoming

def check [DecidableEq α] (original : α → Bool) (before after : Queues α) : Bool :=
 decide (before.outgoing.filter original = after.outgoing.filter original) &&
 decide (barrier original before.incoming = barrier original after.incoming)

theorem check_exact [DecidableEq α] (original : α → Bool) (before after : Queues α) :
 check original before after = true ↔ Preserves original before after := by
 simp only [check,Bool.and_eq_true,decide_eq_true_eq,Preserves]
 constructor
 · rintro ⟨outbound,inbound⟩
   exact ⟨fun i => congrArg (fun xs : List α => xs[i]?) outbound,inbound⟩
 · rintro ⟨outbound,inbound⟩
   exact ⟨List.ext_getElem? outbound,inbound⟩

theorem original_reply_retained (original : α → Bool) (before after : Queues α)
 (frame : Preserves original before after) (message : α)
 (pending : message ∈ before.outgoing) (owned : original message = true) :
 message ∈ after.outgoing := by
 have kept : message ∈ before.outgoing.filter original := List.mem_filter.mpr ⟨pending,owned⟩
 rw [List.ext_getElem? frame.1] at kept
 exact (List.mem_filter.mp kept).1

theorem original_fifo_head_preserved (original : α → Bool) (before after : Queues α)
 (frame : Preserves original before after) (present : before.incoming.any original = true) :
 after.incoming.head? = before.incoming.head? := by
 have remains : after.incoming.any original = true := by
  cases h : after.incoming.any original with
  | true => rfl
  | false =>
    have empty := (barrier_empty original after.incoming).mpr h
    rw [← frame.2] at empty
    have bad := (barrier_empty original before.incoming).mp empty
    simp [present] at bad
 rw [← barrier_head original after.incoming remains,← frame.2,barrier_head original before.incoming present]

theorem unrelated_append_accepted [DecidableEq α] (original : α → Bool)
 (before : Queues α) (outbound inbound : List α)
 (outAbsent : outbound.any original = false) (inAbsent : inbound.any original = false) :
 check original before ⟨before.outgoing++outbound,before.incoming++inbound⟩ = true := by
 simp [check,List.filter_append,irrelevant_filter original outbound outAbsent,
  barrier_append_irrelevant original before.incoming inbound inAbsent]

#print axioms barrier_empty
#print axioms barrier_head
#print axioms irrelevant_filter
#print axioms barrier_append_irrelevant
#print axioms check_exact
#print axioms original_reply_retained
#print axioms original_fifo_head_preserved
#print axioms unrelated_append_accepted
end MirroreaProofFirst.OwnerStatementOriginalEntry.CarrierFrame

namespace MirroreaProofFirst.OwnerStatementOriginalEntry.ReadOrigin
-- Newest-first ACTUAL committed writes, not observer-invented events.
-- Keys include the owner/locus; values are exact; the history's authenticity,
-- exclusive read/commit boundary and actual allocator are physical obligations.
structure Write (K V : Type) where
 key : K
 value : V
 occurrence : Nat
 deriving DecidableEq

variable {K V : Type} [DecidableEq K]

inductive Latest (key : K) : List (Write K V) → Write K V → Prop
 | here (w : Write K V) (rest) (same : w.key = key) : Latest key (w::rest) w
 | skip (w : Write K V) (rest) (chosen) (different : w.key ≠ key)
   (prior : Latest key rest chosen) : Latest key (w::rest) chosen

def select (key : K) : List (Write K V) → Option (Write K V)
 | [] => none
 | w::rest => if w.key = key then some w else select key rest

theorem select_exact (key : K) (history : List (Write K V)) (w : Write K V) :
 select key history = some w ↔ Latest key history w := by
 induction history with
 | nil =>
   constructor
   · simp [select]
   · intro h; cases h
 | cons top rest ih =>
   by_cases same : top.key = key
   · simp only [select,if_pos same,Option.some.injEq]
     constructor
     · intro eq; subst w; exact .here top rest same
     · intro h; cases h with
       | here => rfl
       | skip _ _ _ different _ => exact False.elim (different same)
   · simp only [select,if_neg same]
     constructor
     · intro h; exact .skip top rest w same (ih.mp h)
     · intro h; cases h with
       | here _ _ eq => exact False.elim (same eq)
       | skip _ _ _ _ prior => exact ih.mpr prior

omit [DecidableEq K] in
theorem latest_member {key : K} {history : List (Write K V)} {w : Write K V}
 (h : Latest key history w) : w ∈ history := by
 induction h with
 | here => exact List.mem_cons_self
 | skip _ _ _ _ _ ih => exact List.mem_cons_of_mem _ ih

-- Operational replay updates only the exact key. The initial environment may
-- be arbitrary and is not relabelled as a runtime write.
def replay (initial : K → Option V) (key : K) : List (Write K V) → Option V
 | [] => initial key
 | w::rest => if w.key = key then some w.value else replay initial key rest

theorem latest_value (initial : K → Option V) {key : K}
 {history : List (Write K V)} {w : Write K V} (h : Latest key history w) :
 replay initial key history = some w.value := by
 induction h with
 | here _ _ same => simp [replay,same]
 | skip _ _ _ different _ ih => simp [replay,different,ih]

theorem select_none_initial (initial : K → Option V) (key : K) (history : List (Write K V))
 (absent : select key history = none) : replay initial key history = initial key := by
 induction history with
 | nil => rfl
 | cons w rest ih =>
   by_cases same : w.key = key
   · simp [select,same] at absent
   · simp only [select,if_neg same] at absent
     simp [replay,same,ih absent]

variable [DecidableEq V]

def bindRead (key : K) (history : List (Write K V)) (actual : V) : Option Nat :=
 match select key history with
 | none => none
 | some w => if w.value = actual then some w.occurrence else none

theorem bind_exact (key : K) (history : List (Write K V)) (actual : V) (id : Nat) :
 bindRead key history actual = some id ↔
 ∃ w, Latest key history w ∧ w.value = actual ∧ w.occurrence = id := by
 cases found : select key history with
 | none =>
   simp only [bindRead,found]
   constructor
   · simp
   · rintro ⟨w,latest,_,_⟩
     have eq := (select_exact key history w).mpr latest
     simp [found] at eq
 | some w =>
   have latest := (select_exact key history w).mp found
   simp only [bindRead,found]
   constructor
   · intro eq
     split at eq
     · exact ⟨w,latest,‹w.value = actual›,Option.some.inj eq⟩
     · simp at eq
   · rintro ⟨chosen,h,value,idEq⟩
     have eq := (select_exact key history chosen).mpr h
     rw [found] at eq
     have same := Option.some.inj eq
     subst chosen
     simp [value,idEq]

theorem bind_actual_latest (key : K) (history : List (Write K V)) (w : Write K V)
 (h : Latest key history w) : bindRead key history w.value = some w.occurrence :=
 (bind_exact key history w.value w.occurrence).mpr ⟨w,h,rfl,rfl⟩

theorem unrelated_write_preserved (key : K) (history : List (Write K V))
 (w : Write K V) (actual : V) (different : w.key ≠ key) :
 bindRead key (w::history) actual = bindRead key history actual := by
 simp [bindRead,select,different]

theorem latest_different_value_refused (key : K) (history : List (Write K V))
 (w : Write K V) (actual : V) (same : w.key = key) (different : w.value ≠ actual) :
 bindRead key (w::history) actual = none := by simp [bindRead,select,same,different]

theorem bound_origin_earlier (key : K) (history : List (Write K V)) (actual : V)
 (readId origin : Nat) (past : ∀ w ∈ history, w.occurrence < readId)
 (bound : bindRead key history actual = some origin) : origin < readId := by
 obtain ⟨w,h,_,rfl⟩ := (bind_exact key history actual origin).mp bound
 exact past w (latest_member h)

-- Keep exact value-origin edges across physical execution lanes; keep ordinary
-- FIFO dependencies only inside their semantic lane. Qualification must map
-- only genuine recorded IDs; this function neither creates events nor authority.
def project (sameLane readFrom : Nat → Bool) (raw : List Nat) : List Nat :=
 raw.filter (fun origin => sameLane origin || readFrom origin)

theorem project_exact (sameLane readFrom : Nat → Bool) (raw : List Nat) (origin : Nat) :
 origin ∈ project sameLane readFrom raw ↔
 origin ∈ raw ∧ (sameLane origin = true ∨ readFrom origin = true) := by
 simp [project]

theorem actual_origin_retained (sameLane readFrom : Nat → Bool) (raw : List Nat) (origin : Nat)
 (present : origin ∈ raw) (actual : readFrom origin = true) :
 origin ∈ project sameLane readFrom raw :=
 (project_exact sameLane readFrom raw origin).mpr ⟨present,Or.inr actual⟩

theorem unrelated_lane_refused (sameLane readFrom : Nat → Bool) (raw : List Nat) (origin : Nat)
 (other : sameLane origin = false) (unread : readFrom origin = false) :
 origin ∉ project sameLane readFrom raw := by simp [project,other,unread]

#print axioms select_exact
#print axioms latest_member
#print axioms latest_value
#print axioms select_none_initial
#print axioms bind_exact
#print axioms bind_actual_latest
#print axioms unrelated_write_preserved
#print axioms latest_different_value_refused
#print axioms bound_origin_earlier
#print axioms project_exact
#print axioms actual_origin_retained
#print axioms unrelated_lane_refused
end MirroreaProofFirst.OwnerStatementOriginalEntry.ReadOrigin

namespace MirroreaProofFirst.OwnerStatementOriginalEntry.ReportReservation
-- Resource accounting only: owed includes actual unprojected rows and a
-- bound for a queued original service. It does not assert future trace events.
-- Authentic debt derivation, every writer's cost and admission before effects
-- are separate physical obligations. No OOM/abort recovery follows here.
structure State where
 used : Nat
 owed : Nat
 deriving DecidableEq, Repr

def Fits (limit : Nat) (s : State) : Prop :=
 ∃ spare, s.used + s.owed + spare = limit

def Available (limit cost : Nat) (s : State) : Prop :=
 ∃ spare, s.used + s.owed + spare = limit ∧ cost ≤ spare

theorem fits_exact (limit : Nat) (s : State) :
 Fits limit s ↔ s.used + s.owed ≤ limit := by
 constructor
 · rintro ⟨spare,balance⟩;omega
 · intro bound;exact ⟨limit-(s.used+s.owed),by omega⟩

theorem available_exact (limit cost : Nat) (s : State) :
 Available limit cost s ↔ s.used + cost + s.owed ≤ limit := by
 constructor
 · rintro ⟨spare,balance,enough⟩;omega
 · intro bound;exact ⟨limit-(s.used+s.owed),by omega,by omega⟩

def foreign (limit cost : Nat) (s : State) : Option State :=
 if s.used + cost + s.owed ≤ limit then some ⟨s.used+cost,s.owed⟩ else none

theorem foreign_exact (limit cost : Nat) (s next : State) :
 foreign limit cost s = some next ↔
 Available limit cost s ∧ next=⟨s.used+cost,s.owed⟩ := by
 unfold foreign
 split
 · rename_i enough
   have ready := (available_exact limit cost s).mpr enough
   simp [ready,eq_comm]
 · rename_i insufficient
   have bad : ¬ Available limit cost s := by
    intro h;exact insufficient ((available_exact limit cost s).mp h)
   simp [bad]

theorem foreign_preserves (limit cost : Nat) (s next : State)
 (run : foreign limit cost s = some next) :
 Fits limit next ∧ next.owed=s.owed ∧ next.used=s.used+cost := by
 obtain ⟨ready,rfl⟩ := (foreign_exact limit cost s next).mp run
 exact ⟨(fits_exact limit _).mpr ((available_exact limit cost s).mp ready),rfl,rfl⟩

def acquire (limit amount : Nat) (s : State) : Option State :=
 if s.used + s.owed + amount ≤ limit then some ⟨s.used,s.owed+amount⟩ else none

theorem acquire_exact (limit amount : Nat) (s next : State) :
 acquire limit amount s = some next ↔
 Available limit amount s ∧ next=⟨s.used,s.owed+amount⟩ := by
 unfold acquire
 split
 · rename_i enough
   have ready := (available_exact limit amount s).mpr (by omega)
   simp [ready,eq_comm]
 · rename_i insufficient
   have bad : ¬ Available limit amount s := by
    intro h;have enough := (available_exact limit amount s).mp h;omega
   simp [bad]

theorem acquired_fits (limit amount : Nat) (s next : State)
 (run : acquire limit amount s = some next) : Fits limit next := by
 obtain ⟨ready,rfl⟩ := (acquire_exact limit amount s next).mp run
 apply (fits_exact limit _).mpr
 have enough := (available_exact limit amount s).mp ready
 simp only
 omega

-- Discharge is accounting for actual original rows. A physical implementation
-- must separately show that those rows really occurred and were projected.
def discharge (amount : Nat) (s : State) : Option State :=
 if amount ≤ s.owed then some ⟨s.used+amount,s.owed-amount⟩ else none

theorem discharge_preserves (limit amount : Nat) (s next : State)
 (valid : Fits limit s) (run : discharge amount s = some next) :
 Fits limit next ∧ next.used+next.owed=s.used+s.owed := by
 unfold discharge at run
 split at run
 · rename_i within
   cases Option.some.inj run
   have bound := (fits_exact limit s).mp valid
   constructor
   · apply (fits_exact limit _).mpr;simp only;omega
   · simp only;omega
 · simp at run

theorem all_original_rows_fit (limit : Nat) (s : State) (valid : Fits limit s) :
 s.used+s.owed ≤ limit ∧ discharge s.owed s = some ⟨s.used+s.owed,0⟩ := by
 exact ⟨(fits_exact limit s).mp valid,by simp [discharge]⟩

-- Only actual terminal outcome evidence may justify releasing unused budget.
-- Existing actual rows are discharged, not released as unused.
def release (unused : Nat) (s : State) : Option State :=
 if unused ≤ s.owed then some ⟨s.used,s.owed-unused⟩ else none

theorem release_preserves (limit unused : Nat) (s next : State)
 (valid : Fits limit s) (run : release unused s = some next) :
 Fits limit next ∧ next.used=s.used := by
 unfold release at run
 split at run
 · cases Option.some.inj run
   have bound := (fits_exact limit s).mp valid
   refine ⟨(fits_exact limit _).mpr ?_,rfl⟩
   simp only;omega
 · simp at run

theorem foreign_refuses_owed_capacity (limit cost : Nat) (s : State)
 (insufficient : limit < s.used+cost+s.owed) : foreign limit cost s = none := by
 simp [foreign,show ¬s.used+cost+s.owed≤limit by omega]

#print axioms fits_exact
#print axioms available_exact
#print axioms foreign_exact
#print axioms foreign_preserves
#print axioms acquire_exact
#print axioms acquired_fits
#print axioms discharge_preserves
#print axioms all_original_rows_fit
#print axioms release_preserves
#print axioms foreign_refuses_owed_capacity
end MirroreaProofFirst.OwnerStatementOriginalEntry.ReportReservation

namespace MirroreaProofFirst.OwnerStatementOriginalEntry.ReportReservation
-- Admission may use a proved upper bound. Only the actual produced row
-- count advances used. This is not foreign_exact with the upper bound.
theorem bounded_foreign_preserves (limit upper actual : Nat) (s : State)
 (ready : Available limit upper s) (within : actual ≤ upper) :
 foreign limit actual s = some ⟨s.used+actual,s.owed⟩ ∧
 Fits limit ⟨s.used+actual,s.owed⟩ := by
 have bound := (available_exact limit upper s).mp ready
 have enough : s.used+actual+s.owed ≤ limit := by omega
 exact ⟨by simp [foreign,enough],(fits_exact limit _).mpr enough⟩

-- A physical shared pool must account for the sum of all lane costs.
theorem aliased_foreign_preserves (limit left right : Nat) (s : State)
 (ready : Available limit (left+right) s) :
 Fits limit ⟨s.used+left+right,s.owed⟩ := by
 apply (fits_exact limit _).mpr
 have bound := (available_exact limit (left+right) s).mp ready
 simp only
 omega

def reportOwed (bound actual projected : Nat) (terminal : Bool) : Nat :=
 (if terminal then actual else max actual bound) - projected

theorem pre_lower_owed (bound : Nat) : reportOwed bound 0 0 false = bound := by
 simp [reportOwed]

theorem actual_debt_retained (bound actual projected : Nat) (terminal : Bool) :
 actual-projected ≤ reportOwed bound actual projected terminal := by
 cases terminal <;> simp [reportOwed] <;> omega

theorem terminal_release_keeps_actual (bound actual projected : Nat) :
 reportOwed bound actual projected true = actual-projected ∧
 reportOwed bound actual projected true ≤ reportOwed bound actual projected false := by
 constructor
 · simp [reportOwed]
 · exact actual_debt_retained bound actual projected false

theorem bounded_lower_growth (bound before after projected : Nat)
 (oldBound : before ≤ bound) (newBound : after ≤ bound) :
 reportOwed bound before projected false = reportOwed bound after projected false := by
 simp only [reportOwed,Bool.false_eq_true,↓reduceIte]
 omega

theorem projection_discharges_actual (bound actual projected count : Nat) (terminal : Bool)
 (actualRows : projected+count ≤ actual) :
 reportOwed bound actual projected terminal =
 count + reportOwed bound actual (projected+count) terminal := by
 cases terminal <;> simp only [reportOwed,Bool.false_eq_true,↓reduceIte] <;> omega

-- The resource index denotes physical ownership. Mapping two aliased lanes
-- to different indexes is not established by this pointwise theorem.
theorem all_pools_bounded {Pool : Type} (limit upper actual : Pool → Nat)
 (state : Pool → State) (ready : ∀ p, Available (limit p) (upper p) (state p))
 (within : ∀ p, actual p ≤ upper p) :
 ∀ p, Fits (limit p) ⟨(state p).used+actual p,(state p).owed⟩ := by
 intro p
 exact (bounded_foreign_preserves (limit p) (upper p) (actual p) (state p) (ready p) (within p)).2

#print axioms bounded_foreign_preserves
#print axioms aliased_foreign_preserves
#print axioms pre_lower_owed
#print axioms actual_debt_retained
#print axioms terminal_release_keeps_actual
#print axioms bounded_lower_growth
#print axioms projection_discharges_actual
#print axioms all_pools_bounded
end MirroreaProofFirst.OwnerStatementOriginalEntry.ReportReservation

namespace MirroreaProofFirst.OwnerStatementOriginalEntry.EndpointBudget
-- The list contains costs of remaining genuine original transitions. Prefix
-- counts actual foreign inbox heads that must be removed before the original.
-- Neither is a trace: no future request, write or receipt is asserted to exist.
def Ready (limit used ahead : Nat) (work : List Nat) : Prop :=
 ∃ spare, used + work.sum + ahead + spare = limit

def admits (limit used cost addedPrefix ahead : Nat) (work : List Nat) : Bool :=
 decide (used + cost + work.sum + ahead + addedPrefix ≤ limit)

-- Independent available-space witness, including newly incurred FIFO debt.
theorem admits_exact (limit used cost addedPrefix ahead : Nat) (work : List Nat) :
 admits limit used cost addedPrefix ahead work = true ↔
 ∃ spare, used + work.sum + ahead + spare = limit ∧ cost+addedPrefix ≤ spare := by
 simp only [admits,decide_eq_true_eq]
 constructor
 · intro enough
   exact ⟨limit-(used+work.sum+ahead),by omega,by omega⟩
 · rintro ⟨spare,balance,enough⟩;omega

theorem ready_exact (limit used ahead : Nat) (work : List Nat) :
 Ready limit used ahead work ↔ used+work.sum+ahead ≤ limit := by
 constructor
 · rintro ⟨spare,balance⟩;omega
 · intro enough;exact ⟨limit-(used+work.sum+ahead),by omega⟩

-- Removing exactly one original transition charges its actual cost once.
-- The before/after decomposition must come from that actual trusted phase.
theorem original_piece_preserves (limit used ahead cost : Nat) (before after : List Nat)
 (ready : Ready limit used ahead (before ++ cost::after)) :
 Ready limit (used+cost) ahead (before++after) := by
 apply (ready_exact limit (used+cost) ahead (before++after)).mpr
 have bound := (ready_exact limit used ahead (before++cost::after)).mp ready
 simp only [List.sum_append,List.sum_cons] at *
 omega

theorem original_piece_conserves (used ahead cost : Nat) (before after : List Nat) :
 used+(before++cost::after).sum+ahead = (used+cost)+(before++after).sum+ahead := by
 simp only [List.sum_append,List.sum_cons]
 omega

-- A genuine FIFO predecessor consumes its reserved dequeue slot. It is not
-- charged as an unrelated allocation in addition to that same obligation.
theorem predecessor_preserves (limit used ahead : Nat) (work : List Nat)
 (ready : Ready limit used (ahead+1) work) : Ready limit (used+1) ahead work := by
 apply (ready_exact limit (used+1) ahead work).mpr
 have bound := (ready_exact limit used (ahead+1) work).mp ready
 omega

theorem append_before_preserves (limit used cost ahead : Nat) (work : List Nat)
 (accepted : admits limit used cost 1 ahead work = true) :
 Ready limit (used+cost) (ahead+1) work := by
 apply (ready_exact limit (used+cost) (ahead+1) work).mpr
 simp only [admits,decide_eq_true_eq] at accepted
 omega

theorem append_behind_preserves (limit used cost ahead : Nat) (work : List Nat)
 (accepted : admits limit used cost 0 ahead work = true) :
 Ready limit (used+cost) ahead work := by
 apply (ready_exact limit (used+cost) ahead work).mpr
 simpa [admits] using accepted

theorem rejects_spending_original_tail (limit used cost ahead : Nat) (work : List Nat)
 (notEnough : limit < used+cost+work.sum+ahead) :
 admits limit used cost 0 ahead work = false := by
 simp [admits,show ¬used+cost+work.sum+ahead ≤ limit by omega]

def remote : List Nat := [4,3,1,4,3,1,1]
def localWork : List Nat := [1]
-- Cost correspondence is verified separately at actual Rust producer calls:
-- request enqueue/move/dequeue, reply enqueue/move/dequeue, and acceptance.
#print axioms admits_exact
#print axioms ready_exact
#print axioms original_piece_preserves
#print axioms original_piece_conserves
#print axioms predecessor_preserves
#print axioms append_before_preserves
#print axioms append_behind_preserves
#print axioms rejects_spending_original_tail
end MirroreaProofFirst.OwnerStatementOriginalEntry.EndpointBudget

namespace MirroreaProofFirst.OwnerStatementOriginalEntry.FiniteScan
-- Eligibility is independent of the executable scanner. For the Rust
-- outbox it is the shared endpoint capacity predicate on an actual envelope.
-- The list is the finite flattening of retained outboxes, never inbox items.
def scan {α : Type} (check : α → Bool) : List α → Option α
 | [] => none
 | a::rest => if check a then some a else scan check rest

def examined {α : Type} (check : α → Bool) : List α → Nat
 | [] => 0
 | a::rest => if check a then 1 else 1 + examined check rest

theorem examined_bound {α : Type} (check : α → Bool) (items : List α) :
 examined check items ≤ items.length := by
 induction items with
 | nil => simp [examined]
 | cons a rest ih =>
   simp only [examined,List.length_cons]
   split <;> omega

theorem none_exact {α : Type} (eligible : α → Prop) (check : α → Bool)
 (exact : ∀ a, check a = true ↔ eligible a) (items : List α) :
 scan check items = none ↔ ∀ a ∈ items, ¬eligible a := by
 induction items with
 | nil => simp [scan]
 | cons a rest ih =>
   by_cases allowed : check a = true
   · have ha := (exact a).mp allowed
     simp [scan,allowed,ha]
   · have ha : ¬eligible a := fun h => allowed ((exact a).mpr h)
     simp [scan,allowed,ih,ha]

theorem selects_actual_eligible {α : Type} (eligible : α → Prop) (check : α → Bool)
 (exact : ∀ a, check a = true ↔ eligible a) (items : List α) (chosen : α)
 (selected : scan check items = some chosen) : chosen ∈ items ∧ eligible chosen := by
 induction items with
 | nil => simp [scan] at selected
 | cons a rest ih =>
   by_cases allowed : check a = true
   · have eq : a = chosen := by simpa [scan,allowed] using selected
     subst chosen
     exact ⟨by simp,(exact a).mp allowed⟩
   · have selectedRest : scan check rest = some chosen := by simpa [scan,allowed] using selected
     have found := ih selectedRest
     exact ⟨by simp [found.1],found.2⟩

theorem enabled_is_considered {α : Type} (eligible : α → Prop) (check : α → Bool)
 (exact : ∀ a, check a = true ↔ eligible a) (items : List α)
 (enabled : ∃ a ∈ items, eligible a) :
 ∃ chosen, scan check items = some chosen ∧ chosen ∈ items ∧ eligible chosen := by
 cases h : scan check items with
 | none =>
   have disabled := (none_exact eligible check exact items).mp h
   obtain ⟨a,member,ha⟩ := enabled
   exact False.elim (disabled a member ha)
 | some chosen => exact ⟨chosen,rfl,selects_actual_eligible eligible check exact items chosen h⟩

theorem blocked_prefix_does_not_hide {α : Type} (eligible : α → Prop) (check : α → Bool)
 (exact : ∀ a, check a = true ↔ eligible a) (blocked rest : List α)
 (disabled : ∀ a ∈ blocked, ¬eligible a) :
 scan check (blocked++rest) = scan check rest := by
 induction blocked with
 | nil => rfl
 | cons a tail ih =>
   have no : ¬check a = true := fun h => disabled a (by simp) ((exact a).mp h)
   simp only [List.cons_append,scan,if_neg no]
   apply ih
   intro x hx
   exact disabled x (by simp [hx])

-- A caller falls back to the first actual inbox head only after every
-- outgoing candidate is blocked. This proves availability of selection,
-- not success of that head's service, 32-pass completion, or network fairness.
def withFallback {α β : Type} (check : α → Bool) (outgoing : List α)
 (inboxHead : Option β) : Option (Sum α β) :=
 match scan check outgoing with
 | some a => some (.inl a)
 | none => inboxHead.map Sum.inr

theorem all_blocked_reaches_inbox {α β : Type} (eligible : α → Prop) (check : α → Bool)
 (exact : ∀ a, check a = true ↔ eligible a) (outgoing : List α)
 (disabled : ∀ a ∈ outgoing, ¬eligible a) (head : β) :
 withFallback check outgoing (some head) = some (.inr head) := by
 simp [withFallback,(none_exact eligible check exact outgoing).mpr disabled]

#print axioms examined_bound
#print axioms none_exact
#print axioms selects_actual_eligible
#print axioms enabled_is_considered
#print axioms blocked_prefix_does_not_hide
#print axioms all_blocked_reaches_inbox
end MirroreaProofFirst.OwnerStatementOriginalEntry.FiniteScan
