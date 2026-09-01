//! Candidate B: QUIC reliable bidirectional streams.
//!
//! This module carries only the private I3-0 probe frame. It neither decodes
//! it into semantic authority nor uses transport metadata as an admission
//! decision; the common harness retains that independently bound contract.
//! Datagrams, 0-RTT, platform trust, and permissive verification are excluded.

use std::{io, net::SocketAddr, sync::Arc};

use quinn::{
    Connection, Endpoint, RecvStream, SendStream,
    crypto::rustls::{QuicClientConfig, QuicServerConfig},
};
use rustls::{
    ClientConfig, RootCertStore, ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer},
};

use crate::{
    MAX_PRIVATE_FRAME_BYTES,
    candidates::{
        CandidateCaseInput, CandidateExecutionError, CandidateExecutionErrorKind,
        CandidateTransportDisposition, CandidateTransportObservation,
    },
    process_harness::{
        CandidateChildError, ChildProcessControl, ChildProcessEvent, ChildProcessHarness,
        ChildRole, ChildTransportFailureClass,
    },
    receiver_canary::{ClientChildProbeReplyReceipt, ReceiverChildIngress},
};

const SEND_STREAM_RESET_MECHANISM: &str = "quic-send-stream-reset";
const CONNECTION_CLOSE_MECHANISM: &str = "quic-connection-close";
const BIDIRECTIONAL_STREAM_MECHANISM: &str = "quic-reliable-bidirectional-stream";
const PROBE_ALPN: &[u8] = b"mirrorea-i3-probe-quic-v1";
const RESET_CODE: u32 = 1;
const MAX_PRIVATE_REPLY_BYTES: usize = 16 * 1024;

/// Runs one actual QUIC server/client child pair through the common
/// supervisor. Children report only raw bytes and transport-local status.
pub(crate) fn execute(
    input: CandidateCaseInput,
    harness: &mut ChildProcessHarness,
) -> Result<CandidateTransportObservation, CandidateExecutionError> {
    let server = harness.spawn_server().map_err(map_child_error)?;
    let endpoint = match harness.next_event(server).map_err(map_child_error)? {
        ChildProcessEvent::Ready { endpoint } => endpoint,
        ChildProcessEvent::TransportComplete { .. }
        | ChildProcessEvent::ReceiverChildReport { .. }
        | ChildProcessEvent::ClientProbeReplyReceipts { .. }
        | ChildProcessEvent::TransportFailure { .. } => {
            return Err(CandidateExecutionError::new(
                CandidateExecutionErrorKind::ChildProtocolRejected,
            ));
        }
    };
    let client = harness.spawn_client(endpoint).map_err(map_child_error)?;

    // Both waits spend the common harness deadline. This module creates no
    // candidate-local retry or timeout policy.
    let client_event = match harness.next_event(client).map_err(map_child_error)? {
        event @ ChildProcessEvent::ClientProbeReplyReceipts { .. } => event,
        ChildProcessEvent::TransportComplete { received_frames } if received_frames.is_empty() => {
            ChildProcessEvent::TransportComplete { received_frames }
        }
        ChildProcessEvent::TransportComplete { .. }
        | ChildProcessEvent::Ready { .. }
        | ChildProcessEvent::ReceiverChildReport { .. }
        | ChildProcessEvent::TransportFailure { .. } => {
            return Err(CandidateExecutionError::new(
                CandidateExecutionErrorKind::ChildProtocolRejected,
            ));
        }
    };

    let server_event = harness.next_event(server).map_err(map_child_error)?;
    observation_from_child_events(input.case(), server_event, client_event)
}

/// Performs one independently spawned child role. Credentials travel only via
/// private inherited stdin control; no key/certificate file, argv, environment,
/// OS trust store, or permissive verifier is used.
pub(crate) fn execute_child(
    control: ChildProcessControl,
) -> Result<ChildProcessEvent, CandidateExecutionError> {
    install_ring_provider()?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|_| transport_failure())?;
    match control.role() {
        ChildRole::Server => runtime.block_on(run_server(control)),
        ChildRole::Client => runtime.block_on(run_client(control)),
    }
}

