import OwnerEndpointCodec
import MirroreaProofFirstPublicationInput

namespace MirroreaProofFirst.OwnerResponseProfile

-- The accepted W3 machine already checks every intermediate arithmetic node.
-- Its successful result is Int64-bounded; no second owner evaluation is needed
-- to establish this range, and no proof is a produced-event witness.
theorem machine_result_range
    (executed : InstancePrograms.Machine.Executes code input value) :
    InstancePrograms.Machine.lo ≤ value ∧ value ≤ InstancePrograms.Machine.hi :=
  (ContractExport.CheckedArithmetic.denotes_math executed.2).2.2

theorem produced_result_range
    (produced : OwnerOccurrence.submit owner scopeId ticket = (next,.produced record)) :
    ∃ value, record.result = .value value ∧
      InstancePrograms.Machine.lo ≤ value ∧ value ≤ InstancePrograms.Machine.hi := by
  obtain ⟨value,result,_,executed⟩ := OwnerOccurrence.produced_meaning produced
  exact ⟨value,result,machine_result_range executed⟩

theorem computed_result_range
    (computed : OwnerReservation.compute owner = some (next,.produced record)) :
    ∃ value, record.result = .value value ∧
      InstancePrograms.Machine.lo ≤ value ∧ value ≤ InstancePrograms.Machine.hi := by
  obtain ⟨_,_,_,produced,_⟩ := OwnerReservation.compute_parts computed
  exact produced_result_range produced

theorem submitted_capacity
    (ran : OwnerOccurrence.submit state scope ticket = (next,reply)) :
    next.capacity = state.capacity := by
  unfold OwnerOccurrence.submit at ran
  split at ran
  · cases ran; rfl
  · split at ran
    · split at ran <;> cases ran <;> rfl
    · split at ran <;> cases ran <;> rfl

theorem reservation_step_capacity (step : OwnerReservation.Step state next) :
    next.core.capacity = state.core.capacity := by
  cases step with
  | reservation ran => rw [OwnerReservation.reserve_core ran]
  | computation ran =>
      obtain ⟨_,_,_,submitted,rfl⟩ := OwnerReservation.compute_parts ran
      exact submitted_capacity submitted
  | installation ran => rw [(OwnerReservation.install_idle ran).2]; rfl
  | abandonment => rfl

theorem rooted_capacity
    (path : OwnerReservation.Steps (OwnerReservation.initial assigned scope image capacity) state) :
    state.core.capacity = capacity := by
  induction path with
  | refl => rfl
  | next _ step ih => rw [reservation_step_capacity step]; exact ih

theorem produced_ordinal_bounded
    (produced : OwnerOccurrence.submit state scope ticket = (next,.produced record))
    (capacity : state.capacity ≤ 64) : record.ordinal < 64 := by
  obtain ⟨_,_,_,room,rfl,_⟩ := OwnerOccurrence.produced_parts produced
  exact Nat.lt_of_lt_of_le room capacity

theorem rooted_computed_ordinal
    (path : OwnerReservation.Steps (OwnerReservation.initial assigned scope image capacity) state)
    (small : capacity ≤ 64)
    (computed : OwnerReservation.compute state = some (next,.produced record)) :
    record.ordinal < 64 := by
  obtain ⟨_,_,_,produced,_⟩ := OwnerReservation.compute_parts computed
  exact produced_ordinal_bounded produced (by simpa [rooted_capacity path] using small)

#print axioms submitted_capacity
#print axioms reservation_step_capacity
#print axioms rooted_capacity
#print axioms produced_ordinal_bounded
#print axioms rooted_computed_ordinal

-- General number-encoding lemmas, not fixed-example decision procedures.
theorem nat_bytes_length (small : n < 2^digits) :
    (OwnerByteCodec.writeNat n).length ≤ digits+1 := by
  induction digits generalizing n with
  | zero =>
    have zero : n = 0 := by simpa using small
    simp [zero,OwnerByteCodec.writeNat]
  | succ digits ih =>
    by_cases zero : n = 0
    · simp [zero,OwnerByteCodec.writeNat]
    · have half : n/2 < 2^digits := by
        rw [Nat.pow_succ] at small
        omega
      have bound := ih half
      rw [OwnerByteCodec.writeNat]
      simp only [zero,ite_false]
      split <;> simp only [List.length_cons] <;> omega

