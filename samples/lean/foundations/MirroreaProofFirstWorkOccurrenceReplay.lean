import MirroreaProofFirstWorkOccurrence
namespace MirroreaProofFirst.WorkOccurrenceReplay
open WorkOccurrence

def same (codec : OwnerCodecTree.Codec α) (x y : α) : Bool :=
  decide (OwnerPacketCodec.encode codec x = OwnerPacketCodec.encode codec y)

theorem same_exact : same codec x y = true ↔ x = y := by
  simp only [same,decide_eq_true_eq]
  constructor
  · intro equal
    have decoded := congrArg (OwnerPacketCodec.decode codec) equal
    simpa only [OwnerPacketCodec.roundtrip,Option.some.injEq] using decoded
  · intro equal; subst y; rfl

instance (s : State p a) (vector : Vector (Fin 513) p) : Decidable (vectorAt s vector) :=
  inferInstanceAs (Decidable (∀ i, (s.joint.actual.owners i).remaining = (vector[i.val]).val))

structure Checked (assigned : SourceInput.Assignment p a) (scope : Nat)
    (bootstrap : SourceInput.Bootstrap a) (capacity : Fin p → Nat)
    (base : State p a) (target : Fin p) (ticket : OwnerOccurrence.Ticket)
    (events : List (Event p a)) where
  phase : Phase
  state : State p a
  path : Trace assigned scope bootstrap capacity base target ticket events phase state

def initial (assigned : SourceInput.Assignment p a) (scope : Nat)
    (bootstrap : SourceInput.Bootstrap a) (capacity : Fin p → Nat)
    (base : State p a) (target : Fin p) (ticket : OwnerOccurrence.Ticket) :
    Checked assigned scope bootstrap capacity base target ticket [] := ⟨.waiting,base,.start⟩

