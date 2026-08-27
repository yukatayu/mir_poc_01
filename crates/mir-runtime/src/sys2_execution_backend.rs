//! SYS-2 bounded execution backends.
//!
//! OW1 has one coordinator and one owner worker.  The worker exclusively owns
//! the mutable M8 local runtime and communicates through a zero-capacity
//! synchronous mailbox; no shared mutable M8 state is exposed.

use std::{
    collections::VecDeque,
    sync::mpsc::{self, Receiver, SyncSender},
    thread::{self, JoinHandle},
};

use crate::{
    m8_runtime_authority::M8AuthorityState,
    m8_runtime_local_cut::{
        M8LocalDesignatedTraceContext, M8LocalRuntime, M8LocalTrace, M8LocalTraceKind,
        M8LocalTraceObservation,
    },
    m8_runtime_owner_queue::{
        M8EnqueueDiagnostics, M8Occurrence, M8OwnerRequest, M8ServeDiagnostics, M8ServeOutcome,
        M8StateKey,
    },
    semantic_runtime_kernel::{LocusRef, RequestIdentity},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExecutionProfile {
    St,
    Ow1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Ow1MailboxEvidence {
    target_owner: LocusRef,
    observed_request_order: Vec<RequestIdentity>,
}

impl Ow1MailboxEvidence {
    pub(crate) fn target_owner(&self) -> &LocusRef {
        &self.target_owner
    }

    pub(crate) const fn is_fifo(&self) -> bool {
        true
    }

    pub(crate) fn observed_request_order(&self) -> &[RequestIdentity] {
        &self.observed_request_order
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Ow1WorkerEvidence {
    target_owner: LocusRef,
    mailbox: Ow1MailboxEvidence,
    worker_token: String,
}

impl Ow1WorkerEvidence {
    pub(crate) fn target_owner(&self) -> &LocusRef {
        &self.target_owner
    }

    pub(crate) fn mailbox(&self) -> &Ow1MailboxEvidence {
        &self.mailbox
    }

    /// A debug-only identity captured from the dedicated OS thread.  It is
    /// evidence of which worker executed M8 commands, never an authority.
    pub(crate) fn worker_token(&self) -> &str {
        &self.worker_token
    }

    pub(crate) fn record_mailbox_request(&mut self, identity: RequestIdentity) {
        self.mailbox.observed_request_order.push(identity);
    }

    pub(crate) const fn owns_m8_runtime(&self) -> bool {
        true
    }

    pub(crate) const fn public_shared_store_surface(&self) -> Option<()> {
        None
    }

    pub(crate) fn debug_type_surface(&self) -> &'static str {
        "sync_channel(0) dedicated owner worker; M8LocalRuntime worker-owned"
    }
}

#[derive(Debug)]
pub(crate) enum Ow1WorkerFailure {
    Disconnected,
    WorkerPanicked,
    Enqueue(M8EnqueueDiagnostics),
    Serve(M8ServeDiagnostics),
    /// The coordinator attempted to attribute a kernel request to a worker
    /// FIFO entry other than the actual M8 head.  Do not serve in this case:
    /// any mutation would be misattributed.
    FifoIdentityMismatch,
}

/// Immutable worker acknowledgement of one actual M8 owner transition.
///
/// The coordinator never invents these node identifiers: the dedicated worker
/// reads them from `M8LocalRuntime` only after its acknowledged `serve` call.
/// `owner_write` is therefore the linearization/commit evidence for a
/// successful RMW, while a declared M8 failure has no such observation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Ow1M8ExecutionReceipt {
    request_occurrence: M8Occurrence,
    outcome: M8ServeOutcome,
    owner_read: Option<M8LocalTraceObservation>,
    owner_write: Option<M8LocalTraceObservation>,
    worker_token: String,
}

/// The result of the contextual owner command used after SYS-4 has dequeued
/// an exact generated carrier.  Unlike the legacy enqueue/serve pair, this
/// command carries its immutable carrier provenance directly into M8 and
/// returns the rows allocated by that worker-owned runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Ow1ContextualM8Execution {
    Served(Box<Ow1ContextualM8ExecutionReceipt>),
    Rejected {
        observation: Box<M8LocalTraceObservation>,
    },
}

/// Immutable acknowledgement of a contextual M8 owner execution.  The
/// request and serve observations are both allocated by the OW1 worker; the
/// coordinator only connects them to its already-dequeued carrier occurrence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Ow1ContextualM8ExecutionReceipt {
    outcome: M8ServeOutcome,
    request_observation: M8LocalTraceObservation,
    serve_observation: M8LocalTraceObservation,
}

