import MirroreaProofFirstPublicJointHistory
import MirroreaProofFirstPublicOwnerReplay
namespace MirroreaProofFirst.PublicJointReplay
open SourceRegistration (State ownerResult sourceCheck source_check_exact)
open PublicJointHistory (Runs)

-- Nonproduction history replay: mode names select existing declarative
-- constructors, never example identities. No runtime/transport adoption.
inductive Action (p a : Nat) where
  | owner (i : Fin p) (command : OwnerReservationWorker.Command p a)
  | extraFreeze (i : Fin p) (revision : Nat)
  | failedFreeze (i : Fin p) (revision : Nat)
  | paidFreeze (i : Fin p) (revision : Nat)
  | source (command : PublicationInput.Command p a)
  | notify
  | work (i : Fin p) (probeRevision : Nat)

def floorCheck (s : State p a) : PublicationInput.Command p a → Bool
  | .freeze i revision => (s.actual.records i).contains revision
  | _ => true

theorem floor_exact : floorCheck s command = true ↔ SourceOwnerFloor.sourceAllowed s.actual command := by
  cases command <;> simp [floorCheck,SourceOwnerFloor.sourceAllowed]

def publicSourceCheck : PublicationInput.Command p a → Bool
  | .enter _ | .finish _ => false
  | _ => true

theorem public_source_exact : publicSourceCheck command = true ↔ PublicJointHistory.publicSource command := by
  cases command <;> simp [publicSourceCheck,PublicJointHistory.publicSource]

abbrev Reachable (assigned : Fin p → OwnerEvaluator.Assignment p) (scopeId : Nat)
    (capacity : Fin p → Nat) (budget : Nat) (seed : QualifiedCustody.State p a) :=
  {s : State p a // Runs assigned scopeId capacity budget seed s}

-- The returned value carries its history proof; every branch consumes the
-- actual transition and exact existing guard. Failure returns no witness.
def advance {p a : Nat} {assigned : Fin p → OwnerEvaluator.Assignment p} {scopeId budget : Nat}
    {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
    (s : Reachable assigned scopeId capacity budget seed) (action : Action p a) :
    Option (Reachable assigned scopeId capacity budget seed) := by
  cases action with
  | owner i command =>
    if admin : PublicOwnerReplay.adminCheck (.owner command) = true then
     if clear : s.val.actual.pending = none then
       if checked : SourceOwnerImage.imageCheck s.val.actual.source command = true then
         let result := OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.val.actual.owners i) (.owner command)
         exact some ⟨ownerResult s.val i (.owner command) result.1 result.2,
           Runs.step s.property (.owner clear (PublicOwnerReplay.admin_exact.mp admin) (SourceOwnerImage.image_check_exact.mp checked) rfl)⟩
       else exact none
     else exact none
    else exact none
  | extraFreeze i revision =>
    if clear : s.val.actual.pending = none then
      if bound : revision ≤ s.val.actual.source.publication.barrier.published then
        let result := OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.val.actual.owners i) (.freeze revision)
        exact some ⟨ownerResult s.val i (.freeze revision) result.1 result.2,
          Runs.step s.property (.extraFreeze clear bound rfl)⟩
      else exact none
    else exact none
  | failedFreeze i revision =>
    if clear : s.val.actual.pending = none then
      let result := OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.val.actual.owners i) (.freeze revision)
      if failed : result.2 ≠ .inl 12 then
        exact some ⟨ownerResult s.val i (.freeze revision) result.1 result.2,
          Runs.step s.property (.failedFreeze clear failed rfl)⟩
      else exact none
    else exact none
  | paidFreeze i revision =>
    if clear : s.val.actual.pending = none then
      let result := OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.val.actual.owners i) (.freeze revision)
      if success : result.2 = .inl 12 then
        have ran : OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.val.actual.owners i) (.freeze revision) =
            (result.1,.inl 12) := by rw [←success]
        exact some ⟨{ownerResult s.val i (.freeze revision) result.1 (.inl 12) with actual.pending := some (i,revision)},
          Runs.step s.property (.paidFreeze clear ran)⟩
      else exact none
    else exact none
  | source command =>
    if allowed : publicSourceCheck command = true then
     if clear : s.val.actual.pending = none then
       if floor : floorCheck s.val command = true then
         if install : sourceCheck s.val command = true then
           match ran : PublicationInput.execute scopeId s.val.actual.source command with
           | none => exact none
           | some next => exact some ⟨{s.val with actual.source := next},
               Runs.step s.property (.source clear (public_source_exact.mp allowed) (floor_exact.mp floor) (source_check_exact.mp install) ran)⟩
         else exact none
       else exact none
     else exact none
    else exact none
  | notify =>
    match pending : s.val.actual.pending with
    | none => exact none
    | some (i,revision) =>
      match ran : PublicationInput.execute scopeId s.val.actual.source (.freeze i revision) with
      | none => exact none
      | some next => exact some ⟨{s.val with actual.source := next,actual.pending := none},
          Runs.step s.property (.notify pending ran)⟩

  | work i revision =>
    if clear : s.val.actual.pending = none then
      match enter : PublicationInput.execute scopeId s.val.actual.source (.enter i) with
      | none => exact none
      | some entered =>
        match dispatch : entered.dispatch with
        | none => exact none
        | some dispatched =>
          if contextAt : dispatched = ⟨i,⟨scopeId,s.val.actual.source.publication.barrier.installed i⟩,dispatched.ticket⟩ then
            let first := OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.val.actual.owners i) (.owner (.reserve dispatched.ticket))
            if reserved : first.2 = .inl 6 then
              let second := OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) first.1 (.freeze revision)
              if probed : second.2 = .inl 2 then
                let third := OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) second.1 (.owner .compute)
                match produced : third.2 with
                | .inl _ => exact none
                | .inr envelope =>
                  if correlated : envelope.ticket = dispatched.ticket ∧ envelope.scopeId = scopeId ∧
                      envelope.revision = s.val.actual.source.publication.barrier.installed i then
                    match finish : PublicationInput.execute scopeId entered (.finish i) with
                    | none => exact none
                    | some after =>
                      have ran1 : OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.val.actual.owners i) (.owner (.reserve dispatched.ticket)) =
                          (first.1,.inl 6) := by rw [←reserved]
                      have ran2 : OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) first.1 (.freeze revision) =
                          (second.1,.inl 2) := by rw [←probed]
                      have ran3 : OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) second.1 (.owner .compute) =
                          (third.1,.inr envelope) := by rw [←produced]
                      exact some ⟨PublicJointHistory.workResult s.val i dispatched.ticket revision first.1 second.1 third.1 envelope after,
                        Runs.step s.property (.work clear enter (dispatch.trans (congrArg some contextAt))
                          ran1 ran2 ran3 correlated.1 correlated.2.1 correlated.2.2 finish)⟩
                  else exact none
              else exact none
            else exact none
          else exact none
    else exact none

