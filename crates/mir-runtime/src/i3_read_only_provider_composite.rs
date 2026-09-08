//! Private Stage 2b/2c inactive-composite admission for the finite read-only
//! provider profile.
//!
//! This module binds the complete checked profile to a genuine trusted
//! resource context, requires actual M9 membership authentication and exact
//! composite verification, then consumes a separate provider policy to retain
//! an inactive, resource-scoped M8 component.
//!
//! Provider activation, invocation, host reads, and runtime-requirement
//! discharge remain unsupported here.

#![cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the inactive composite awaits the deferred provider activation consumer"
    )
)]

use std::{
    fmt, fs,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
};

use mir_ast::surface_v0::FixtureSource;
use mir_semantics::{
    m9_finite_refinement::{
        M9CompositeContractCandidate, M9CompositeFiniteRefinementChecker,
        M9CompositeFiniteRefinementDischarge, M9ReadOnlyProviderEffectCoverage,
    },
    surface_v0_pipeline::{
        CheckedProgramIdentity, CheckedSurfaceV0, check_and_elaborate_surface_v0,
    },
};
use serde::{Deserialize, Serialize};

use crate::sys3_projection::{
    DeclaredLogicalTopology, ReadOnlyProviderEffectStaticProjection,
    verify_read_only_provider_effect_static_projection,
};
use crate::{
    m8_runtime_admission::{
        M8ReadOnlyProviderEffectComponentInventory, M8ReadOnlyProviderEffectComponentSnapshot,
    },
    m9_auth_verification::{
        M9InactiveReadOnlyProviderComposite, M9ReadOnlyProviderCompositeBootstrap,
        M9ReadOnlyProviderPolicyProof, M9ReadOnlyProviderTerminalObservationPolicyProof,
        M9VerifiedReadOnlyProviderComposite, trusted_read_only_provider_composite_bootstrap,
        verify_read_only_provider_composite,
    },
    sys4_dispatch::Sys4InactiveProviderAdmission,
    sys5_i3_process_runtime::{
        Sys5I3DeploymentSlot, Sys5I3InactiveProviderCohort, Sys5I3PreparedProviderLocalnetLaunch,
        Sys5I3ProviderLaunchError, Sys5I3ProviderLaunchErrorKind,
    },
};

const TRUSTED_LOGICAL_RESOURCE_SLOT: &str = "sample_input";
static NEXT_TRUSTED_FIXTURE_INCARNATION: AtomicU64 = AtomicU64::new(1);

/// Finite T0-controlled fixture choices for source-real provider execution
/// tests. These names select already fixed local setup profiles; they accept
/// neither a caller path nor caller-provided provider input or outcome.
#[cfg(feature = "i3-process-test-seams")]
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3ReadOnlyProviderFixtureProfile {
    Value41,
    ValueNegative7,
    DeclaredTargetAbsent,
    NoncanonicalInteger,
    ReadBoundary33Bytes,
    DeclaredTargetDirectory,
}

/// Build the finite canonical provider fixture through the actual checked
/// source, static projection, M9 composite verification, trusted binding, and
/// separate policy path. This private feature boundary deliberately accepts no
/// caller-selected source, fixture contents, authority, control, or result.
#[cfg(feature = "i3-process-test-seams")]
#[doc(hidden)]
pub fn prepare_source_real_i3_read_only_provider_localnet_launch()
-> Result<Sys5I3PreparedProviderLocalnetLaunch, Sys5I3ProviderLaunchError> {
    prepare_source_real_i3_read_only_provider_localnet_launch_with_profiles(
        I3ReadOnlyProviderFixtureProfile::Value41,
        None,
        None,
    )
}

/// Build the fixed source-real launch whose genuine retained executor result
/// is withheld from its first stream and receives that first send only on the
/// verified second provider session. The profile is sealed trusted-control
/// data, not an input to a request, result, or transport bootstrap.
#[cfg(feature = "i3-process-test-seams")]
#[doc(hidden)]
pub fn prepare_source_real_i3_read_only_provider_localnet_launch_held_result_first_send_on_second_session()
-> Result<Sys5I3PreparedProviderLocalnetLaunch, Sys5I3ProviderLaunchError> {
    prepare_source_real_i3_read_only_provider_localnet_launch_with_profiles(
        I3ReadOnlyProviderFixtureProfile::Value41,
        None,
        Some(I3PrivateFixedProviderNetworkConformanceProfile::HeldResultFirstSendOnSecondSession),
    )
}

/// Build the fixed source-real launch whose first executor result is actually
/// read, exact-bound, and deliberately discarded before semantic consumption
/// across the bounded second-session observation. The retained factory
/// spelling denotes the absence of a consume, not a claim of wire loss. No
/// caller can select a loss mode or result payload.
#[cfg(feature = "i3-process-test-seams")]
#[doc(hidden)]
pub fn prepare_source_real_i3_read_only_provider_localnet_launch_sent_result_lost_before_consume()
-> Result<Sys5I3PreparedProviderLocalnetLaunch, Sys5I3ProviderLaunchError> {
    prepare_source_real_i3_read_only_provider_localnet_launch_with_profiles(
        I3ReadOnlyProviderFixtureProfile::Value41,
        None,
        Some(I3PrivateFixedProviderNetworkConformanceProfile::SentResultLostBeforeConsume),
    )
}

/// Build the fixed source-real launch for the one bounded second-session
/// duplicate-delivery experiment. It has no caller-provided carrier, replay
/// count, or retry policy input.
#[cfg(feature = "i3-process-test-seams")]
#[doc(hidden)]
pub fn prepare_source_real_i3_read_only_provider_localnet_launch_duplicate_provider_delivery_on_second_session()
-> Result<Sys5I3PreparedProviderLocalnetLaunch, Sys5I3ProviderLaunchError> {
    prepare_source_real_i3_read_only_provider_localnet_launch_with_profiles(
        I3ReadOnlyProviderFixtureProfile::Value41,
        None,
        Some(I3PrivateFixedProviderNetworkConformanceProfile::DuplicateProviderDeliveryOnSecondSession),
    )
}

/// Build the fixed source-real launch that retires the executor's real effect
/// authorization after its retained call and before the withheld result's
/// first second-session send.
#[cfg(feature = "i3-process-test-seams")]
#[doc(hidden)]
pub fn prepare_source_real_i3_read_only_provider_localnet_launch_post_call_effect_retired_before_held_result_send()
-> Result<Sys5I3PreparedProviderLocalnetLaunch, Sys5I3ProviderLaunchError> {
    prepare_source_real_i3_read_only_provider_localnet_launch_with_profiles(
        I3ReadOnlyProviderFixtureProfile::Value41,
        None,
        Some(
            I3PrivateFixedProviderNetworkConformanceProfile::PostCallEffectRetiredBeforeHeldResultSend,
        ),
    )
}

