import MirroreaProofFirstCohortPhysicalOrdinal
namespace MirroreaProofFirst.CohortFundingReplay
open CohortPhase
open WorkOccurrenceReplay (same same_exact)

-- Funding projection for the future shared lifetime checker. Every result certifies
-- an independently declared funding transition on this exact complete state.
-- Coupling to Joint and local host commits remains a separate unfinished consumer.
instance (s : Live p a) (vector : Vector (Fin 513) p) : Decidable (VectorAt s vector) :=
  inferInstanceAs (Decidable (∀ i, (s.owners i).remaining = (vector[i.val]).val))

structure SourceNext (assigned : SourceInput.Assignment p a) (scope : Nat)
    (bootstrap : SourceInput.Bootstrap a) (capacity : Fin p → Nat)
    (before : Live p a) (request : SourceFundingInput.Request p a) where
  val : Live p a
  step : Step assigned scope bootstrap capacity (some before) (.source request) (some val)

def source {p a : Nat} {assigned : SourceInput.Assignment p a} {scope : Nat}
    {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat}
    (s : Live p a) (request : SourceFundingInput.Request p a) :
    Option (SourceNext assigned scope bootstrap capacity s request) := by
  obtain ⟨vector,input⟩ := request
  if bound : VectorAt s vector then
    let result := SourceFundingInput.execute assigned scope bootstrap s.driver (vector,input)
    if accepted : result.2 = .accepted then
      have ran : SourceFundingInput.execute assigned scope bootstrap s.driver (vector,input) = (result.1,.accepted) := by rw [←accepted]
      match mode : s.mode with
      | .prelude owed =>
        match input with
        | .launch program =>
          if fresh : s.driver.source = none then
            exact some ⟨sourceResult s result.1 (.prelude (fun _ => true)),.launch mode fresh bound ran⟩
          else exact none
        | _ => exact none
      | .ordinary =>
        match input with
        | .launch _ => exact none
        | .step command =>
          if publicCheck : PublicJointReplay.publicSourceCheck command = true then
            exact some ⟨sourceResult s result.1 .ordinary,.source mode (PublicJointReplay.public_source_exact.mp publicCheck) bound ran⟩
          else
            match command with
            | .enter target =>
              match present : result.1.source with
              | none => exact none
              | some entered =>
                match dispatched : entered.dispatch with
                | none => exact none
                | some dispatch =>
                  if endpoint : dispatch.endpoint = target then
                    match headAt : result.1.suffix with
                    | [] => exact none
                    | first :: rest =>
                      if checked : same (PublicationInput.command p a) first (.finish dispatch.endpoint) = true then
                        have head : result.1.suffix = .finish dispatch.endpoint :: rest := by rw [headAt,same_exact.mp checked]
                        exact some ⟨sourceResult s result.1 (.entered dispatch),by
                          have actualRan : SourceFundingInput.execute assigned scope bootstrap s.driver
                              (vector,.step (.enter dispatch.endpoint)) = (result.1,.accepted) := by simpa only [endpoint] using ran
                          simpa only [endpoint] using (Step.enter mode bound actualRan present dispatched head)⟩
                      else exact none
                  else exact none
            | _ => exact none
      | .paid pending =>
        match input with
        | .launch _ => exact none
        | .step command =>
          if permitted : PaidHeadPhase.permits pending s.ordinal command = true then
            exact some ⟨sourceResult s result.1 .ordinary,.notify mode permitted bound ran⟩
          else exact none
      | .computed dispatch envelope =>
        if checked : same (PublicationInput.input p a) input (.step (.finish dispatch.endpoint)) = true then
          have equal := same_exact.mp checked
          exact some ⟨sourceResult s result.1 .ordinary,by
            have actualRan : SourceFundingInput.execute assigned scope bootstrap s.driver
                (vector,.step (.finish dispatch.endpoint)) = (result.1,.accepted) := by simpa only [equal] using ran
            simpa only [equal] using (Step.finish mode bound actualRan)⟩
        else exact none
      | _ => exact none
    else
      match mode : s.mode with
      | .ordinary => exact some ⟨{s with ordinal := s.ordinal+1},.refused (Or.inl mode) bound accepted⟩
      | .prelude owed => exact some ⟨{s with ordinal := s.ordinal+1},.refused (Or.inr ⟨owed,mode⟩) bound accepted⟩
      | _ => exact none
  else exact none