def initial (assigned : Fin p → OwnerEvaluator.Assignment p) (scopeId : Nat)
    (capacity : Fin p → Nat) (budget : Nat) (seed : QualifiedCustody.State p a) :
    Reachable assigned scopeId capacity budget seed := ⟨SourceRegistration.initial seed budget,.fresh⟩

def replay {p a : Nat} {assigned : Fin p → OwnerEvaluator.Assignment p} {scopeId budget : Nat}
    {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
    (s : Reachable assigned scopeId capacity budget seed) :
    List (Action p a) → Option (Reachable assigned scopeId capacity budget seed)
  | [] => some s
  | action :: rest => (advance s action).bind (fun next => replay next rest)

#print axioms floor_exact
#print axioms advance
#print axioms initial
#print axioms replay


#print axioms public_source_exact
-- Relative completeness for each declarative event constructor. These show
-- acceptance, with exact state output for owner/source/notify and isSome
-- for freeze variants. History acceptance includes actual refused owner results
-- and is distinct from success7/10/12 or a produced envelope.
theorem owner_complete
    (s : Reachable assigned scopeId capacity budget seed)
    (clear : s.val.actual.pending = none)
    (admin : PublicOwnerBoundary.Admin (.owner command))
    (allowed : SourceOwnerImage.ImageAllowed s.val.actual.source command) :
    (advance s (.owner i command)).map Subtype.val =
      some (ownerResult s.val i (.owner command)
        (OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.val.actual.owners i) (.owner command)).1
        (OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.val.actual.owners i) (.owner command)).2) := by
  simp [advance,PublicOwnerReplay.admin_exact.mpr admin,clear,SourceOwnerImage.image_check_exact.mpr allowed]

theorem extra_freeze_complete
    (s : Reachable assigned scopeId capacity budget seed)
    (clear : s.val.actual.pending = none)
    (bound : revision ≤ s.val.actual.source.publication.barrier.published) :
    (advance s (.extraFreeze i revision)).isSome = true := by
  simp [advance,clear,bound]

theorem failed_freeze_complete
    (s : Reachable assigned scopeId capacity budget seed)
    (clear : s.val.actual.pending = none)
    (failed : (OwnerEndpointBudget.transition (assigned i) scopeId (capacity i)
      (s.val.actual.owners i) (.freeze revision)).2 ≠ .inl 12) :
    (advance s (.failedFreeze i revision)).isSome = true := by
  simp [advance,clear,failed]

theorem paid_freeze_complete
    (s : Reachable assigned scopeId capacity budget seed)
    (clear : s.val.actual.pending = none)
    (paid : (OwnerEndpointBudget.transition (assigned i) scopeId (capacity i)
      (s.val.actual.owners i) (.freeze revision)).2 = .inl 12) :
    (advance s (.paidFreeze i revision)).isSome = true := by
  simp [advance,clear,paid]