theorem nat_digits_suffix (small : n < 2^digits) (space : count+digits ≤ limit) :
    OwnerPayload.digitCheck limit count (OwnerByteCodec.writeNat n ++ tail) =
      OwnerPayload.digitCheck limit 0 tail := by
  induction digits generalizing n count with
  | zero =>
    have zero : n = 0 := by simpa using small
    have countFits : count ≤ limit := by simpa using space
    simp [zero,OwnerByteCodec.writeNat,OwnerPayload.digitCheck,OwnerPayload.nextDigits,countFits]
  | succ digits ih =>
    have countFits : count ≤ limit := by omega
    by_cases zero : n = 0
    · simp [zero,OwnerByteCodec.writeNat,OwnerPayload.digitCheck,OwnerPayload.nextDigits,countFits]
    · have half : n/2 < 2^digits := by rw [Nat.pow_succ] at small; omega
      have room : count+1+digits ≤ limit := by omega
      have rest := ih half room
      rw [OwnerByteCodec.writeNat]
      simp only [zero,ite_false]
      split <;> simpa [OwnerPayload.digitCheck,OwnerPayload.nextDigits,countFits] using rest

theorem nat_text_suffix (n limit : Nat) (tail : List UInt8) :
    OwnerPayload.textCheck limit (OwnerByteCodec.writeNat n ++ tail) =
      OwnerPayload.textCheck limit tail := by
  induction n using Nat.strongRecOn with
  | ind n ih =>
    by_cases zero : n = 0
    · simp [zero,OwnerByteCodec.writeNat,OwnerPayload.textCheck,OwnerPayload.textHeadCheck]
    · have half : n/2 < n := Nat.div_lt_self (Nat.pos_of_ne_zero zero) (by decide)
      have rest := ih (n/2) half
      rw [OwnerByteCodec.writeNat]
      simp only [zero,ite_false]
      split <;> simpa [OwnerPayload.textCheck,OwnerPayload.textHeadCheck] using rest

def magnitude : Int → Nat
  | .ofNat n => n
  | .negSucc n => n

theorem int64_magnitude (range : InstancePrograms.Machine.lo ≤ value ∧ value ≤ InstancePrograms.Machine.hi) :
    magnitude value < 2^63 := by
  cases value <;> simp only [magnitude,InstancePrograms.Machine.lo,InstancePrograms.Machine.hi,Int.ofNat_eq_natCast] at * <;> omega

theorem int64_bytes (range : InstancePrograms.Machine.lo ≤ value ∧ value ≤ InstancePrograms.Machine.hi) :
    (OwnerPacketCodec.encode OwnerCodecTree.integer value).length ≤ 65 := by
  have bound := nat_bytes_length (int64_magnitude range)
  cases value with
  | ofNat n =>
      simp only [OwnerPacketCodec.encode,OwnerCodecTree.integer,OwnerTreeBytes.encode,OwnerTreeBytes.writeTree]
      change (4 :: OwnerByteCodec.writeNat n).length ≤ 65
      change (OwnerByteCodec.writeNat n).length ≤ 63+1 at bound
      simp only [List.length_cons]
      omega
  | negSucc n =>
      simp only [OwnerPacketCodec.encode,OwnerCodecTree.integer,OwnerTreeBytes.encode,OwnerTreeBytes.writeTree]
      change (5 :: OwnerByteCodec.writeNat n).length ≤ 65
      change (OwnerByteCodec.writeNat n).length ≤ 63+1 at bound
      simp only [List.length_cons]
      omega

theorem int64_digits_suffix (range : InstancePrograms.Machine.lo ≤ value ∧ value ≤ InstancePrograms.Machine.hi)
    (tail : List UInt8) :
    OwnerPayload.digitCheck 128 0 (OwnerPacketCodec.encode OwnerCodecTree.integer value ++ tail) =
      OwnerPayload.digitCheck 128 0 tail := by
  have bound := nat_digits_suffix (int64_magnitude range) (count:=0) (limit:=128) (tail:=tail) (by decide)
  cases value with
  | ofNat n =>
      simp only [OwnerPacketCodec.encode,OwnerCodecTree.integer,OwnerTreeBytes.encode,OwnerTreeBytes.writeTree]
      change OwnerPayload.digitCheck 128 0 ((4 :: OwnerByteCodec.writeNat n) ++ tail) = _
      simpa [OwnerPayload.digitCheck,OwnerPayload.nextDigits,magnitude] using bound
  | negSucc n =>
      simp only [OwnerPacketCodec.encode,OwnerCodecTree.integer,OwnerTreeBytes.encode,OwnerTreeBytes.writeTree]
      change OwnerPayload.digitCheck 128 0 ((5 :: OwnerByteCodec.writeNat n) ++ tail) = _
      simpa [OwnerPayload.digitCheck,OwnerPayload.nextDigits,magnitude] using bound