-- Executable prefix checker. It examines actual typed inputs, complete
-- transitions, full owner-vector equality and consumed source ordinals.
-- It does not check the desired Joint-history theorem as an admission guard.
def advance {p a : Nat} {assigned : SourceInput.Assignment p a} {scope : Nat}
    {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat}
    {base : State p a} {target : Fin p} {ticket : OwnerOccurrence.Ticket}
    {events : List (Event p a)}
    (current : Checked assigned scope bootstrap capacity base target ticket events)
    (event : Event p a) :
    Option (Checked assigned scope bootstrap capacity base target ticket (events ++ [event])) := by
  obtain ⟨phase,s,path⟩ := current
  cases phase with
  | waiting =>
    if empty : events = [] then
     subst events
     match event with
     | .source ordinal (vector,input) =>
       if atOrdinal : ordinal = base.ordinal then
        if inputAt : same (PublicationInput.input p a) input (.step (.enter target)) = true then
         have equal := same_exact.mp inputAt
         subst ordinal; subst input
         if clear : base.joint.actual.pending = none then
          if bound : vectorAt base vector then
           match ran : SourceFundingInput.execute assigned scope bootstrap base.driver (vector,.step (.enter target)) with
           | (next,.accepted) =>
             match present : next.source with
             | none => exact none
             | some entered =>
               if dispatched : entered.dispatch = some ⟨target,⟨scope,base.joint.actual.source.publication.barrier.installed target⟩,ticket⟩ then
                 exact some ⟨.entered,onSource base next entered,.enter clear bound ran present dispatched⟩
               else exact none
           | _ => exact none
          else exact none
         else exact none
        else exact none
       else exact none
     | _ => exact none
    else exact none
  | entered =>
    match event with
    | .query ordinal request =>
      if atOrdinal : ordinal = s.ordinal then
        subst ordinal
        exact some ⟨.entered,{s with ordinal := s.ordinal+1},.query path⟩
      else exact none
    | .owner i command =>
      if targetAt : i = target then
       if commandAt : same (OwnerEndpointWorker.command p a) command (.owner (.reserve ticket)) = true then
        have equal := same_exact.mp commandAt
        subst i; subst command
        let result := OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target) (s.joint.actual.owners target) (.owner (.reserve ticket))
        if accepted : result.2 = .inl 6 then
          have ran : OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target) (s.joint.actual.owners target) (.owner (.reserve ticket)) = (result.1,.inl 6) := by rw [←accepted]
          exact some ⟨.reserved,onOwner s target (.owner (.reserve ticket)) result.1 (.inl 6),.reserve path ran⟩
        else exact none
       else exact none
      else exact none
    | _ => exact none
  | reserved =>
    match event with
    | .owner i command =>
      if targetAt : i = target then
       if commandAt : same (OwnerEndpointWorker.command p a) command (.freeze (base.joint.actual.source.publication.barrier.published+1)) = true then
        have equal := same_exact.mp commandAt
        subst i; subst command
        let result := OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target) (s.joint.actual.owners target) (.freeze (base.joint.actual.source.publication.barrier.published+1))
        if accepted : result.2 = .inl 2 then
          have ran : OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target) (s.joint.actual.owners target) (.freeze (base.joint.actual.source.publication.barrier.published+1)) = (result.1,.inl 2) := by rw [←accepted]
          exact some ⟨.probed,onOwner s target (.freeze (base.joint.actual.source.publication.barrier.published+1)) result.1 (.inl 2),.probe path ran⟩
        else exact none
       else exact none
      else exact none
    | _ => exact none
  | probed =>
    match event with
    | .owner i command =>
      if targetAt : i = target then
       if commandAt : same (OwnerEndpointWorker.command p a) command (.owner .compute) = true then
        have equal := same_exact.mp commandAt
        subst i; subst command
        let result := OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target) (s.joint.actual.owners target) (.owner .compute)
        match produced : result.2 with
        | .inl _ => exact none
        | .inr envelope =>
          if correlated : envelope.ticket = ticket ∧ envelope.scopeId = scope ∧ envelope.revision = base.joint.actual.source.publication.barrier.installed target then
            have ran : OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target) (s.joint.actual.owners target) (.owner .compute) = (result.1,.inr envelope) := by rw [←produced]
            exact some ⟨.computed envelope,onOwner s target (.owner .compute) result.1 (.inr envelope),.compute path ran correlated.1 correlated.2.1 correlated.2.2⟩
          else exact none
       else exact none
      else exact none
    | _ => exact none
  | computed envelope =>
    match event with
    | .source ordinal (vector,input) =>
      if atOrdinal : ordinal = s.ordinal then
       if inputAt : same (PublicationInput.input p a) input (.step (.finish target)) = true then
        have equal := same_exact.mp inputAt
        subst ordinal; subst input
        if bound : vectorAt s vector then
          match ran : SourceFundingInput.execute assigned scope bootstrap s.driver (vector,.step (.finish target)) with
          | (next,.accepted) =>
            match present : next.source with
            | none => exact none
            | some after => exact some ⟨.finished envelope,onSource s next after,.finish path bound ran present⟩
          | _ => exact none
        else exact none
       else exact none
      else exact none
    | _ => exact none
  | finished _ => exact none

variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat}
  {base s : State p a} {target : Fin p} {ticket : OwnerOccurrence.Ticket}
  {events : List (Event p a)} {vector : Vector (Fin 513) p}

-- Relative completeness covers every declared interval constructor, for
-- arbitrary earlier certified prefixes. These are general laws, not fixed
-- examples or an assumption that the checker has already accepted.
theorem enter_complete
    (clear : base.joint.actual.pending = none) (bound : vectorAt base vector)
    (ran : SourceFundingInput.execute assigned scope bootstrap base.driver (vector,.step (.enter target)) = (next,.accepted))
    (present : next.source = some entered)
    (dispatched : entered.dispatch = some ⟨target,⟨scope,base.joint.actual.source.publication.barrier.installed target⟩,ticket⟩) :
    (advance (initial assigned scope bootstrap capacity base target ticket)
      (.source base.ordinal (vector,.step (.enter target)))).map (fun c => c.state) = some (onSource base next entered) := by
  simp only [advance,initial,dif_pos True.intro,dif_pos (same_exact.mpr rfl),dif_pos clear,dif_pos bound]
  split
  · rename_i actual atRun
    have same := Prod.mk.inj (atRun.symm.trans ran)
    rcases same with ⟨rfl,_⟩
    split
    · rename_i absent; simp [present] at absent
    · rename_i actualSource atSource
      have same := Option.some.inj (atSource.symm.trans present)
      subst actualSource
      simp [dispatched]
  · simp_all [ran]

