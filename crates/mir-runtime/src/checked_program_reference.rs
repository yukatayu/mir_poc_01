//! Shared opaque checked-program reference for internal transport consumers.
//!
//! This preserves the established SYS-5 domain and output exactly while
//! allowing authority kernels to retain their full private checked identity.

use sha2::{Digest, Sha256};

const CHECKED_PROGRAM_REF_DOMAIN: &[u8] = b"mirrorea/sys5/checked-program-ref/v1\0";
const CHECKED_PROGRAM_REF_OUTPUT_PREFIX: &str = "sys5-checked-program-sha256-v1:";

/// Returns the established domain-separated SHA-256 reference for one exact
/// checked-program stable key. The key itself remains local/private state.
pub(crate) fn checked_program_identity_ref(stable_key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(CHECKED_PROGRAM_REF_DOMAIN);
    hasher.update(
        u64::try_from(stable_key.len())
            .expect("logical source input length fits u64")
            .to_le_bytes(),
    );
    hasher.update(stable_key.as_bytes());
    format!("sys5-checked-program-sha256-v1:{:x}", hasher.finalize())
}

/// Recognizes only the fixed opaque representation emitted by
/// [`checked_program_identity_ref`]. This is a structural transport check;
/// consumers still compare the reference with one recomputed from retained
/// full private identity before accepting a peer.
pub(crate) fn is_checked_program_identity_ref(reference: &str) -> bool {
    let Some(digest) = reference.strip_prefix(CHECKED_PROGRAM_REF_OUTPUT_PREFIX) else {
        return false;
    };
    digest.len() == 64
        && digest
            .bytes()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}
