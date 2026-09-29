import OwnerStatementRegistryInvariant
namespace MirroreaProofFirst.OwnerStatementLiveCustodian

-- This discriminator denotes a designated live custodian, not an authority or
-- serialized object ID. Uniqueness and exclusive physical ownership are TCB
-- obligations; copying this mathematical State is NOT an admitted new owner.
structure Ticket where
 designation : Nat
 serial : Nat
 saved : OwnerSavedPending.Saved
 deriving DecidableEq
inductive Phase where
 | queued | unreported | reported
 deriving DecidableEq
structure Held where
 ticket : Ticket
 phase : Phase
 deriving DecidableEq
structure State (p a : Nat) where
 designation : Nat
 live : OwnerStatementRegistrySelection.State p a
 nextSerial : Nat
 held : Option Held
 dispatched : List Ticket

def start (designation : Nat) (live : OwnerStatementRegistrySelection.State p a) : State p a :=
 ⟨designation,live,0,none,[]⟩

-- Stage only the real live queue. No candidate executable state is accepted.
-- Budget pays a retained serial slot; reporting loss never reclaims it.
def stage (capacity : Nat) (s : State p a) : Option (State p a × Ticket) := do
 if s.held.isSome || !(s.nextSerial < capacity) then none else do
 let saved ← s.live.session.state.owner.queued
 let ticket := Ticket.mk s.designation s.nextSerial saved
 return ({s with nextSerial := s.nextSerial+1, held := some ⟨ticket,.queued⟩},ticket)

def Staged (capacity : Nat) (s next : State p a) (ticket : Ticket) : Prop :=
 s.held = none ∧ s.nextSerial < capacity ∧
 s.live.session.state.owner.queued = some ticket.saved ∧
 ticket.designation = s.designation ∧ ticket.serial = s.nextSerial ∧
 next = {s with nextSerial := s.nextSerial+1,held := some ⟨ticket,.queued⟩}

theorem stage_exact : stage capacity s = some (next,ticket) ↔ Staged capacity s next ticket := by
 unfold stage Staged
 by_cases idle : s.held = none
 · by_cases room : s.nextSerial < capacity
   · simp only [idle,Option.isSome_none,Bool.false_or,room,decide_true,Bool.not_true,
       Bool.false_eq_true,↓reduceIte,Option.bind_eq_bind]
     cases found : s.live.session.state.owner.queued with
     | none => simp
     | some saved =>
       simp only [Option.bind_some,Option.pure_def,Option.some.injEq,Prod.mk.injEq]
       constructor
       · rintro ⟨rfl,rfl⟩; simp
       · rintro ⟨_,_,same,d,serial,rfl⟩
         cases ticket
         simp_all
   · simp [idle,room]
 · have occupied : s.held.isSome = true := by cases h : s.held <;> simp_all
   simp [occupied,idle]

-- Only independently checkable current metadata and actual source/queue
-- custody open the body route. Authorization of the operation is still
-- performed by the original service; this guard cannot issue it.
def currentCheck (s : OwnerStatementRegistrySelection.State p a) : Bool :=
 match s.packet,s.session.state.owner.queued with
 | some packet,some saved =>
   match OwnerStatementRegistrySelection.select s.session s.bank saved with
   | none => false
   | some (place,registry) => decide (packet.owner = place.val) &&
     OwnerStatementEntryGate.serviceCheck (OwnerStatementRegistrySelection.view s registry) &&
     MixedOwnerSourceTrace.serviceReady s.session.state
 | _,_ => false

def Ready (s : State p a) (ticket : Ticket) : Prop :=
 ticket.designation = s.designation ∧ s.held = some ⟨ticket,.queued⟩ ∧
 s.live.session.state.owner.queued = some ticket.saved ∧ currentCheck s.live = true

def execute (s : State p a) (ticket : Ticket) (ops : FallibleFlow.Arithmetic) : Option (State p a) :=
 if ticket.designation = s.designation ∧ s.held = some ⟨ticket,.queued⟩ ∧
   s.live.session.state.owner.queued = some ticket.saved ∧ currentCheck s.live = true then
 some {s with
   live := OwnerStatementRegistrySelection.service s.live ops
   held := some ⟨ticket,.unreported⟩
   dispatched := ticket :: s.dispatched}
 else none

def Executed (s next : State p a) (ticket : Ticket) (ops : FallibleFlow.Arithmetic) : Prop :=
 Ready s ticket ∧ next = {s with
   live := OwnerStatementRegistrySelection.service s.live ops
   held := some ⟨ticket,.unreported⟩
   dispatched := ticket::s.dispatched}

theorem execute_exact : execute s ticket ops = some next ↔ Executed s next ticket ops := by
 unfold execute
 split
 · rename_i ready
   constructor
   · intro same; exact ⟨ready,(Option.some.inj same).symm⟩
   · rintro ⟨_,rfl⟩; rfl
 · rename_i denied
   constructor
   · intro impossible; cases impossible
   · rintro ⟨ready,_⟩; exact False.elim (denied ready)