impl Ow1ContextualM8ExecutionReceipt {
    pub(crate) fn outcome(&self) -> &M8ServeOutcome {
        &self.outcome
    }

    pub(crate) fn request_observation(&self) -> &M8LocalTraceObservation {
        &self.request_observation
    }

    pub(crate) fn serve_observation(&self) -> &M8LocalTraceObservation {
        &self.serve_observation
    }
}

impl Ow1M8ExecutionReceipt {
    pub(crate) fn request_occurrence(&self) -> &M8Occurrence {
        &self.request_occurrence
    }

    pub(crate) fn outcome(&self) -> &M8ServeOutcome {
        &self.outcome
    }

    pub(crate) fn owner_read(&self) -> Option<&M8LocalTraceObservation> {
        self.owner_read.as_ref()
    }

    pub(crate) fn owner_write(&self) -> Option<&M8LocalTraceObservation> {
        self.owner_write.as_ref()
    }

    pub(crate) fn worker_token(&self) -> &str {
        &self.worker_token
    }
}

enum Ow1Command {
    Enqueue {
        request: M8OwnerRequest,
        reply: SyncSender<Result<M8Occurrence, M8EnqueueDiagnostics>>,
    },
    Serve {
        owner_locus: String,
        expected_request_occurrence: M8Occurrence,
        reply: SyncSender<Result<Ow1M8ExecutionReceipt, Ow1WorkerFailure>>,
    },
    ExecuteOwnerWithContext {
        owner_locus: String,
        request: M8OwnerRequest,
        context: Box<M8LocalDesignatedTraceContext>,
        reply: SyncSender<Result<Ow1ContextualM8Execution, Ow1WorkerFailure>>,
    },
    ReadOwnerInt {
        key: M8StateKey,
        reply: SyncSender<Option<i64>>,
    },
    RefreshAuthority {
        authority_state: M8AuthorityState,
        reply: SyncSender<()>,
    },
    Snapshot {
        reply: SyncSender<M8LocalRuntime>,
    },
    TraceSnapshot {
        reply: SyncSender<M8LocalTrace>,
    },
    #[cfg(test)]
    ArmOwnerOperationRejection {
        context: M8LocalDesignatedTraceContext,
        reply: SyncSender<()>,
    },
    ShutdownExtract {
        reply: SyncSender<M8LocalRuntime>,
    },
}

pub(crate) struct Ow1WorkerBackend {
    owner: LocusRef,
    commands: SyncSender<Ow1Command>,
    join: Option<JoinHandle<()>>,
    worker_token: String,
}

impl Ow1WorkerBackend {
    pub(crate) fn spawn(owner: LocusRef, runtime: M8LocalRuntime) -> Self {
        let (commands, receiver) = mpsc::sync_channel(0);
        let join = thread::spawn(move || run_worker(receiver, runtime));
        let worker_token = format!("{:?}", join.thread().id());
        Self {
            owner,
            commands,
            join: Some(join),
            worker_token,
        }
    }

    pub(crate) fn evidence(&self) -> Ow1WorkerEvidence {
        Ow1WorkerEvidence {
            target_owner: self.owner.clone(),
            mailbox: Ow1MailboxEvidence {
                target_owner: self.owner.clone(),
                observed_request_order: Vec::new(),
            },
            worker_token: self.worker_token.clone(),
        }
    }

    pub(crate) fn worker_token(&self) -> &str {
        &self.worker_token
    }

    pub(crate) fn enqueue(
        &self,
        request: M8OwnerRequest,
    ) -> Result<M8Occurrence, Ow1WorkerFailure> {
        let (reply, receiver) = mpsc::sync_channel(0);
        self.send(Ow1Command::Enqueue { request, reply })?;
        receiver
            .recv()
            .map_err(|_| Ow1WorkerFailure::Disconnected)?
            .map_err(Ow1WorkerFailure::Enqueue)
    }