/// Build the fixed source-real launch for the sealed conformance experiment
/// that retires terminal observation authority before its initial preflight.
/// No caller selects the profile, a value, a native path, or an expected
/// result.
#[cfg(feature = "i3-process-test-seams")]
#[doc(hidden)]
pub fn prepare_source_real_i3_read_only_provider_localnet_launch_observer_retired_before_preflight()
-> Result<Sys5I3PreparedProviderLocalnetLaunch, Sys5I3ProviderLaunchError> {
    prepare_source_real_i3_read_only_provider_localnet_launch_with_profiles(
        I3ReadOnlyProviderFixtureProfile::Value41,
        Some(I3PrivateFixedTerminalObservationConformanceProfile::RetireBeforeInitialPreflight),
        None,
    )
}

/// Build the fixed source-real launch for the sealed conformance experiment
/// that retires terminal observation authority after projection and before
/// the final currentness commit.
#[cfg(feature = "i3-process-test-seams")]
#[doc(hidden)]
pub fn prepare_source_real_i3_read_only_provider_localnet_launch_observer_retired_after_projection_before_commit()
-> Result<Sys5I3PreparedProviderLocalnetLaunch, Sys5I3ProviderLaunchError> {
    prepare_source_real_i3_read_only_provider_localnet_launch_with_profiles(
        I3ReadOnlyProviderFixtureProfile::Value41,
        Some(
            I3PrivateFixedTerminalObservationConformanceProfile::RetireAfterProjectionBeforeCommit,
        ),
        None,
    )
}

/// Build the fixed source-real launch for the sealed conformance experiment
/// that performs one valid terminal export attempt and rejects a second.
#[cfg(feature = "i3-process-test-seams")]
#[doc(hidden)]
pub fn prepare_source_real_i3_read_only_provider_localnet_launch_observer_repeat_export()
-> Result<Sys5I3PreparedProviderLocalnetLaunch, Sys5I3ProviderLaunchError> {
    prepare_source_real_i3_read_only_provider_localnet_launch_with_profiles(
        I3ReadOnlyProviderFixtureProfile::Value41,
        Some(I3PrivateFixedTerminalObservationConformanceProfile::RepeatExport),
        None,
    )
}

/// Build the fixed source-real launch for the sealed conformance experiment
/// that retires effect use after the call while retaining separately current
/// terminal observation authorization.
#[cfg(feature = "i3-process-test-seams")]
#[doc(hidden)]
pub fn prepare_source_real_i3_read_only_provider_localnet_launch_effect_retired_observer_current()
-> Result<Sys5I3PreparedProviderLocalnetLaunch, Sys5I3ProviderLaunchError> {
    prepare_source_real_i3_read_only_provider_localnet_launch_with_profiles(
        I3ReadOnlyProviderFixtureProfile::Value41,
        Some(I3PrivateFixedTerminalObservationConformanceProfile::EffectRetiredObserverCurrent),
        None,
    )
}

/// Build the same finite source-real launch with one fixed, T0-selected
/// physical fixture profile. This feature-gated test seam never accepts a
/// caller-provided native path, bytes, authority, control, or result.
#[cfg(feature = "i3-process-test-seams")]
#[doc(hidden)]
pub fn prepare_source_real_i3_read_only_provider_localnet_launch_with_fixture_profile(
    profile: I3ReadOnlyProviderFixtureProfile,
) -> Result<Sys5I3PreparedProviderLocalnetLaunch, Sys5I3ProviderLaunchError> {
    prepare_source_real_i3_read_only_provider_localnet_launch_with_profiles(profile, None, None)
}

#[cfg(feature = "i3-process-test-seams")]
fn prepare_source_real_i3_read_only_provider_localnet_launch_with_profiles(
    profile: I3ReadOnlyProviderFixtureProfile,
    terminal_observation_conformance_profile: Option<
        I3PrivateFixedTerminalObservationConformanceProfile,
    >,
    provider_network_conformance_profile: Option<I3PrivateFixedProviderNetworkConformanceProfile>,
) -> Result<Sys5I3PreparedProviderLocalnetLaunch, Sys5I3ProviderLaunchError> {
    const SOURCE_PATH: &str = "samples/clean-near-end/mirrorea-i3-provider-effect/main.mir";
    const SOURCE: &str =
        include_str!("../../../samples/clean-near-end/mirrorea-i3-provider-effect/main.mir");

    let rejected = || {
        Sys5I3ProviderLaunchError::new(
            Sys5I3ProviderLaunchErrorKind::SourceCompositePreparationRejected,
        )
    };
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(SOURCE_PATH, SOURCE))
        .map_err(|_| rejected())?;
    let topology = DeclaredLogicalTopology::try_new(
        checked.program_identity().clone(),
        ["WorldAuthority", "ParticipantA", "ParticipantB", "ViewerC"],
    )
    .map_err(|_| rejected())?;
    let coverage =
        M9ReadOnlyProviderEffectCoverage::from_checked(&checked).map_err(|_| rejected())?;
    let static_plan = crate::sys3_projection::project_read_only_provider_effect_static(
        &checked, &topology, coverage,
    )
    .map_err(|_| rejected())?;
    verify_read_only_provider_effect_static_projection(&checked, &topology, &static_plan)
        .map_err(|_| rejected())?;
    let composite_discharge = M9CompositeFiniteRefinementChecker::default()
        .discharge_composite_candidate(
            &checked,
            M9CompositeContractCandidate::from_checked_surface(&checked)
                .map_err(|_| rejected())?
                .membership_auth_strengthening(),
        )
        .map_err(|_| rejected())?;
    let setup =
        I3TrustedReadOnlyProviderFixtureSetup::provision_source_real_profile_with_conformance(
            profile,
            terminal_observation_conformance_profile,
            provider_network_conformance_profile,
        )
        .map_err(|_| rejected())?;
    let bootstrap = setup
        .bootstrap_facts(&checked, &static_plan)
        .map_err(|_| rejected())?;
    let binding = setup
        .admit_resource_binding(&checked, &static_plan)
        .map_err(|_| rejected())?;
    let verified = I3ReadOnlyProviderCompositeCandidate::from_trusted_inputs(
        checked,
        static_plan,
        Some(bootstrap),
        Some(composite_discharge),
        Some(binding),
    )
    .map_err(|_| rejected())?
    .verify_membership_and_discharge()
    .map_err(|_| rejected())?;
    let effect_policy = setup
        .decide_fixed_policy(&verified)
        .map_err(|_| rejected())?;
    let terminal_observation_policy = setup
        .decide_fixed_terminal_observation_policy(&verified)
        .map_err(|_| rejected())?;
    verified
        .seal_with_policies(Some(effect_policy), Some(terminal_observation_policy))
        .map_err(|_| rejected())?
        .prepare_provider_localnet_launch(
            setup,
            [
                Sys5I3DeploymentSlot::new("process-a", "127.0.0.1:0", ["ParticipantA", "ViewerC"]),
                Sys5I3DeploymentSlot::new(
                    "process-b",
                    "127.0.0.1:0",
                    ["WorldAuthority", "ParticipantB"],
                ),
            ],
        )
        .map_err(|_| rejected())
}