theorem integer_text_suffix (value : Int) (limit : Nat) (tail : List UInt8) :
    OwnerPayload.textCheck limit (OwnerPacketCodec.encode OwnerCodecTree.integer value ++ tail) =
      OwnerPayload.textCheck limit tail := by
  have bound := nat_text_suffix (magnitude value) limit tail
  cases value with
  | ofNat n =>
      simp only [OwnerPacketCodec.encode,OwnerCodecTree.integer,OwnerTreeBytes.encode,OwnerTreeBytes.writeTree]
      change OwnerPayload.textCheck limit ((4 :: OwnerByteCodec.writeNat n) ++ tail) = _
      simpa [OwnerPayload.textCheck,OwnerPayload.textHeadCheck,magnitude] using bound
  | negSucc n =>
      simp only [OwnerPacketCodec.encode,OwnerCodecTree.integer,OwnerTreeBytes.encode,OwnerTreeBytes.writeTree]
      change OwnerPayload.textCheck limit ((5 :: OwnerByteCodec.writeNat n) ++ tail) = _
      simpa [OwnerPayload.textCheck,OwnerPayload.textHeadCheck,magnitude] using bound

-- A bounded canonical reader returning any value must return this exact
-- encoded value. This general lemma closes isSome-versus-equality shortcuts.
theorem bounded_decode_unique (read : OwnerPacketCodec.decodeAt codec fuel
    (OwnerPacketCodec.encode codec value) = some other) : other = value := by
  have same := OwnerPacketCodec.bounded_canonical codec fuel _ other read
  have decoded := congrArg (OwnerPacketCodec.decode codec) same
  simpa only [OwnerPacketCodec.roundtrip,Option.some.injEq] using decoded.symm

def WireFits (codec : OwnerCodecTree.Codec α) (textLimit : Nat) (value : α) : Prop :=
  let bytes := OwnerPacketCodec.encode codec value
  bytes.length ≤ 65536 ∧ OwnerPayload.DigitBound 128 0 bytes ∧
    OwnerPayload.TextBound textLimit bytes ∧
    OwnerPacketCodec.decodeAt codec 256 bytes = some value

def wireCheck (codec : OwnerCodecTree.Codec α) (textLimit : Nat) (value : α) : Bool :=
  let bytes := OwnerPacketCodec.encode codec value
  decide (bytes.length ≤ 65536) && OwnerPayload.digitCheck 128 0 bytes &&
    OwnerPayload.textCheck textLimit bytes && (OwnerPacketCodec.decodeAt codec 256 bytes).isSome

theorem decode_some_exact (codec : OwnerCodecTree.Codec α) (fuel : Nat) (value : α) :
    (OwnerPacketCodec.decodeAt codec fuel (OwnerPacketCodec.encode codec value)).isSome = true ↔
      OwnerPacketCodec.decodeAt codec fuel (OwnerPacketCodec.encode codec value) = some value := by
  cases read : OwnerPacketCodec.decodeAt codec fuel (OwnerPacketCodec.encode codec value) with
  | none => simp
  | some other =>
      have equal := bounded_decode_unique read
      subst other
      simp

theorem wireCheck_exact : wireCheck codec textLimit value = true ↔ WireFits codec textLimit value := by
  simp [wireCheck,WireFits,OwnerPayload.digits_exact,OwnerPayload.text_exact,decode_some_exact,and_assoc]