fn observation_from_child_events(
    case: crate::CandidateCase,
    server_event: ChildProcessEvent,
    client_event: ChildProcessEvent,
) -> Result<CandidateTransportObservation, CandidateExecutionError> {
    let disposition = match (&server_event, case) {
        (ChildProcessEvent::ReceiverChildReport { .. }, _) => match case {
            crate::CandidateCase::DeterministicFragmentedRoundTrip => {
                CandidateTransportDisposition::CompleteFrame
            }
            crate::CandidateCase::DisconnectAfterAdmissionBeforeResult => {
                CandidateTransportDisposition::DisconnectAfterAdmissionBeforeResult
            }
            crate::CandidateCase::DuplicateAcrossReconnect => {
                CandidateTransportDisposition::DuplicateAcrossReconnect
            }
            crate::CandidateCase::TamperedSemanticAdmissionReference => {
                CandidateTransportDisposition::TamperedSemanticAdmissionReference
            }
            crate::CandidateCase::ObserverSafeEvidence => {
                CandidateTransportDisposition::ObserverSafeEvidence
            }
            crate::CandidateCase::ConnectWithoutSemanticAdmission
            | crate::CandidateCase::TruncatedFrame
            | crate::CandidateCase::OversizedFrame
            | crate::CandidateCase::DisconnectBeforeAdmission => return Err(transport_failure()),
        },
        (ChildProcessEvent::TransportComplete { .. }, _) => match case {
            crate::CandidateCase::ConnectWithoutSemanticAdmission => {
                CandidateTransportDisposition::Connected
            }
            _ => return Err(transport_failure()),
        },
        (ChildProcessEvent::TransportFailure { class, .. }, _) => match (case, *class) {
            (crate::CandidateCase::TruncatedFrame, ChildTransportFailureClass::TruncatedFrame) => {
                CandidateTransportDisposition::TruncatedFrame
            }
            (crate::CandidateCase::OversizedFrame, ChildTransportFailureClass::OversizedFrame) => {
                CandidateTransportDisposition::OversizedFrame
            }
            (
                crate::CandidateCase::DisconnectBeforeAdmission,
                ChildTransportFailureClass::DisconnectBeforeAdmission,
            ) => CandidateTransportDisposition::DisconnectBeforeAdmission,
            _ => return Err(transport_failure()),
        },
        (
            ChildProcessEvent::Ready { .. } | ChildProcessEvent::ClientProbeReplyReceipts { .. },
            _,
        ) => {
            return Err(CandidateExecutionError::new(
                CandidateExecutionErrorKind::ChildProtocolRejected,
            ));
        }
    };
    CandidateTransportObservation::from_child_process_events(
        mechanism_for(case),
        disposition,
        server_event,
        client_event,
    )
}

fn map_child_error(error: CandidateChildError) -> CandidateExecutionError {
    let kind = match error {
        CandidateChildError::Protocol | CandidateChildError::CandidateUnavailable => {
            CandidateExecutionErrorKind::ChildProtocolRejected
        }
        CandidateChildError::Lifecycle => CandidateExecutionErrorKind::ChildLifecycleFailed,
        CandidateChildError::Deadline => CandidateExecutionErrorKind::DeadlineExceeded,
    };
    CandidateExecutionError::new(kind)
}

fn mechanism_for(case: crate::CandidateCase) -> &'static str {
    match case {
        crate::CandidateCase::TruncatedFrame | crate::CandidateCase::DisconnectBeforeAdmission => {
            SEND_STREAM_RESET_MECHANISM
        }
        crate::CandidateCase::DisconnectAfterAdmissionBeforeResult => CONNECTION_CLOSE_MECHANISM,
        crate::CandidateCase::ConnectWithoutSemanticAdmission
        | crate::CandidateCase::DeterministicFragmentedRoundTrip
        | crate::CandidateCase::OversizedFrame
        | crate::CandidateCase::DuplicateAcrossReconnect
        | crate::CandidateCase::TamperedSemanticAdmissionReference
        | crate::CandidateCase::ObserverSafeEvidence => BIDIRECTIONAL_STREAM_MECHANISM,
    }
}

