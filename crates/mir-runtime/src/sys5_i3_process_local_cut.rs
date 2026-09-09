//! Private Row20 process-local cut custody boundary.
//!
//! A whole-cohort factory issues the existing trusted image/control frames
//! with one private purpose.  The only consumer constructs a new runtime and
//! inspected QUIC session from those frames; it never attaches an existing
//! runtime, stream, session, or decoded control.

use crate::{
    sys5_i3_private_quic::{
        Sys5I3PrivateQuicError, Sys5I3PrivateQuicOriginalOwnerReplyOutcome,
        Sys5I3PrivateQuicPendingIngress, Sys5I3PrivateQuicProcessLocalCutRetainedOwnerReply,
        Sys5I3PrivateQuicSession,
    },
    sys5_i3_process_runtime::{
        Sys5I3OriginalOwnerRequestPending, Sys5I3PrivateProcessCodec,
        Sys5I3ProcessLocalCutAdmissionToken, Sys5I3ProcessLocalCutControlRole,
        Sys5I3ProcessMessage, Sys5I3ProcessRuntime, Sys5I3ProcessRuntimeErrorKind,
        Sys5I3TrustedLocalnetControl,
    },
};

#[cfg(all(test, feature = "i3-private-quic", feature = "i3-process-test-seams"))]
#[path = "sys5_i3_process_local_cut_tests.rs"]
mod sys5_i3_process_local_cut_tests;

/// Typed failures while deriving one Row20 custody launch from a real cohort.
/// No variant carries an image, control, authority, or source value.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys5I3ProcessLocalCutLaunchErrorKind {
    Unsupported,
    CohortInventoryIncomplete,
    StagedLifecycle,
    UnsupportedProviderProfile,
    StartBindingMismatch,
    BootstrapAlreadyTaken,
}

/// Opaque failure at the private Row20 launch boundary.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3ProcessLocalCutLaunchError {
    kind: Sys5I3ProcessLocalCutLaunchErrorKind,
}

impl Sys5I3ProcessLocalCutLaunchError {
    pub(crate) const fn new(kind: Sys5I3ProcessLocalCutLaunchErrorKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> Sys5I3ProcessLocalCutLaunchErrorKind {
        self.kind
    }
}

/// Whole-cohort, one-shot custody launch. Its fields remain private so a
/// caller cannot mix an image/control pair or attach a used child runtime.
///
/// ```rust
/// use mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessLocalCutLaunch;
///
/// fn consume_issued_bootstraps(mut launch: Sys5I3ProcessLocalCutLaunch) {
///     let _ = launch.take_requester_bootstrap();
///     let _ = launch.take_owner_bootstrap();
/// }
/// ```
///
/// ```compile_fail
/// use mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessLocalCutLaunch;
///
/// let _ = Sys5I3ProcessLocalCutLaunch::from_issued_frames(
///     Vec::new(), Vec::new(), Vec::new(), Vec::new(),
/// );
/// ```
///
/// ```compile_fail
/// use mir_runtime::sys5_i3_process_local_cut::Sys5I3ProcessLocalCutChildCapsule;
/// ```
///
/// ```compile_fail
/// use mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessRuntime;
///
/// let _ = Sys5I3ProcessRuntime::commit_i3_process_local_cut_after_initial_receipt;
/// ```
#[doc(hidden)]
pub struct Sys5I3ProcessLocalCutLaunch {
    requester: Option<Sys5I3ProcessLocalCutChildBootstrap>,
    owner: Option<Sys5I3ProcessLocalCutChildBootstrap>,
}

/// One child bootstrap derived only from a whole eligible custody launch.
#[doc(hidden)]
pub struct Sys5I3ProcessLocalCutChildBootstrap {
    frames: Option<Sys5I3ProcessLocalCutChildFrames>,
}

/// Existing inherited child image/control frames, kept paired until the
/// trusted process launcher consumes them.
#[doc(hidden)]
pub struct Sys5I3ProcessLocalCutChildFrames {
    image_frame: Vec<u8>,
    trusted_control_frame: Vec<u8>,
}

/// Slot-only completion returned by the runtime-owned Row20 child driver.
#[doc(hidden)]
pub struct Sys5I3ProcessLocalCutChildCompletion {
    _private: (),
}

/// Typed local custody/cut failure. It is intentionally non-observational.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Sys5I3ProcessLocalCutErrorKind {
    BootstrapPurposeRejected,
    WrongRole,
    AsyncObligation,
    RetainedIngress,
    OwnerAdmissionReservation,
    RemoteReplyUnresolved,
    SessionUnavailable,
    InitialRoundTripIncomplete,
    RuntimeNotQuiescent,
    AlreadyCommitted,
}

/// Opaque local cut failure. The kind is available only to the private test
/// seam; child/probe completion maps all failures to its generic route.
#[doc(hidden)]
#[derive(Clone, PartialEq, Eq)]
pub struct Sys5I3ProcessLocalCutError {
    kind: Sys5I3ProcessLocalCutErrorKind,
}

impl Sys5I3ProcessLocalCutError {
    const fn new(kind: Sys5I3ProcessLocalCutErrorKind) -> Self {
        Self { kind }
    }

    pub(crate) const fn kind(&self) -> Sys5I3ProcessLocalCutErrorKind {
        self.kind
    }
}

impl std::fmt::Debug for Sys5I3ProcessLocalCutError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Sys5I3ProcessLocalCutError(..)")
    }
}