theorem text_append (fits : OwnerPayload.textCheck limit front = true) :
    OwnerPayload.textCheck limit (front ++ suffix) = OwnerPayload.textCheck limit suffix := by
  induction front with
  | nil => rfl
  | cons byte rest ih =>
      have bound := (OwnerPayload.text_exact _ _).mp fits
      cases bound with
      | cons head remaining =>
          have restFits := (OwnerPayload.text_exact _ _).mpr remaining
          have headFits : OwnerPayload.textHeadCheck limit (byte :: (rest ++ suffix)) = true := by
            apply (OwnerPayload.text_head_exact _ _).mpr
            by_cases marker : byte = 6
            · simp only [OwnerPayload.TextHead,marker,ite_true] at head ⊢
              obtain ⟨count,tail,read,small⟩ := head
              refine ⟨count,tail ++ suffix,?_,small⟩
              rw [OwnerByteCodec.nat_canonical rest (count,tail) read,List.append_assoc]
              exact OwnerByteCodec.nat_roundtrip _ _
            · simp [OwnerPayload.TextHead,marker]
          simpa [OwnerPayload.textCheck,headFits] using ih restFits

theorem digits_append (fits : OwnerPayload.digitCheck limit count front = true) :
    OwnerPayload.digitCheck limit count (front ++ suffix) =
      OwnerPayload.digitCheck limit (front.foldl OwnerPayload.nextDigits count) suffix := by
  induction front generalizing count with
  | nil => rfl
  | cons byte rest ih =>
      have bound := (OwnerPayload.digits_exact _ _ _).mp fits
      cases bound with
      | cons small remaining =>
          have restFits := (OwnerPayload.digits_exact _ _ _).mpr remaining
          simpa [OwnerPayload.digitCheck,small] using ih restFits

theorem nat_resets (n count : Nat) :
    (OwnerByteCodec.writeNat n).foldl OwnerPayload.nextDigits count = 0 := by
  induction n using Nat.strongRecOn generalizing count with
  | ind n ih =>
      by_cases zero : n = 0
      · simp [zero,OwnerByteCodec.writeNat,OwnerPayload.nextDigits]
      · have half : n/2 < n := Nat.div_lt_self (Nat.pos_of_ne_zero zero) (by decide)
        rw [OwnerByteCodec.writeNat]
        simp only [zero,ite_false,List.foldl_cons]
        exact ih (n/2) half _

theorem nats_resets (ns : List Nat) (count : Nat) :
    (OwnerByteCodec.writeNats ns).foldl OwnerPayload.nextDigits count =
      if ns.isEmpty then count else 0 := by
  induction ns generalizing count with
  | nil => rfl
  | cons n rest ih =>
      simp only [OwnerByteCodec.writeNats,List.flatMap_cons,List.foldl_append,nat_resets]
      rw [show (rest.flatMap OwnerByteCodec.writeNat).foldl OwnerPayload.nextDigits 0 =
          if rest.isEmpty then 0 else 0 from ih 0]
      simp

theorem tree_resets (tree : OwnerCodecTree.Tree) (count : Nat) :
    (OwnerTreeBytes.writeTree tree).foldl OwnerPayload.nextDigits count = 0 := by
  have all : ∀ tree count, (OwnerTreeBytes.writeTree tree).foldl OwnerPayload.nextDigits count = 0 := by
    apply OwnerCodeCodec.tree_induction
    · intro n count; simp only [OwnerTreeBytes.writeTree,List.foldl_cons,nat_resets]
    · intro value count; cases value <;>
        simp only [OwnerTreeBytes.writeTree,List.foldl_cons,nat_resets]
    · intro s count
      simp only [OwnerTreeBytes.writeTree,List.foldl_cons,OwnerByteCodec.writeText,
        List.foldl_append,nat_resets,nats_resets]
      split <;> rfl
    · intro tag fields ih count
      have fieldsReset : ∀ c, (OwnerTreeBytes.writeTrees fields).foldl OwnerPayload.nextDigits c =
          if fields.isEmpty then c else 0 := by
        induction fields with
        | nil => intro c; simp [OwnerTreeBytes.writeTrees]
        | cons first rest tail =>
            intro c
            have firstReset := ih first (by simp) c
            have restReset := tail (fun t member => ih t (by simp [member])) 0
            simp only [OwnerTreeBytes.writeTrees,List.foldl_append,firstReset,restReset]
            simp
      simp only [OwnerTreeBytes.writeTree,List.foldl_cons,List.foldl_append,nat_resets]
      rw [fieldsReset]
      split <;> rfl
  exact all tree count

theorem encoded_digits_append
    (fits : OwnerPayload.digitCheck limit 0 (OwnerPacketCodec.encode codec value) = true) :
    OwnerPayload.digitCheck limit 0 (OwnerPacketCodec.encode codec value ++ suffix) =
      OwnerPayload.digitCheck limit 0 suffix := by
  rw [digits_append fits]
  exact congrArg (fun count => OwnerPayload.digitCheck limit count suffix)
    (tree_resets (codec.encode value) 0)