-- Relative completeness is a real call to the original service on the owned
-- live state. It does not assume an accepted wrapper invocation or commit.
theorem execute_complete (ready : Ready s ticket) :
 execute s ticket ops = some {s with
   live := OwnerStatementRegistrySelection.service s.live ops
   held := some ⟨ticket,.unreported⟩
   dispatched := ticket::s.dispatched} :=
 execute_exact.mpr ⟨ready,rfl⟩

theorem wrong_designation (wrong : ticket.designation ≠ s.designation) : execute s ticket ops = none := by
 simp [execute,wrong]
theorem wrong_queue (wrong : s.live.session.state.owner.queued ≠ some ticket.saved) : execute s ticket ops = none := by
 simp [execute,wrong]
theorem stale (old : currentCheck s.live = false) : execute s ticket ops = none := by simp [execute,old]

theorem executed_no_repeat (run : execute s ticket ops = some next) : execute next other moreOps = none := by
 obtain ⟨_,rfl⟩ := execute_exact.mp run
 simp [execute,Held.mk.injEq]

-- Response construction/transport follows an actual dispatch. A failed or
-- missing report leaves unreported held, not a fabricated no-effect reply.
def report (s : State p a) (ticket : Ticket) : Option (State p a) :=
 if s.held = some ⟨ticket,.unreported⟩ then some {s with held := some ⟨ticket,.reported⟩} else none

def accept (s : State p a) (ticket : Ticket) : Option (State p a) :=
 if s.held = some ⟨ticket,.reported⟩ then some {s with held := none} else none

theorem report_exact : report s ticket = some next ↔
 s.held = some ⟨ticket,.unreported⟩ ∧ next = {s with held := some ⟨ticket,.reported⟩} := by
 unfold report; split <;> simp_all [eq_comm]
theorem accept_exact : accept s ticket = some next ↔
 s.held = some ⟨ticket,.reported⟩ ∧ next = {s with held := none} := by
 unfold accept; split <;> simp_all [eq_comm]

theorem unreported_no_accept (held : s.held = some ⟨ticket,.unreported⟩) : accept s other = none := by
 simp [accept,held,Held.mk.injEq]
theorem held_no_stage (held : s.held = some lease) : stage capacity s = none := by simp [stage,held]
theorem exhausted (full : capacity ≤ s.nextSerial) : stage capacity s = none := by
 have no : ¬s.nextSerial < capacity := by omega
 simp [stage,no]

-- This bounds only custody serial/history slots, not total source/queue/trace
-- storage, string sizes, CPU, numeric overflow or transport buffers.
def reserved (s : State p a) : Nat :=
 match s.held with | some ⟨_,.queued⟩ => 1 | _ => 0
def Bounded (capacity : Nat) (s : State p a) : Prop :=
 s.nextSerial ≤ capacity ∧ s.dispatched.length + reserved s ≤ s.nextSerial

theorem start_bounded : Bounded capacity (start designation live) := by simp [Bounded,start,reserved]
theorem stage_bounded (bounded : Bounded capacity s) (run : stage capacity s = some (next,ticket)) : Bounded capacity next := by
 obtain ⟨idle,room,_,_,_,rfl⟩ := stage_exact.mp run
 have old := bounded.2
 simp only [reserved,idle] at old
 simp only [Bounded,reserved]
 omega

theorem execute_bounded (bounded : Bounded capacity s) (run : execute s ticket ops = some next) : Bounded capacity next := by
 obtain ⟨ready,rfl⟩ := execute_exact.mp run
 have old := bounded.2
 simp only [reserved,ready.2.1] at old
 simpa only [Bounded,reserved,List.length_cons,Nat.add_zero] using And.intro bounded.1 old

theorem report_bounded (bounded : Bounded capacity s) (run : report s ticket = some next) : Bounded capacity next := by
 obtain ⟨held,rfl⟩ := report_exact.mp run
 simpa only [Bounded,reserved,held] using bounded

theorem accept_bounded (bounded : Bounded capacity s) (run : accept s ticket = some next) : Bounded capacity next := by
 obtain ⟨held,rfl⟩ := accept_exact.mp run
 simpa only [Bounded,reserved,held] using bounded

-- A local lifecycle, not authentication/import or all physical mutator closure.
-- Later mutation routes must explicitly refine this interface or remain refused.
inductive Path (capacity : Nat) (initial : State p a) : State p a → Prop where
 | initial : Path capacity initial initial
 | staged : Path capacity initial s → stage capacity s = some (next,ticket) → Path capacity initial next
 | executed : Path capacity initial s → execute s ticket ops = some next → Path capacity initial next
 | reported : Path capacity initial s → report s ticket = some next → Path capacity initial next
 | accepted : Path capacity initial s → accept s ticket = some next → Path capacity initial next

theorem path_bounded (startBounded : Bounded capacity initial) (path : Path capacity initial s) : Bounded capacity s := by
 induction path with
 | initial => exact startBounded
 | staged _ run ih => exact stage_bounded ih run
 | executed _ run ih => exact execute_bounded ih run
 | reported _ run ih => exact report_bounded ih run
 | accepted _ run ih => exact accept_bounded ih run