fn install_ring_provider() -> Result<(), CandidateExecutionError> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .map_err(|_| transport_failure())
}

async fn run_server(
    control: ChildProcessControl,
) -> Result<ChildProcessEvent, CandidateExecutionError> {
    let endpoint = Endpoint::server(server_config(&control)?, loopback_unspecified())
        .map_err(|_| transport_failure())?;
    emit_ready(
        endpoint
            .local_addr()
            .map_err(|_| transport_failure())?
            .to_string(),
    )?;

    let mut receiver_child = control.receiver_child_canary().map_err(map_child_error)?;
    let connections =
        usize::from(control.case() == crate::CandidateCase::DuplicateAcrossReconnect) + 1;
    let mut received_frames = Vec::with_capacity(connections);
    for _ in 0..connections {
        let connection = accept_connection(&endpoint).await?;
        match read_server_connection(control.case(), connection).await? {
            ServerRead::Complete {
                frame,
                connection,
                mut reply_stream,
            } => {
                let ingress = receiver_child.receive_complete_frame(&frame);
                received_frames.push(frame);
                if control.case() == crate::CandidateCase::DisconnectAfterAdmissionBeforeResult {
                    receiver_child.cut_result_path_before_reply();
                    connection.close(RESET_CODE.into(), b"i3-0 result path closed");
                    continue;
                }
                if let ReceiverChildIngress::Reply(reply) = ingress {
                    reply_stream
                        .write_all(reply.as_bytes())
                        .await
                        .map_err(|_| transport_failure())?;
                }
                reply_stream.finish().map_err(|_| transport_failure())?;
                wait_for_peer_read(&reply_stream).await?;
            }
            ServerRead::NoFrame => {
                return Ok(match control.case() {
                    crate::CandidateCase::DisconnectBeforeAdmission => {
                        ChildProcessEvent::TransportFailure {
                            class: ChildTransportFailureClass::DisconnectBeforeAdmission,
                            received_capture: Vec::new(),
                        }
                    }
                    crate::CandidateCase::ConnectWithoutSemanticAdmission => {
                        ChildProcessEvent::TransportComplete {
                            received_frames: Vec::new(),
                        }
                    }
                    _ => return Err(transport_failure()),
                });
            }
            ServerRead::Truncated(received_capture) => {
                return Ok(ChildProcessEvent::TransportFailure {
                    class: ChildTransportFailureClass::TruncatedFrame,
                    received_capture,
                });
            }
            ServerRead::Oversized(received_capture) => {
                return Ok(ChildProcessEvent::TransportFailure {
                    class: ChildTransportFailureClass::OversizedFrame,
                    received_capture,
                });
            }
        }
    }
    if control.case() == crate::CandidateCase::DisconnectAfterAdmissionBeforeResult {
        // `Connection::close` initiates a close but cannot guarantee the
        // peer observes its CONNECTION_CLOSE before this endpoint drops. The
        // endpoint stays live until that actual close drains; the common
        // supervisor remains the only deadline authority.
        endpoint.wait_idle().await;
    }
    Ok(ChildProcessEvent::ReceiverChildReport {
        received_frames,
        report: receiver_child.report(),
    })
}

async fn run_client(
    control: ChildProcessControl,
) -> Result<ChildProcessEvent, CandidateExecutionError> {
    let server = control
        .endpoint()
        .ok_or_else(transport_failure)?
        .parse::<SocketAddr>()
        .map_err(|_| transport_failure())?;
    let mut endpoint = Endpoint::client(loopback_unspecified()).map_err(|_| transport_failure())?;
    endpoint.set_default_client_config(client_config(&control)?);
    let connections =
        usize::from(control.case() == crate::CandidateCase::DuplicateAcrossReconnect) + 1;

    let mut receipts = Vec::new();
    // Exactly one explicit new connection for the duplicate case, and no
    // retry loop for any other case.
    for sequence in 1..=connections {
        let connection = endpoint
            .connect(server, "localhost")
            .map_err(|_| transport_failure())?
            .await
            .map_err(|_| transport_failure())?;
        if let Some(receipt) =
            write_client_connection(control.case(), control.frame(), connection, sequence).await?
        {
            receipts.push(receipt);
        }
    }
    endpoint.close(0_u32.into(), b"i3-0 probe complete");
    endpoint.wait_idle().await;
    Ok(ChildProcessEvent::ClientProbeReplyReceipts { receipts })
}