#print axioms text_append
#print axioms digits_append
#print axioms nat_resets
#print axioms tree_resets
#print axioms encoded_digits_append

-- A compositional sufficient profile. Its numerical margins are explicit;
-- this is not a completeness claim for every byte string the reader accepts.
def Fits (codec : OwnerCodecTree.Codec α) (textLimit bytes fuel : Nat) (value : α) : Prop :=
  (OwnerPacketCodec.encode codec value).length ≤ bytes ∧
  OwnerPayload.digitCheck 128 0 (OwnerPacketCodec.encode codec value) = true ∧
  OwnerPayload.textCheck textLimit (OwnerPacketCodec.encode codec value) = true ∧
  OwnerTreeBytes.treeCost (codec.encode value) ≤ fuel

theorem fits_wire (fit : Fits codec textLimit bytes fuel value)
    (size : bytes ≤ 65536) (space : fuel ≤ 256) : WireFits codec textLimit value :=
  ⟨Nat.le_trans fit.1 size,(OwnerPayload.digits_exact _ _ _).mp fit.2.1,
    (OwnerPayload.text_exact _ _).mp fit.2.2.1,
    OwnerPacketCodec.bounded_roundtrip _ _ _ (Nat.le_trans fit.2.2.2 space)⟩

theorem product_bytes (left : OwnerCodecTree.Codec α) (right : OwnerCodecTree.Codec β) (x : α) (y : β) :
    OwnerPacketCodec.encode (OwnerCodecTree.product left right) (x,y) =
      [7,0,1,2,0] ++ OwnerPacketCodec.encode left x ++ OwnerPacketCodec.encode right y := by
  simp [OwnerPacketCodec.encode,OwnerCodecTree.product,OwnerTreeBytes.encode,
    OwnerTreeBytes.writeTree,OwnerTreeBytes.writeTrees,OwnerByteCodec.writeNat]

theorem inr_bytes (left : OwnerCodecTree.Codec α) (right : OwnerCodecTree.Codec β) (y : β) :
    OwnerPacketCodec.encode (OwnerCodecTree.sum left right) (.inr y) =
      [7,2,0,2,0] ++ OwnerPacketCodec.encode right y := by
  simp [OwnerPacketCodec.encode,OwnerCodecTree.sum,OwnerTreeBytes.encode,
    OwnerTreeBytes.writeTree,OwnerTreeBytes.writeTrees,OwnerByteCodec.writeNat]

theorem inl_bytes (left : OwnerCodecTree.Codec α) (right : OwnerCodecTree.Codec β) (x : α) :
    OwnerPacketCodec.encode (OwnerCodecTree.sum left right) (.inl x) =
      [7,0,2,0] ++ OwnerPacketCodec.encode left x := by
  simp [OwnerPacketCodec.encode,OwnerCodecTree.sum,OwnerTreeBytes.encode,
    OwnerTreeBytes.writeTree,OwnerTreeBytes.writeTrees,OwnerByteCodec.writeNat]

theorem fits_product (first : Fits left textLimit b₁ f₁ x) (second : Fits right textLimit b₂ f₂ y) :
    Fits (OwnerCodecTree.product left right) textLimit (b₁+b₂+5) (f₁+f₂+3) (x,y) := by
  refine ⟨?_,?_,?_,?_⟩
  · rw [product_bytes]
    simp only [List.length_append,List.length_cons,List.length_nil]
    have := first.1; have := second.1; omega
  · rw [product_bytes,List.append_assoc]
    simp only [List.cons_append,List.nil_append,OwnerPayload.digitCheck,OwnerPayload.nextDigits]
    simpa using (encoded_digits_append first.2.1 (suffix:=OwnerPacketCodec.encode right y)).trans second.2.1
  · rw [product_bytes,List.append_assoc]
    simp only [List.cons_append,List.nil_append,OwnerPayload.textCheck,OwnerPayload.textHeadCheck]
    simpa using (text_append first.2.2.1 (suffix:=OwnerPacketCodec.encode right y)).trans second.2.2.1
  · have := first.2.2.2; have := second.2.2.2
    simp only [OwnerCodecTree.product,OwnerTreeBytes.treeCost,OwnerTreeBytes.treesCost]
    omega