/// Runtime/session custody stays crate-private. Public probe code invokes the
/// bounded child drivers below rather than receiving this value.
pub(crate) struct Sys5I3ProcessLocalCutChildCapsule {
    role: Sys5I3ProcessLocalCutControlRole,
    runtime: Sys5I3ProcessRuntime,
    session: Option<Sys5I3PrivateQuicSession>,
    initial_pending: Option<Sys5I3OriginalOwnerRequestPending>,
    retained_ingress: Option<Sys5I3PrivateQuicPendingIngress>,
    committed_admission: Option<Sys5I3ProcessLocalCutAdmissionToken>,
    async_obligation_active: bool,
}

/// Nested-test-only pair. It will consume only bootstraps issued by a genuine
/// whole-cohort launch; no arbitrary image/control decode-and-install helper
/// is part of this surface.
#[cfg(test)]
pub(crate) struct Sys5I3ProcessLocalCutTestPair {
    requester: Sys5I3ProcessLocalCutChildCapsule,
    owner: Sys5I3ProcessLocalCutChildCapsule,
}

impl Sys5I3ProcessLocalCutLaunch {
    pub(crate) fn from_issued_frames(
        requester_image_frame: Vec<u8>,
        requester_trusted_control_frame: Vec<u8>,
        owner_image_frame: Vec<u8>,
        owner_trusted_control_frame: Vec<u8>,
    ) -> Self {
        Self {
            requester: Some(Sys5I3ProcessLocalCutChildBootstrap {
                frames: Some(Sys5I3ProcessLocalCutChildFrames {
                    image_frame: requester_image_frame,
                    trusted_control_frame: requester_trusted_control_frame,
                }),
            }),
            owner: Some(Sys5I3ProcessLocalCutChildBootstrap {
                frames: Some(Sys5I3ProcessLocalCutChildFrames {
                    image_frame: owner_image_frame,
                    trusted_control_frame: owner_trusted_control_frame,
                }),
            }),
        }
    }

    pub fn take_requester_bootstrap(
        &mut self,
    ) -> Result<Sys5I3ProcessLocalCutChildBootstrap, Sys5I3ProcessLocalCutLaunchError> {
        self.requester.take().ok_or_else(|| {
            Sys5I3ProcessLocalCutLaunchError::new(
                Sys5I3ProcessLocalCutLaunchErrorKind::BootstrapAlreadyTaken,
            )
        })
    }

    pub fn take_owner_bootstrap(
        &mut self,
    ) -> Result<Sys5I3ProcessLocalCutChildBootstrap, Sys5I3ProcessLocalCutLaunchError> {
        self.owner.take().ok_or_else(|| {
            Sys5I3ProcessLocalCutLaunchError::new(
                Sys5I3ProcessLocalCutLaunchErrorKind::BootstrapAlreadyTaken,
            )
        })
    }

    #[cfg(test)]
    pub(crate) fn test_only_start_local_capsule_pair_from_issued_bootstraps(
        &mut self,
        codec: &Sys5I3PrivateProcessCodec,
    ) -> Result<Sys5I3ProcessLocalCutTestPair, Sys5I3ProcessLocalCutError> {
        let requester = self
            .take_requester_bootstrap()
            .map_err(|_| {
                Sys5I3ProcessLocalCutError::new(
                    Sys5I3ProcessLocalCutErrorKind::BootstrapPurposeRejected,
                )
            })?
            .into_private_child_frames()
            .map_err(|_| {
                Sys5I3ProcessLocalCutError::new(
                    Sys5I3ProcessLocalCutErrorKind::BootstrapPurposeRejected,
                )
            })?;
        let owner = self
            .take_owner_bootstrap()
            .map_err(|_| {
                Sys5I3ProcessLocalCutError::new(
                    Sys5I3ProcessLocalCutErrorKind::BootstrapPurposeRejected,
                )
            })?
            .into_private_child_frames()
            .map_err(|_| {
                Sys5I3ProcessLocalCutError::new(
                    Sys5I3ProcessLocalCutErrorKind::BootstrapPurposeRejected,
                )
            })?;
        Ok(Sys5I3ProcessLocalCutTestPair {
            requester: start_capsule_from_issued_frames(
                codec,
                requester,
                Sys5I3ProcessLocalCutControlRole::Requester,
            )?
            .0,
            owner: start_capsule_from_issued_frames(
                codec,
                owner,
                Sys5I3ProcessLocalCutControlRole::Owner,
            )?
            .0,
        })
    }
}

impl Sys5I3ProcessLocalCutChildBootstrap {
    pub fn into_private_child_frames(
        self,
    ) -> Result<Sys5I3ProcessLocalCutChildFrames, Sys5I3ProcessLocalCutLaunchError> {
        self.frames.ok_or_else(|| {
            Sys5I3ProcessLocalCutLaunchError::new(
                Sys5I3ProcessLocalCutLaunchErrorKind::BootstrapAlreadyTaken,
            )
        })
    }
}

impl Sys5I3ProcessLocalCutChildFrames {
    /// The existing trusted child launcher is the only intended consumer.
    /// The implementation will move the paired frames together; it never
    /// yields a runtime, decoded control, session, or authority object.
    pub fn into_image_and_trusted_control_frames(self) -> (Vec<u8>, Vec<u8>) {
        (self.image_frame, self.trusted_control_frame)
    }
}

fn start_capsule_from_issued_frames(
    codec: &Sys5I3PrivateProcessCodec,
    frames: Sys5I3ProcessLocalCutChildFrames,
    role: Sys5I3ProcessLocalCutControlRole,
) -> Result<
    (
        Sys5I3ProcessLocalCutChildCapsule,
        Sys5I3TrustedLocalnetControl,
    ),
    Sys5I3ProcessLocalCutError,