/// Safe, observer-limited failure classification for the private composite
/// boundary. It never contains a fixture path, native error, grant, or
/// provider payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum I3ProviderCompositeErrorKind {
    TrustedFixtureProvisionFailed,
    StaticPlanMismatch,
    MissingTrustedBootstrapFacts,
    TrustedBootstrapMismatch,
    MissingCompositeDischarge,
    CompositeDischargeMismatch,
    MissingResourceBinding,
    ResourceBindingMismatch,
    ResourceBindingNotCurrent,
    AdmissionAlreadyUsed,
    MembershipAuthenticationFailed,
    SeparatePolicyRequired,
    PolicyMismatch,
    EffectAuthorizationRetired,
    OrdinarySnapshotRejected,
    ScopedSnapshotRestoreRejected,
    InactiveProcessHandoffRejected,
    ProviderRuntimeActivationPending,
}

/// Safe failure carrier for the private Stage 2b composite boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct I3ProviderCompositeError {
    kind: I3ProviderCompositeErrorKind,
}

impl I3ProviderCompositeError {
    const fn new(kind: I3ProviderCompositeErrorKind) -> Self {
        Self { kind }
    }

    pub(crate) const fn kind(&self) -> I3ProviderCompositeErrorKind {
        self.kind
    }

    pub(crate) const fn returned_effect_authorization(&self) -> bool {
        false
    }

    pub(crate) const fn returned_legacy_component(&self) -> bool {
        false
    }

    pub(crate) const fn provider_call_count(&self) -> usize {
        0
    }
}

pub(crate) struct TrustedFixtureContext {
    resource_runtime_nonce: [u8; 32],
    root: PathBuf,
    // This finite T0-selected expectation is copied only into the requester
    // child control. It is not source input, a host result, or observer data.
    requester_fixture_assertion: I3PrivateReadOnlyProviderFixtureAssertion,
    terminal_observation_conformance_profile:
        Option<I3PrivateFixedTerminalObservationConformanceProfile>,
    provider_network_conformance_profile: Option<I3PrivateFixedProviderNetworkConformanceProfile>,
    // The parent-held inactive cohort must observe the same retirement cell as
    // the trusted setup, rather than a copied point-in-time value.
    current: Arc<AtomicBool>,
    admission_sealed: AtomicBool,
}

/// Fixed non-value fixture expectation for the feature-gated A-side
/// post-consume assertion. This is private trusted-control data, never a
/// provider request/result input or observer projection.
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum I3PrivateReadOnlyProviderFixtureAssertion {
    ValueZero,
    Value41,
    ValueNegative7,
    DeclaredTargetAbsent,
    NoncanonicalInteger,
    ReadBoundary33Bytes,
    DeclaredTargetDirectory,
}

/// Fixed, T0-provisioned physical layouts for the declared logical resource
/// slot. This remains private setup data: source-real fixture profiles select
/// one of these finite forms, never a caller-provided path or byte payload.
enum I3PrivateDeclaredFixtureTarget {
    Absent,
    RegularFile(&'static [u8]),
    Directory,
}

/// Four sealed, finite observer conformance experiments. This private FD3
/// field is emitted only by the named source-real factories and cannot be
/// selected by a request, result, transport bootstrap, or decoded control.
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum I3PrivateFixedTerminalObservationConformanceProfile {
    RetireBeforeInitialPreflight,
    RetireAfterProjectionBeforeCommit,
    RepeatExport,
    EffectRetiredObserverCurrent,
}

/// Four sealed, finite provider-network experiments. This private T0 setup
/// field reaches an installed child only through its authenticated FD3 record;
/// it is never selected by a request, result, decoded frame, or transport
/// bootstrap value.
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum I3PrivateFixedProviderNetworkConformanceProfile {
    HeldResultFirstSendOnSecondSession,
    SentResultLostBeforeConsume,
    DuplicateProviderDeliveryOnSecondSession,
    PostCallEffectRetiredBeforeHeldResultSend,
}

/// The executor-only native binding carried inside the private provider FD3
/// control record. It is not an image field, capability, or source-derived
/// value, and intentionally has no Debug/path accessor.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct I3PrivateReadOnlyProviderResourceEnvelope {
    version: u8,
    root: PathBuf,
    logical_resource_slot: String,
    resource_runtime_nonce: [u8; 32],
}

impl fmt::Debug for I3PrivateReadOnlyProviderResourceEnvelope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("I3PrivateReadOnlyProviderResourceEnvelope(..)")
    }
}

impl I3PrivateReadOnlyProviderResourceEnvelope {
    pub(crate) fn matches_runtime_nonce(&self, nonce: &[u8; 32]) -> bool {
        self.version == 1
            && self.resource_runtime_nonce == *nonce
            && self.logical_resource_slot == TRUSTED_LOGICAL_RESOURCE_SLOT
            && self.root.is_absolute()
    }

    pub(crate) fn private_resource_path(&self) -> PathBuf {
        self.root.join(&self.logical_resource_slot)
    }
}

impl TrustedFixtureContext {
    pub(crate) fn is_current(&self) -> bool {
        self.current.load(Ordering::Acquire)
    }

    fn reserve_successful_admission(&self) -> bool {
        self.admission_sealed
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }

    fn release_unsuccessful_admission_reservation(&self) {
        self.admission_sealed.store(false, Ordering::Release);
    }
}

/// T0-owned setup for the sole nonsecret local fixture namespace. The native
/// location remains private; callers can neither select it nor inspect it.
pub(crate) struct I3TrustedReadOnlyProviderFixtureSetup {
    context: Arc<TrustedFixtureContext>,
}

impl fmt::Debug for I3TrustedReadOnlyProviderFixtureSetup {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("I3TrustedReadOnlyProviderFixtureSetup(..)")
    }
}

impl I3TrustedReadOnlyProviderFixtureSetup {
    /// Provision the trusted fixture with the declared nonsecret target.
    pub(crate) fn provision_default() -> Result<Self, I3ProviderCompositeError> {
        Self::provision_with_declared_target(
            I3PrivateDeclaredFixtureTarget::RegularFile(b"0\n"),
            I3PrivateReadOnlyProviderFixtureAssertion::ValueZero,
            None,
            None,
        )
    }