theorem query_complete
    (path : Trace assigned scope bootstrap capacity base target ticket events .entered s) :
    (advance ⟨.entered,s,path⟩ (.query s.ordinal request)).map (fun c => c.state) =
      some {s with ordinal := s.ordinal+1} := by
  simp [advance]

theorem reserve_complete
    (path : Trace assigned scope bootstrap capacity base target ticket events .entered s)
    (ran : OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target) (s.joint.actual.owners target)
      (.owner (.reserve ticket)) = (next,.inl 6)) :
    (advance ⟨.entered,s,path⟩ (.owner target (.owner (.reserve ticket)))).map (fun c => c.state) =
      some (onOwner s target (.owner (.reserve ticket)) next (.inl 6)) := by
  simp [advance,same_exact,ran]

theorem probe_complete
    (path : Trace assigned scope bootstrap capacity base target ticket events .reserved s)
    (ran : OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target) (s.joint.actual.owners target)
      (.freeze (base.joint.actual.source.publication.barrier.published+1)) = (next,.inl 2)) :
    (advance ⟨.reserved,s,path⟩ (.owner target (.freeze (base.joint.actual.source.publication.barrier.published+1)))).map (fun c => c.state) =
      some (onOwner s target (.freeze (base.joint.actual.source.publication.barrier.published+1)) next (.inl 2)) := by
  simp [advance,same_exact,ran]

theorem compute_complete
    (path : Trace assigned scope bootstrap capacity base target ticket events .probed s)
    (ran : OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target) (s.joint.actual.owners target)
      (.owner .compute) = (next,.inr envelope))
    (correlated : envelope.ticket = ticket ∧ envelope.scopeId = scope ∧ envelope.revision = base.joint.actual.source.publication.barrier.installed target) :
    (advance ⟨.probed,s,path⟩ (.owner target (.owner .compute))).map (fun c => c.state) =
      some (onOwner s target (.owner .compute) next (.inr envelope)) := by
  simp only [advance,dif_pos True.intro,dif_pos (same_exact.mpr rfl)]
  split
  · rename_i code atReply; simp [ran] at atReply
  · rename_i actualEnvelope atReply
    have same : actualEnvelope = envelope := by simpa [ran] using atReply.symm
    subst actualEnvelope
    simp [correlated,ran]

theorem finish_complete
    (path : Trace assigned scope bootstrap capacity base target ticket events (.computed envelope) s)
    (bound : vectorAt s vector)
    (ran : SourceFundingInput.execute assigned scope bootstrap s.driver (vector,.step (.finish target)) = (next,.accepted))
    (present : next.source = some after) :
    (advance ⟨.computed envelope,s,path⟩ (.source s.ordinal (vector,.step (.finish target)))).map (fun c => c.state) =
      some (onSource s next after) := by
  simp only [advance,dif_pos True.intro,dif_pos (same_exact.mpr rfl),dif_pos bound]
  split
  · rename_i actual atRun
    have same := Prod.mk.inj (atRun.symm.trans ran)
    rcases same with ⟨rfl,_⟩
    split
    · rename_i absent; simp [present] at absent
    · rename_i actualSource atSource
      have same := Option.some.inj (atSource.symm.trans present)
      subst actualSource
      rfl
  · simp_all [ran]

theorem finished_terminal
    (path : Trace assigned scope bootstrap capacity base target ticket events (.finished envelope) s) :
    advance ⟨.finished envelope,s,path⟩ event = none := rfl

#print axioms enter_complete
#print axioms query_complete
#print axioms reserve_complete
#print axioms probe_complete
#print axioms compute_complete
#print axioms finish_complete
#print axioms finished_terminal

#print axioms same_exact
#print axioms advance
end MirroreaProofFirst.WorkOccurrenceReplay