> {
    let (image_frame, trusted_control_frame) = frames.into_image_and_trusted_control_frames();
    let image = codec.decode_untrusted_image(&image_frame).map_err(|_| {
        Sys5I3ProcessLocalCutError::new(Sys5I3ProcessLocalCutErrorKind::BootstrapPurposeRejected)
    })?;
    let control = codec
        .decode_trusted_localnet_control(&trusted_control_frame)
        .map_err(|_| {
            Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::BootstrapPurposeRejected,
            )
        })?;
    let (runtime, control) = codec
        .validate_and_start_image_with_process_local_cut_control(image, control, role)
        .map_err(|_| {
            Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::BootstrapPurposeRejected,
            )
        })?;
    Ok((
        Sys5I3ProcessLocalCutChildCapsule {
            role,
            runtime,
            session: None,
            initial_pending: None,
            retained_ingress: None,
            committed_admission: None,
            async_obligation_active: false,
        },
        control,
    ))
}

impl Sys5I3ProcessLocalCutChildCapsule {
    pub(crate) fn admit_process_local_cut(&mut self) -> Result<(), Sys5I3ProcessLocalCutError> {
        if self.async_obligation_active {
            return Err(Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::AsyncObligation,
            ));
        }
        if self.retained_ingress.is_some() {
            return Err(Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::RetainedIngress,
            ));
        }
        if self.runtime.has_i3_owner_admission_reservation() {
            return Err(Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::OwnerAdmissionReservation,
            ));
        }
        if self.role != Sys5I3ProcessLocalCutControlRole::Requester {
            return Err(Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::WrongRole,
            ));
        }
        if self.committed_admission.is_some() {
            return Err(Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::AlreadyCommitted,
            ));
        }
        if self.initial_pending.is_some() {
            return Err(Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::RemoteReplyUnresolved,
            ));
        }
        if !self.session.as_ref().is_some_and(|session| {
            session.has_unacquired_verified_process_local_cut_custody(
                Sys5I3ProcessLocalCutControlRole::Requester,
            )
        }) {
            return Err(Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::SessionUnavailable,
            ));
        }
        let admission = self
            .runtime
            .commit_i3_process_local_cut_after_initial_receipt()
            .map_err(|_| {
                Sys5I3ProcessLocalCutError::new(
                    Sys5I3ProcessLocalCutErrorKind::InitialRoundTripIncomplete,
                )
            })?;
        self.committed_admission = Some(admission);
        Ok(())
    }

    fn begin_async_obligation(&mut self) -> Result<(), Sys5I3ProcessLocalCutError> {
        if self.async_obligation_active {
            return Err(Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::AsyncObligation,
            ));
        }
        self.async_obligation_active = true;
        Ok(())
    }

    fn complete_async_obligation(&mut self) {
        self.async_obligation_active = false;
    }

    fn emit_post_cut_source_action(
        &mut self,
    ) -> Result<Sys5I3ProcessMessage, Sys5I3ProcessLocalCutError> {
        let admission = self.committed_admission.take().ok_or_else(|| {
            Sys5I3ProcessLocalCutError::new(Sys5I3ProcessLocalCutErrorKind::RuntimeNotQuiescent)
        })?;
        let result = self
            .runtime
            .emit_generated_owner_request_after_process_local_cut("init_avatar_hp", &admission)
            .map_err(runtime_error);
        self.committed_admission = Some(admission);
        result
    }

    fn runtime_and_session_mut(
        &mut self,
    ) -> Result<
        (&mut Sys5I3ProcessRuntime, &mut Sys5I3PrivateQuicSession),
        Sys5I3ProcessLocalCutError,
    > {
        let runtime = &mut self.runtime;
        let session = self.session.as_mut().ok_or_else(|| {
            Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::BootstrapPurposeRejected,
            )
        })?;
        Ok((runtime, session))
    }
}

#[cfg(test)]
impl Sys5I3ProcessLocalCutTestPair {
    pub(crate) fn owner_mut(&mut self) -> &mut Sys5I3ProcessLocalCutChildCapsule {
        &mut self.owner
    }
}

/// Bounded requester child entry. It deliberately accepts framed inherited
/// inputs rather than a runtime/session/control value. The implementation
/// will validate the private Row20 purpose before constructing custody.
#[doc(hidden)]
pub async fn drive_i3_process_local_cut_requester_child_from_inherited(
    codec: &Sys5I3PrivateProcessCodec,
    image_frame: Vec<u8>,
    trusted_control_frame: Vec<u8>,
    connection: quinn::Connection,
) -> Result<Sys5I3ProcessLocalCutChildCompletion, Sys5I3ProcessLocalCutError> {
    let (mut capsule, control) = start_capsule_from_issued_frames(
        codec,
        Sys5I3ProcessLocalCutChildFrames {
            image_frame,
            trusted_control_frame,
        },
        Sys5I3ProcessLocalCutControlRole::Requester,
    )?;
    capsule.begin_async_obligation()?;
    let session = Sys5I3PrivateQuicSession::connect_process_local_cut(connection, control)
        .await
        .map_err(adapter_error)?;
    capsule.session = Some(session);
    capsule.complete_async_obligation();
    send_local_preface(&mut capsule).await?;
    receive_local_preface(&mut capsule).await?;

    let first_request = capsule
        .runtime
        .emit_generated_owner_request("init_avatar_hp")
        .map_err(runtime_error)?;
    capsule.initial_pending = Some(
        capsule
            .runtime
            .into_original_owner_request_pending(first_request)
            .map_err(runtime_error)?,
    );
    let expected_pending_error = capsule
        .admit_process_local_cut()
        .expect_err("the just-emitted first request remains unresolved before its real reply");
    if expected_pending_error.kind() != Sys5I3ProcessLocalCutErrorKind::RemoteReplyUnresolved {
        return Err(Sys5I3ProcessLocalCutError::new(
            Sys5I3ProcessLocalCutErrorKind::RuntimeNotQuiescent,
        ));
    }
    send_initial_request(&mut capsule).await?;
    receive_initial_reply_and_commit_cut(&mut capsule).await?;
    let repeated_cut = capsule
        .admit_process_local_cut()
        .expect_err("the same actual requester custody cut remains one-shot");
    if repeated_cut.kind() != Sys5I3ProcessLocalCutErrorKind::AlreadyCommitted {
        return Err(Sys5I3ProcessLocalCutError::new(
            Sys5I3ProcessLocalCutErrorKind::RuntimeNotQuiescent,
        ));
    }

    let second_request = capsule.emit_post_cut_source_action()?;
    let second_pending = capsule
        .runtime
        .into_original_owner_request_pending(second_request)
        .map_err(runtime_error)?;
    send_request_with_pending(&mut capsule, &second_pending).await?;
    finish_local_send(&mut capsule)?;
    receive_final_reply(&mut capsule, second_pending).await?;
    close_local_session(&capsule)?;
    if capsule.committed_admission.is_none() {
        return Err(Sys5I3ProcessLocalCutError::new(
            Sys5I3ProcessLocalCutErrorKind::RuntimeNotQuiescent,
        ));
    }
    Ok(Sys5I3ProcessLocalCutChildCompletion { _private: () })
}