async fn accept_connection(endpoint: &Endpoint) -> Result<Connection, CandidateExecutionError> {
    endpoint
        .accept()
        .await
        .ok_or_else(transport_failure)?
        .await
        .map_err(|_| transport_failure())
}

async fn write_client_connection(
    case: crate::CandidateCase,
    frame: &[u8],
    connection: Connection,
    reply_sequence: usize,
) -> Result<Option<ClientChildProbeReplyReceipt>, CandidateExecutionError> {
    match case {
        crate::CandidateCase::ConnectWithoutSemanticAdmission => {
            connection.close(0_u32.into(), b"i3-0 handshake-only");
            Ok(None)
        }
        crate::CandidateCase::DisconnectBeforeAdmission => {
            let (mut send, _receive) = connection
                .open_bi()
                .await
                .map_err(|_| transport_failure())?;
            send.reset(RESET_CODE.into())
                .map_err(|_| transport_failure())?;
            // The enclosing client endpoint closes immediately after this
            // case. Waiting for a peer close here is incorrect: a server
            // child that has already reported the reset may have exited and
            // therefore cannot complete a reciprocal QUIC close exchange.
            Ok(None)
        }
        crate::CandidateCase::DeterministicFragmentedRoundTrip => {
            let (mut send, receive) = connection
                .open_bi()
                .await
                .map_err(|_| transport_failure())?;
            write_fragmented(&mut send, frame).await?;
            send.finish().map_err(|_| transport_failure())?;
            receive_probe_reply(receive, reply_sequence).await
        }
        crate::CandidateCase::TruncatedFrame => {
            let (mut send, mut receive) = connection
                .open_bi()
                .await
                .map_err(|_| transport_failure())?;
            let bytes = truncated_client_bytes(frame)?;
            let prefix = bytes.get(..4).ok_or_else(transport_failure)?;
            send.write_all(prefix)
                .await
                .map_err(|_| transport_failure())?;
            // A private transport-only acknowledgement establishes that the
            // prefix crossed the stream before a partial body is reset.
            let mut acknowledgement = [0_u8; 1];
            receive
                .read_exact(&mut acknowledgement)
                .await
                .map_err(|_| transport_failure())?;
            if acknowledgement != [1] {
                return Err(transport_failure());
            }
            send.write_all(&bytes[4..])
                .await
                .map_err(|_| transport_failure())?;
            // The second transport-local acknowledgement establishes that at
            // least one partial body octet crossed the stream before reset.
            receive
                .read_exact(&mut acknowledgement)
                .await
                .map_err(|_| transport_failure())?;
            if acknowledgement != [2] {
                return Err(transport_failure());
            }
            send.reset(RESET_CODE.into())
                .map_err(|_| transport_failure())?;
            Ok(None)
        }
        crate::CandidateCase::OversizedFrame => {
            let (mut send, _receive) = connection
                .open_bi()
                .await
                .map_err(|_| transport_failure())?;
            send.write_all(&oversized_client_prefix()?)
                .await
                .map_err(|_| transport_failure())?;
            send.finish().map_err(|_| transport_failure())?;
            wait_for_peer_read(&send).await?;
            Ok(None)
        }
        crate::CandidateCase::DisconnectAfterAdmissionBeforeResult => {
            let (mut send, mut receive) = connection
                .open_bi()
                .await
                .map_err(|_| transport_failure())?;
            send.write_all(frame)
                .await
                .map_err(|_| transport_failure())?;
            send.finish().map_err(|_| transport_failure())?;
            // The server canary commits before it actively closes the QUIC
            // connection. A close or reset while reading is the only valid
            // result for this ambiguous-delivery case: no reply bytes exist.
            match receive.read_to_end(MAX_PRIVATE_REPLY_BYTES).await {
                Err(_) => Ok(None),
                Ok(_) => Err(transport_failure()),
            }
        }
        crate::CandidateCase::DuplicateAcrossReconnect
        | crate::CandidateCase::ObserverSafeEvidence => {
            write_complete_frame(connection, frame, reply_sequence).await
        }
        crate::CandidateCase::TamperedSemanticAdmissionReference => {
            write_complete_frame(connection, frame, reply_sequence).await
        }
    }
}

