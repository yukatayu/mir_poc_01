import MirroreaProofFirstSourceFundingQuery

namespace MirroreaProofFirst.SourceFundingFrame
open OwnerCodecTree OwnerResponseProfile

-- These are bounds on the complete private carrier, not a claim that the
-- old packet's full framing budget can be reused without leaving headroom.
theorem credit_fits (value : Fin 513) : Fits (finite 513) textLimit 12 1 value := by
  have bound := fits_natural (n:=value.val) (digits:=10) (textLimit:=textLimit)
    (by have := value.isLt; omega) (by decide)
  exact bound

theorem credit_fields (values : List (Fin 513)) :
    (OwnerTreeBytes.writeTrees (values.map (finite 513).encode)).length ≤ 12*values.length ∧
    (∀ tail, OwnerPayload.digitCheck 128 0
      (OwnerTreeBytes.writeTrees (values.map (finite 513).encode) ++ tail) = OwnerPayload.digitCheck 128 0 tail) ∧
    (∀ tail, OwnerPayload.textCheck textLimit
      (OwnerTreeBytes.writeTrees (values.map (finite 513).encode) ++ tail) = OwnerPayload.textCheck textLimit tail) ∧
    OwnerTreeBytes.treesCost (values.map (finite 513).encode) ≤ values.length+1 := by
  induction values with
  | nil => simp [OwnerTreeBytes.writeTrees,OwnerTreeBytes.treesCost]
  | cons value rest ih =>
    have fit := credit_fits (textLimit:=textLimit) value
    change _ ≤ 12*(rest.length+1) ∧ _ ∧ _ ∧ _
    refine ⟨?_,?_,?_,?_⟩
    · simp only [List.map_cons,OwnerTreeBytes.writeTrees,List.length_append]
      change (OwnerPacketCodec.encode (finite 513) value).length + _ ≤ _
      have := fit.1; have := ih.1; omega
    · intro tail
      simp only [List.map_cons,OwnerTreeBytes.writeTrees,List.append_assoc]
      change OwnerPayload.digitCheck 128 0 (OwnerPacketCodec.encode (finite 513) value ++ _) = _
      rw [encoded_digits_append fit.2.1,ih.2.1]
    · intro tail
      simp only [List.map_cons,OwnerTreeBytes.writeTrees,List.append_assoc]
      change OwnerPayload.textCheck textLimit (OwnerPacketCodec.encode (finite 513) value ++ _) = _
      rw [text_append fit.2.2.1,ih.2.2.1]
    · have := ih.2.2.2
      simp only [List.map_cons,finite,OwnerTreeBytes.treesCost,OwnerTreeBytes.treeCost,List.length_cons] at *
      omega

theorem vector_fits (value : Vector (Fin 513) p) (small : p ≤ 64) :
    Fits (vector (finite 513) p) textLimit (12*p+10) (p+2) value := by
  let values := value.toArray.toList
  have size : values.length = p := by simp [values]
  have fields := credit_fields (textLimit:=textLimit) values
  have count : values.length < 2^7 := by omega
  have countBytes := nat_bytes_length count
  have countDigits := nat_digits_suffix count (count:=0) (limit:=128)
    (tail:=OwnerTreeBytes.writeTrees (values.map (finite 513).encode)) (by decide)
  have countText := nat_text_suffix values.length textLimit
    (OwnerTreeBytes.writeTrees (values.map (finite 513).encode))
  have shape : OwnerPacketCodec.encode (vector (finite 513) p) value =
      7 :: (OwnerByteCodec.writeNat 0 ++ OwnerByteCodec.writeNat values.length ++
        OwnerTreeBytes.writeTrees (values.map (finite 513).encode)) := by
    simp only [OwnerPacketCodec.encode,vector,list,OwnerTreeBytes.encode,OwnerTreeBytes.writeTree,List.length_map,values]
  have costShape : OwnerTreeBytes.treeCost ((vector (finite 513) p).encode value) =
      1 + OwnerTreeBytes.treesCost (values.map (finite 513).encode) := by
    simp only [vector,list,OwnerTreeBytes.treeCost,values]
  refine ⟨?_,?_,?_,?_⟩
  · rw [shape]
    change (7 :: (OwnerByteCodec.writeNat 0 ++ OwnerByteCodec.writeNat values.length ++
        OwnerTreeBytes.writeTrees (values.map (finite 513).encode))).length ≤ _
    rw [show OwnerByteCodec.writeNat 0 = [0] by simp [OwnerByteCodec.writeNat]]
    simp only [List.length_cons,List.length_nil,List.length_append]
    have := fields.1
    omega
  · rw [shape]
    change OwnerPayload.digitCheck 128 0
      (7 :: (OwnerByteCodec.writeNat 0 ++ OwnerByteCodec.writeNat values.length ++
        OwnerTreeBytes.writeTrees (values.map (finite 513).encode))) = true
    rw [show OwnerByteCodec.writeNat 0 = [0] by simp [OwnerByteCodec.writeNat]]
    simp only [List.cons_append,List.nil_append,OwnerPayload.digitCheck,OwnerPayload.nextDigits]
    simpa [OwnerPayload.digitCheck] using countDigits.trans (by simpa using fields.2.1 [])
  · rw [shape]
    change OwnerPayload.textCheck textLimit
      (7 :: (OwnerByteCodec.writeNat 0 ++ OwnerByteCodec.writeNat values.length ++
        OwnerTreeBytes.writeTrees (values.map (finite 513).encode))) = true
    rw [show OwnerByteCodec.writeNat 0 = [0] by simp [OwnerByteCodec.writeNat]]
    simp only [List.cons_append,List.nil_append,OwnerPayload.textCheck,OwnerPayload.textHeadCheck]
    simpa [OwnerPayload.textCheck] using countText.trans (by simpa using fields.2.2.1 [])
  · rw [costShape]
    change 1 + OwnerTreeBytes.treesCost (values.map (finite 513).encode) ≤ p+2
    have := fields.2.2.2
    omega