/// Bounded owner child entry corresponding to the requester entry above.
#[doc(hidden)]
pub async fn drive_i3_process_local_cut_owner_child_from_inherited(
    codec: &Sys5I3PrivateProcessCodec,
    image_frame: Vec<u8>,
    trusted_control_frame: Vec<u8>,
    connection: quinn::Connection,
) -> Result<Sys5I3ProcessLocalCutChildCompletion, Sys5I3ProcessLocalCutError> {
    let (mut capsule, control) = start_capsule_from_issued_frames(
        codec,
        Sys5I3ProcessLocalCutChildFrames {
            image_frame,
            trusted_control_frame,
        },
        Sys5I3ProcessLocalCutControlRole::Owner,
    )?;
    capsule.begin_async_obligation()?;
    let session = Sys5I3PrivateQuicSession::accept_process_local_cut(connection, control)
        .await
        .map_err(adapter_error)?;
    capsule.session = Some(session);
    capsule.complete_async_obligation();
    send_local_preface(&mut capsule).await?;
    receive_local_preface(&mut capsule).await?;

    receive_initial_ingress_and_assert_cut_denied(&mut capsule).await?;
    let first_reply = admit_retained_initial_ingress(&mut capsule)?;
    send_owner_reply(&mut capsule, first_reply).await?;
    let second_reply = receive_second_request_and_admit(&mut capsule).await?;
    send_owner_reply(&mut capsule, second_reply).await?;
    finish_local_send(&mut capsule)?;
    wait_for_peer_close(&mut capsule).await?;
    close_local_session(&capsule)?;
    Ok(Sys5I3ProcessLocalCutChildCompletion { _private: () })
}

/// Fixed sealed entry for the late-first-reply conformance route. It accepts
/// no caller-selected data and preserves the same source-real bootstrap and
/// custody boundary as the canonical route.
#[doc(hidden)]
pub async fn drive_i3_process_local_cut_late_reply_after_cut_requester_child_from_inherited(
    codec: &Sys5I3PrivateProcessCodec,
    image_frame: Vec<u8>,
    trusted_control_frame: Vec<u8>,
    connection: quinn::Connection,
) -> Result<Sys5I3ProcessLocalCutChildCompletion, Sys5I3ProcessLocalCutError> {
    let (mut capsule, control) = start_capsule_from_issued_frames(
        codec,
        Sys5I3ProcessLocalCutChildFrames {
            image_frame,
            trusted_control_frame,
        },
        Sys5I3ProcessLocalCutControlRole::Requester,
    )?;
    capsule.begin_async_obligation()?;
    let session = Sys5I3PrivateQuicSession::connect_process_local_cut(connection, control)
        .await
        .map_err(adapter_error)?;
    capsule.session = Some(session);
    capsule.complete_async_obligation();
    send_local_preface(&mut capsule).await?;
    receive_local_preface(&mut capsule).await?;

    let first_request = capsule
        .runtime
        .emit_generated_owner_request("init_avatar_hp")
        .map_err(runtime_error)?;
    capsule.initial_pending = Some(
        capsule
            .runtime
            .into_original_owner_request_pending(first_request)
            .map_err(runtime_error)?,
    );
    let expected_pending_error = capsule
        .admit_process_local_cut()
        .expect_err("the just-emitted first request remains unresolved before its real reply");
    if expected_pending_error.kind() != Sys5I3ProcessLocalCutErrorKind::RemoteReplyUnresolved {
        return Err(Sys5I3ProcessLocalCutError::new(
            Sys5I3ProcessLocalCutErrorKind::RuntimeNotQuiescent,
        ));
    }
    send_initial_request(&mut capsule).await?;
    receive_initial_reply_and_commit_cut(&mut capsule).await?;
    let repeated_cut = capsule
        .admit_process_local_cut()
        .expect_err("the same actual requester custody cut remains one-shot");
    if repeated_cut.kind() != Sys5I3ProcessLocalCutErrorKind::AlreadyCommitted {
        return Err(Sys5I3ProcessLocalCutError::new(
            Sys5I3ProcessLocalCutErrorKind::RuntimeNotQuiescent,
        ));
    }

    let second_request = capsule.emit_post_cut_source_action()?;
    let second_pending = capsule
        .runtime
        .into_original_owner_request_pending(second_request)
        .map_err(runtime_error)?;
    send_request_with_pending(&mut capsule, &second_pending).await?;
    finish_local_send(&mut capsule)?;
    let second_pending = receive_rejected_late_process_local_cut_reply_preserving_pending(
        &mut capsule,
        second_pending,
    )
    .await?;
    receive_final_reply(&mut capsule, second_pending).await?;
    close_local_session(&capsule)?;
    Ok(Sys5I3ProcessLocalCutChildCompletion { _private: () })
}