    /// Provision the same trusted namespace with its declared target absent.
    /// Binding stays valid; a later CallStarted path is the first point at
    /// which the absence may become a ProviderResourceNotFound outcome.
    pub(crate) fn provision_without_declared_target() -> Result<Self, I3ProviderCompositeError> {
        Self::provision_with_declared_target(
            I3PrivateDeclaredFixtureTarget::Absent,
            I3PrivateReadOnlyProviderFixtureAssertion::DeclaredTargetAbsent,
            None,
            None,
        )
    }

    #[cfg(feature = "i3-process-test-seams")]
    fn provision_source_real_profile_with_conformance(
        profile: I3ReadOnlyProviderFixtureProfile,
        terminal_observation_conformance_profile: Option<
            I3PrivateFixedTerminalObservationConformanceProfile,
        >,
        provider_network_conformance_profile: Option<
            I3PrivateFixedProviderNetworkConformanceProfile,
        >,
    ) -> Result<Self, I3ProviderCompositeError> {
        match profile {
            I3ReadOnlyProviderFixtureProfile::Value41 => Self::provision_with_declared_target(
                I3PrivateDeclaredFixtureTarget::RegularFile(b"41\n"),
                I3PrivateReadOnlyProviderFixtureAssertion::Value41,
                terminal_observation_conformance_profile,
                provider_network_conformance_profile,
            ),
            I3ReadOnlyProviderFixtureProfile::ValueNegative7 => {
                Self::provision_with_declared_target(
                    I3PrivateDeclaredFixtureTarget::RegularFile(b"-7\n"),
                    I3PrivateReadOnlyProviderFixtureAssertion::ValueNegative7,
                    terminal_observation_conformance_profile,
                    provider_network_conformance_profile,
                )
            }
            I3ReadOnlyProviderFixtureProfile::DeclaredTargetAbsent => {
                Self::provision_with_declared_target(
                    I3PrivateDeclaredFixtureTarget::Absent,
                    I3PrivateReadOnlyProviderFixtureAssertion::DeclaredTargetAbsent,
                    terminal_observation_conformance_profile,
                    provider_network_conformance_profile,
                )
            }
            I3ReadOnlyProviderFixtureProfile::NoncanonicalInteger => {
                Self::provision_with_declared_target(
                    I3PrivateDeclaredFixtureTarget::RegularFile(b"01\n"),
                    I3PrivateReadOnlyProviderFixtureAssertion::NoncanonicalInteger,
                    terminal_observation_conformance_profile,
                    provider_network_conformance_profile,
                )
            }
            I3ReadOnlyProviderFixtureProfile::ReadBoundary33Bytes => {
                Self::provision_with_declared_target(
                    I3PrivateDeclaredFixtureTarget::RegularFile(
                        b"00000000000000000000000000000000\n",
                    ),
                    I3PrivateReadOnlyProviderFixtureAssertion::ReadBoundary33Bytes,
                    terminal_observation_conformance_profile,
                    provider_network_conformance_profile,
                )
            }
            I3ReadOnlyProviderFixtureProfile::DeclaredTargetDirectory => {
                Self::provision_with_declared_target(
                    I3PrivateDeclaredFixtureTarget::Directory,
                    I3PrivateReadOnlyProviderFixtureAssertion::DeclaredTargetDirectory,
                    terminal_observation_conformance_profile,
                    provider_network_conformance_profile,
                )
            }
        }
    }