-- A product's decoder depth is bounded by the maximum of its branches plus
-- three; the previous additive sufficient lemma is deliberately not assumed
-- tight. This matters for a 64-owner vector alongside a nontrivial input.
theorem fits_product_max (first : Fits left textLimit b₁ f₁ x)
    (second : Fits right textLimit b₂ f₂ y) :
    Fits (product left right) textLimit (b₁+b₂+5) (max f₁ f₂+3) (x,y) := by
  have basic := fits_product first second
  refine ⟨basic.1,basic.2.1,basic.2.2.1,?_⟩
  have := first.2.2.2; have := second.2.2.2
  simp only [product,OwnerTreeBytes.treeCost,OwnerTreeBytes.treesCost]
  omega

theorem wrapped_fits (small : p ≤ 64) (vectorValue : Vector (Fin 513) p)
    (base : Fits (PublicationInput.input p a) textLimit bytes fuel command) :
    Fits (SourceFundingInput.request p a) textLimit (bytes+12*p+15)
      (max (p+2) fuel+3) (vectorValue,command) := by
  have wrapped := fits_product_max (vector_fits (textLimit:=textLimit) vectorValue small) base
  have arithmetic : 12*p+10+bytes+5 = bytes+12*p+15 := by omega
  simpa only [SourceFundingInput.request,arithmetic] using wrapped

theorem wrapped_readable (small : p ≤ 64) (vectorValue : Vector (Fin 513) p)
    (base : Fits (PublicationInput.input p a) 4096 bytes fuel command)
    (byteSpace : bytes+12*p+15 ≤ 65536) (fuelSpace : max (p+2) fuel+3 ≤ 256) :
    SourceCodec.compactFits (OwnerPacketCodec.encode (SourceFundingInput.request p a) (vectorValue,command)) = true ∧
    OwnerPacketCodec.decodeAt (SourceFundingInput.request p a) 256
      (OwnerPacketCodec.encode (SourceFundingInput.request p a) (vectorValue,command)) = some (vectorValue,command) := by
  have fit := wrapped_fits small vectorValue base
  have readable := fits_wire fit byteSpace fuelSpace
  refine ⟨?_,readable.2.2.2⟩
  simp [SourceCodec.compactFits,readable.1,fit.2.1,fit.2.2.1]

#print axioms credit_fits
#print axioms credit_fields
#print axioms vector_fits
#print axioms fits_product_max
#print axioms wrapped_fits
#print axioms wrapped_readable

-- Independent resource profile on the retained pre-reservation ticket. It
-- includes explicit carrier headroom; a Boolean checker is proved equivalent.
-- This is a sufficient finite private profile, not all-parser completeness.
def CarrierResponseFits (p scopeId revision : Nat) (ticket : InvocationBoundary.Ticket) : Prop :=
  OwnerResponseProfile.ResponseFits scopeId revision ticket ∧ p ≤ 64 ∧
    (OwnerPacketCodec.encode OwnerFullCodec.ticket ticket).length+372+12*p+15 ≤ 65536 ∧
    max (p+2) (OwnerTreeBytes.treeCost (OwnerFullCodec.ticket.encode ticket)+24)+3 ≤ 256