/// Fixed owner counterpart for the late-first-reply conformance route.
#[doc(hidden)]
pub async fn drive_i3_process_local_cut_late_reply_after_cut_owner_child_from_inherited(
    codec: &Sys5I3PrivateProcessCodec,
    image_frame: Vec<u8>,
    trusted_control_frame: Vec<u8>,
    connection: quinn::Connection,
) -> Result<Sys5I3ProcessLocalCutChildCompletion, Sys5I3ProcessLocalCutError> {
    let (mut capsule, control) = start_capsule_from_issued_frames(
        codec,
        Sys5I3ProcessLocalCutChildFrames {
            image_frame,
            trusted_control_frame,
        },
        Sys5I3ProcessLocalCutControlRole::Owner,
    )?;
    capsule.begin_async_obligation()?;
    let session = Sys5I3PrivateQuicSession::accept_process_local_cut(connection, control)
        .await
        .map_err(adapter_error)?;
    capsule.session = Some(session);
    capsule.complete_async_obligation();
    send_local_preface(&mut capsule).await?;
    receive_local_preface(&mut capsule).await?;

    receive_initial_ingress_and_assert_cut_denied(&mut capsule).await?;
    let first_reply = admit_retained_initial_ingress(&mut capsule)?;
    let retained =
        send_owner_reply_and_retain_for_late_admission(&mut capsule, first_reply).await?;
    let second_reply = receive_second_request_and_admit(&mut capsule).await?;
    send_retained_owner_reply_after_later_admission(&mut capsule, retained, &second_reply).await?;
    send_owner_reply(&mut capsule, second_reply).await?;
    finish_local_send(&mut capsule)?;
    wait_for_peer_close(&mut capsule).await?;
    close_local_session(&capsule)?;
    Ok(Sys5I3ProcessLocalCutChildCompletion { _private: () })
}

/// Fixed partial-receive/cancel requester path. It starts the genuine
/// authorized request but writes only its bounded frame prefix, then retains
/// the original pending while the peer proves a body read remains pending and
/// closes the physical connection without a body write.
#[doc(hidden)]
pub async fn drive_i3_process_local_cut_partial_receive_cancel_and_close_requester_child_from_inherited(
    codec: &Sys5I3PrivateProcessCodec,
    image_frame: Vec<u8>,
    trusted_control_frame: Vec<u8>,
    connection: quinn::Connection,
) -> Result<Sys5I3ProcessLocalCutChildCompletion, Sys5I3ProcessLocalCutError> {
    let (mut capsule, control) = start_capsule_from_issued_frames(
        codec,
        Sys5I3ProcessLocalCutChildFrames {
            image_frame,
            trusted_control_frame,
        },
        Sys5I3ProcessLocalCutControlRole::Requester,
    )?;
    capsule.begin_async_obligation()?;
    let session = Sys5I3PrivateQuicSession::connect_process_local_cut(connection, control)
        .await
        .map_err(adapter_error)?;
    capsule.session = Some(session);
    capsule.complete_async_obligation();
    send_local_preface(&mut capsule).await?;
    receive_local_preface(&mut capsule).await?;

    let first_request = capsule
        .runtime
        .emit_generated_owner_request("init_avatar_hp")
        .map_err(runtime_error)?;
    capsule.initial_pending = Some(
        capsule
            .runtime
            .into_original_owner_request_pending(first_request)
            .map_err(runtime_error)?,
    );
    assert_cut_denied_with_kind(
        &mut capsule,
        Sys5I3ProcessLocalCutErrorKind::RemoteReplyUnresolved,
        "the started request remains pending before the cancellation boundary",
    )?;
    send_initial_request_length_prefix_then_hold(&mut capsule).await?;

    // The peer closes only after it has read the complete header and observed
    // the body read pending.  This is physical lifecycle coordination, not a
    // semantic acknowledgement or a replacement delivery path.
    wait_for_peer_close(&mut capsule).await?;
    close_local_session(&capsule)?;
    assert_cut_denied_with_kind(
        &mut capsule,
        Sys5I3ProcessLocalCutErrorKind::RemoteReplyUnresolved,
        "a cancelled started frame preserves the original requester pending",
    )?;
    if capsule.initial_pending.is_none() {
        return Err(Sys5I3ProcessLocalCutError::new(
            Sys5I3ProcessLocalCutErrorKind::RuntimeNotQuiescent,
        ));
    }
    let pending = capsule.initial_pending.as_ref().ok_or_else(|| {
        Sys5I3ProcessLocalCutError::new(Sys5I3ProcessLocalCutErrorKind::RuntimeNotQuiescent)
    })?;
    if !capsule
        .runtime
        .preserves_i3_process_local_cut_cancelled_initial_pending(pending)
    {
        return Err(Sys5I3ProcessLocalCutError::new(
            Sys5I3ProcessLocalCutErrorKind::RuntimeNotQuiescent,
        ));
    }
    Ok(Sys5I3ProcessLocalCutChildCompletion { _private: () })
}