async fn write_complete_frame(
    connection: Connection,
    frame: &[u8],
    reply_sequence: usize,
) -> Result<Option<ClientChildProbeReplyReceipt>, CandidateExecutionError> {
    let (mut send, receive) = connection
        .open_bi()
        .await
        .map_err(|_| transport_failure())?;
    send.write_all(frame)
        .await
        .map_err(|_| transport_failure())?;
    send.finish().map_err(|_| transport_failure())?;
    receive_probe_reply(receive, reply_sequence).await
}

async fn receive_probe_reply(
    mut receive: RecvStream,
    sequence: usize,
) -> Result<Option<ClientChildProbeReplyReceipt>, CandidateExecutionError> {
    let reply = receive
        .read_to_end(MAX_PRIVATE_REPLY_BYTES)
        .await
        .map_err(|_| transport_failure())?;
    if reply.is_empty() {
        return Ok(None);
    }
    ClientChildProbeReplyReceipt::from_received_reply(sequence, &reply)
        .map(Some)
        .ok_or_else(transport_failure)
}

async fn wait_for_peer_read(send: &SendStream) -> Result<(), CandidateExecutionError> {
    send.stopped()
        .await
        .map_err(|_| transport_failure())
        .map(|_| ())
}

async fn read_server_connection(
    case: crate::CandidateCase,
    connection: Connection,
) -> Result<ServerRead, CandidateExecutionError> {
    match case {
        crate::CandidateCase::ConnectWithoutSemanticAdmission => {
            let _ = connection.closed().await;
            Ok(ServerRead::NoFrame)
        }
        crate::CandidateCase::DisconnectBeforeAdmission => match connection.accept_bi().await {
            Ok((_send, mut receive)) => match receive.read(&mut [0_u8; 1]).await {
                Ok(Some(_)) => Err(transport_failure()),
                Ok(None) | Err(_) => Ok(ServerRead::NoFrame),
            },
            Err(_) => Ok(ServerRead::NoFrame),
        },
        crate::CandidateCase::TruncatedFrame => {
            let (mut send, mut receive) = connection
                .accept_bi()
                .await
                .map_err(|_| transport_failure())?;
            match read_capture(&mut receive, 4).await? {
                ReadCapture::Complete(prefix) => {
                    let prefix: [u8; 4] = prefix.try_into().map_err(|_| transport_failure())?;
                    send.write_all(&[1])
                        .await
                        .map_err(|_| transport_failure())?;
                    let declared = usize::try_from(u32::from_be_bytes(prefix))
                        .map_err(|_| transport_failure())?;
                    if declared > MAX_PRIVATE_FRAME_BYTES {
                        return Ok(ServerRead::Oversized(prefix.to_vec()));
                    }
                    let mut first_body = [0_u8; 1];
                    let first_body_length = match receive.read(&mut first_body).await {
                        Ok(Some(length)) => length,
                        Ok(None) | Err(_) => return Ok(ServerRead::Truncated(prefix.to_vec())),
                    };
                    send.write_all(&[2])
                        .await
                        .map_err(|_| transport_failure())?;
                    let mut capture = prefix.to_vec();
                    capture.extend_from_slice(&first_body[..first_body_length]);
                    match read_capture(&mut receive, declared - first_body_length).await? {
                        ReadCapture::Complete(body) => {
                            capture.extend_from_slice(&body);
                            Err(transport_failure())
                        }
                        ReadCapture::EndOfStream(body) => {
                            capture.extend_from_slice(&body);
                            Ok(ServerRead::Truncated(capture))
                        }
                    }
                }
                ReadCapture::EndOfStream(capture) => Ok(ServerRead::Truncated(capture)),
            }
        }
        crate::CandidateCase::OversizedFrame
        | crate::CandidateCase::DisconnectAfterAdmissionBeforeResult => {
            read_bidirectional_frame(connection).await
        }
        crate::CandidateCase::DeterministicFragmentedRoundTrip
        | crate::CandidateCase::DuplicateAcrossReconnect
        | crate::CandidateCase::TamperedSemanticAdmissionReference
        | crate::CandidateCase::ObserverSafeEvidence => read_bidirectional_frame(connection).await,
    }
}