def responseCheck (p scopeId revision : Nat) (ticket : InvocationBoundary.Ticket) : Bool :=
  OwnerResponseProfile.responseCheck scopeId revision ticket &&
    decide (p ≤ 64 ∧
      (OwnerPacketCodec.encode OwnerFullCodec.ticket ticket).length+372+12*p+15 ≤ 65536 ∧
      max (p+2) (OwnerTreeBytes.treeCost (OwnerFullCodec.ticket.encode ticket)+24)+3 ≤ 256)

theorem response_exact : responseCheck p scopeId revision ticket = true ↔
    CarrierResponseFits p scopeId revision ticket := by
  simp [responseCheck,CarrierResponseFits,OwnerResponseProfile.responseCheck_exact]

theorem receipt_arrival_fits (profile : CarrierResponseFits p scopeId revision ticket)
    (ordinalSmall : ordinal < 64)
    (range : InstancePrograms.Machine.lo ≤ value ∧ value ≤ InstancePrograms.Machine.hi)
    (vectorValue : Vector (Fin 513) p) :
    WireFits (SourceFundingInput.request p a) 256
      (vectorValue,.step (.arrival ⟨scopeId,ordinal,revision,ticket,.value value⟩)) := by
  have old := profile.1
  have envelope := fits_envelope old.1 old.2.1 ordinalSmall (response_ticket_fits old) range
  have commandWrapped : Fits (PublicationInput.command p a) 256
      ((OwnerPacketCodec.encode OwnerFullCodec.ticket ticket).length+358+4+5)
      (OwnerTreeBytes.treeCost (OwnerFullCodec.ticket.encode ticket)+18+2+2)
      (.arrival ⟨scopeId,ordinal,revision,ticket,.value value⟩) :=
    fits_inr (fits_inl envelope _) (SourceInput.command p a)
  have base : Fits (PublicationInput.input p a) 256
      ((OwnerPacketCodec.encode OwnerFullCodec.ticket ticket).length+372)
      (OwnerTreeBytes.treeCost (OwnerFullCodec.ticket.encode ticket)+24)
      (.step (.arrival ⟨scopeId,ordinal,revision,ticket,.value value⟩)) :=
    fits_inr commandWrapped (SourceCodec.compact p)
  exact fits_wire (wrapped_fits profile.2.1 vectorValue base) profile.2.2.1 profile.2.2.2

-- Actual model computation, not an assumed produced value: reserve progress
-- plus the independent ticket/profile premises implies a complete readable
-- funded arrival, with the same record as the actual compute transition.
theorem rooted_computation_arrival
    (path : OwnerReservation.Steps (OwnerReservation.initial assigned scope image capacity) state)
    (small : capacity ≤ 64)
    (profile : CarrierResponseFits p state.core.scopeId state.core.revision ticket)
    (reserved : OwnerReservation.reserve state requestedScope ticket = (next,.reserved))
    (vectorValue : Vector (Fin 513) p) :
    ∃ later record, OwnerReservation.compute next = some (later,.produced record) ∧
      WireFits (SourceFundingInput.request p a) 256
        (vectorValue,.step (.arrival (OwnerReceipt.project record))) := by
  obtain ⟨later,record,computed⟩ := OwnerReservation.rooted_reserved_computes path reserved
  obtain ⟨value,result,range⟩ := computed_result_range computed
  have nextPath := OwnerReservation.Steps.next path (.reservation reserved)
  have ordinal := rooted_computed_ordinal nextPath small computed
  obtain ⟨activeTicket,core,active,submitted,_⟩ := OwnerReservation.compute_parts computed
  have activeAt : next.active = some ticket := by rw [(OwnerReservation.reserve_parts reserved).2.2.2.2.2]
  have same : activeTicket = ticket := Option.some.inj (active.symm.trans activeAt)
  subst activeTicket
  have recordAt := (OwnerOccurrence.produced_parts submitted).2.2.2.2.1
  have coreAt := OwnerReservation.reserve_core reserved
  have scopeAt : record.scopeId = state.core.scopeId := by rw [recordAt,coreAt]
  have revisionAt : record.revision = state.core.revision := by rw [recordAt,coreAt]
  have ticketAt : record.ticket = ticket := by rw [recordAt]
  have projected : OwnerReceipt.project record =
      ⟨state.core.scopeId,record.ordinal,state.core.revision,ticket,.value value⟩ := by
    simp only [OwnerReceipt.project,scopeAt,revisionAt,ticketAt,result]
  refine ⟨later,record,computed,?_⟩
  rw [projected]
  exact receipt_arrival_fits profile ordinal range vectorValue