    fn provision_with_declared_target(
        declared_target: I3PrivateDeclaredFixtureTarget,
        requester_fixture_assertion: I3PrivateReadOnlyProviderFixtureAssertion,
        terminal_observation_conformance_profile: Option<
            I3PrivateFixedTerminalObservationConformanceProfile,
        >,
        provider_network_conformance_profile: Option<
            I3PrivateFixedProviderNetworkConformanceProfile,
        >,
    ) -> Result<Self, I3ProviderCompositeError> {
        let fixture_directory_counter = NEXT_TRUSTED_FIXTURE_INCARNATION
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                current.checked_add(1)
            })
            .map_err(|_| {
                I3ProviderCompositeError::new(
                    I3ProviderCompositeErrorKind::TrustedFixtureProvisionFailed,
                )
            })?;
        let mut resource_runtime_nonce = [0_u8; 32];
        getrandom::fill(&mut resource_runtime_nonce).map_err(|_| {
            I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::TrustedFixtureProvisionFailed,
            )
        })?;
        let root = std::env::temp_dir().join(format!(
            "mir-runtime-i3-read-only-provider-fixture-{}-{fixture_directory_counter}",
            std::process::id()
        ));
        fs::create_dir(&root).map_err(|_| {
            I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::TrustedFixtureProvisionFailed,
            )
        })?;

        let target_path = root.join(TRUSTED_LOGICAL_RESOURCE_SLOT);
        let target_provisioned = match declared_target {
            I3PrivateDeclaredFixtureTarget::Absent => true,
            I3PrivateDeclaredFixtureTarget::RegularFile(contents) => {
                fs::write(&target_path, contents).is_ok()
            }
            I3PrivateDeclaredFixtureTarget::Directory => fs::create_dir(&target_path).is_ok(),
        };
        if !target_provisioned {
            let _ = fs::remove_dir_all(&root);
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::TrustedFixtureProvisionFailed,
            ));
        }

        Ok(Self {
            context: Arc::new(TrustedFixtureContext {
                resource_runtime_nonce,
                root,
                requester_fixture_assertion,
                terminal_observation_conformance_profile,
                provider_network_conformance_profile,
                current: Arc::new(AtomicBool::new(true)),
                admission_sealed: AtomicBool::new(false),
            }),
        })
    }

    /// Issue setup-owned bootstrap facts. They identify this actual trusted
    /// setup, but cannot themselves create membership or effect authority.
    pub(crate) fn bootstrap_facts(
        &self,
        checked: &CheckedSurfaceV0,
        static_plan: &ReadOnlyProviderEffectStaticProjection,
    ) -> Result<I3ReadOnlyProviderCompositeBootstrap, I3ProviderCompositeError> {
        if !self.context.current.load(Ordering::Acquire) {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::ResourceBindingNotCurrent,
            ));
        }
        trusted_read_only_provider_composite_bootstrap(
            checked,
            static_plan,
            self.context.resource_runtime_nonce,
        )
        .map(I3ReadOnlyProviderCompositeBootstrap::from_m9)
        .map_err(|_| {
            I3ProviderCompositeError::new(I3ProviderCompositeErrorKind::TrustedBootstrapMismatch)
        })
    }

    /// Bind the already-declared checked logical slot to this actual fixture
    /// incarnation. No target lookup is performed here.
    pub(crate) fn admit_resource_binding(
        &self,
        checked: &CheckedSurfaceV0,
        static_plan: &ReadOnlyProviderEffectStaticProjection,
    ) -> Result<I3AdmittedReadOnlyProviderResourceBinding, I3ProviderCompositeError> {
        if !self.context.current.load(Ordering::Acquire) {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::ResourceBindingNotCurrent,
            ));
        }
        let Some(coverage) = exact_static_coverage(checked, static_plan) else {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::StaticPlanMismatch,
            ));
        };
        if coverage.contract().logical_resource_slot() != TRUSTED_LOGICAL_RESOURCE_SLOT {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::ResourceBindingMismatch,
            ));
        }

        Ok(I3AdmittedReadOnlyProviderResourceBinding {
            context: Arc::clone(&self.context),
            program_identity: checked.program_identity().clone(),
            coverage,
        })
    }

    /// Retire the real fixture incarnation. Existing bindings keep no live
    /// authority after this operation.
    pub(crate) fn retire(&mut self) {
        self.context.current.store(false, Ordering::Release);
    }

    pub(crate) fn matches_live_context(&self, context: &Arc<TrustedFixtureContext>) -> bool {
        Arc::ptr_eq(&self.context, context) && self.context.current.load(Ordering::Acquire)
    }

    pub(crate) fn i3_private_resource_runtime_nonce(&self) -> [u8; 32] {
        self.context.resource_runtime_nonce
    }

    /// Move no semantic result through this boundary: return only the finite
    /// T0 fixture expectation that the requester control must retain for its
    /// post-consume test assertion.
    pub(crate) fn i3_private_requester_fixture_assertion(
        &self,
    ) -> I3PrivateReadOnlyProviderFixtureAssertion {
        self.context.requester_fixture_assertion
    }

    /// The profile is selected only by a named feature-gated source-real
    /// factory. It is copied into the private FD3 record, never into a
    /// request, result, image, or physical transport bootstrap.
    pub(crate) fn i3_private_terminal_observation_conformance_profile(
        &self,
    ) -> Option<I3PrivateFixedTerminalObservationConformanceProfile> {
        self.context.terminal_observation_conformance_profile
    }

    /// The fixed provider-network profile is source-real trusted setup data.
    /// Callers can only observe its presence through the prepared launch;
    /// neither the selector nor a fault outcome crosses this boundary.
    pub(crate) fn i3_private_provider_network_conformance_profile(
        &self,
    ) -> Option<I3PrivateFixedProviderNetworkConformanceProfile> {
        self.context.provider_network_conformance_profile
    }

    pub(crate) fn i3_private_executor_resource_envelope(
        &self,
    ) -> Result<I3PrivateReadOnlyProviderResourceEnvelope, I3ProviderCompositeError> {
        if !self.context.current.load(Ordering::Acquire) {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::ResourceBindingNotCurrent,
            ));
        }
        Ok(I3PrivateReadOnlyProviderResourceEnvelope {
            version: 1,
            root: self.context.root.clone(),
            logical_resource_slot: TRUSTED_LOGICAL_RESOURCE_SLOT.to_string(),
            resource_runtime_nonce: self.context.resource_runtime_nonce,
        })
    }

    pub(crate) fn decide_fixed_policy(
        &self,
        verified: &I3VerifiedReadOnlyProviderComposite,
    ) -> Result<I3ReadOnlyProviderPolicyDecision, I3ProviderCompositeError> {
        if !self.context.current.load(Ordering::Acquire) {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::ResourceBindingNotCurrent,
            ));
        }
        verified
            .inner
            .issue_fixed_policy_proof(&self.context.resource_runtime_nonce)
            .map(|inner| I3ReadOnlyProviderPolicyDecision { inner })
            .map_err(|_| {
                I3ProviderCompositeError::new(I3ProviderCompositeErrorKind::PolicyMismatch)
            })
    }

    /// Make the separate fixed T0 decision for the reference-only terminal
    /// observation profile. It is distinct from effect-use policy and cannot
    /// be inferred from the source-visible observer label.
    pub(crate) fn decide_fixed_terminal_observation_policy(
        &self,
        verified: &I3VerifiedReadOnlyProviderComposite,
    ) -> Result<I3ReadOnlyProviderTerminalObservationPolicyDecision, I3ProviderCompositeError> {
        if !self.context.current.load(Ordering::Acquire) {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::ResourceBindingNotCurrent,
            ));
        }
        verified
            .inner
            .issue_fixed_terminal_observation_policy_proof(&self.context.resource_runtime_nonce)
            .map(|inner| I3ReadOnlyProviderTerminalObservationPolicyDecision { inner })
            .map_err(|_| {
                I3ProviderCompositeError::new(I3ProviderCompositeErrorKind::PolicyMismatch)
            })
    }
}

impl Drop for I3TrustedReadOnlyProviderFixtureSetup {
    fn drop(&mut self) {
        self.context.current.store(false, Ordering::Release);
        let _ = fs::remove_dir_all(&self.context.root);
    }
}

/// Opaque trusted membership/bootstrap input. It is not an authentication
/// result and cannot mint M9 membership or an effect capability.
pub(crate) struct I3ReadOnlyProviderCompositeBootstrap {
    inner: M9ReadOnlyProviderCompositeBootstrap,
}

impl fmt::Debug for I3ReadOnlyProviderCompositeBootstrap {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("I3ReadOnlyProviderCompositeBootstrap(..)")
    }
}

impl I3ReadOnlyProviderCompositeBootstrap {
    fn from_m9(inner: M9ReadOnlyProviderCompositeBootstrap) -> Self {
        Self { inner }
    }
}

/// Opaque source-bound resource context. It carries no native path, target
/// contents, grant, or adapter handle.
pub(crate) struct I3AdmittedReadOnlyProviderResourceBinding {
    context: Arc<TrustedFixtureContext>,
    program_identity: CheckedProgramIdentity,
    coverage: M9ReadOnlyProviderEffectCoverage,
}

impl fmt::Debug for I3AdmittedReadOnlyProviderResourceBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("I3AdmittedReadOnlyProviderResourceBinding(..)")
    }
}

impl I3AdmittedReadOnlyProviderResourceBinding {
    fn is_current(&self) -> bool {
        self.context.current.load(Ordering::Acquire)
    }

    fn matches(
        &self,
        checked: &CheckedSurfaceV0,
        static_plan: &ReadOnlyProviderEffectStaticProjection,
    ) -> bool {
        self.program_identity == *checked.program_identity()
            && exact_static_coverage(checked, static_plan)
                .is_some_and(|coverage| coverage == self.coverage)
    }

    fn resource_runtime_nonce(&self) -> [u8; 32] {
        self.context.resource_runtime_nonce
    }
}

pub(crate) struct I3ReadOnlyProviderCompositeCandidate {
    checked: CheckedSurfaceV0,
    static_plan: ReadOnlyProviderEffectStaticProjection,
    bootstrap: I3ReadOnlyProviderCompositeBootstrap,
    composite_discharge: M9CompositeFiniteRefinementDischarge,
    resource_binding: I3AdmittedReadOnlyProviderResourceBinding,
}