async fn read_bidirectional_frame(
    connection: Connection,
) -> Result<ServerRead, CandidateExecutionError> {
    let (reply_stream, mut receive) = connection
        .accept_bi()
        .await
        .map_err(|_| transport_failure())?;
    match read_complete_frame(&mut receive).await? {
        FrameRead::Complete(frame) => Ok(ServerRead::Complete {
            frame,
            connection,
            reply_stream,
        }),
        FrameRead::Truncated(received_capture) => Ok(ServerRead::Truncated(received_capture)),
        FrameRead::Oversized(received_capture) => Ok(ServerRead::Oversized(received_capture)),
    }
}

async fn read_complete_frame(
    receive: &mut RecvStream,
) -> Result<FrameRead, CandidateExecutionError> {
    let prefix = match read_capture(receive, 4).await? {
        ReadCapture::Complete(prefix) => prefix,
        ReadCapture::EndOfStream(capture) => return Ok(FrameRead::Truncated(capture)),
    };
    let prefix: [u8; 4] = prefix.try_into().map_err(|_| transport_failure())?;
    let declared = usize::try_from(u32::from_be_bytes(prefix)).map_err(|_| transport_failure())?;
    // Validate size before body allocation and before semantic admission.
    if declared > MAX_PRIVATE_FRAME_BYTES {
        return Ok(FrameRead::Oversized(prefix.to_vec()));
    }
    match read_capture(receive, declared).await? {
        ReadCapture::Complete(body) => {
            let mut frame = Vec::with_capacity(4 + body.len());
            frame.extend_from_slice(&prefix);
            frame.extend_from_slice(&body);
            Ok(FrameRead::Complete(frame))
        }
        ReadCapture::EndOfStream(body) => {
            let mut capture = prefix.to_vec();
            capture.extend_from_slice(&body);
            Ok(FrameRead::Truncated(capture))
        }
    }
}

/// Reads only actual received octets. A reset or close returns the accumulated
/// capture; it never substitutes sender input or fills missing bytes.
async fn read_capture(
    receive: &mut RecvStream,
    expected: usize,
) -> Result<ReadCapture, CandidateExecutionError> {
    let mut capture = Vec::with_capacity(expected);
    let mut buffer = [0_u8; 4096];
    while capture.len() < expected {
        let remaining = expected - capture.len();
        let chunk_length = remaining.min(buffer.len());
        let read = receive.read(&mut buffer[..chunk_length]).await;
        let length = match read {
            Ok(Some(length)) => length,
            Ok(None) | Err(_) => return Ok(ReadCapture::EndOfStream(capture)),
        };
        capture.extend_from_slice(&buffer[..length]);
    }
    Ok(ReadCapture::Complete(capture))
}

async fn write_fragmented(
    send: &mut SendStream,
    frame: &[u8],
) -> Result<(), CandidateExecutionError> {
    let first = frame.first_chunk::<1>().ok_or_else(transport_failure)?;
    send.write_all(first)
        .await
        .map_err(|_| transport_failure())?;
    let remainder = &frame[1..];
    let split = remainder.len().min(7);
    if split != 0 {
        send.write_all(&remainder[..split])
            .await
            .map_err(|_| transport_failure())?;
    }
    if split != remainder.len() {
        send.write_all(&remainder[split..])
            .await
            .map_err(|_| transport_failure())?;
    }
    Ok(())
}