theorem source_complete {p a : Nat} {command : PublicationInput.Command p a} {assigned : Fin p → OwnerEvaluator.Assignment p}
    {scopeId budget : Nat} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
    {next : PublicationInput.State p a}
    (s : Reachable assigned scopeId capacity budget seed)
    (clear : s.val.actual.pending = none)
    (allowedPublic : PublicJointHistory.publicSource command)
    (floor : SourceOwnerFloor.sourceAllowed s.val.actual command)
    (install : SourceRegistration.sourceAllowed s.val command)
    (ran : PublicationInput.execute scopeId s.val.actual.source command = some next) :
    (advance s (.source command)).map Subtype.val = some {s.val with actual.source := next} := by
  simp only [advance, dif_pos (public_source_exact.mpr allowedPublic), dif_pos clear, dif_pos (floor_exact.mpr floor),
    dif_pos (SourceRegistration.source_check_exact.mpr install)]
  split <;> rename_i h
  · simp [ran] at h
  · have equal := Option.some.inj (h.symm.trans ran)
    subst next
    rfl

theorem notify_complete {p a : Nat} {i : Fin p} {revision : Nat} {assigned : Fin p → OwnerEvaluator.Assignment p}
    {scopeId budget : Nat} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
    {next : PublicationInput.State p a}
    (s : Reachable assigned scopeId capacity budget seed)
    (pending : s.val.actual.pending = some (i,revision))
    (ran : PublicationInput.execute scopeId s.val.actual.source (.freeze i revision) = some next) :
    (advance s .notify).map Subtype.val = some {s.val with actual.source := next,actual.pending := none} := by
  simp only [advance]
  split
  · rename_i h; simp [pending] at h
  · rename_i j r h
    have pair := Option.some.inj (h.symm.trans pending)
    cases pair
    split <;> rename_i executed
    · simp [ran] at executed
    · have equal := Option.some.inj (executed.symm.trans ran)
      subst next
      rfl

#print axioms owner_complete
#print axioms extra_freeze_complete
#print axioms failed_freeze_complete
#print axioms paid_freeze_complete
#print axioms source_complete
#print axioms notify_complete


theorem work_complete {p a : Nat} {assigned : Fin p → OwnerEvaluator.Assignment p}
    {scopeId budget : Nat} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
    {i : Fin p} {ticket : OwnerOccurrence.Ticket} {revision : Nat}
    {entered after : PublicationInput.State p a}
    {first second third : OwnerEndpointBudget.State p a} {envelope : OwnerReceipt.Envelope}
    (s : Reachable assigned scopeId capacity budget seed)
    (clear : s.val.actual.pending = none)
    (enter : PublicationInput.execute scopeId s.val.actual.source (.enter i) = some entered)
    (dispatch : entered.dispatch = some ⟨i,⟨scopeId,s.val.actual.source.publication.barrier.installed i⟩,ticket⟩)
    (reserved : OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.val.actual.owners i)
      (.owner (.reserve ticket)) = (first,.inl 6))
    (probed : OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) first (.freeze revision) = (second,.inl 2))
    (produced : OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) second (.owner .compute) = (third,.inr envelope))
    (correlated : envelope.ticket = ticket ∧ envelope.scopeId = scopeId ∧
      envelope.revision = s.val.actual.source.publication.barrier.installed i)
    (finish : PublicationInput.execute scopeId entered (.finish i) = some after) :
    (advance s (.work i revision)).map Subtype.val =
      some (PublicJointHistory.workResult s.val i ticket revision first second third envelope after) := by
  simp only [advance,dif_pos clear]
  split
  · rename_i absent; simp [enter] at absent
  · rename_i actual h
    have same := Option.some.inj (h.symm.trans enter)
    subst actual
    split
    · rename_i absent; simp [dispatch] at absent
    · rename_i dispatched atDispatch
      have same := Option.some.inj (atDispatch.symm.trans dispatch)
      subst dispatched
      dsimp only
      simp only [reserved,probed,produced,dif_pos True.intro]
      split
      · rename_i code atEnvelope
        simp only [reserved,probed,produced] at atEnvelope
        contradiction
      · rename_i actualEnvelope atEnvelope
        simp only [reserved,probed,produced] at atEnvelope
        have same := Sum.inr.inj atEnvelope
        subst actualEnvelope
        simp only [dif_pos correlated]
        split
        · rename_i absent; simp [finish] at absent
        · rename_i finalState atFinish
          have same := Option.some.inj (atFinish.symm.trans finish)
          subst finalState
          rfl

#print axioms work_complete

end MirroreaProofFirst.PublicJointReplay