impl fmt::Debug for I3ReadOnlyProviderCompositeCandidate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("I3ReadOnlyProviderCompositeCandidate(..)")
    }
}

impl I3ReadOnlyProviderCompositeCandidate {
    pub(crate) fn from_trusted_inputs(
        checked: CheckedSurfaceV0,
        static_plan: ReadOnlyProviderEffectStaticProjection,
        bootstrap: Option<I3ReadOnlyProviderCompositeBootstrap>,
        composite_discharge: Option<M9CompositeFiniteRefinementDischarge>,
        resource_binding: Option<I3AdmittedReadOnlyProviderResourceBinding>,
    ) -> Result<Self, I3ProviderCompositeError> {
        let Some(coverage) = exact_static_coverage(&checked, &static_plan) else {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::StaticPlanMismatch,
            ));
        };
        let Some(bootstrap) = bootstrap else {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::MissingTrustedBootstrapFacts,
            ));
        };
        let Some(discharge) = composite_discharge else {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::MissingCompositeDischarge,
            ));
        };
        let Some(binding) = resource_binding else {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::MissingResourceBinding,
            ));
        };
        if !binding.is_current() {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::ResourceBindingNotCurrent,
            ));
        }
        if binding.resource_runtime_nonce() != *bootstrap.inner.resource_runtime_nonce() {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::TrustedBootstrapMismatch,
            ));
        }
        if !binding.matches(&checked, &static_plan) {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::ResourceBindingMismatch,
            ));
        }
        let expected_discharge = M9CompositeFiniteRefinementChecker::default()
            .discharge_composite_candidate(
                &checked,
                M9CompositeContractCandidate::from_checked_surface(&checked)
                    .map_err(|_| {
                        I3ProviderCompositeError::new(
                            I3ProviderCompositeErrorKind::CompositeDischargeMismatch,
                        )
                    })?
                    .membership_auth_strengthening(),
            )
            .map_err(|_| {
                I3ProviderCompositeError::new(
                    I3ProviderCompositeErrorKind::CompositeDischargeMismatch,
                )
            })?;
        if discharge != expected_discharge || discharge.provider_coverage() != &coverage {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::CompositeDischargeMismatch,
            ));
        }

        Ok(Self {
            checked,
            static_plan,
            bootstrap,
            composite_discharge: discharge,
            resource_binding: binding,
        })
    }

    pub(crate) fn verify_membership_and_discharge(
        self,
    ) -> Result<I3VerifiedReadOnlyProviderComposite, I3ProviderCompositeError> {
        let I3ReadOnlyProviderCompositeCandidate {
            checked,
            static_plan,
            bootstrap,
            composite_discharge,
            resource_binding,
        } = self;
        if !resource_binding.is_current() {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::ResourceBindingNotCurrent,
            ));
        }
        let resource_runtime_nonce = resource_binding.resource_runtime_nonce();
        verify_read_only_provider_composite(
            &checked,
            &static_plan,
            bootstrap.inner,
            &composite_discharge,
            resource_runtime_nonce,
        )
        .map(|inner| I3VerifiedReadOnlyProviderComposite {
            inner,
            context: Arc::clone(&resource_binding.context),
            checked,
            static_plan,
        })
        .map_err(|_| {
            I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::MembershipAuthenticationFailed,
            )
        })
    }
}

pub(crate) struct I3VerifiedReadOnlyProviderComposite {
    inner: M9VerifiedReadOnlyProviderComposite,
    context: Arc<TrustedFixtureContext>,
    checked: CheckedSurfaceV0,
    static_plan: ReadOnlyProviderEffectStaticProjection,
}

impl fmt::Debug for I3VerifiedReadOnlyProviderComposite {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("I3VerifiedReadOnlyProviderComposite(..)")
    }
}

impl I3VerifiedReadOnlyProviderComposite {
    pub(crate) fn seal_with_policy(
        self,
        policy: Option<I3ReadOnlyProviderPolicyDecision>,
    ) -> Result<I3InactiveReadOnlyProviderComposite, I3ProviderCompositeError> {
        let Some(policy) = policy else {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::SeparatePolicyRequired,
            ));
        };
        if !self.context.current.load(Ordering::Acquire) {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::ResourceBindingNotCurrent,
            ));
        }
        let I3VerifiedReadOnlyProviderComposite {
            inner,
            context,
            checked,
            static_plan,
        } = self;
        if !context.reserve_successful_admission() {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::AdmissionAlreadyUsed,
            ));
        }
        match inner.seal_with_fixed_policy(policy.inner) {
            Ok(inner) if context.current.load(Ordering::Acquire) => {
                Ok(I3InactiveReadOnlyProviderComposite {
                    inner,
                    context,
                    checked,
                    static_plan,
                })
            }
            Ok(_) => {
                context.release_unsuccessful_admission_reservation();
                Err(I3ProviderCompositeError::new(
                    I3ProviderCompositeErrorKind::ResourceBindingNotCurrent,
                ))
            }
            Err(_) => {
                context.release_unsuccessful_admission_reservation();
                Err(I3ProviderCompositeError::new(
                    I3ProviderCompositeErrorKind::PolicyMismatch,
                ))
            }
        }
    }

    /// Seal the actual Stage 3 provider component with independent effect-use
    /// and reference-only terminal-observation policy decisions. Existing
    /// callers using `seal_with_policy` intentionally retain no observer
    /// authorization.
    pub(crate) fn seal_with_policies(
        self,
        policy: Option<I3ReadOnlyProviderPolicyDecision>,
        terminal_observation_policy: Option<I3ReadOnlyProviderTerminalObservationPolicyDecision>,
    ) -> Result<I3InactiveReadOnlyProviderComposite, I3ProviderCompositeError> {
        let (Some(policy), Some(terminal_observation_policy)) =
            (policy, terminal_observation_policy)
        else {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::SeparatePolicyRequired,
            ));
        };
        if !self.context.current.load(Ordering::Acquire) {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::ResourceBindingNotCurrent,
            ));
        }
        let I3VerifiedReadOnlyProviderComposite {
            inner,
            context,
            checked,
            static_plan,
        } = self;
        if !context.reserve_successful_admission() {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::AdmissionAlreadyUsed,
            ));
        }
        match inner.seal_with_fixed_policies(policy.inner, terminal_observation_policy.inner) {
            Ok(inner) if context.current.load(Ordering::Acquire) => {
                Ok(I3InactiveReadOnlyProviderComposite {
                    inner,
                    context,
                    checked,
                    static_plan,
                })
            }
            Ok(_) => {
                context.release_unsuccessful_admission_reservation();
                Err(I3ProviderCompositeError::new(
                    I3ProviderCompositeErrorKind::ResourceBindingNotCurrent,
                ))
            }
            Err(_) => {
                context.release_unsuccessful_admission_reservation();
                Err(I3ProviderCompositeError::new(
                    I3ProviderCompositeErrorKind::PolicyMismatch,
                ))
            }
        }
    }
}