-- Soundness is supplied by the exact transition field, not by checking the
-- desired invariant as a runtime predicate. Root validity is still required.
theorem source_preserves
    (valid : Invariant assigned scope bootstrap (some s))
    (_checked : source (assigned:=assigned) (scope:=scope) (bootstrap:=bootstrap) (capacity:=capacity) s request = some next) :
    Invariant assigned scope bootstrap (some next.val) :=
  step_preserves valid next.step

-- Every independently declared source transition is recognized with exactly
-- its resulting full state, including known refusals. No successful check is
-- a premise; this is constructor-relative completeness, not finite testing.
theorem source_complete {p a : Nat} {assigned : SourceInput.Assignment p a} {scope : Nat}
    {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat}
    {s next : Live p a} {request : SourceFundingInput.Request p a}
    (step : Step assigned scope bootstrap capacity (some s) (.source request) (some next)) :
    (source (assigned:=assigned) (scope:=scope) (bootstrap:=bootstrap) (capacity:=capacity) s request).map (fun c => c.val) = some next := by
  cases step with
  | launch mode fresh bound ran =>
    rcases s with ⟨driver,owners,ordinal,phase⟩
    dsimp at mode
    subst phase
    dsimp at fresh ran
    simp [source,bound,ran,fresh]
  | source mode allowed bound ran =>
    rcases s with ⟨driver,owners,ordinal,phase⟩
    dsimp at mode
    subst phase
    have checked := PublicJointReplay.public_source_exact.mpr allowed
    simp [source,bound,ran,checked]
  | refused mode bound refused =>
    rcases s with ⟨driver,owners,ordinal,phase⟩
    rcases mode with mode | ⟨owed,mode⟩
    all_goals dsimp at mode; subst phase; simp [source,bound,refused]
  | notify mode allowed bound ran =>
    rcases s with ⟨driver,owners,ordinal,phase⟩
    dsimp at mode
    subst phase
    simp [source,bound,ran,allowed]
  | @enter s vector next entered dispatch rest mode bound ran present dispatched head =>
    rcases s with ⟨driver,owners,ordinal,phase⟩
    dsimp at mode
    subst phase
    dsimp at ran
    simp [source,bound,ran,PublicJointReplay.publicSourceCheck,present,dispatched,head,same_exact]
    split
    · rename_i absent
      simp [ran,present] at absent
    · rename_i actual atSource
      have equal : actual = entered := by simpa [ran,present] using atSource.symm
      subst actual
      split
      · rename_i absent
        simp [dispatched] at absent
      · rename_i actualDispatch atDispatch
        have equal : actualDispatch = dispatch := by simpa [dispatched] using atDispatch.symm
        subst actualDispatch
        simp [ran,head,same_exact]
        split
        · rename_i absent
          simp [ran,head] at absent
        · rename_i first tail atHead
          have equal : first = PublicationInput.Command.finish dispatch.endpoint ∧ tail = rest := by simpa [ran,head] using atHead.symm
          rcases equal with ⟨rfl,rfl⟩
          simp [same_exact]
  | finish mode bound ran =>
    rcases s with ⟨driver,owners,ordinal,phase⟩
    dsimp at mode
    subst phase
    simp [source,bound,ran,same_exact]

#print axioms source_complete