-- The unimplemented query successor has a distinct whole-message budget.
-- No old arrival proof is silently reused after adding its Sum constructor.
theorem query_input_fits (small : p ≤ 64) (owner : Fin p) :
    Fits (SourceFundingQuery.input p a) 4096 13 3 (.inl owner) := by
  have finFit : Fits (finite p) 4096 9 1 owner :=
    fits_natural (digits:=7) (by have := owner.isLt; omega) (by decide)
  exact fits_inl finFit (SourceFundingInput.request p a)

theorem query_execution_fits (small : p ≤ 64) (vectorValue : Vector (Fin 513) p)
    (base : Fits (PublicationInput.input p a) textLimit bytes fuel command) :
    Fits (SourceFundingQuery.input p a) textLimit (bytes+12*p+20)
      (max (p+2) fuel+5) (.inr (vectorValue,command)) := by
  have fit := fits_inr (wrapped_fits small vectorValue base) (finite p)
  simpa only [SourceFundingQuery.input,Nat.add_assoc] using fit

#print axioms response_exact
#print axioms receipt_arrival_fits
#print axioms rooted_computation_arrival
#print axioms query_input_fits
#print axioms query_execution_fits


def HeadSmall : SourceFundingQuery.HeadRequest p → Prop
  | .freeze _ revision | .install _ revision => revision < 2^128
  | .finish _ => True

theorem head_input_fits (small : p ≤ 64) (request : SourceFundingQuery.HeadRequest p)
    (bounded : HeadSmall request) :
    Fits (SourceFundingQuery.checkedInput p a) 4096 157 11 (.inl request) := by
  have finiteFit (owner : Fin p) : Fits (finite p) 4096 9 1 owner :=
    fits_natural (digits:=7) (by have := owner.isLt; omega) (by decide)
  have headFit : Fits (SourceFundingQuery.headRequest p) 4096 153 9 request := by
    cases request with
    | freeze owner revision =>
      have revisionFit := fits_natural bounded (by decide : 128 ≤ 128) (textLimit:=4096)
      have first := fits_inl (fits_product (finiteFit owner) revisionFit)
        (sum (product (finite p) natural) (finite p))
      refine ⟨?_,first.2.1,first.2.2.1,?_⟩
      · have := first.1; change (OwnerPacketCodec.encode (sum (product (finite p) natural) (sum (product (finite p) natural) (finite p))) (.inl (owner,revision))).length ≤ 153; omega
      · have := first.2.2.2; change OwnerTreeBytes.treeCost ((sum (product (finite p) natural) (sum (product (finite p) natural) (finite p))).encode (.inl (owner,revision))) ≤ 9; omega
    | install owner revision =>
      have revisionFit := fits_natural bounded (by decide : 128 ≤ 128) (textLimit:=4096)
      exact fits_inr (fits_inl (fits_product (finiteFit owner) revisionFit) (finite p)) (product (finite p) natural)
    | finish owner =>
      have first := fits_inr (fits_inr (finiteFit owner) (product (finite p) natural)) (product (finite p) natural)
      refine ⟨?_,first.2.1,first.2.2.1,?_⟩
      · have := first.1; change (OwnerPacketCodec.encode (sum (product (finite p) natural) (sum (product (finite p) natural) (finite p))) (.inr (.inr owner))).length ≤ 153; omega
      · have := first.2.2.2; change OwnerTreeBytes.treeCost ((sum (product (finite p) natural) (sum (product (finite p) natural) (finite p))).encode (.inr (.inr owner))) ≤ 9; omega
  exact fits_inl headFit (SourceFundingQuery.input p a)

theorem checked_amount_input_fits (small : p ≤ 64) (owner : Fin p) :
    Fits (SourceFundingQuery.checkedInput p a) 4096 18 5 (.inr (.inl owner)) :=
  fits_inr (query_input_fits small owner) (SourceFundingQuery.headRequest p)

theorem checked_execution_fits (small : p ≤ 64) (vectorValue : Vector (Fin 513) p)
    (base : Fits (PublicationInput.input p a) textLimit bytes fuel command) :
    Fits (SourceFundingQuery.checkedInput p a) textLimit (bytes+12*p+25)
      (max (p+2) fuel+7) (.inr (.inr (vectorValue,command))) := by
  have fit := fits_inr (query_execution_fits small vectorValue base) (SourceFundingQuery.headRequest p)
  simpa only [SourceFundingQuery.checkedInput,Nat.add_assoc] using fit

#print axioms head_input_fits
#print axioms checked_amount_input_fits
#print axioms checked_execution_fits

end MirroreaProofFirst.SourceFundingFrame