theorem fits_inr (fit : Fits right textLimit bytes fuel value) (left : OwnerCodecTree.Codec α) :
    Fits (OwnerCodecTree.sum left right) textLimit (bytes+5) (fuel+2) (.inr value) := by
  refine ⟨?_,?_,?_,?_⟩
  · have := fit.1
    rw [inr_bytes]; simp only [List.length_append,List.length_cons,List.length_nil]; omega
  · rw [inr_bytes]
    simpa [OwnerPayload.digitCheck,OwnerPayload.nextDigits] using fit.2.1
  · rw [inr_bytes]
    simpa [OwnerPayload.textCheck,OwnerPayload.textHeadCheck] using fit.2.2.1
  · have := fit.2.2.2
    simp only [OwnerCodecTree.sum,OwnerTreeBytes.treeCost,OwnerTreeBytes.treesCost]
    omega

theorem fits_inl (fit : Fits left textLimit bytes fuel value) (right : OwnerCodecTree.Codec α) :
    Fits (OwnerCodecTree.sum left right) textLimit (bytes+4) (fuel+2) (.inl value) := by
  refine ⟨?_,?_,?_,?_⟩
  · have := fit.1
    rw [inl_bytes]; simp only [List.length_append,List.length_cons,List.length_nil]; omega
  · rw [inl_bytes]
    simpa [OwnerPayload.digitCheck,OwnerPayload.nextDigits] using fit.2.1
  · rw [inl_bytes]
    simpa [OwnerPayload.textCheck,OwnerPayload.textHeadCheck] using fit.2.2.1
  · have := fit.2.2.2
    simp only [OwnerCodecTree.sum,OwnerTreeBytes.treeCost,OwnerTreeBytes.treesCost]
    omega

theorem fits_natural (small : n < 2^digits) (room : digits ≤ 128) :
    Fits OwnerCodecTree.natural textLimit (digits+2) 1 n := by
  refine ⟨?_,?_,?_,?_⟩
  · have := nat_bytes_length small
    simpa only [OwnerPacketCodec.encode,OwnerCodecTree.natural,OwnerTreeBytes.encode,
      OwnerTreeBytes.writeTree,List.length_cons] using Nat.add_le_add_right this 1
  · have check := nat_digits_suffix small (count:=0) (limit:=128) (tail:=[]) (by omega)
    simpa [OwnerPacketCodec.encode,OwnerCodecTree.natural,OwnerTreeBytes.encode,
      OwnerTreeBytes.writeTree,OwnerPayload.digitCheck,OwnerPayload.nextDigits] using check
  · have check := nat_text_suffix n textLimit []
    simpa [OwnerPacketCodec.encode,OwnerCodecTree.natural,OwnerTreeBytes.encode,
      OwnerTreeBytes.writeTree,OwnerPayload.textCheck,OwnerPayload.textHeadCheck] using check
  · simp [OwnerCodecTree.natural,OwnerTreeBytes.treeCost]

theorem fits_int64 (range : InstancePrograms.Machine.lo ≤ value ∧ value ≤ InstancePrograms.Machine.hi) :
    Fits OwnerCodecTree.integer textLimit 65 1 value := by
  refine ⟨int64_bytes range,?_,?_,?_⟩
  · simpa [OwnerPayload.digitCheck] using int64_digits_suffix range []
  · simpa [OwnerPayload.textCheck] using integer_text_suffix value textLimit []
  · simp [OwnerCodecTree.integer,OwnerTreeBytes.treeCost]

#print axioms fits_wire
#print axioms fits_product
#print axioms fits_inr
#print axioms fits_inl
#print axioms fits_natural
#print axioms fits_int64

theorem fits_value (range : InstancePrograms.Machine.lo ≤ value ∧ value ≤ InstancePrograms.Machine.hi) :
    Fits OwnerPacketCodec.result textLimit 70 3 (.value value) :=
  fits_inr (fits_int64 range) OwnerPacketCodec.failure