-- Head-payment recognition examines the actual command and full source image.
-- Arithmetic slack alone cannot manufacture a payment occurrence.
def headPaymentCheck (driver : PublicationCapacityDriver.State p a)
    (target : Fin p) (request : OwnerEndpoint.Command p a) : Bool :=
  match driver.suffix with
  | .freeze i revision :: _ => decide (i=target) &&
      same (OwnerEndpointWorker.command p a) request (.freeze revision)
  | .install i revision :: _ =>
      match driver.source with
      | none => false
      | some source => decide (i=target) && same (OwnerEndpointWorker.command p a) request
          (.owner (.install revision (CustodyPublication.image source.publication.current)))
  | _ => false

theorem head_payment_exact : headPaymentCheck driver target request = true ↔ HeadPayment driver target request := by
  cases suffix : driver.suffix with
  | nil => simp [headPaymentCheck,HeadPayment,suffix]
  | cons first rest =>
    cases first <;> simp [headPaymentCheck,HeadPayment,suffix,same_exact]
    all_goals cases present : driver.source <;> try simp [present,same_exact]
    all_goals
      constructor
      · rintro ⟨targetAt,requestAt⟩
        exact ⟨_,⟨targetAt,rfl⟩,requestAt⟩
      · rintro ⟨revision,⟨targetAt,rfl⟩,requestAt⟩
        exact ⟨targetAt,requestAt⟩

structure OwnerNext (assigned : SourceInput.Assignment p a) (scope : Nat)
    (bootstrap : SourceInput.Bootstrap a) (capacity : Fin p → Nat)
    (before : Live p a) (target : Fin p) (request : OwnerEndpoint.Command p a) where
  val : Live p a
  step : Step assigned scope bootstrap capacity (some before) (.owner target request) (some val)