theorem path_refines (path : Path capacity initial s) : OwnerStatementRegistrySelection.Rooted initial.live s.live := by
 induction path with
 | initial => exact .initial
 | staged _ run ih => obtain ⟨_,_,_,_,_,rfl⟩ := stage_exact.mp run; exact ih
 | executed _ run ih => obtain ⟨_,rfl⟩ := execute_exact.mp run; exact .service ih
 | reported _ run ih => obtain ⟨_,rfl⟩ := report_exact.mp run; exact ih
 | accepted _ run ih => obtain ⟨_,rfl⟩ := accept_exact.mp run; exact ih

theorem path_admitted (initialAdmitted : OwnerStatementRegistryInvariant.Admitted realm authority policy store initial.live)
 (path : Path capacity initial s) : OwnerStatementRegistryInvariant.Admitted realm authority policy store s.live := by
 obtain ⟨origin,entered,prior⟩ := initialAdmitted
 exact ⟨origin,entered,OwnerStatementRegistrySelection.rooted_trans prior (path_refines path)⟩

-- Retained dispatch serials are globally distinct along this ONE designated
-- live lifecycle, including report/accept cycles. This is stronger than an
-- immediate replay test, and still does not authorize copying the live state.
def Serials (s : State p a) : Prop :=
 (s.dispatched.map Ticket.serial).Nodup ∧
 (∀ ticket ∈ s.dispatched, ticket.serial < s.nextSerial) ∧
 (∀ held, s.held = some held → held.ticket.serial < s.nextSerial ∧
   (held.phase = .queued → ∀ old ∈ s.dispatched, old.serial < held.ticket.serial))

theorem start_serials : Serials (start designation live) := by simp [Serials,start]

theorem stage_serials (valid : Serials s) (run : stage capacity s = some (next,ticket)) : Serials next := by
 obtain ⟨idle,_,_,_,serial,rfl⟩ := stage_exact.mp run
 refine ⟨valid.1,?_,?_⟩
 · intro old member; have bound := valid.2.1 old member; dsimp only; omega
 · intro held found
   have equal : held = ⟨ticket,.queued⟩ := (Option.some.inj found).symm
   subst held
   dsimp only
   constructor
   · omega
   · intro _ old member; simpa only [serial] using valid.2.1 old member

theorem execute_serials (valid : Serials s) (run : execute s ticket ops = some next) : Serials next := by
 obtain ⟨ready,rfl⟩ := execute_exact.mp run
 have bound := valid.2.2 ⟨ticket,.queued⟩ ready.2.1
 have fresh : ticket.serial ∉ s.dispatched.map Ticket.serial := by
   intro member
   obtain ⟨old,held,equal⟩ := List.mem_map.mp member
   have before := bound.2 rfl old held
   dsimp only at before
   omega
 refine ⟨by simpa only [List.map_cons,List.nodup_cons] using And.intro fresh valid.1,?_,?_⟩
 · intro old member
   rcases List.mem_cons.mp member with same | rest
   · subst old; exact bound.1
   · exact valid.2.1 old rest
 · intro held found
   have equal : held = ⟨ticket,.unreported⟩ := (Option.some.inj found).symm
   subst held
   exact ⟨bound.1,by intro impossible; cases impossible⟩

theorem report_serials (valid : Serials s) (run : report s ticket = some next) : Serials next := by
 obtain ⟨held,rfl⟩ := report_exact.mp run
 refine ⟨valid.1,valid.2.1,?_⟩
 intro other found
 have same : other = ⟨ticket,.reported⟩ := (Option.some.inj found).symm
 subst other
 exact ⟨(valid.2.2 ⟨ticket,.unreported⟩ held).1,by intro impossible; cases impossible⟩

theorem accept_serials (valid : Serials s) (run : accept s ticket = some next) : Serials next := by
 obtain ⟨_,rfl⟩ := accept_exact.mp run
 exact ⟨valid.1,valid.2.1,by intro held impossible; cases impossible⟩

theorem path_serials (initialValid : Serials initial) (path : Path capacity initial s) : Serials s := by
 induction path with
 | initial => exact initialValid
 | staged _ run ih => exact stage_serials ih run
 | executed _ run ih => exact execute_serials ih run
 | reported _ run ih => exact report_serials ih run
 | accepted _ run ih => exact accept_serials ih run

theorem no_duplicate_dispatch_serials (path : Path capacity (start designation live) s) :
 (s.dispatched.map Ticket.serial).Nodup := (path_serials start_serials path).1

#print axioms no_duplicate_dispatch_serials

#print axioms stage_exact
#print axioms execute_exact
#print axioms execute_complete
#print axioms executed_no_repeat
#print axioms path_bounded
#print axioms path_refines
#print axioms path_admitted
end MirroreaProofFirst.OwnerStatementLiveCustodian