    pub(crate) fn serve_next(
        &self,
        owner_locus: &str,
        expected_request_occurrence: M8Occurrence,
    ) -> Result<Ow1M8ExecutionReceipt, Ow1WorkerFailure> {
        let (reply, receiver) = mpsc::sync_channel(0);
        self.send(Ow1Command::Serve {
            owner_locus: owner_locus.to_string(),
            expected_request_occurrence,
            reply,
        })?;
        receiver
            .recv()
            .map_err(|_| Ow1WorkerFailure::Disconnected)?
    }

    /// Execute the contextual owner operation entirely inside the worker.
    /// The caller has already performed endpoint dequeue and M9 admission; it
    /// cannot choose M8 rows or reconstruct carrier provenance afterward.
    pub(crate) fn execute_owner_with_context(
        &self,
        owner_locus: &str,
        request: M8OwnerRequest,
        context: M8LocalDesignatedTraceContext,
    ) -> Result<Ow1ContextualM8Execution, Ow1WorkerFailure> {
        let (reply, receiver) = mpsc::sync_channel(0);
        self.send(Ow1Command::ExecuteOwnerWithContext {
            owner_locus: owner_locus.to_string(),
            request,
            context: Box::new(context),
            reply,
        })?;
        receiver
            .recv()
            .map_err(|_| Ow1WorkerFailure::Disconnected)?
    }

    pub(crate) fn read_owner_int(&self, key: M8StateKey) -> Result<Option<i64>, Ow1WorkerFailure> {
        let (reply, receiver) = mpsc::sync_channel(0);
        self.send(Ow1Command::ReadOwnerInt { key, reply })?;
        receiver.recv().map_err(|_| Ow1WorkerFailure::Disconnected)
    }

    /// The acknowledgement is returned only after the sole M8 owner has
    /// installed the new sealed inventory.
    pub(crate) fn refresh_authority_and_ack(
        &self,
        authority_state: M8AuthorityState,
    ) -> Result<(), Ow1WorkerFailure> {
        let (reply, receiver) = mpsc::sync_channel(0);
        self.send(Ow1Command::RefreshAuthority {
            authority_state,
            reply,
        })?;
        receiver.recv().map_err(|_| Ow1WorkerFailure::Disconnected)
    }

    pub(crate) fn snapshot(&self) -> Result<M8LocalRuntime, Ow1WorkerFailure> {
        let (reply, receiver) = mpsc::sync_channel(0);
        self.send(Ow1Command::Snapshot { reply })?;
        receiver.recv().map_err(|_| Ow1WorkerFailure::Disconnected)
    }

    /// Obtain the worker-owned local trace for observer/devtools projection.
    /// This is a clone-only snapshot; the coordinator never receives mutable
    /// M8 state through this API.
    pub(crate) fn local_trace_snapshot(&self) -> Result<M8LocalTrace, Ow1WorkerFailure> {
        let (reply, receiver) = mpsc::sync_channel(0);
        self.send(Ow1Command::TraceSnapshot { reply })?;
        receiver.recv().map_err(|_| Ow1WorkerFailure::Disconnected)
    }

    #[cfg(test)]
    pub(crate) fn arm_owner_operation_rejection(
        &self,
        context: M8LocalDesignatedTraceContext,
    ) -> Result<(), Ow1WorkerFailure> {
        let (reply, receiver) = mpsc::sync_channel(0);
        self.send(Ow1Command::ArmOwnerOperationRejection { context, reply })?;
        receiver.recv().map_err(|_| Ow1WorkerFailure::Disconnected)
    }

    pub(crate) fn shutdown_extract(mut self) -> Result<M8LocalRuntime, Ow1WorkerFailure> {
        let (reply, receiver) = mpsc::sync_channel(0);
        self.commands
            .send(Ow1Command::ShutdownExtract { reply })
            .map_err(|_| Ow1WorkerFailure::Disconnected)?;
        let runtime = receiver
            .recv()
            .map_err(|_| Ow1WorkerFailure::Disconnected)?;
        if self.join.take().is_some_and(|join| join.join().is_err()) {
            return Err(Ow1WorkerFailure::WorkerPanicked);
        }
        Ok(runtime)
    }

    fn send(&self, command: Ow1Command) -> Result<(), Ow1WorkerFailure> {
        self.commands
            .send(command)
            .map_err(|_| Ow1WorkerFailure::Disconnected)
    }
}

