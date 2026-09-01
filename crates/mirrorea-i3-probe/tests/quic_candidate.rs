// This integration target exercises the probe facade; direct candidate dependencies live in the library and child binary.
#![allow(unused_crate_dependencies)]

mod support;

use mirrorea_i3_probe::{CandidateCase, TransportCandidate};

use support::{assert_common_candidate_contract, row, run_candidate};

#[test]
fn quic_runs_the_full_source_bound_inventory_on_reliable_bidirectional_streams() {
    let run = run_candidate(TransportCandidate::QuicReliableBidirectionalStream);

    assert_common_candidate_contract(&run, TransportCandidate::QuicReliableBidirectionalStream);
    assert!(
        run.transport_features()
            .reliable_bidirectional_streams_only(),
        "the QUIC candidate must use reliable bidirectional streams only"
    );
    assert!(
        !run.transport_features().datagram_enabled(),
        "QUIC datagrams are excluded from I3-0"
    );
    assert!(
        !run.transport_features().zero_rtt_enabled(),
        "0-RTT is outside the bounded comparison"
    );
    assert_eq!(
        row(&run, CandidateCase::TruncatedFrame).mechanism(),
        "quic-send-stream-reset"
    );
    assert_eq!(
        row(&run, CandidateCase::DisconnectAfterAdmissionBeforeResult).mechanism(),
        "quic-connection-close"
    );
}