/// Fixed partial-receive/cancel owner path. It reserves the one ingress,
/// reads the real frame header, verifies the body read is pending once, then
/// retains that reserved obligation through physical stream closure.
#[doc(hidden)]
pub async fn drive_i3_process_local_cut_partial_receive_cancel_and_close_owner_child_from_inherited(
    codec: &Sys5I3PrivateProcessCodec,
    image_frame: Vec<u8>,
    trusted_control_frame: Vec<u8>,
    connection: quinn::Connection,
) -> Result<Sys5I3ProcessLocalCutChildCompletion, Sys5I3ProcessLocalCutError> {
    let (mut capsule, control) = start_capsule_from_issued_frames(
        codec,
        Sys5I3ProcessLocalCutChildFrames {
            image_frame,
            trusted_control_frame,
        },
        Sys5I3ProcessLocalCutControlRole::Owner,
    )?;
    capsule.begin_async_obligation()?;
    let session = Sys5I3PrivateQuicSession::accept_process_local_cut(connection, control)
        .await
        .map_err(adapter_error)?;
    capsule.session = Some(session);
    capsule.complete_async_obligation();
    send_local_preface(&mut capsule).await?;
    receive_local_preface(&mut capsule).await?;

    capsule.begin_async_obligation()?;
    let cancellation = capsule
        .session
        .as_mut()
        .ok_or_else(|| {
            Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::BootstrapPurposeRejected,
            )
        })?
        .cancel_process_local_cut_pending_ingress_after_complete_header()
        .await;
    cancellation.map_err(adapter_error)?;
    assert_cut_denied_with_kind(
        &mut capsule,
        Sys5I3ProcessLocalCutErrorKind::AsyncObligation,
        "a cancelled physical body read retains its local acquisition obligation",
    )?;
    let reacquisition = capsule
        .session
        .as_mut()
        .ok_or_else(|| {
            Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::BootstrapPurposeRejected,
            )
        })?
        .cancel_process_local_cut_pending_ingress_after_complete_header()
        .await;
    if !matches!(
        reacquisition,
        Err(Sys5I3PrivateQuicError::LocalAttemptRejected)
    ) {
        return Err(Sys5I3ProcessLocalCutError::new(
            Sys5I3ProcessLocalCutErrorKind::RuntimeNotQuiescent,
        ));
    }
    if !capsule
        .runtime
        .validates_i3_process_local_cut_cancelled_owner_state()
    {
        return Err(Sys5I3ProcessLocalCutError::new(
            Sys5I3ProcessLocalCutErrorKind::RuntimeNotQuiescent,
        ));
    }
    close_local_session(&capsule)?;
    Ok(Sys5I3ProcessLocalCutChildCompletion { _private: () })
}

fn runtime_error(
    _: crate::sys5_i3_process_runtime::Sys5I3ProcessRuntimeError,
) -> Sys5I3ProcessLocalCutError {
    Sys5I3ProcessLocalCutError::new(Sys5I3ProcessLocalCutErrorKind::RuntimeNotQuiescent)
}

fn adapter_error(
    _: crate::sys5_i3_private_quic::Sys5I3PrivateQuicError,
) -> Sys5I3ProcessLocalCutError {
    Sys5I3ProcessLocalCutError::new(Sys5I3ProcessLocalCutErrorKind::RuntimeNotQuiescent)
}

async fn send_local_preface(
    capsule: &mut Sys5I3ProcessLocalCutChildCapsule,
) -> Result<(), Sys5I3ProcessLocalCutError> {
    capsule.begin_async_obligation()?;
    let result = capsule
        .session
        .as_mut()
        .ok_or_else(|| {
            Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::BootstrapPurposeRejected,
            )
        })?
        .send_local_preface()
        .await;
    result.map_err(adapter_error)?;
    capsule.complete_async_obligation();
    Ok(())
}

fn finish_local_send(
    capsule: &mut Sys5I3ProcessLocalCutChildCapsule,
) -> Result<(), Sys5I3ProcessLocalCutError> {
    capsule
        .session
        .as_mut()
        .ok_or_else(|| {
            Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::BootstrapPurposeRejected,
            )
        })?
        .finish_send()
        .map_err(adapter_error)
}

async fn wait_for_peer_close(
    capsule: &mut Sys5I3ProcessLocalCutChildCapsule,
) -> Result<(), Sys5I3ProcessLocalCutError> {
    capsule.begin_async_obligation()?;
    capsule
        .session
        .as_ref()
        .ok_or_else(|| {
            Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::BootstrapPurposeRejected,
            )
        })?
        .wait_for_peer_close()
        .await;
    capsule.complete_async_obligation();
    Ok(())
}

fn close_local_session(
    capsule: &Sys5I3ProcessLocalCutChildCapsule,
) -> Result<(), Sys5I3ProcessLocalCutError> {
    capsule
        .session
        .as_ref()
        .ok_or_else(|| {
            Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::BootstrapPurposeRejected,
            )
        })?
        .close();
    Ok(())
}

async fn receive_local_preface(
    capsule: &mut Sys5I3ProcessLocalCutChildCapsule,
) -> Result<(), Sys5I3ProcessLocalCutError> {
    capsule.begin_async_obligation()?;
    let result = capsule
        .session
        .as_mut()
        .ok_or_else(|| {
            Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::BootstrapPurposeRejected,
            )
        })?
        .receive_and_validate_peer_preface()
        .await;
    result.map_err(adapter_error)?;
    capsule.complete_async_obligation();
    Ok(())
}