theorem fits_envelope (scopeFits : scopeId < 2^128) (revisionFits : revision < 2^128)
    (ordinalFits : ordinal < 64) (ticketFits : Fits OwnerFullCodec.ticket textLimit bytes fuel ticket)
    (range : InstancePrograms.Machine.lo ≤ value ∧ value ≤ InstancePrograms.Machine.hi) :
    Fits OwnerReceipt.codec textLimit (bytes+358) (fuel+18)
      ⟨scopeId,ordinal,revision,ticket,.value value⟩ := by
  have first := fits_natural scopeFits (by decide : 128 ≤ 128) (textLimit:=textLimit)
  have second := fits_natural (show ordinal < 2^6 from ordinalFits) (by decide : 6 ≤ 128) (textLimit:=textLimit)
  have third := fits_natural revisionFits (by decide : 128 ≤ 128) (textLimit:=textLimit)
  have last := fits_value range (textLimit:=textLimit)
  have assembled := fits_product first (fits_product second (fits_product third (fits_product ticketFits last)))
  simpa only [Fits,OwnerReceipt.codec,OwnerCodecTree.iso,Nat.add_assoc,Nat.add_comm,Nat.add_left_comm] using assembled

def ResponseFits (scopeId revision : Nat) (ticket : InvocationBoundary.Ticket) : Prop :=
  let bytes := OwnerPacketCodec.encode OwnerFullCodec.ticket ticket
  let cost := OwnerTreeBytes.treeCost (OwnerFullCodec.ticket.encode ticket)
  scopeId < 2^128 ∧ revision < 2^128 ∧ bytes.length+373 ≤ 65536 ∧ cost+24 ≤ 256 ∧
    OwnerPayload.DigitBound 128 0 bytes ∧ OwnerPayload.TextBound 256 bytes

def responseCheck (scopeId revision : Nat) (ticket : InvocationBoundary.Ticket) : Bool :=
  let bytes := OwnerPacketCodec.encode OwnerFullCodec.ticket ticket
  let cost := OwnerTreeBytes.treeCost (OwnerFullCodec.ticket.encode ticket)
  decide (scopeId < 2^128 ∧ revision < 2^128 ∧ bytes.length+373 ≤ 65536 ∧ cost+24 ≤ 256) &&
    OwnerPayload.digitCheck 128 0 bytes && OwnerPayload.textCheck 256 bytes

theorem responseCheck_exact : responseCheck scopeId revision ticket = true ↔ ResponseFits scopeId revision ticket := by
  simp [responseCheck,ResponseFits,OwnerPayload.digits_exact,OwnerPayload.text_exact,and_assoc]

theorem response_ticket_fits (fits : ResponseFits scopeId revision ticket) :
    Fits OwnerFullCodec.ticket 256 (OwnerPacketCodec.encode OwnerFullCodec.ticket ticket).length
      (OwnerTreeBytes.treeCost (OwnerFullCodec.ticket.encode ticket)) ticket :=
  ⟨Nat.le_refl _,(OwnerPayload.digits_exact _ _ _).mpr fits.2.2.2.2.1,
    (OwnerPayload.text_exact _ _).mpr fits.2.2.2.2.2,Nat.le_refl _⟩

theorem owner_reply_readable (fits : ResponseFits scopeId revision ticket)
    (ordinalFits : ordinal < 64)
    (range : InstancePrograms.Machine.lo ≤ value ∧ value ≤ InstancePrograms.Machine.hi) :
    WireFits OwnerReservationWorker.replyCodec 256 (.inr ⟨scopeId,ordinal,revision,ticket,.value value⟩) := by
  have envelope := fits_envelope fits.1 fits.2.1 ordinalFits (response_ticket_fits fits) range
  apply fits_wire (fits_inr envelope OwnerCodecTree.natural)
  · have := fits.2.2.1; omega
  · have := fits.2.2.2.1; omega

theorem arrival_readable (fits : ResponseFits scopeId revision ticket)
    (ordinalFits : ordinal < 64)
    (range : InstancePrograms.Machine.lo ≤ value ∧ value ≤ InstancePrograms.Machine.hi) :
    WireFits (PublicationInput.input p a) 256
      (.step (.arrival ⟨scopeId,ordinal,revision,ticket,.value value⟩)) := by
  have envelope := fits_envelope fits.1 fits.2.1 ordinalFits (response_ticket_fits fits) range
  have commandWrapped : Fits (PublicationInput.command p a) 256
      ((OwnerPacketCodec.encode OwnerFullCodec.ticket ticket).length+358+4+5)
      (OwnerTreeBytes.treeCost (OwnerFullCodec.ticket.encode ticket)+18+2+2)
      (.arrival ⟨scopeId,ordinal,revision,ticket,.value value⟩) :=
    fits_inr (fits_inl envelope _) (SourceInput.command p a)
  have wrapped : Fits (PublicationInput.input p a) 256
      ((OwnerPacketCodec.encode OwnerFullCodec.ticket ticket).length+358+4+5+5)
      (OwnerTreeBytes.treeCost (OwnerFullCodec.ticket.encode ticket)+18+2+2+2)
      (.step (.arrival ⟨scopeId,ordinal,revision,ticket,.value value⟩)) :=
    fits_inr commandWrapped (SourceCodec.compact p)
  apply fits_wire wrapped
  · have := fits.2.2.1; omega
  · have := fits.2.2.2.1; omega

