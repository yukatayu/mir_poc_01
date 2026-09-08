//! Private Stage 2b inactive-composite admission for the finite read-only
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
        reason = "Stage 2b inactive composite awaits its direct SYS-4/SYS-5 consumer"
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

use mir_semantics::{
    m9_finite_refinement::{
        M9CompositeContractCandidate, M9CompositeFiniteRefinementChecker,
        M9CompositeFiniteRefinementDischarge, M9ReadOnlyProviderEffectCoverage,
    },
    surface_v0_pipeline::{CheckedProgramIdentity, CheckedSurfaceV0},
};

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
        M9ReadOnlyProviderPolicyProof, M9VerifiedReadOnlyProviderComposite,
        trusted_read_only_provider_composite_bootstrap, verify_read_only_provider_composite,
    },
};

const TRUSTED_LOGICAL_RESOURCE_SLOT: &str = "sample_input";
static NEXT_TRUSTED_FIXTURE_INCARNATION: AtomicU64 = AtomicU64::new(1);

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

struct TrustedFixtureContext {
    resource_runtime_nonce: [u8; 32],
    root: PathBuf,
    current: AtomicBool,
    admission_sealed: AtomicBool,
}

impl TrustedFixtureContext {
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
        Self::provision(true)
    }

    /// Provision the same trusted namespace with its declared target absent.
    /// Binding stays valid; a later CallStarted path is the first point at
    /// which the absence may become a ProviderResourceNotFound outcome.
    pub(crate) fn provision_without_declared_target() -> Result<Self, I3ProviderCompositeError> {
        Self::provision(false)
    }

    fn provision(create_declared_target: bool) -> Result<Self, I3ProviderCompositeError> {
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

        if create_declared_target
            && fs::write(root.join(TRUSTED_LOGICAL_RESOURCE_SLOT), b"0\n").is_err()
        {
            let _ = fs::remove_dir_all(&root);
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::TrustedFixtureProvisionFailed,
            ));
        }

        Ok(Self {
            context: Arc::new(TrustedFixtureContext {
                resource_runtime_nonce,
                root,
                current: AtomicBool::new(true),
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
        let I3VerifiedReadOnlyProviderComposite { inner, context } = self;
        if !context.reserve_successful_admission() {
            return Err(I3ProviderCompositeError::new(
                I3ProviderCompositeErrorKind::AdmissionAlreadyUsed,
            ));
        }
        match inner.seal_with_fixed_policy(policy.inner) {
            Ok(inner) if context.current.load(Ordering::Acquire) => {
                Ok(I3InactiveReadOnlyProviderComposite { inner, context })
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

pub(crate) struct I3InactiveReadOnlyProviderComposite {
    inner: M9InactiveReadOnlyProviderComposite,
    context: Arc<TrustedFixtureContext>,
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