impl Drop for Ow1WorkerBackend {
    fn drop(&mut self) {
        let Some(join) = self.join.take() else {
            return;
        };
        let (reply, receiver) = mpsc::sync_channel(0);
        if self
            .commands
            .send(Ow1Command::ShutdownExtract { reply })
            .is_ok()
        {
            let _ = receiver.recv();
        }
        let _ = join.join();
    }
}

fn run_worker(receiver: Receiver<Ow1Command>, mut runtime: M8LocalRuntime) {
    // This local FIFO records M8's own issued occurrence identity.  It is
    // deliberately worker-owned: the coordinator may validate carriers but
    // cannot choose a later M8 request to execute.
    let mut pending_m8_requests = VecDeque::new();
    while let Ok(command) = receiver.recv() {
        match command {
            Ow1Command::Enqueue { request, reply } => {
                let outcome = runtime.enqueue_owner(request);
                if let Ok(occurrence) = &outcome {
                    pending_m8_requests.push_back(occurrence.clone());
                }
                let _ = reply.send(outcome);
            }
            Ow1Command::Serve {
                owner_locus,
                expected_request_occurrence,
                reply,
            } => {
                let Some(head) = pending_m8_requests.front().cloned() else {
                    let _ = reply.send(Err(Ow1WorkerFailure::FifoIdentityMismatch));
                    continue;
                };
                if head != expected_request_occurrence {
                    let _ = reply.send(Err(Ow1WorkerFailure::FifoIdentityMismatch));
                    continue;
                }
                let outcome = runtime
                    .serve_next_owner(&owner_locus)
                    .map_err(Ow1WorkerFailure::Serve);
                // M8 removes its FIFO head before either success or declared
                // service failure.  The worker mirror advances only after the
                // actual M8 call returns an acknowledged result.
                if outcome.is_ok() || matches!(&outcome, Err(Ow1WorkerFailure::Serve(_))) {
                    let _ = pending_m8_requests.pop_front();
                }
                let receipt = outcome.map(|outcome| {
                    let trace = runtime.trace();
                    Ow1M8ExecutionReceipt {
                        request_occurrence: head.clone(),
                        owner_read: trace.latest_observation_for_occurrence(
                            M8LocalTraceKind::OwnerRead,
                            head.id(),
                        ),
                        owner_write: trace.latest_observation_for_occurrence(
                            M8LocalTraceKind::OwnerWrite,
                            head.id(),
                        ),
                        outcome,
                        worker_token: format!("{:?}", thread::current().id()),
                    }
                });
                let _ = reply.send(receipt);
            }
            Ow1Command::ExecuteOwnerWithContext {
                owner_locus,
                request,
                context,
                reply,
            } => {
                // Contextual SYS-4 execution cannot bypass or interleave with
                // a legacy M8 FIFO head.  This check and the following M8
                // invocation share the worker turn, so no coordinator-side
                // race can attach B's carrier context to A's pending state.
                if !pending_m8_requests.is_empty() {
                    let _ = reply.send(Err(Ow1WorkerFailure::FifoIdentityMismatch));
                    continue;
                }
                let result =
                    match runtime.execute_owner_with_context(&owner_locus, request, *context) {
                        Ok((outcome, request_observation, serve_observation)) => {
                            Ow1ContextualM8Execution::Served(Box::new(
                                Ow1ContextualM8ExecutionReceipt {
                                    outcome,
                                    request_observation,
                                    serve_observation,
                                },
                            ))
                        }
                        Err(observation) => Ow1ContextualM8Execution::Rejected { observation },
                    };
                let _ = reply.send(Ok(result));
            }
            Ow1Command::ReadOwnerInt { key, reply } => {
                let _ = reply.send(runtime.owner_state().int(&key));
            }
            Ow1Command::RefreshAuthority {
                authority_state,
                reply,
            } => {
                runtime.refresh_m9_authority_state(authority_state);
                let _ = reply.send(());
            }
            Ow1Command::Snapshot { reply } => {
                let _ = reply.send(runtime.clone());
            }
            Ow1Command::TraceSnapshot { reply } => {
                let _ = reply.send(runtime.trace());
            }
            #[cfg(test)]
            Ow1Command::ArmOwnerOperationRejection { context, reply } => {
                runtime.arm_owner_operation_rejection(context);
                let _ = reply.send(());
            }
            Ow1Command::ShutdownExtract { reply } => {
                let _ = reply.send(runtime);
                break;
            }
        }
    }
}