fn truncated_client_bytes(frame: &[u8]) -> Result<Vec<u8>, CandidateExecutionError> {
    let body = frame.get(4..).ok_or_else(transport_failure)?;
    if body.len() < 2 {
        return Err(transport_failure());
    }
    let partial_body_length = (body.len() / 2).max(1);
    let mut output = Vec::with_capacity(4 + partial_body_length);
    output.extend_from_slice(&frame[..4]);
    output.extend_from_slice(&body[..partial_body_length]);
    Ok(output)
}

fn oversized_client_prefix() -> Result<Vec<u8>, CandidateExecutionError> {
    let declared = u32::try_from(MAX_PRIVATE_FRAME_BYTES + 1).map_err(|_| transport_failure())?;
    Ok(declared.to_be_bytes().to_vec())
}

fn server_config(
    control: &ChildProcessControl,
) -> Result<quinn::ServerConfig, CandidateExecutionError> {
    let private_key = control
        .server_private_key_der()
        .ok_or_else(transport_failure)?
        .to_vec();
    let mut crypto = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(
            vec![CertificateDer::from(control.certificate_der().to_vec())],
            PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(private_key)),
        )
        .map_err(|_| transport_failure())?;
    crypto.alpn_protocols = vec![PROBE_ALPN.to_vec()];
    let mut configuration = quinn::ServerConfig::with_crypto(Arc::new(
        QuicServerConfig::try_from(crypto).map_err(|_| transport_failure())?,
    ));
    let transport = Arc::get_mut(&mut configuration.transport).ok_or_else(transport_failure)?;
    // The server admits the one client-originated reliable bidirectional
    // stream needed by a case. Uni streams and datagrams are disabled rather
    // than merely left unused by the implementation.
    transport
        .max_concurrent_bidi_streams(1_u32.into())
        .max_concurrent_uni_streams(0_u32.into())
        .datagram_receive_buffer_size(None)
        .datagram_send_buffer_size(0);
    Ok(configuration)
}

fn client_config(
    control: &ChildProcessControl,
) -> Result<quinn::ClientConfig, CandidateExecutionError> {
    let mut roots = RootCertStore::empty();
    roots
        .add(CertificateDer::from(control.certificate_der().to_vec()))
        .map_err(|_| transport_failure())?;
    let mut crypto = ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();
    crypto.alpn_protocols = vec![PROBE_ALPN.to_vec()];
    let mut configuration = quinn::ClientConfig::new(Arc::new(
        QuicClientConfig::try_from(crypto).map_err(|_| transport_failure())?,
    ));
    let mut transport = quinn::TransportConfig::default();
    // No server-originated stream is needed: the one reverse acknowledgement
    // for truncation rides on the same client-originated bidirectional stream.
    transport
        .max_concurrent_bidi_streams(0_u32.into())
        .max_concurrent_uni_streams(0_u32.into())
        .datagram_receive_buffer_size(None)
        .datagram_send_buffer_size(0);
    configuration.transport_config(Arc::new(transport));
    Ok(configuration)
}

fn loopback_unspecified() -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], 0))
}

fn emit_ready(endpoint: String) -> Result<(), CandidateExecutionError> {
    let event = ChildProcessEvent::Ready { endpoint };
    serde_json::to_writer(io::stdout(), &event).map_err(|_| transport_failure())?;
    use std::io::Write as _;
    io::stdout()
        .write_all(b"\n")
        .and_then(|_| io::stdout().flush())
        .map_err(|_| transport_failure())
}

fn transport_failure() -> CandidateExecutionError {
    CandidateExecutionError::new(CandidateExecutionErrorKind::TransportFailed)
}

enum ServerRead {
    Complete {
        frame: Vec<u8>,
        connection: Connection,
        reply_stream: SendStream,
    },
    NoFrame,
    Truncated(Vec<u8>),
    Oversized(Vec<u8>),
}

enum FrameRead {
    Complete(Vec<u8>),
    Truncated(Vec<u8>),
    Oversized(Vec<u8>),
}

enum ReadCapture {
    Complete(Vec<u8>),
    EndOfStream(Vec<u8>),
}
