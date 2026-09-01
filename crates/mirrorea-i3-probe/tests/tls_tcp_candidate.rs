// This integration target exercises the probe facade; direct candidate dependencies live in the library and child binary.
#![allow(unused_crate_dependencies)]

mod support;

use mirrorea_i3_probe::{CandidateCase, TransportCandidate};

use support::{assert_common_candidate_contract, row, run_candidate};

#[test]
fn tls_over_tcp_runs_the_full_source_bound_inventory_in_child_processes() {
    let run = run_candidate(TransportCandidate::TlsOverTcpFramedReliableStream);

    assert_common_candidate_contract(&run, TransportCandidate::TlsOverTcpFramedReliableStream);
    assert_eq!(
        row(&run, CandidateCase::DisconnectBeforeAdmission).mechanism(),
        "tls-tcp-connection-close"
    );
    assert_eq!(
        row(&run, CandidateCase::DisconnectAfterAdmissionBeforeResult).mechanism(),
        "tls-tcp-connection-close"
    );
}