def owner {p a : Nat} {assigned : SourceInput.Assignment p a} {scope : Nat}
    {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat}
    (s : Live p a) (target : Fin p) (request : OwnerEndpoint.Command p a) :
    Option (OwnerNext assigned scope bootstrap capacity s target request) := by
  let result := OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target) (s.owners target) request
  have evaluated : OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target) (s.owners target) request = (result.1,result.2) := (Prod.eta result).symm
  match mode : s.mode with
  | .prelude owed =>
    match present : s.driver.source with
    | none => exact none
    | some source =>
      if matching : same (OwnerEndpointWorker.command p a) request
          (.owner (.initialize (CustodyPublication.image source.publication.current))) = true then
        have equal := same_exact.mp matching
        if due : owed target = true then
          if accepted : result.2 = .inl 10 then
            have ran : OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target) (s.owners target)
                (.owner (.initialize (CustodyPublication.image source.publication.current))) = (result.1,.inl 10) := by
              rw [←accepted]; simpa only [←equal] using evaluated
            exact some ⟨ownerResult s target result.1 (.prelude (fun i => if i=target then false else owed i)),by
              simpa only [equal] using (Step.initialize mode due present ran)⟩
          else exact none
        else
          if slack : 1+PublicationOwnerBudget.demand s.driver.suffix target ≤ credits s target then
            have done : owed target = false := by cases h : owed target <;> simp_all
            have ran : OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target) (s.owners target)
                (.owner (.initialize (CustodyPublication.image source.publication.current))) = (result.1,result.2) := by simpa only [←equal] using evaluated
            exact some ⟨ownerResult s target result.1 (.prelude owed),by
              simpa only [equal] using (Step.extraInitialize (bootstrap:=bootstrap) (next:=result.1) (reply:=result.2) mode done present slack ran)⟩
          else exact none
      else exact none
  | .ordinary =>
    if head : headPaymentCheck s.driver target request = true then
      match present : s.driver.source with
      | none => exact none
      | some source =>
        match atHead : s.driver.suffix with
        | .freeze i revision :: rest =>
          if atTarget : i=target then
            let pending : PaidHeadPhase.Pending p := ⟨true,target,revision,s.ordinal⟩
            if matching : same (OwnerEndpointWorker.command p a) request (.freeze revision) = true then
              if accepted : result.2 = .inl 12 then
                have equal := same_exact.mp matching
                have ran : OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target) (s.owners target)
                    (paymentRequest pending source) = (result.1,paymentReply pending) := by
                  simp only [paymentRequest,paymentReply,pending,ite_true]
                  rw [←accepted]; simpa only [←equal] using evaluated
                have headAt : s.driver.suffix = PaidHeadPhase.command pending :: rest := by
                  simpa [PaidHeadPhase.command,pending,atTarget] using atHead
                exact some ⟨ownerResult s target result.1 (.paid pending),by
                  simpa only [paymentRequest,pending,ite_true,equal] using (Step.pay mode present headAt rfl ran)⟩
              else exact none
            else exact none
          else exact none
        | .install i revision :: rest =>
          if atTarget : i=target then
            let pending : PaidHeadPhase.Pending p := ⟨false,target,revision,s.ordinal⟩
            if matching : same (OwnerEndpointWorker.command p a) request
                (.owner (.install revision (CustodyPublication.image source.publication.current))) = true then
              if accepted : result.2 = .inl 7 then
                have equal := same_exact.mp matching
                have ran : OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target) (s.owners target)
                    (paymentRequest pending source) = (result.1,paymentReply pending) := by
                  simp only [paymentRequest,paymentReply,pending,Bool.false_eq_true,ite_false]
                  rw [←accepted]; simpa only [←equal] using evaluated
                have headAt : s.driver.suffix = PaidHeadPhase.command pending :: rest := by
                  simpa [PaidHeadPhase.command,pending,atTarget] using atHead
                exact some ⟨ownerResult s target result.1 (.paid pending),by
                  simpa only [paymentRequest,pending,Bool.false_eq_true,ite_false,equal] using (Step.pay mode present headAt rfl ran)⟩
              else exact none
            else exact none
          else exact none
        | _ => exact none
    else
      if admin : PublicOwnerReplay.adminCheck request = true then
        if slack : 1+PublicationOwnerBudget.demand s.driver.suffix target ≤ credits s target then
          exact some ⟨ownerResult s target result.1 .ordinary,
            .administration mode (PublicOwnerReplay.admin_exact.mp admin) (fun h => head (head_payment_exact.mpr h)) slack rfl⟩
        else exact none
      else exact none
  | .entered dispatch =>
    if atTarget : target=dispatch.endpoint then
      if matching : same (OwnerEndpointWorker.command p a) request (.owner (.reserve dispatch.ticket)) = true then
        if accepted : result.2 = .inl 6 then
          have equal := same_exact.mp matching
          have ran : OwnerEndpointBudget.transition ⟨assigned.realm,dispatch.endpoint⟩ scope (capacity dispatch.endpoint)
              (s.owners dispatch.endpoint) (.owner (.reserve dispatch.ticket)) = (result.1,.inl 6) := by
            rw [←accepted]; simpa only [←equal,←atTarget] using evaluated
          exact some ⟨ownerResult s target result.1 (.reserved dispatch),by
            simpa only [equal,atTarget] using (Step.reserve mode ran)⟩
        else exact none
      else exact none
    else exact none
  | .reserved dispatch =>
    if atTarget : target=dispatch.endpoint then
      match shape : request with
      | .freeze revision =>
        if accepted : result.2 = .inl 2 then
          have ran : OwnerEndpointBudget.transition ⟨assigned.realm,dispatch.endpoint⟩ scope (capacity dispatch.endpoint)
              (s.owners dispatch.endpoint) (.freeze revision) = (result.1,.inl 2) := by
            rw [←accepted]; simpa only [atTarget,shape] using evaluated
          exact some ⟨ownerResult s target result.1 (.probed dispatch),by
            simpa only [atTarget,shape] using (Step.probe mode ran)⟩
        else exact none
      | _ => exact none
    else exact none
  | .probed dispatch =>
    if atTarget : target=dispatch.endpoint then
      if matching : same (OwnerEndpointWorker.command p a) request (.owner .compute) = true then
        match produced : result.2 with
        | .inl _ => exact none
        | .inr envelope =>
          if correlated : envelope.ticket=dispatch.ticket ∧ envelope.scopeId=dispatch.context.scopeId ∧ envelope.revision=dispatch.context.revision then
            have equal := same_exact.mp matching
            have ran : OwnerEndpointBudget.transition ⟨assigned.realm,dispatch.endpoint⟩ scope (capacity dispatch.endpoint)
                (s.owners dispatch.endpoint) (.owner .compute) = (result.1,.inr envelope) := by
              rw [←produced]; simpa only [←equal,←atTarget] using evaluated
            exact some ⟨ownerResult s target result.1 (.computed dispatch envelope),by
              simpa only [equal,atTarget] using (Step.compute mode ran correlated.1 correlated.2.1 correlated.2.2)⟩
          else exact none
      else exact none
    else exact none
  | _ => exact none