#print axioms fits_envelope
#print axioms responseCheck_exact
#print axioms owner_reply_readable
#print axioms arrival_readable

-- Construct a real model computation from rooted reservation progress. The
-- profile is checked on the pre-reservation scope/revision/ticket, without
-- assuming a produced result or a readable response in its premises.
theorem rooted_reservation_readable
    (path : OwnerReservation.Steps (OwnerReservation.initial assigned scope image capacity) state)
    (small : capacity ≤ 64)
    (profile : ResponseFits state.core.scopeId state.core.revision ticket)
    (reserved : OwnerReservation.reserve state requestedScope ticket = (next,.reserved)) :
    ∃ later record, OwnerReservation.compute next = some (later,.produced record) ∧
      WireFits OwnerReservationWorker.replyCodec 256 (.inr (OwnerReceipt.project record)) ∧
      WireFits (PublicationInput.input p a) 256 (.step (.arrival (OwnerReceipt.project record))) := by
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
  refine ⟨later,record,computed,?_,?_⟩
  · rw [projected]; exact owner_reply_readable profile ordinal range
  · rw [projected]; exact arrival_readable profile ordinal range

#print axioms rooted_reservation_readable

theorem reserve_readable (fits : ResponseFits scopeId revision ticket) :
    WireFits (OwnerEndpointWorker.command p a) 256 (.owner (.reserve ticket)) := by
  have reserve : Fits (OwnerReservationWorker.commandCodec p a) 256
      ((OwnerPacketCodec.encode OwnerFullCodec.ticket ticket).length+4+5)
      (OwnerTreeBytes.treeCost (OwnerFullCodec.ticket.encode ticket)+2+2) (.reserve ticket) :=
    fits_inr (fits_inl (response_ticket_fits fits) _) (OwnerFullCodec.image p a)
  have wrapped : Fits (OwnerEndpointWorker.command p a) 256
      ((OwnerPacketCodec.encode OwnerFullCodec.ticket ticket).length+4+5+4)
      (OwnerTreeBytes.treeCost (OwnerFullCodec.ticket.encode ticket)+2+2+2) (.owner (.reserve ticket)) :=
    fits_inl reserve OwnerCodecTree.natural
  apply fits_wire wrapped
  · have := fits.2.2.1; omega
  · have := fits.2.2.2.1; omega

theorem text_bound_monotone (small : before ≤ after) (bound : OwnerPayload.TextBound before bytes) :
    OwnerPayload.TextBound after bytes := by
  induction bound with
  | nil => exact .nil
  | @cons byte rest head tail ih =>
      refine .cons ?_ ih
      by_cases marker : byte = 6
      · simp only [OwnerPayload.TextHead,marker,ite_true] at head ⊢
        obtain ⟨count,remaining,read,fits⟩ := head
        exact ⟨count,remaining,read,Nat.le_trans fits small⟩
      · simp [OwnerPayload.TextHead,marker]

theorem wire_text_monotone (small : before ≤ after) (fits : WireFits codec before value) :
    WireFits codec after value :=
  ⟨fits.1,fits.2.1,text_bound_monotone small fits.2.2.1,fits.2.2.2⟩

#print axioms reserve_readable
#print axioms text_bound_monotone
#print axioms wire_text_monotone

#print axioms decode_some_exact
#print axioms wireCheck_exact

#print axioms nat_text_suffix
#print axioms int64_magnitude
#print axioms int64_bytes
#print axioms int64_digits_suffix
#print axioms integer_text_suffix
#print axioms bounded_decode_unique

#print axioms machine_result_range
#print axioms produced_result_range
#print axioms computed_result_range
#print axioms nat_bytes_length
#print axioms nat_digits_suffix
end MirroreaProofFirst.OwnerResponseProfile