pub(crate) struct I3ReadOnlyProviderPolicyDecision {
    inner: M9ReadOnlyProviderPolicyProof,
}

impl fmt::Debug for I3ReadOnlyProviderPolicyDecision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("I3ReadOnlyProviderPolicyDecision(..)")
    }
}

/// Opaque result of the separate trusted terminal-observation policy decision.
/// It is not effect permission and has no observer payload getter.
pub(crate) struct I3ReadOnlyProviderTerminalObservationPolicyDecision {
    inner: M9ReadOnlyProviderTerminalObservationPolicyProof,
}

impl fmt::Debug for I3ReadOnlyProviderTerminalObservationPolicyDecision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("I3ReadOnlyProviderTerminalObservationPolicyDecision(..)")
    }
}

pub(crate) struct I3InactiveReadOnlyProviderComposite {
    inner: M9InactiveReadOnlyProviderComposite,
    context: Arc<TrustedFixtureContext>,
    checked: CheckedSurfaceV0,
    static_plan: ReadOnlyProviderEffectStaticProjection,
}

impl fmt::Debug for I3InactiveReadOnlyProviderComposite {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("I3InactiveReadOnlyProviderComposite(..)")
    }
}

impl I3InactiveReadOnlyProviderComposite {
    pub(crate) const fn activation_pending(&self) -> bool {
        true
    }

    pub(crate) const fn provider_runtime_active(&self) -> bool {
        false
    }

    pub(crate) const fn provider_call_count(&self) -> usize {
        0
    }

    pub(crate) fn has_effect_authorization(&self) -> bool {
        self.context.current.load(Ordering::Acquire) && self.inner.effect_authorization_is_current()
    }

    pub(crate) fn has_legacy_component(&self) -> bool {
        self.inner.component().is_component_scoped()
    }

    pub(crate) fn binding_is_current(&self) -> bool {
        self.context.current.load(Ordering::Acquire)
            && self
                .inner
                .resource_runtime_nonce_matches(&self.context.resource_runtime_nonce)
            && self.inner.effect_authorization_is_current()
    }

    pub(crate) fn retire_effect_authorization(&mut self) -> Result<(), I3ProviderCompositeError> {
        self.inner.retire_effect_authorization().map_err(|_| {
            I3ProviderCompositeError::new(I3ProviderCompositeErrorKind::EffectAuthorizationRetired)
        })
    }

    pub(crate) fn scoped_component_snapshot(&self) -> I3ReadOnlyProviderScopedComponentSnapshot {
        I3ReadOnlyProviderScopedComponentSnapshot {
            inner: self.inner.component().scoped_snapshot(),
        }
    }

    pub(crate) fn restricted_component_snapshot(
        &self,
        assigned_loci: &std::collections::BTreeSet<String>,
    ) -> I3ReadOnlyProviderScopedComponentSnapshot {
        I3ReadOnlyProviderScopedComponentSnapshot {
            inner: self
                .inner
                .component()
                .restricted_to_loci(assigned_loci)
                .scoped_snapshot(),
        }
    }

    /// T0-internal, non-authorizing inspection of the sealed component's
    /// checked identity and ordered lowering inventory. It never exposes the
    /// M8 instance, resource binding, effect authority, or witness.
    pub(crate) fn scoped_component_inventory(&self) -> I3ReadOnlyProviderScopedComponentInventory {
        I3ReadOnlyProviderScopedComponentInventory {
            inner: self.inner.component().inventory(),
        }
    }

    /// T0-internal, non-authorizing inspection of a restricted component
    /// inventory without creating an ordinary M8 instance or process image.
    pub(crate) fn restricted_component_inventory(
        &self,
        assigned_loci: &std::collections::BTreeSet<String>,
    ) -> I3ReadOnlyProviderScopedComponentInventory {
        I3ReadOnlyProviderScopedComponentInventory {
            inner: self
                .inner
                .component()
                .restricted_to_loci(assigned_loci)
                .inventory(),
        }
    }