#print axioms head_payment_exact
#print axioms owner

theorem owner_complete {p a : Nat} {assigned : SourceInput.Assignment p a} {scope : Nat}
    {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat}
    {s next : Live p a} {target : Fin p} {request : OwnerEndpoint.Command p a}
    (step : Step assigned scope bootstrap capacity (some s) (.owner target request) (some next)) :
    (owner (assigned:=assigned) (scope:=scope) (bootstrap:=bootstrap) (capacity:=capacity) s target request).map (fun c => c.val) = some next := by
  cases step with
  | «initialize» mode due present ran =>
    rcases s with ⟨⟨source,remaining,suffix⟩,owners,ordinal,phase⟩
    dsimp only at mode present ran
    subst phase; subst source
    simp [owner,due,ran,same_exact]
  | extraInitialize mode done present slack ran =>
    rcases s with ⟨⟨source,remaining,suffix⟩,owners,ordinal,phase⟩
    dsimp only at mode present ran
    subst phase; subst source
    simp [owner,done,slack,ran,same_exact]
  | administration mode admin offhead slack ran =>
    have notHead : headPaymentCheck s.driver target request ≠ true := fun h => offhead (head_payment_exact.mp h)
    have checked := PublicOwnerReplay.admin_exact.mpr admin
    rcases s with ⟨driver,owners,ordinal,phase⟩
    try dsimp only at mode ran
    subst phase
    simp [owner,notHead,checked,slack,ran]
  | @pay source pending rest next s mode present head atOrdinal ran =>
    rcases s with ⟨⟨sourceAt,remaining,suffix⟩,owners,ordinal,phase⟩
    rcases pending with ⟨freeze,target,revision,paidOrdinal⟩
    try dsimp only at mode present atOrdinal
    subst phase; subst sourceAt; subst paidOrdinal
    cases freeze <;> dsimp [PaidHeadPhase.command] at head
    all_goals subst suffix; simp [owner,headPaymentCheck,paymentRequest,paymentReply,same_exact] at ran ⊢; simp [ran]
  | reserve mode ran =>
    rcases s with ⟨driver,owners,ordinal,phase⟩
    try dsimp only at mode ran
    subst phase
    simp [owner,ran,same_exact]
  | probe mode ran =>
    rcases s with ⟨driver,owners,ordinal,phase⟩
    try dsimp only at mode ran
    subst phase
    simp [owner,ran]
  | @compute next envelope s dispatch mode ran ticket scopeAt revision =>
    rcases s with ⟨driver,owners,ordinal,phase⟩
    try dsimp only at mode ran
    subst phase
    simp [owner,ran,same_exact,ticket,scopeAt,revision]
    split
    · rename_i code atReply
      simp [ran] at atReply
    · rename_i actualEnvelope atReply
      have equal : actualEnvelope = envelope := by simpa [ran] using atReply.symm
      subst actualEnvelope
      simp [ticket,scopeAt,revision]