async fn send_initial_request(
    capsule: &mut Sys5I3ProcessLocalCutChildCapsule,
) -> Result<(), Sys5I3ProcessLocalCutError> {
    capsule.begin_async_obligation()?;
    let result = {
        let Sys5I3ProcessLocalCutChildCapsule {
            runtime,
            session,
            initial_pending,
            ..
        } = capsule;
        let pending = initial_pending.as_ref().ok_or_else(|| {
            Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::InitialRoundTripIncomplete,
            )
        })?;
        let session = session.as_mut().ok_or_else(|| {
            Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::BootstrapPurposeRejected,
            )
        })?;
        session
            .send_process_local_cut_original_owner_request(runtime, pending)
            .await
    };
    result.map_err(adapter_error)?;
    capsule.complete_async_obligation();
    Ok(())
}

async fn send_initial_request_length_prefix_then_hold(
    capsule: &mut Sys5I3ProcessLocalCutChildCapsule,
) -> Result<(), Sys5I3ProcessLocalCutError> {
    capsule.begin_async_obligation()?;
    let result = {
        let Sys5I3ProcessLocalCutChildCapsule {
            runtime,
            session,
            initial_pending,
            ..
        } = capsule;
        let pending = initial_pending.as_ref().ok_or_else(|| {
            Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::InitialRoundTripIncomplete,
            )
        })?;
        let session = session.as_mut().ok_or_else(|| {
            Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::BootstrapPurposeRejected,
            )
        })?;
        session
            .send_process_local_cut_original_owner_request_length_prefix_then_hold(runtime, pending)
            .await
    };
    result.map_err(adapter_error)?;
    capsule.complete_async_obligation();
    Ok(())
}

fn assert_cut_denied_with_kind(
    capsule: &mut Sys5I3ProcessLocalCutChildCapsule,
    expected: Sys5I3ProcessLocalCutErrorKind,
    assertion: &str,
) -> Result<(), Sys5I3ProcessLocalCutError> {
    let actual = capsule.admit_process_local_cut().expect_err(assertion);
    if actual.kind() == expected {
        Ok(())
    } else {
        Err(Sys5I3ProcessLocalCutError::new(
            Sys5I3ProcessLocalCutErrorKind::RuntimeNotQuiescent,
        ))
    }
}

async fn send_request_with_pending(
    capsule: &mut Sys5I3ProcessLocalCutChildCapsule,
    pending: &Sys5I3OriginalOwnerRequestPending,
) -> Result<(), Sys5I3ProcessLocalCutError> {
    capsule.begin_async_obligation()?;
    let result = {
        let (runtime, session) = capsule.runtime_and_session_mut()?;
        session
            .send_process_local_cut_original_owner_request(runtime, pending)
            .await
    };
    result.map_err(adapter_error)?;
    capsule.complete_async_obligation();
    Ok(())
}

async fn receive_initial_reply_and_commit_cut(
    capsule: &mut Sys5I3ProcessLocalCutChildCapsule,
) -> Result<(), Sys5I3ProcessLocalCutError> {
    let pending = capsule.initial_pending.take().ok_or_else(|| {
        Sys5I3ProcessLocalCutError::new(Sys5I3ProcessLocalCutErrorKind::InitialRoundTripIncomplete)
    })?;
    capsule.begin_async_obligation()?;
    let outcome = {
        let (runtime, session) = capsule.runtime_and_session_mut()?;
        session
            .receive_and_admit_process_local_cut_original_owner_reply(runtime, pending)
            .await
    };
    match outcome {
        Sys5I3PrivateQuicOriginalOwnerReplyOutcome::Consumed { .. } => {
            capsule.complete_async_obligation();
            capsule.admit_process_local_cut()
        }
        Sys5I3PrivateQuicOriginalOwnerReplyOutcome::TerminalFailureConsumed { .. }
        | Sys5I3PrivateQuicOriginalOwnerReplyOutcome::Pending { .. } => {
            Err(Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::InitialRoundTripIncomplete,
            ))
        }
    }
}

async fn receive_final_reply(
    capsule: &mut Sys5I3ProcessLocalCutChildCapsule,
    pending: Sys5I3OriginalOwnerRequestPending,
) -> Result<(), Sys5I3ProcessLocalCutError> {
    capsule.begin_async_obligation()?;
    let outcome = {
        let (runtime, session) = capsule.runtime_and_session_mut()?;
        session
            .receive_and_admit_process_local_cut_original_owner_reply(runtime, pending)
            .await
    };
    match outcome {
        Sys5I3PrivateQuicOriginalOwnerReplyOutcome::Consumed { .. } => {
            capsule.complete_async_obligation();
            Ok(())
        }
        Sys5I3PrivateQuicOriginalOwnerReplyOutcome::TerminalFailureConsumed { .. }
        | Sys5I3PrivateQuicOriginalOwnerReplyOutcome::Pending { .. } => Err(
            Sys5I3ProcessLocalCutError::new(Sys5I3ProcessLocalCutErrorKind::RuntimeNotQuiescent),
        ),
    }
}