    /// Consume only a dedicated snapshot whose M8 scope retains this exact
    /// admitted binding context. It returns a component, never an ordinary
    /// M8 instance or process image.
    pub(crate) fn consume_scoped_component_snapshot(
        &self,
        snapshot: I3ReadOnlyProviderScopedComponentSnapshot,
    ) -> Result<I3RestoredReadOnlyProviderScopedComponent, I3ProviderCompositeError> {
        if !self.context.current.load(Ordering::Acquire) {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::ResourceBindingNotCurrent,
            ));
        }
        if !self.inner.effect_authorization_is_current() {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::EffectAuthorizationRetired,
            ));
        }
        snapshot
            .inner
            .restore_for_resource_runtime_nonce(&self.context.resource_runtime_nonce)
            .map(|inner| I3RestoredReadOnlyProviderScopedComponent { inner })
            .map_err(|_| {
                I3ProviderCompositeError::new(I3ProviderCompositeErrorKind::ResourceBindingMismatch)
            })
    }

    pub(crate) fn ordinary_i3_private_snapshot(&self) -> Result<(), I3ProviderCompositeError> {
        self.inner
            .component()
            .ordinary_i3_private_snapshot()
            .map(|_| ())
            .map_err(|_| {
                I3ProviderCompositeError::new(
                    I3ProviderCompositeErrorKind::OrdinarySnapshotRejected,
                )
            })
    }

    /// Stage 2c's private path into the existing inactive process-image
    /// handoff. It retains the actual M9/SYS-4 facts through decoded-image
    /// validation, but does not activate a provider or start a runtime.
    pub(crate) fn into_inactive_process_cohort<I>(
        self,
        slots: I,
    ) -> Result<Sys5I3InactiveProviderCohort, I3ProviderCompositeError>
    where
        I: IntoIterator<Item = Sys5I3DeploymentSlot>,
    {
        if !self.context.current.load(Ordering::Acquire) {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::ResourceBindingNotCurrent,
            ));
        }
        let admission = Sys4InactiveProviderAdmission::from_m9_inactive_provider_composite(
            self.checked.program_identity().clone(),
            self.static_plan,
            self.inner,
        )
        .map_err(|_| {
            I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::InactiveProcessHandoffRejected,
            )
        })?;
        Sys5I3InactiveProviderCohort::from_parent_inactive_provider_admission(
            self.checked,
            admission,
            Arc::clone(&self.context),
            slots,
        )
        .map_err(|_| {
            I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::InactiveProcessHandoffRejected,
            )
        })
    }

    /// Consume the real trusted setup into the future supervised provider
    /// launch. Keeping the setup guard—not merely its retirement cell—makes
    /// fixture lifetime extend through the supervisor's eventual child reap.
    pub(crate) fn prepare_provider_localnet_launch<I>(
        self,
        setup: I3TrustedReadOnlyProviderFixtureSetup,
        slots: I,
    ) -> Result<Sys5I3PreparedProviderLocalnetLaunch, I3ProviderCompositeError>
    where
        I: IntoIterator<Item = Sys5I3DeploymentSlot>,
    {
        if !setup.matches_live_context(&self.context) {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::ResourceBindingMismatch,
            ));
        }
        let cohort = self.into_inactive_process_cohort(slots)?;
        Sys5I3PreparedProviderLocalnetLaunch::from_inactive_provider_cohort(cohort, setup).map_err(
            |_| {
                I3ProviderCompositeError::new(
                    I3ProviderCompositeErrorKind::ProviderRuntimeActivationPending,
                )
            },
        )
    }

    #[cfg(test)]
    pub(crate) fn test_only_i3_private_provider_role_snapshot(
        &self,
        role: crate::m9_auth_verification::M9I3ReadOnlyProviderChildRole,
    ) -> Result<
        crate::m9_auth_verification::M9I3PrivateReadOnlyProviderRoleSnapshot,
        I3ProviderCompositeError,
    > {
        if !self.context.current.load(Ordering::Acquire) {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::ResourceBindingNotCurrent,
            ));
        }
        self.inner
            .i3_private_provider_role_snapshot(role)
            .map_err(|_| {
                I3ProviderCompositeError::new(I3ProviderCompositeErrorKind::ResourceBindingMismatch)
            })
    }

    #[cfg(test)]
    pub(crate) fn test_only_install_i3_private_provider_role_snapshot(
        &self,
        snapshot: crate::m9_auth_verification::M9I3PrivateReadOnlyProviderRoleSnapshot,
        role: crate::m9_auth_verification::M9I3ReadOnlyProviderChildRole,
    ) -> Result<
        crate::m9_auth_verification::M9I3ReadOnlyProviderLocalAuthority,
        I3ProviderCompositeError,
    > {
        if !self.context.current.load(Ordering::Acquire) {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::ResourceBindingNotCurrent,
            ));
        }
        snapshot
            .install_local_authority(
                role,
                self.checked.program_identity().stable_key().as_str(),
                &self.context.resource_runtime_nonce,
            )
            .map_err(|_| {
                I3ProviderCompositeError::new(I3ProviderCompositeErrorKind::ResourceBindingMismatch)
            })
    }
}

/// The dedicated component snapshot preserves its provider scope and has no
/// ordinary process-image route.
pub(crate) struct I3ReadOnlyProviderScopedComponentSnapshot {
    inner: M8ReadOnlyProviderEffectComponentSnapshot,
}

impl fmt::Debug for I3ReadOnlyProviderScopedComponentSnapshot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("I3ReadOnlyProviderScopedComponentSnapshot(..)")
    }
}

impl I3ReadOnlyProviderScopedComponentSnapshot {
    pub(crate) fn component_scope_retained(&self) -> bool {
        self.inner.is_component_scoped()
    }

    pub(crate) fn restore(
        self,
    ) -> Result<I3RestoredReadOnlyProviderScopedComponent, I3ProviderCompositeError> {
        self.inner
            .restore()
            .map(|inner| I3RestoredReadOnlyProviderScopedComponent { inner })
            .map_err(|_| {
                I3ProviderCompositeError::new(
                    I3ProviderCompositeErrorKind::ScopedSnapshotRestoreRejected,
                )
            })
    }
}

/// T0-internal, non-authorizing component inventory. The wrapped M8 inventory
/// retains only checked identity and lowering correspondence; it carries no
/// authority, witness, resource path, or native handle.
pub(crate) struct I3ReadOnlyProviderScopedComponentInventory {
    inner: M8ReadOnlyProviderEffectComponentInventory,
}

impl fmt::Debug for I3ReadOnlyProviderScopedComponentInventory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("I3ReadOnlyProviderScopedComponentInventory(..)")
    }
}

impl I3ReadOnlyProviderScopedComponentInventory {
    pub(crate) fn matches_checked_program_identity(&self, checked: &CheckedSurfaceV0) -> bool {
        self.inner.matches_checked_program_identity(checked)
    }

    /// The number of provider rows declared by the checked source profile;
    /// this remains distinct from the retained ordinary lowering inventory.
    pub(crate) const fn provider_lowering_count(&self) -> usize {
        self.inner.provider_lowering_count()
    }

    pub(crate) fn retained_lowering_ordinals(&self) -> Vec<usize> {
        self.inner.retained_lowering_ordinals()
    }

    pub(crate) fn retains_exact_non_provider_ordered_lowering_associations(
        &self,
        checked: &CheckedSurfaceV0,
    ) -> bool {
        self.inner
            .retains_exact_non_provider_ordered_lowering_associations(checked)
    }
}

/// Restored only from the dedicated scoped snapshot.  It remains a component,
/// not a whole-program M8 instance.
pub(crate) struct I3RestoredReadOnlyProviderScopedComponent {
    inner: crate::m8_runtime_admission::M8ReadOnlyProviderEffectComponent,
}

impl fmt::Debug for I3RestoredReadOnlyProviderScopedComponent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("I3RestoredReadOnlyProviderScopedComponent(..)")
    }
}

impl I3RestoredReadOnlyProviderScopedComponent {
    pub(crate) fn is_component_scoped(&self) -> bool {
        self.inner.is_component_scoped()
    }

    pub(crate) fn ordinary_i3_private_snapshot(&self) -> Result<(), I3ProviderCompositeError> {
        self.inner
            .ordinary_i3_private_snapshot()
            .map(|_| ())
            .map_err(|_| {
                I3ProviderCompositeError::new(
                    I3ProviderCompositeErrorKind::OrdinarySnapshotRejected,
                )
            })
    }
}

fn exact_static_coverage(
    checked: &CheckedSurfaceV0,
    static_plan: &ReadOnlyProviderEffectStaticProjection,
) -> Option<M9ReadOnlyProviderEffectCoverage> {
    let topology = DeclaredLogicalTopology::try_new(
        checked.program_identity().clone(),
        checked
            .static_environment()
            .loci()
            .iter()
            .map(|locus| locus.name()),
    )
    .ok()?;
    verify_read_only_provider_effect_static_projection(checked, &topology, static_plan).ok()?;
    let coverage = static_plan.provider_coverage().clone();
    (coverage.program_identity() == checked.program_identity() && coverage.matches_checked(checked))
        .then_some(coverage)
}