#print axioms owner_complete

instance (mode : Mode p) : Decidable (QueryAllowed mode) := by
  cases mode <;> unfold QueryAllowed <;> infer_instance

def query {p a : Nat} {assigned : SourceInput.Assignment p a} {scope : Nat}
    {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat}
    (s : Live p a) (request : Sum (SourceFundingQuery.HeadRequest p) (Fin p)) :
    Option {next : Live p a // Step assigned scope bootstrap capacity (some s) (.query request) (some next)} :=
  if allowed : QueryAllowed s.mode then some ⟨{s with ordinal := s.ordinal+1},.query allowed⟩ else none

-- This named local action is the prelude-completion rule. The unrelated frame
-- rule also uses Event.local; no determinism/completeness for their union is claimed.
def preludeDone {p a : Nat} {assigned : SourceInput.Assignment p a} {scope : Nat}
    {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} (s : Live p a) :
    Option {next : Live p a // Step assigned scope bootstrap capacity (some s) .local (some next)} :=
  match mode : s.mode with
  | .prelude owed =>
    if done : ∀ i, owed i = false then some ⟨{s with mode := .ordinary},.preludeDone mode done⟩ else none
  | _ => none

theorem query_complete {p a : Nat} {assigned : SourceInput.Assignment p a} {scope : Nat}
    {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {s next : Live p a}
    {request : Sum (SourceFundingQuery.HeadRequest p) (Fin p)}
    (step : Step assigned scope bootstrap capacity (some s) (.query request) (some next)) :
    (query (assigned:=assigned) (scope:=scope) (bootstrap:=bootstrap) (capacity:=capacity) s request).map (fun c => c.val) = some next := by
  cases step with
  | query allowed => simp [query,allowed]

theorem prelude_done_complete {p a : Nat} {assigned : SourceInput.Assignment p a} {scope : Nat}
    {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {s : Live p a}
    {owed : Fin p → Bool} (mode : s.mode = .prelude owed) (done : ∀ i, owed i = false) :
    (preludeDone (assigned:=assigned) (scope:=scope) (bootstrap:=bootstrap) (capacity:=capacity) s).map (fun c => c.val) = some {s with mode := .ordinary} := by
  rcases s with ⟨driver,owners,ordinal,phase⟩
  dsimp at mode
  subst phase
  simp [preludeDone,done]

-- Physical equality is about the full native state, not an equal public snapshot.
-- It supplies the next consumer's common-state composition obligation.
theorem source_physics
    (valid : PublicationLifecycle.Invariant assigned scope bootstrap s.driver)
    (step : Step assigned scope bootstrap capacity (some s) (.source request) (some next)) :
    next.driver = (SourceFundingInput.execute assigned scope bootstrap s.driver request).1 ∧
    next.owners = s.owners ∧ next.ordinal = s.ordinal+1 := by
  cases step with
  | launch mode fresh bound ran => simp [sourceResult,ran]
  | source mode allowedPublic bound ran => simp [sourceResult,ran]
  | refused mode bound refused =>
    simp [SourceFundingWork.known_refusal_frames valid refused]
  | notify mode allowed bound ran => simp [sourceResult,ran]
  | enter mode bound ran present dispatched head => simp [sourceResult,ran]
  | finish mode bound ran => simp [sourceResult,ran]

theorem owner_physics
    (step : Step assigned scope bootstrap capacity (some s) (.owner target request) (some next)) :
    next.driver = s.driver ∧
    next.owners = PublicOwnerBoundary.put s.owners target
      (OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target) (s.owners target) request).1 ∧
    next.ordinal = s.ordinal := by
  cases step <;> simp_all [ownerResult]

#print axioms query_complete
#print axioms prelude_done_complete
#print axioms source_physics
#print axioms owner_physics


#print axioms source
#print axioms source_preserves
end MirroreaProofFirst.CohortFundingReplay