/// Consume exactly one decoded old Reply after the post-cut request is live.
/// It must fail at the existing pending-identity boundary and return the
/// later pending handle intact so only the genuine later Reply may consume it.
async fn receive_rejected_late_process_local_cut_reply_preserving_pending(
    capsule: &mut Sys5I3ProcessLocalCutChildCapsule,
    pending: Sys5I3OriginalOwnerRequestPending,
) -> Result<Sys5I3OriginalOwnerRequestPending, Sys5I3ProcessLocalCutError> {
    capsule.begin_async_obligation()?;
    let outcome = {
        let (runtime, session) = capsule.runtime_and_session_mut()?;
        session
            .receive_and_admit_process_local_cut_original_owner_reply(runtime, pending)
            .await
    };
    match outcome {
        Sys5I3PrivateQuicOriginalOwnerReplyOutcome::Pending {
            pending,
            error:
                Sys5I3PrivateQuicError::SemanticRejected {
                    error,
                    rejected_attempt: _,
                },
        } if error.kind() == Sys5I3ProcessRuntimeErrorKind::OriginalRequestPendingRejected => {
            if !capsule
                .runtime
                .preserves_i3_process_local_cut_later_pending_after_old_reply_rejection(&pending)
            {
                return Err(Sys5I3ProcessLocalCutError::new(
                    Sys5I3ProcessLocalCutErrorKind::RuntimeNotQuiescent,
                ));
            }
            capsule.complete_async_obligation();
            Ok(pending)
        }
        Sys5I3PrivateQuicOriginalOwnerReplyOutcome::Consumed { .. }
        | Sys5I3PrivateQuicOriginalOwnerReplyOutcome::TerminalFailureConsumed { .. }
        | Sys5I3PrivateQuicOriginalOwnerReplyOutcome::Pending { .. } => Err(
            Sys5I3ProcessLocalCutError::new(Sys5I3ProcessLocalCutErrorKind::RuntimeNotQuiescent),
        ),
    }
}

async fn receive_initial_ingress_and_assert_cut_denied(
    capsule: &mut Sys5I3ProcessLocalCutChildCapsule,
) -> Result<(), Sys5I3ProcessLocalCutError> {
    capsule.begin_async_obligation()?;
    let pending = capsule
        .session
        .as_mut()
        .ok_or_else(|| {
            Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::BootstrapPurposeRejected,
            )
        })?
        .receive_complete_process_local_cut_pending_ingress()
        .await
        .map_err(adapter_error)?;
    capsule.complete_async_obligation();
    capsule.retained_ingress = Some(pending);
    let error = capsule
        .admit_process_local_cut()
        .expect_err("a complete owner ingress remains held before semantic admission");
    if error.kind() != Sys5I3ProcessLocalCutErrorKind::RetainedIngress {
        return Err(Sys5I3ProcessLocalCutError::new(
            Sys5I3ProcessLocalCutErrorKind::RuntimeNotQuiescent,
        ));
    }
    Ok(())
}

fn admit_retained_initial_ingress(
    capsule: &mut Sys5I3ProcessLocalCutChildCapsule,
) -> Result<Sys5I3ProcessMessage, Sys5I3ProcessLocalCutError> {
    let pending = capsule.retained_ingress.take().ok_or_else(|| {
        Sys5I3ProcessLocalCutError::new(Sys5I3ProcessLocalCutErrorKind::RetainedIngress)
    })?;
    let admitted = {
        let (runtime, session) = capsule.runtime_and_session_mut()?;
        session
            .admit_process_local_cut_pending_ingress(runtime, pending)
            .map_err(adapter_error)?
    };
    capsule
        .runtime
        .resolve_i3_process_local_cut_owner_admission(admitted.0)
        .map_err(runtime_error)
}

async fn receive_second_request_and_admit(
    capsule: &mut Sys5I3ProcessLocalCutChildCapsule,
) -> Result<Sys5I3ProcessMessage, Sys5I3ProcessLocalCutError> {
    capsule.begin_async_obligation()?;
    let admitted = {
        let (runtime, session) = capsule.runtime_and_session_mut()?;
        session
            .receive_and_admit_process_local_cut_generated_message(runtime)
            .await
            .map_err(adapter_error)?
    };
    capsule.complete_async_obligation();
    capsule
        .runtime
        .resolve_i3_process_local_cut_owner_admission(admitted.0)
        .map_err(runtime_error)
}

async fn send_owner_reply(
    capsule: &mut Sys5I3ProcessLocalCutChildCapsule,
    reply: Sys5I3ProcessMessage,
) -> Result<(), Sys5I3ProcessLocalCutError> {
    capsule.begin_async_obligation()?;
    let result = capsule
        .session
        .as_mut()
        .ok_or_else(|| {
            Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::BootstrapPurposeRejected,
            )
        })?
        .send_process_local_cut_generated_message(reply)
        .await;
    result.map_err(adapter_error)?;
    capsule.complete_async_obligation();
    Ok(())
}

async fn send_owner_reply_and_retain_for_late_admission(
    capsule: &mut Sys5I3ProcessLocalCutChildCapsule,
    reply: Sys5I3ProcessMessage,
) -> Result<Sys5I3PrivateQuicProcessLocalCutRetainedOwnerReply, Sys5I3ProcessLocalCutError> {
    capsule.begin_async_obligation()?;
    let result = capsule
        .session
        .as_mut()
        .ok_or_else(|| {
            Sys5I3ProcessLocalCutError::new(
                Sys5I3ProcessLocalCutErrorKind::BootstrapPurposeRejected,
            )
        })?
        .send_process_local_cut_generated_reply_and_retain_for_late_admission(reply)
        .await;
    let retained = result.map_err(adapter_error)?;
    capsule.complete_async_obligation();
    Ok(retained)
}

async fn send_retained_owner_reply_after_later_admission(
    capsule: &mut Sys5I3ProcessLocalCutChildCapsule,
    retained: Sys5I3PrivateQuicProcessLocalCutRetainedOwnerReply,
    second_reply: &Sys5I3ProcessMessage,
) -> Result<(), Sys5I3ProcessLocalCutError> {
    capsule.begin_async_obligation()?;
    let result = {
        let (runtime, session) = capsule.runtime_and_session_mut()?;
        session
            .send_retained_process_local_cut_reply_after_later_admission(
                runtime,
                retained,
                second_reply,
            )
            .await
    };
    result.map_err(adapter_error)?;
    capsule.complete_async_obligation();
    Ok(())
}
